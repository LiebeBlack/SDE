use sqlx::SqlitePool;
use escuela_core::domain::{expediente::{ExpedienteDocente, ExpedienteId, EstadoExpediente}, documento::Documento, usuario::UsuarioId};
use escuela_shared::{AppResult, AppError, Cedula};
use chrono::{DateTime, Utc};

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

        let mut expediente = row.to_expediente()?;
        
        let documentos = sqlx::query_as::<_, DocumentoRow>(
            "SELECT id, nombre_archivo, categoria, hash, ruta_local, tamaño_bytes, tipo_mime, foliado, fecha_foliado, creado_en, actualizado_en, observaciones FROM documentos WHERE expediente_id = ?"
        )
        .bind(id.as_uuid().to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        for doc_row in documentos {
            expediente.documentos.push(doc_row.to_documento()?);
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

        let mut expediente = row.to_expediente()?;
        
        let documentos = sqlx::query_as::<_, DocumentoRow>(
            "SELECT id, nombre_archivo, categoria, hash, ruta_local, tamaño_bytes, tipo_mime, foliado, fecha_foliado, creado_en, actualizado_en, observaciones FROM documentos WHERE expediente_id = ?"
        )
        .bind(expediente.id.as_uuid().to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        for doc_row in documentos {
            expediente.documentos.push(doc_row.to_documento()?);
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

        let mut expedientes = Vec::new();
        for row in rows {
            let mut expediente = row.to_expediente()?;
            
            let documentos = sqlx::query_as::<_, DocumentoRow>(
                "SELECT id, nombre_archivo, categoria, hash, ruta_local, tamaño_bytes, tipo_mime, foliado, fecha_foliado, creado_en, actualizado_en, observaciones FROM documentos WHERE expediente_id = ?"
            )
            .bind(expediente.id.as_uuid().to_string())
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

            for doc_row in documentos {
                expediente.documentos.push(doc_row.to_documento()?);
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
            let mut expediente = row.to_expediente()?;
            
            let documentos = sqlx::query_as::<_, DocumentoRow>(
                "SELECT id, nombre_archivo, categoria, hash, ruta_local, tamaño_bytes, tipo_mime, foliado, fecha_foliado, creado_en, actualizado_en, observaciones FROM documentos WHERE expediente_id = ?"
            )
            .bind(expediente.id.as_uuid().to_string())
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

            for doc_row in documentos {
                expediente.documentos.push(doc_row.to_documento()?);
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

#[derive(sqlx::FromRow)]
struct ExpedienteRow {
    id: String,
    nombres: String,
    apellidos: String,
    cedula: String,
    email: String,
    telefono: Option<String>,
    direccion: Option<String>,
    fecha_nacimiento: Option<String>,
    nacionalidad: Option<String>,
    estado_civil: Option<String>,
    estado: String,
    creado_por: Option<String>,
    creado_en: String,
    actualizado_por: Option<String>,
    actualizado_en: Option<String>,
    observaciones: Option<String>,
}

impl ExpedienteRow {
    fn to_expediente(self) -> AppResult<ExpedienteDocente> {
        Ok(ExpedienteDocente {
            id: ExpedienteId::from_uuid(uuid::Uuid::parse_str(&self.id).map_err(|e| AppError::InternalError(e.to_string()))?),
            nombres: self.nombres,
            apellidos: self.apellidos,
            cedula: Cedula::new(self.cedula)?,
            email: self.email,
            telefono: self.telefono,
            direccion: self.direccion,
            fecha_nacimiento: self.fecha_nacimiento
                .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
                .map(|dt| dt.with_timezone(&Utc)),
            nacionalidad: self.nacionalidad,
            estado_civil: self.estado_civil,
            estado: EstadoExpediente::from_str(&self.estado)?,
            documentos: Vec::new(),
            creado_por: self.creado_por
                .and_then(|s| uuid::Uuid::parse_str(&s).ok())
                .map(UsuarioId::from_uuid),
            creado_en: DateTime::parse_from_rfc3339(&self.creado_en)
                .map_err(|e| AppError::InternalError(e.to_string()))?
                .with_timezone(&Utc),
            actualizado_por: self.actualizado_por
                .and_then(|s| uuid::Uuid::parse_str(&s).ok())
                .map(UsuarioId::from_uuid),
            actualizado_en: self.actualizado_en
                .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
                .map(|dt| dt.with_timezone(&Utc)),
            observaciones: self.observaciones,
        })
    }
}

#[derive(sqlx::FromRow)]
struct DocumentoRow {
    id: String,
    nombre_archivo: String,
    categoria: String,
    hash: String,
    ruta_local: String,
    tamaño_bytes: Option<i64>,
    tipo_mime: Option<String>,
    foliado: i32,
    fecha_foliado: Option<String>,
    creado_en: String,
    actualizado_en: Option<String>,
    observaciones: Option<String>,
}

impl DocumentoRow {
    fn to_documento(self) -> AppResult<Documento> {
        Ok(Documento {
            id: escuela_core::domain::documento::DocumentoId::from_uuid(uuid::Uuid::parse_str(&self.id).map_err(|e| AppError::InternalError(e.to_string()))?),
            nombre_archivo: self.nombre_archivo,
            categoria: escuela_core::domain::documento::CategoriaDocumento::from_str(&self.categoria)?,
            hash: escuela_core::domain::documento::HashArchivo::from_string(self.hash)?,
            ruta_local: self.ruta_local,
            tamaño_bytes: self.tamaño_bytes.map(|b| b as u64),
            tipo_mime: self.tipo_mime,
            foliado: self.foliado == 1,
            fecha_foliado: self.fecha_foliado
                .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
                .map(|dt| dt.with_timezone(&Utc)),
            creado_en: DateTime::parse_from_rfc3339(&self.creado_en)
                .map_err(|e| AppError::InternalError(e.to_string()))?
                .with_timezone(&Utc),
            actualizado_en: self.actualizado_en
                .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
                .map(|dt| dt.with_timezone(&Utc)),
            observaciones: self.observaciones,
        })
    }
}
