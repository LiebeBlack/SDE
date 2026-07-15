use sqlx::SqlitePool;
use escuela_core::domain::{documento::{Documento, DocumentoId, CategoriaDocumento}, expediente::ExpedienteId};
use escuela_shared::{AppResult, AppError};
use crate::mappers::{DocumentoRow, map_documento_row};
use chrono::Utc;

pub struct DocumentoRepository {
    pool: SqlitePool,
}

impl DocumentoRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn crear(&self, documento: &Documento, expediente_id: &ExpedienteId) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO documentos (id, expediente_id, nombre_archivo, categoria, hash, ruta_local, tamaño_bytes, tipo_mime, foliado, fecha_foliado, creado_en, actualizado_en, observaciones)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(documento.id.as_uuid().to_string())
        .bind(expediente_id.as_uuid().to_string())
        .bind(&documento.nombre_archivo)
        .bind(documento.categoria.as_str())
        .bind(documento.hash.as_str())
        .bind(&documento.ruta_local)
        .bind(documento.tamaño_bytes.map(|b| b as i64))
        .bind(&documento.tipo_mime)
        .bind(documento.foliado as i32)
        .bind(documento.fecha_foliado.map(|d| d.to_rfc3339()))
        .bind(documento.creado_en.to_rfc3339())
        .bind(documento.actualizado_en.map(|d| d.to_rfc3339()))
        .bind(&documento.observaciones)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    pub async fn obtener_por_id(&self, id: &DocumentoId) -> AppResult<Documento> {
        let row = sqlx::query_as::<_, DocumentoRow>(
            "SELECT id, expediente_id, nombre_archivo, categoria, hash, ruta_local, tamaño_bytes, tipo_mime, foliado, fecha_foliado, creado_en, actualizado_en, observaciones FROM documentos WHERE id = ?"
        )
        .bind(id.as_uuid().to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Documento no encontrado".to_string()))?;

        map_documento_row(row)
    }

    pub async fn listar_por_expediente(&self, expediente_id: &ExpedienteId) -> AppResult<Vec<Documento>> {
        let rows = sqlx::query_as::<_, DocumentoRow>(
            "SELECT id, expediente_id, nombre_archivo, categoria, hash, ruta_local, tamaño_bytes, tipo_mime, foliado, fecha_foliado, creado_en, actualizado_en, observaciones FROM documentos WHERE expediente_id = ?"
        )
        .bind(expediente_id.as_uuid().to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        rows.into_iter()
            .map(|row| map_documento_row(row))
            .collect()
    }

    pub async fn listar_por_categoria(&self, expediente_id: &ExpedienteId, categoria: &CategoriaDocumento) -> AppResult<Vec<Documento>> {
        let rows = sqlx::query_as::<_, DocumentoRow>(
            "SELECT id, expediente_id, nombre_archivo, categoria, hash, ruta_local, tamaño_bytes, tipo_mime, foliado, fecha_foliado, creado_en, actualizado_en, observaciones FROM documentos WHERE expediente_id = ? AND categoria = ?"
        )
        .bind(expediente_id.as_uuid().to_string())
        .bind(categoria.as_str())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        rows.into_iter()
            .map(|row| map_documento_row(row))
            .collect()
    }

    pub async fn actualizar(&self, documento: &Documento) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE documentos 
            SET nombre_archivo = ?, categoria = ?, hash = ?, ruta_local = ?, tamaño_bytes = ?, tipo_mime = ?, foliado = ?, fecha_foliado = ?, actualizado_en = ?, observaciones = ?
            WHERE id = ?
            "#,
        )
        .bind(&documento.nombre_archivo)
        .bind(documento.categoria.as_str())
        .bind(documento.hash.as_str())
        .bind(&documento.ruta_local)
        .bind(documento.tamaño_bytes.map(|b| b as i64))
        .bind(&documento.tipo_mime)
        .bind(documento.foliado as i32)
        .bind(documento.fecha_foliado.map(|d| d.to_rfc3339()))
        .bind(documento.actualizado_en.map(|d| d.to_rfc3339()))
        .bind(&documento.observaciones)
        .bind(documento.id.as_uuid().to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    pub async fn eliminar(&self, id: &DocumentoId) -> AppResult<()> {
        sqlx::query("DELETE FROM documentos WHERE id = ?")
            .bind(id.as_uuid().to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    pub async fn foliar(&self, id: &DocumentoId) -> AppResult<()> {
        sqlx::query(
            "UPDATE documentos SET foliado = 1, fecha_foliado = ?, actualizado_en = ? WHERE id = ?"
        )
        .bind(Utc::now().to_rfc3339())
        .bind(Utc::now().to_rfc3339())
        .bind(id.as_uuid().to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    pub async fn buscar(&self, termino: &str) -> AppResult<Vec<Documento>> {
        let pattern = format!("%{}%", termino);
        let rows = sqlx::query_as::<_, DocumentoRow>(
            r#"
            SELECT id, expediente_id, nombre_archivo, categoria, hash, ruta_local, tamaño_bytes, tipo_mime, foliado, fecha_foliado, creado_en, actualizado_en, observaciones 
            FROM documentos 
            WHERE nombre_archivo LIKE ? OR categoria LIKE ?
            "#,
        )
        .bind(&pattern)
        .bind(&pattern)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        rows.into_iter()
            .map(|row| map_documento_row(row))
            .collect()
    }

    pub async fn verificar_integridad(&self, id: &DocumentoId, bytes: &[u8]) -> AppResult<bool> {
        let documento = self.obtener_por_id(id).await?;
        Ok(documento.verificar_integridad_archivo(bytes))
    }
}

