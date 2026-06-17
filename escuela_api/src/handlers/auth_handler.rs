use axum::{
    extract::State,
    Json,
    response::IntoResponse,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, Header, EncodingKey};
use escuela_shared::{AppResult, AppError};
use escuela_core::security::crypto::verify_password;
use crate::state::AppState;
use std::time::{SystemTime, UNIX_EPOCH};
use escuela_storage::audit::AccionAuditoria;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub cedula: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub usuario: UsuarioInfo,
}

#[derive(Debug, Serialize)]
pub struct UsuarioInfo {
    pub id: String,
    pub nombre: String,
    pub apellido: String,
    pub rol: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub rol: String,
    pub exp: usize,
}

// Clave secreta (en producción debe venir de una variable de entorno)
const JWT_SECRET: &[u8] = b"tesis_yoangel_secret_key_2026";

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> AppResult<impl IntoResponse> {
    // buscar por cédula
    let row = sqlx::query_as::<_, (String, String, String, String, String, i32)>(
        "SELECT id, nombre, apellido, rol, password_hash, activo FROM usuarios WHERE cedula = ?"
    )
    .bind(&req.cedula)
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let (id, nombre, apellido, rol, password_hash, activo) = match row {
        Some(r) => r,
        None => return Err(AppError::AuthenticationError("Credenciales inválidas".to_string())),
    };

    if activo == 0 {
        return Err(AppError::AuthenticationError("Usuario inactivo".to_string()));
    }

    let is_valid = verify_password(&req.password, &password_hash)?;
    if !is_valid {
        return Err(AppError::AuthenticationError("Credenciales inválidas".to_string()));
    }

    // registrar acceso
    sqlx::query("UPDATE usuarios SET ultimo_acceso = CURRENT_TIMESTAMP WHERE id = ?")
        .bind(&id)
        .execute(&state.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    // Generar JWT
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize + 24 * 3600; // 24 horas

    let claims = Claims {
        sub: id.clone(),
        rol: rol.clone(),
        exp: expiration,
    };

    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_SECRET))
        .map_err(|e| AppError::InternalError(format!("Error generando token: {}", e)))?;

    let _ = state.audit_service.registrar_accion(
        Some(id.clone()),
        AccionAuditoria::LoginUsuario,
        format!("Inicio de sesión exitoso. Cédula: {}", req.cedula),
        None,
        None,
    ).await;

    let response = LoginResponse {
        token,
        usuario: UsuarioInfo {
            id,
            nombre,
            apellido,
            rol,
        },
    };

    Ok((StatusCode::OK, Json(response)))
}
