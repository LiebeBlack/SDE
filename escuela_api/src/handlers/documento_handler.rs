use axum::{
    extract::{State, Path, Multipart},
    Json,
    response::{IntoResponse, Response},
    http::StatusCode,
    body::Body,
};
use serde::Serialize;
use escuela_core::domain::documento::{Documento, CategoriaDocumento};
use escuela_core::domain::usuario::Usuario;
use escuela_shared::AppResult;
use crate::state::AppState;
use escuela_core::security::rbac::{require_permission, Action, Resource};
use escuela_storage::audit::AccionAuditoria;
use std::fs;

#[derive(Debug, Serialize)]
pub struct DocumentoResponse {
    pub id: String,
    pub nombre_archivo: String,
    pub categoria: String,
    pub hash: String,
    pub ruta_local: String,
    pub tamaño_bytes: Option<u64>,
    pub tipo_mime: Option<String>,
    pub foliado: bool,
    pub fecha_foliado: Option<String>,
    pub creado_en: String,
}

impl From<Documento> for DocumentoResponse {
    fn from(documento: Documento) -> Self {
        DocumentoResponse {
            id: documento.id.as_uuid().to_string(),
            nombre_archivo: documento.nombre_archivo,
            categoria: documento.categoria.as_str().to_string(),
            hash: documento.hash.as_str().to_string(),
            ruta_local: documento.ruta_local,
            tamaño_bytes: documento.tamaño_bytes,
            tipo_mime: documento.tipo_mime,
            foliado: documento.foliado,
            fecha_foliado: documento.fecha_foliado.map(|d| d.to_rfc3339()),
            creado_en: documento.creado_en.to_rfc3339(),
        }
    }
}

pub async fn crear_documento(
    State(state): State<AppState>,
    auth_user: Usuario,
    Path(expediente_id): Path<String>,
    mut multipart: Multipart,
) -> AppResult<impl IntoResponse> {
    require_permission(&auth_user, Action::Write, Resource::Documento)?;
    let expediente_uuid = uuid::Uuid::parse_str(&expediente_id)
        .map_err(|_| escuela_shared::AppError::ValidationError("ID de expediente inválido".to_string()))?;
    let expediente_id_obj = escuela_core::domain::expediente::ExpedienteId::from_uuid(expediente_uuid);
    
    let mut nombre_archivo = None;
    let mut categoria = None;
    let mut file_bytes: Option<Vec<u8>> = None;
    let mut observaciones = None;

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        escuela_shared::AppError::InternalError(format!("Error al leer multipart: {}", e))
    })? {
        let name = field.name().unwrap_or("");
        
        match name {
            "nombre_archivo" => {
                nombre_archivo = Some(field.text().await.map_err(|e| {
                    escuela_shared::AppError::InternalError(format!("Error al leer nombre_archivo: {}", e))
                })?);
            }
            "categoria" => {
                categoria = Some(field.text().await.map_err(|e| {
                    escuela_shared::AppError::InternalError(format!("Error al leer categoria: {}", e))
                })?);
            }
            "archivo" => {
                let bytes = field.bytes().await.map_err(|e| {
                    escuela_shared::AppError::InternalError(format!("Error al leer archivo: {}", e))
                })?;
                
                // Validar tamaño máximo de archivo (10MB)
                const MAX_FILE_SIZE: usize = 10 * 1024 * 1024; // 10MB
                if bytes.len() > MAX_FILE_SIZE {
                    return Err(escuela_shared::AppError::ValidationError(
                        format!("El archivo excede el tamaño máximo de 10MB. Tamaño actual: {} bytes", bytes.len())
                    ));
                }
                
                file_bytes = Some(bytes.to_vec());
            }
            "observaciones" => {
                observaciones = Some(field.text().await.map_err(|e| {
                    escuela_shared::AppError::InternalError(format!("Error al leer observaciones: {}", e))
                })?);
            }
            _ => {}
        }
    }

    let nombre_archivo = nombre_archivo.ok_or_else(|| {
        escuela_shared::AppError::ValidationError("nombre_archivo es requerido".to_string())
    })?;
    
    let categoria_str = categoria.ok_or_else(|| {
        escuela_shared::AppError::ValidationError("categoria es requerida".to_string())
    })?;
    let categoria = CategoriaDocumento::from_str(&categoria_str)?;
    
    let file_bytes = file_bytes.ok_or_else(|| {
        escuela_shared::AppError::ValidationError("archivo es requerido".to_string())
    })?;

    let tipo_mime = infer::get(&file_bytes)
        .map(|info| info.mime_type().to_string())
        .or_else(|| {
            mime_guess::from_path(&nombre_archivo).first_raw().map(|s| s.to_string())
        });

    let archivo_guardado = state.file_storage.guardar_archivo_con_nombre_personalizado(
        &expediente_uuid.to_string(),
        &nombre_archivo,
        &file_bytes,
        Some("doc"),
    )?;

    let nombre_archivo_clone = nombre_archivo.clone();
    let mut documento = Documento::nuevo(
        nombre_archivo,
        categoria,
        archivo_guardado.ruta_local,
        &file_bytes,
        tipo_mime,
    )?;

    if let Some(obs) = observaciones {
        documento.agregar_observaciones(obs);
    }

    state.documento_repo.crear(&documento, &expediente_id_obj).await?;

    let _ = state.audit_service.registrar_accion(
        Some(auth_user.id.as_uuid().to_string()),
        AccionAuditoria::SubidaDocumento,
        format!("Subida de documento '{}' para expediente ID: {}", nombre_archivo_clone, expediente_id),
        None,
        None,
    ).await;

    let response = DocumentoResponse::from(documento);
    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn listar_documentos(
    State(state): State<AppState>,
    auth_user: Usuario,
    Path(expediente_id): Path<String>,
) -> AppResult<impl IntoResponse> {
    require_permission(&auth_user, Action::Read, Resource::Documento)?;
    let expediente_uuid = uuid::Uuid::parse_str(&expediente_id)
        .map_err(|_| escuela_shared::AppError::ValidationError("ID de expediente inválido".to_string()))?;
    let expediente_id_obj = escuela_core::domain::expediente::ExpedienteId::from_uuid(expediente_uuid);
    
    let documentos = state.documento_repo.listar_por_expediente(&expediente_id_obj).await?;

    let _ = state.audit_service.registrar_accion(
        Some(auth_user.id.as_uuid().to_string()),
        AccionAuditoria::ConsultaDocumento,
        format!("Consulta de documentos para expediente ID: {}", expediente_id),
        None,
        None,
    ).await;

    let response: Vec<DocumentoResponse> = documentos
        .into_iter()
        .map(DocumentoResponse::from)
        .collect();
    
    Ok(Json(response))
}

