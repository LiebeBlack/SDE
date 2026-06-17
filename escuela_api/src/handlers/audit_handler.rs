use axum::{
    extract::{State, Query},
    Json,
    response::IntoResponse,
};
use serde::Deserialize;
use escuela_shared::{AppResult, AppError};
use crate::state::AppState;
use escuela_core::domain::usuario::Usuario;
use escuela_core::security::rbac::{require_permission, Action, Resource};

#[derive(Debug, Deserialize)]
pub struct AuditQuery {
    #[serde(default = "default_limite")]
    pub limite: u32,
    #[serde(default = "default_offset")]
    pub offset: u32,
}

fn default_limite() -> u32 {
    100
}

fn default_offset() -> u32 {
    0
}

pub async fn listar_auditoria(
    State(state): State<AppState>,
    auth_user: Usuario,
    Query(params): Query<AuditQuery>,
) -> AppResult<impl IntoResponse> {
    require_permission(&auth_user, Action::Audit, Resource::Sistema)?;

    let registros = state.audit_service.obtener_historial_completo(params.limite, params.offset).await?;
    
    Ok(Json(registros))
}
