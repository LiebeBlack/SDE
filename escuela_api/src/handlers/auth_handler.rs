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

// Clave secreta desde variable de entorno o valor por defecto
pub fn get_jwt_secret() -> Vec<u8> {
    std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "tesis_yoangel_secret_key_2026".to_string())
        .into_bytes()
}

pub async fn login(
    headers: axum::http::HeaderMap,
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> AppResult<impl IntoResponse> {
    let ip_address = headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .or_else(|| headers.get("x-real-ip").and_then(|v| v.to_str().ok()).map(|s| s.to_string()));
    
    let user_agent = headers
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let rate_limit_key = ip_address.clone().unwrap_or_else(|| req.cedula.clone());
    
    // Validar rate limit
    {
        let mut attempts = state.login_attempts.lock().unwrap();
        let now = chrono::Utc::now();
        attempts.retain(|_, (_, timestamp)| now.signed_duration_since(*timestamp).num_minutes() < 15);
        
        if let Some((count, _)) = attempts.get(&rate_limit_key) {
            if *count >= 5 {
                return Err(AppError::AuthenticationError("Demasiados intentos fallidos. Intente nuevamente en 15 minutos.".to_string()));
            }
        }
    }

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
        None => {
            let _ = state.audit_service.registrar_accion(
                None,
                AccionAuditoria::LoginFallido,
                format!("Intento de login con cédula no registrada: {}", req.cedula),
                ip_address.clone(),
                user_agent.clone(),
            ).await;
            
            let mut attempts = state.login_attempts.lock().map_err(|_| AppError::InternalError("Error al obtener lock de intentos de login".to_string()))?;
            let entry = attempts.entry(rate_limit_key).or_insert((0, chrono::Utc::now()));
            entry.0 += 1;
            entry.1 = chrono::Utc::now();
            
            return Err(AppError::AuthenticationError("Credenciales inválidas".to_string()));
        }
    };

    if activo == 0 {
        let _ = state.audit_service.registrar_accion(
            Some(id.clone()),
            AccionAuditoria::LoginFallido,
            "Intento de login de usuario inactivo".to_string(),
            ip_address.clone(),
            user_agent.clone(),
        ).await;
        return Err(AppError::AuthenticationError("Usuario inactivo".to_string()));
    }

    let is_valid = verify_password(&req.password, &password_hash)?;
    if !is_valid {
        let _ = state.audit_service.registrar_accion(
            Some(id.clone()),
            AccionAuditoria::LoginFallido,
            "Contraseña incorrecta".to_string(),
            ip_address.clone(),
            user_agent.clone(),
        ).await;
        
        let mut attempts = state.login_attempts.lock().unwrap();
        let entry = attempts.entry(rate_limit_key).or_insert((0, chrono::Utc::now()));
        entry.0 += 1;
        entry.1 = chrono::Utc::now();
        
        return Err(AppError::AuthenticationError("Credenciales inválidas".to_string()));
    }

    // Resetear intentos al loguearse con éxito
    {
        let mut attempts = state.login_attempts.lock().map_err(|_| AppError::InternalError("Error al obtener lock de intentos de login".to_string()))?;
        attempts.remove(&rate_limit_key);
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
        .map_err(|_| AppError::InternalError("Error al obtener tiempo del sistema".to_string()))?
        .as_secs() as usize + 24 * 3600; // 24 horas

    let claims = Claims {
        sub: id.clone(),
        rol: rol.clone(),
        exp: expiration,
    };

    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(&get_jwt_secret()))
        .map_err(|e| AppError::InternalError(format!("Error generando token: {}", e)))?;

    let _ = state.audit_service.registrar_accion(
        Some(id.clone()),
        AccionAuditoria::LoginUsuario,
        format!("Inicio de sesión exitoso. Cédula: {}", req.cedula),
        ip_address,
        user_agent,
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

