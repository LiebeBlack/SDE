use axum::{
    extract::{State, Query},
    Json,
    response::IntoResponse,
};
use escuela_storage::SearchCriteria;
use escuela_shared::AppResult;
use escuela_core::domain::usuario::Usuario;
use escuela_core::security::rbac::{require_permission, Action, Resource};
use escuela_storage::audit::AccionAuditoria;

#[derive(Debug, serde::Deserialize)]
pub struct BuscarExpedientesQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cedula: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apellido: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nombre: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estado: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foliado: Option<bool>,
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_page_size")]
    pub page_size: u32,
}

fn default_page() -> u32 {
    1
}

fn default_page_size() -> u32 {
    20
}

pub async fn buscar_expedientes_avanzado(
    State(state): State<crate::state::AppState>,
    auth_user: Usuario,
    Query(params): Query<BuscarExpedientesQuery>,
) -> AppResult<impl IntoResponse> {
    require_permission(&auth_user, Action::Read, Resource::Expediente)?;
    let criteria = SearchCriteria {
        cedula: params.cedula,
        apellido: params.apellido,
        nombre: params.nombre,
        categoria_documento: None,
        estado: params.estado,
        foliado: params.foliado,
        page: params.page,
        page_size: params.page_size,
    };

    let result = state.search_service.buscar_expedientes(criteria).await?;

    let _ = state.audit_service.registrar_accion(
        Some(auth_user.id.as_uuid().to_string()),
        AccionAuditoria::BusquedaAvanzada,
        "Búsqueda avanzada de expedientes".to_string(),
        None,
        None,
    ).await;

    Ok(Json(result))
}

#[derive(Debug, serde::Deserialize)]
pub struct BuscarDocumentosQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cedula: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apellido: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nombre: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categoria: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estado: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foliado: Option<bool>,
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_page_size")]
    pub page_size: u32,
}

pub async fn buscar_documentos_avanzado(
    State(state): State<crate::state::AppState>,
    auth_user: Usuario,
    Query(params): Query<BuscarDocumentosQuery>,
) -> AppResult<impl IntoResponse> {
    require_permission(&auth_user, Action::Read, Resource::Documento)?;
    let criteria = SearchCriteria {
        cedula: params.cedula,
        apellido: params.apellido,
        nombre: params.nombre,
        categoria_documento: params.categoria,
        estado: params.estado,
        foliado: params.foliado,
        page: params.page,
        page_size: params.page_size,
    };

    let result = state.search_service.buscar_documentos(criteria).await?;

    let _ = state.audit_service.registrar_accion(
        Some(auth_user.id.as_uuid().to_string()),
        AccionAuditoria::BusquedaAvanzada,
        "Búsqueda avanzada de documentos".to_string(),
        None,
        None,
    ).await;

    Ok(Json(result))
}

#[derive(Debug, serde::Deserialize)]
pub struct BuscarGeneralQuery {
    pub termino: String,
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_page_size")]
    pub page_size: u32,
}

pub async fn buscar_general(
    State(state): State<crate::state::AppState>,
    auth_user: Usuario,
    Query(params): Query<BuscarGeneralQuery>,
) -> AppResult<impl IntoResponse> {
    require_permission(&auth_user, Action::Read, Resource::Expediente)?;
    let result = state.search_service.buscar_por_termino_general(
        &params.termino,
        params.page,
        params.page_size,
    ).await?;

    let _ = state.audit_service.registrar_accion(
        Some(auth_user.id.as_uuid().to_string()),
        AccionAuditoria::BusquedaAvanzada,
        format!("Búsqueda general: {}", params.termino),
        None,
        None,
    ).await;

    Ok(Json(result))
}
