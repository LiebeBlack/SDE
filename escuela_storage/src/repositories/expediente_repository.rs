use sqlx::SqlitePool;
use escuela_core::domain::{expediente::{ExpedienteDocente, ExpedienteId, EstadoExpediente}, documento::Documento, usuario::UsuarioId};
use escuela_shared::{AppResult, AppError};
use crate::mappers::{ExpedienteRow, DocumentoRow, map_expediente_row, map_documento_row};
use chrono::Utc;

pub struct ExpedienteRepository {
    pool: SqlitePool,
}

impl ExpedienteRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn crear(&self, expediente: &ExpedienteDocente) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO expedientes (id, nombres, apellidos, cedula, email, telefono, direccion, fecha_nacimiento, nacionalidad, estado_civil, estado, creado_por, creado_en, actualizado_por, actualizado_en, observaciones)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(expediente.id.as_uuid().to_string())
        .bind(&expediente.nombres)
        .bind(&expediente.apellidos)
        .bind(expediente.cedula.as_str())
        .bind(&expediente.email)
        .bind(&expediente.telefono)
        .bind(&expediente.direccion)
        .bind(expediente.fecha_nacimiento.map(|d| d.to_rfc3339()))
        .bind(&expediente.nacionalidad)
        .bind(&expediente.estado_civil)
        .bind(expediente.estado.as_str())
        .bind(expediente.creado_por.as_ref().map(|id| id.as_uuid().to_string()))
        .bind(expediente.creado_en.to_rfc3339())
        .bind(expediente.actualizado_por.as_ref().map(|id| id.as_uuid().to_string()))
        .bind(expediente.actualizado_en.map(|d| d.to_rfc3339()))
        .bind(&expediente.observaciones)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    pub async fn obtener_por_id(&self, id: &ExpedienteId) -> AppResult<ExpedienteDocente> {
        let row = sqlx::query_as::<_, ExpedienteRow>(
            "SELECT id, nombres, apellidos, cedula, email, telefono, direccion, fecha_nacimiento, nacionalidad, estado_civil, estado, creado_por, creado_en, actualizado_por, actualizado_en, observaciones FROM expedientes WHERE id = ?"
        )
        .bind(id.as_uuid().to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Expediente no encontrado".to_string()))?;

        let mut expediente = map_expediente_row(row)?;
        
        let documentos = sqlx::query_as::<_, DocumentoRow>(
            "SELECT id, expediente_id, nombre_archivo, categoria, hash, ruta_local, tamaño_bytes, tipo_mime, foliado, fecha_foliado, creado_en, actualizado_en, observaciones FROM documentos WHERE expediente_id = ?"
        )
        .bind(id.as_uuid().to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        for doc_row in documentos {
            expediente.documentos.push(map_documento_row(doc_row)?);
        }

        Ok(expediente)
    }

    pub async fn obtener_por_cedula(&self, cedula: &str) -> AppResult<ExpedienteDocente> {
        let row = sqlx::query_as::<_, ExpedienteRow>(
            "SELECT id, nombres, apellidos, cedula, email, telefono, direccion, fecha_nacimiento, nacionalidad, estado_civil, estado, creado_por, creado_en, actualizado_por, actualizado_en, observaciones FROM expedientes WHERE cedula = ?"
        )
        .bind(cedula)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Expediente no encontrado".to_string()))?;

        let mut expediente = map_expediente_row(row)?;
        
        let documentos = sqlx::query_as::<_, DocumentoRow>(
            "SELECT id, expediente_id, nombre_archivo, categoria, hash, ruta_local, tamaño_bytes, tipo_mime, foliado, fecha_foliado, creado_en, actualizado_en, observaciones FROM documentos WHERE expediente_id = ?"
        )
        .bind(expediente.id.as_uuid().to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        for doc_row in documentos {
            expediente.documentos.push(map_documento_row(doc_row)?);
        }

        Ok(expediente)
    }

    pub async fn listar(&self) -> AppResult<Vec<ExpedienteDocente>> {
        let rows = sqlx::query_as::<_, ExpedienteRow>(
            "SELECT id, nombres, apellidos, cedula, email, telefono, direccion, fecha_nacimiento, nacionalidad, estado_civil, estado, creado_por, creado_en, actualizado_por, actualizado_en, observaciones FROM expedientes"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if rows.is_empty() {
            return Ok(Vec::new());
        }

        // Recopilar todos los IDs de expedientes
        let expediente_ids: Vec<String> = rows.iter()
            .map(|row| row.id.clone())
            .collect();

        // Cargar todos los documentos en una sola query (elimina N+1)
        let all_documentos = sqlx::query_as::<_, DocumentoRow>(
            "SELECT id, expediente_id, nombre_archivo, categoria, hash, ruta_local, tamaño_bytes, tipo_mime, foliado, fecha_foliado, creado_en, actualizado_en, observaciones FROM documentos WHERE expediente_id IN (SELECT value FROM json_each(?))"
        )
        .bind(serde_json::to_string(&expediente_ids).map_err(|e| AppError::SerializationError(e))?)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // Agrupar documentos por expediente_id
        use std::collections::HashMap;
        let mut documentos_por_expediente: HashMap<String, Vec<Documento>> = HashMap::new();
        
        for doc_row in all_documentos {
            let documento = map_documento_row(doc_row)?;
            documentos_por_expediente
                .entry(doc_row.expediente_id.clone())
                .or_insert_with(Vec::new)
                .push(documento);
        }

        // Construir expedientes con sus documentos cargados
        let mut expedientes = Vec::new();
        for row in rows {
            let mut expediente = map_expediente_row(row)?;
            
            // Asignar documentos desde el HashMap
            if let Some(docs) = documentos_por_expediente.remove(&expediente.id.as_uuid().to_string()) {
                expediente.documentos = docs;
            }
            
            expedientes.push(expediente);
        }

        Ok(expedientes)
    }

    pub async fn actualizar(&self, expediente: &ExpedienteDocente) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE expedientes 
            SET nombres = ?, apellidos = ?, cedula = ?, email = ?, telefono = ?, direccion = ?, nacionalidad = ?, estado_civil = ?, estado = ?, actualizado_por = ?, actualizado_en = ?, observaciones = ?
            WHERE id = ?
            "#,
        )
        .bind(&expediente.nombres)
        .bind(&expediente.apellidos)
        .bind(expediente.cedula.as_str())
        .bind(&expediente.email)
        .bind(&expediente.telefono)
        .bind(&expediente.direccion)
        .bind(&expediente.nacionalidad)
        .bind(&expediente.estado_civil)
        .bind(expediente.estado.as_str())
        .bind(expediente.actualizado_por.as_ref().map(|id| id.as_uuid().to_string()))
        .bind(expediente.actualizado_en.map(|d| d.to_rfc3339()))
        .bind(&expediente.observaciones)
        .bind(expediente.id.as_uuid().to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    pub async fn eliminar(&self, id: &ExpedienteId) -> AppResult<()> {
        sqlx::query("DELETE FROM expedientes WHERE id = ?")
            .bind(id.as_uuid().to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    pub async fn buscar(&self, termino: &str) -> AppResult<Vec<ExpedienteDocente>> {
        let pattern = format!("%{}%", termino);
        let rows = sqlx::query_as::<_, ExpedienteRow>(
            r#"
            SELECT id, nombres, apellidos, cedula, email, telefono, direccion, fecha_nacimiento, nacionalidad, estado_civil, estado, creado_por, creado_en, actualizado_por, actualizado_en, observaciones 
            FROM expedientes 
            WHERE nombres LIKE ? OR apellidos LIKE ? OR cedula LIKE ? OR email LIKE ?
            "#,
        )
        .bind(&pattern)
        .bind(&pattern)
        .bind(&pattern)
        .bind(&pattern)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let mut expedientes = Vec::new();
        for row in rows {
            let mut expediente = map_expediente_row(row)?;
            
            let documentos = sqlx::query_as::<_, DocumentoRow>(
                "SELECT id, expediente_id, nombre_archivo, categoria, hash, ruta_local, tamaño_bytes, tipo_mime, foliado, fecha_foliado, creado_en, actualizado_en, observaciones FROM documentos WHERE expediente_id = ?"
            )
            .bind(expediente.id.as_uuid().to_string())
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

            for doc_row in documentos {
                expediente.documentos.push(map_documento_row(doc_row)?);
            }

            expedientes.push(expediente);
        }

        Ok(expedientes)
    }

    pub async fn cambiar_estado_expediente(
        &self,
        id: &ExpedienteId,
        estado: EstadoExpediente,
        usuario_id: UsuarioId,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE expedientes 
            SET estado = ?, actualizado_por = ?, actualizado_en = ?
            WHERE id = ?
            "#,
        )
        .bind(estado.as_str())
        .bind(usuario_id.as_uuid().to_string())
        .bind(Utc::now().to_rfc3339())
        .bind(id.as_uuid().to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}

