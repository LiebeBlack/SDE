//! Mappers compartidos para conversión entre entidades del dominio y filas de base de datos
//! Elimina duplicación de código entre repositorios

use escuela_core::domain::usuario::{Usuario, UsuarioId, Rol};
use escuela_core::domain::documento::{Documento, DocumentoId, CategoriaDocumento, HashArchivo};
use escuela_core::domain::expediente::{ExpedienteDocente, ExpedienteId, EstadoExpediente};
use escuela_shared::{AppResult, AppError, Email, Cedula};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Mapper para convertir filas de usuarios a entidades del dominio
pub fn map_usuario_row(row: UsuarioRow) -> AppResult<Usuario> {
    Ok(Usuario {
        id: UsuarioId::from_uuid(parse_uuid(&row.id)?),
        nombre: row.nombre,
        apellido: row.apellido,
        email: Email::new(row.email)?,
        cedula: Cedula::new(row.cedula)?,
        password_hash: row.password_hash,
        rol: Rol::from_str(&row.rol)?,
        telefono: row.telefono,
        activo: row.activo == 1,
        creado_en: parse_datetime(&row.creado_en)?,
        actualizado_en: parse_datetime(&row.actualizado_en)?,
        ultimo_acceso: row.ultimo_acceso.and_then(|s| parse_datetime(&s).ok()),
    })
}

/// Mapper para convertir filas de expedientes a entidades del dominio
pub fn map_expediente_row(row: ExpedienteRow) -> AppResult<ExpedienteDocente> {
    Ok(ExpedienteDocente {
        id: ExpedienteId::from_uuid(parse_uuid(&row.id)?),
        nombres: row.nombres,
        apellidos: row.apellidos,
        cedula: Cedula::new(row.cedula)?,
        email: row.email,
        telefono: row.telefono,
        direccion: row.direccion,
        fecha_nacimiento: row.fecha_nacimiento.and_then(|s| parse_datetime(&s).ok()),
        nacionalidad: row.nacionalidad,
        estado_civil: row.estado_civil,
        estado: EstadoExpediente::from_str(&row.estado)?,
        documentos: Vec::new(), // Se llenará con carga eager
        creado_por: row.creado_por.and_then(|s| parse_uuid(&s).ok()).map(UsuarioId::from_uuid),
        creado_en: parse_datetime(&row.creado_en)?,
        actualizado_por: row.actualizado_por.and_then(|s| parse_uuid(&s).ok()).map(UsuarioId::from_uuid),
        actualizado_en: row.actualizado_en.and_then(|s| parse_datetime(&s).ok()),
        observaciones: row.observaciones,
    })
}

/// Mapper para convertir filas de documentos a entidades del dominio
pub fn map_documento_row(row: DocumentoRow) -> AppResult<Documento> {
    Ok(Documento {
        id: DocumentoId::from_uuid(parse_uuid(&row.id)?),
        nombre_archivo: row.nombre_archivo,
        categoria: CategoriaDocumento::from_str(&row.categoria)?,
        hash: HashArchivo::from_string(row.hash)?,
        ruta_local: row.ruta_local,
        tamaño_bytes: row.tamaño_bytes.map(|b| b as u64),
        tipo_mime: row.tipo_mime,
        foliado: row.foliado == 1,
        fecha_foliado: row.fecha_foliado.and_then(|s| parse_datetime(&s).ok()),
        creado_en: parse_datetime(&row.creado_en)?,
        actualizado_en: row.actualizado_en.and_then(|s| parse_datetime(&s).ok()),
        observaciones: row.observaciones,
    })
}

/// Función auxiliar para parsear UUID de forma segura

fn parse_uuid(s: &str) -> Result<Uuid, AppError> {
    Uuid::parse_str(s).map_err(|e| AppError::InternalError(format!("Error al parsear UUID: {}", e)))
}

/// Función auxiliar para parsear DateTime RFC3339 de forma segura
fn parse_datetime(s: &str) -> Result<DateTime<Utc>, AppError> {
    DateTime::parse_from_rfc3339(s)
        .map_err(|e| AppError::InternalError(format!("Error al parsear datetime: {}", e)))
        .map(|dt| dt.with_timezone(&Utc))
}

// Estructuras de fila para mapeo
#[derive(sqlx::FromRow)]
pub struct UsuarioRow {
    pub id: String,
    pub nombre: String,
    pub apellido: String,
    pub email: String,
    pub cedula: String,
    pub password_hash: String,
    pub rol: String,
    pub telefono: Option<String>,
    pub activo: i32,
    pub creado_en: String,
    pub actualizado_en: String,
    pub ultimo_acceso: Option<String>,
}

#[derive(sqlx::FromRow)]
pub struct ExpedienteRow {
    pub id: String,
    pub nombres: String,
    pub apellidos: String,
    pub cedula: String,
    pub email: String,
    pub telefono: Option<String>,
    pub direccion: Option<String>,
    pub fecha_nacimiento: Option<String>,
    pub nacionalidad: Option<String>,
    pub estado_civil: Option<String>,
    pub estado: String,
    pub creado_por: Option<String>,
    pub creado_en: String,
    pub actualizado_por: Option<String>,
    pub actualizado_en: Option<String>,
    pub observaciones: Option<String>,
}

#[derive(sqlx::FromRow)]
pub struct DocumentoRow {
    pub id: String,
    pub expediente_id: String,
    pub nombre_archivo: String,
    pub categoria: String,
    pub hash: String,
    pub ruta_local: String,
    pub tamaño_bytes: Option<i64>,
    pub tipo_mime: Option<String>,
    pub foliado: i32,
    pub fecha_foliado: Option<String>,
    pub creado_en: String,
    pub actualizado_en: Option<String>,
    pub observaciones: Option<String>,
}

/// Estructura combinada para JOIN de expedientes con documentos
#[derive(sqlx::FromRow)]
pub struct ExpedienteWithDocumentosRow {
    // Campos de expediente
    pub id: String,
    pub nombres: String,
    pub apellidos: String,
    pub cedula: String,
    pub email: String,
    pub telefono: Option<String>,
    pub direccion: Option<String>,
    pub fecha_nacimiento: Option<String>,
    pub nacionalidad: Option<String>,
    pub estado_civil: Option<String>,
    pub estado: String,
    pub creado_por: Option<String>,
    pub creado_en: String,
    pub actualizado_por: Option<String>,
    pub actualizado_en: Option<String>,
    pub observaciones: Option<String>,
    // Campos de documento (pueden ser NULL si no tiene documentos)
    pub documento_id: Option<String>,
    pub documento_nombre_archivo: Option<String>,
    pub documento_categoria: Option<String>,
    pub documento_hash: Option<String>,
    pub documento_ruta_local: Option<String>,
    pub documento_tamaño_bytes: Option<i64>,
    pub documento_tipo_mime: Option<String>,
    pub documento_foliado: Option<i32>,
    pub documento_fecha_foliado: Option<String>,
    pub documento_creado_en: Option<String>,
    pub documento_actualizado_en: Option<String>,
    pub documento_observaciones: Option<String>,
}