pub async fn foliar_documento(
    State(state): State<AppState>,
    auth_user: Usuario,
    Path((expediente_id, documento_id)): Path<(String, String)>,
) -> AppResult<impl IntoResponse> {
    require_permission(&auth_user, Action::Approve, Resource::Documento)?;
    let documento_uuid = uuid::Uuid::parse_str(&documento_id)
        .map_err(|_| escuela_shared::AppError::ValidationError("ID de documento inválido".to_string()))?;
    let documento_id_obj = escuela_core::domain::documento::DocumentoId::from_uuid(documento_uuid);
    
    state.documento_repo.foliar(&documento_id_obj).await?;

    let _ = state.audit_service.registrar_accion(
        Some(auth_user.id.as_uuid().to_string()),
        AccionAuditoria::FoliadoDocumento,
        format!("Foliado de documento ID: {} (Expediente ID: {})", documento_id, expediente_id),
        None,
        None,
    ).await;
    
    Ok(StatusCode::NO_CONTENT)
}

pub async fn descargar_documento(
    State(state): State<AppState>,
    auth_user: Usuario,
    Path((expediente_id, documento_id)): Path<(String, String)>,
) -> AppResult<Response> {
    require_permission(&auth_user, Action::Read, Resource::Documento)?;
    let documento_uuid = uuid::Uuid::parse_str(&documento_id)
        .map_err(|_| escuela_shared::AppError::ValidationError("ID de documento inválido".to_string()))?;
    
    let expediente_uuid = uuid::Uuid::parse_str(&expediente_id)
        .map_err(|_| escuela_shared::AppError::ValidationError("ID de expediente inválido".to_string()))?;
    let expediente_id_obj = escuela_core::domain::expediente::ExpedienteId::from_uuid(expediente_uuid);
    
    // Obtener el documento a través del expediente para verificar la relación
    let documentos = state.documento_repo.listar_por_expediente(&expediente_id_obj).await?;
    let documento = documentos.into_iter()
        .find(|d| d.id.as_uuid() == documento_uuid)
        .ok_or_else(|| escuela_shared::AppError::NotFound("Documento no encontrado en este expediente".to_string()))?;
    
    // Leer el archivo del sistema de archivos
    let file_bytes = fs::read(&documento.ruta_local)
        .map_err(|e| escuela_shared::AppError::InternalError(format!("Error leyendo archivo: {}", e)))?;
    
    // Obtener el tipo MIME
    let content_type = documento.tipo_mime.as_deref().unwrap_or("application/octet-stream");
    
    let _ = state.audit_service.registrar_accion(
        Some(auth_user.id.as_uuid().to_string()),
        AccionAuditoria::ConsultaDocumento,
        format!("Descarga de documento ID: {} (Expediente ID: {})", documento_id, expediente_id),
        None,
        None,
    ).await;
    
    // Crear respuesta con el archivo
    let headers = [
        ("Content-Type", content_type.to_string()),
        ("Content-Disposition", format!("attachment; filename=\"{}\"", documento.nombre_archivo)),
    ];
    
    Ok((StatusCode::OK, headers, Body::from(file_bytes)).into_response())
}
