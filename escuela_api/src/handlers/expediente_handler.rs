use axum::{
    extract::{State, Path},
    Json,
    response::IntoResponse,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use escuela_core::domain::expediente::{ExpedienteDocente, EstadoExpediente};
use escuela_core::domain::usuario::UsuarioId;
use escuela_shared::{AppResult, AppError};
use crate::state::AppState;
use escuela_core::domain::usuario::Usuario;
use escuela_core::security::rbac::{require_permission, Action, Resource};
use escuela_storage::audit::AccionAuditoria;

#[derive(Debug, Deserialize)]
pub struct CrearExpedienteRequest {
    pub nombres: String,
    pub apellidos: String,
    pub cedula: String,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telefono: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direccion: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fecha_nacimiento: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nacionalidad: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estado_civil: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creado_por: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ExpedienteResponse {
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
    pub documentos_count: usize,
    pub creado_en: String,
    pub actualizado_en: Option<String>,
}

impl From<ExpedienteDocente> for ExpedienteResponse {
    fn from(expediente: ExpedienteDocente) -> Self {
        let documentos_count = expediente.contar_documentos();
        ExpedienteResponse {
            id: expediente.id.as_uuid().to_string(),
            nombres: expediente.nombres,
            apellidos: expediente.apellidos,
            cedula: expediente.cedula.as_str().to_string(),
            email: expediente.email,
            telefono: expediente.telefono,
            direccion: expediente.direccion,
            fecha_nacimiento: expediente.fecha_nacimiento.map(|d| d.to_rfc3339()),
            nacionalidad: expediente.nacionalidad,
            estado_civil: expediente.estado_civil,
            estado: expediente.estado.as_str().to_string(),
            documentos_count,
            creado_en: expediente.creado_en.to_rfc3339(),
            actualizado_en: expediente.actualizado_en.map(|d| d.to_rfc3339()),
        }
    }
}

pub async fn crear_expediente(
    State(state): State<AppState>,
    auth_user: Usuario,
    Json(req): Json<CrearExpedienteRequest>,
) -> AppResult<impl IntoResponse> {
    require_permission(&auth_user, Action::Write, Resource::Expediente)?;
    let cedula = escuela_shared::Cedula::new(req.cedula.clone())?;
    
    // Validar que no exista un expediente con la misma cédula
    let existing = state.expediente_repo.obtener_por_cedula(&req.cedula).await;
    if existing.is_ok() {
        return Err(AppError::ValidationError("Ya existe un expediente con esta cédula".to_string()));
    }
    
    let fecha_nacimiento = req.fecha_nacimiento
        .and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok())
        .map(|dt| dt.with_timezone(&chrono::Utc));

    let creado_por = req.creado_por
        .and_then(|s| uuid::Uuid::parse_str(&s).ok())
        .map(UsuarioId::from_uuid);

    let expediente = ExpedienteDocente::nuevo(
        req.nombres,
        req.apellidos,
        cedula,
        req.email,
        req.telefono,
        req.direccion,
        fecha_nacimiento,
        req.nacionalidad,
        req.estado_civil,
        creado_por,
    )?;

    state.expediente_repo.crear(&expediente).await?;

    let _ = state.audit_service.registrar_accion(
        Some(auth_user.id.as_uuid().to_string()),
        AccionAuditoria::CreacionExpediente,
        format!("Creación de expediente para Cédula: {}", req.cedula),
        None,
        None,
    ).await;

    let response = ExpedienteResponse::from(expediente);
    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn obtener_expediente(
    State(state): State<AppState>,
    auth_user: Usuario,
    Path(id): Path<String>,
) -> AppResult<impl IntoResponse> {
    require_permission(&auth_user, Action::Read, Resource::Expediente)?;
    let uuid = uuid::Uuid::parse_str(&id)
        .map_err(|_| AppError::ValidationError("ID inválido".to_string()))?;
    let expediente_id = escuela_core::domain::expediente::ExpedienteId::from_uuid(uuid);
    
    let expediente = state.expediente_repo.obtener_por_id(&expediente_id).await?;

    let _ = state.audit_service.registrar_accion(
        Some(auth_user.id.as_uuid().to_string()),
        AccionAuditoria::ConsultaExpediente,
        format!("Consulta de expediente ID: {}", id),
        None,
        None,
    ).await;

    let response = ExpedienteResponse::from(expediente);
    
    Ok(Json(response))
}

pub async fn obtener_expediente_por_cedula(
    State(state): State<AppState>,
    auth_user: Usuario,
    Path(cedula): Path<String>,
) -> AppResult<impl IntoResponse> {
    require_permission(&auth_user, Action::Read, Resource::Expediente)?;
    let expediente = state.expediente_repo.obtener_por_cedula(&cedula).await?;

    let _ = state.audit_service.registrar_accion(
        Some(auth_user.id.as_uuid().to_string()),
        AccionAuditoria::ConsultaExpediente,
        format!("Consulta de expediente por Cédula: {}", cedula),
        None,
        None,
    ).await;

    let response = ExpedienteResponse::from(expediente);
    
    Ok(Json(response))
}

pub async fn listar_expedientes(
    State(state): State<AppState>,
    auth_user: Usuario,
) -> AppResult<impl IntoResponse> {
    require_permission(&auth_user, Action::Read, Resource::Expediente)?;
    let expedientes = state.expediente_repo.listar().await?;

    let _ = state.audit_service.registrar_accion(
        Some(auth_user.id.as_uuid().to_string()),
        AccionAuditoria::ConsultaExpediente,
        "Listado general de expedientes".to_string(),
        None,
        None,
    ).await;

    let response: Vec<ExpedienteResponse> = expedientes
        .into_iter()
        .map(ExpedienteResponse::from)
        .collect();
    
    Ok(Json(response))
}

pub async fn buscar_expedientes(
    State(state): State<AppState>,
    auth_user: Usuario,
    Path(termino): Path<String>,
) -> AppResult<impl IntoResponse> {
    require_permission(&auth_user, Action::Read, Resource::Expediente)?;
    let expedientes = state.expediente_repo.buscar(&termino).await?;

    let _ = state.audit_service.registrar_accion(
        Some(auth_user.id.as_uuid().to_string()),
        AccionAuditoria::ConsultaExpediente,
        format!("Búsqueda simple de expedientes, término: {}", termino),
        None,
        None,
    ).await;

    let response: Vec<ExpedienteResponse> = expedientes
        .into_iter()
        .map(ExpedienteResponse::from)
        .collect();
    
    Ok(Json(response))
}

#[derive(Debug, Deserialize)]
pub struct CambiarEstadoRequest {
    pub estado: String,
}

pub async fn cambiar_estado(
    State(state): State<AppState>,
    auth_user: Usuario,
    Path(id): Path<String>,
    Json(req): Json<CambiarEstadoRequest>,
) -> AppResult<impl IntoResponse> {
    require_permission(&auth_user, Action::Modify, Resource::Expediente)?;
    let uuid = uuid::Uuid::parse_str(&id)
        .map_err(|_| AppError::ValidationError("ID inválido".to_string()))?;
    let expediente_id = escuela_core::domain::expediente::ExpedienteId::from_uuid(uuid);
    
    let estado = EstadoExpediente::from_str(&req.estado)?;
    let usuario_id = auth_user.id.clone();
    
    state.expediente_repo.cambiar_estado_expediente(&expediente_id, estado, usuario_id).await?;
    
    let _ = state.audit_service.registrar_accion(
        Some(auth_user.id.as_uuid().to_string()),
        AccionAuditoria::CambioEstadoExpediente,
        format!("Cambio de estado a {} para expediente ID: {}", req.estado, id),
        None,
        None,
    ).await;

    Ok(StatusCode::NO_CONTENT)
}
