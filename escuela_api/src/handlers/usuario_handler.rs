use axum::{
    extract::State,
    Json,
    response::IntoResponse,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use escuela_core::domain::usuario::{Usuario, Rol};
use escuela_shared::{AppResult, AppError, Email};
use crate::state::AppState;
use escuela_core::security::rbac::{require_permission, Action, Resource};
use escuela_storage::audit::AccionAuditoria;

#[derive(Debug, Deserialize)]
pub struct CrearUsuarioRequest {
    pub nombre: String,
    pub apellido: String,
    pub cedula: String,
    pub email: String,
    pub rol: String,
    pub password: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UsuarioResponse {
    pub id: String,
    pub nombre: String,
    pub apellido: String,
    pub cedula: String,
    pub email: String,
    pub rol: String,
    pub activo: bool,
    pub ultimo_acceso: Option<String>,
    pub creado_en: String,
}

impl From<Usuario> for UsuarioResponse {
    fn from(usuario: Usuario) -> Self {
        UsuarioResponse {
            id: usuario.id.as_uuid().to_string(),
            nombre: usuario.nombre,
            apellido: usuario.apellido,
            cedula: usuario.cedula.as_str().to_string(),
            email: usuario.email.as_str().to_string(),
            rol: usuario.rol.as_str().to_string(),
            activo: usuario.activo,
            ultimo_acceso: usuario.ultimo_acceso.map(|d| d.to_rfc3339()),
            creado_en: usuario.creado_en.to_rfc3339(),
        }
    }
}

pub async fn crear_usuario(
    State(state): State<AppState>,
    auth_user: Usuario,
    Json(req): Json<CrearUsuarioRequest>,
) -> AppResult<impl IntoResponse> {
    require_permission(&auth_user, Action::Write, Resource::Usuario)?;

    let cedula = escuela_shared::Cedula::new(req.cedula.clone())?;
    let email = Email::new(req.email.clone())?;
    let rol = Rol::from_str(&req.rol)?;
    
    // Hash password (por defecto usar cedula si no mandan password)
    let password_plain = req.password.unwrap_or_else(|| req.cedula.clone());
    let password_hash = escuela_core::security::crypto::hash_password(&password_plain)?;

    let usuario = Usuario::nuevo(
        req.nombre.clone(),
        req.apellido.clone(),
        email,
        cedula,
        password_hash,
        rol,
        None, // telefono no implementado en request aún
    )?;

    state.usuario_repo.crear(&usuario).await?;

    let _ = state.audit_service.registrar_accion(
        Some(auth_user.id.as_uuid().to_string()),
        AccionAuditoria::BusquedaAvanzada, // Como no hay un enum para Usuarios, usamos uno genérico
        format!("Creación de usuario: {} {}", req.nombre, req.apellido),
        None,
        None,
    ).await;

    let response = UsuarioResponse::from(usuario);
    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn listar_usuarios(
    State(state): State<AppState>,
    auth_user: Usuario,
) -> AppResult<impl IntoResponse> {
    require_permission(&auth_user, Action::Read, Resource::Usuario)?;

    let usuarios = state.usuario_repo.listar().await?;
    let response: Vec<UsuarioResponse> = usuarios
        .into_iter()
        .map(UsuarioResponse::from)
        .collect();
    
    Ok(Json(response))
}
