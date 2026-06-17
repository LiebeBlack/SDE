use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
    RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde_json::json;
use crate::handlers::auth_handler::Claims;

use crate::state::AppState;
use escuela_core::domain::usuario::{Usuario, UsuarioId};
use uuid::Uuid;

// Ojo: En producción usar variable de entorno. Usamos el mismo que en auth_handler
const JWT_SECRET: &[u8] = b"tesis_yoangel_secret_key_2026";

pub enum AuthError {
    InvalidToken,
    WrongCredentials,
    MissingCredentials,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Credenciales incorrectas"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Token no proporcionado"),
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Token inválido o expirado"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

#[async_trait]
impl FromRequestParts<AppState> for Usuario {
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        // Extraer el token Bearer del header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::MissingCredentials)?;

        // Decodificar y validar token
        let token_data = decode::<Claims>(
            bearer.token(),
            &DecodingKey::from_secret(JWT_SECRET),
            &Validation::default(),
        )
        .map_err(|_| AuthError::InvalidToken)?;

        // Parsear UUID del token
        let uuid = Uuid::parse_str(&token_data.claims.sub).map_err(|_| AuthError::InvalidToken)?;
        let usuario_id = UsuarioId::from_uuid(uuid);

        // Obtener usuario desde la base de datos (validación en tiempo real)
        let usuario = state.usuario_repo.obtener_por_id(&usuario_id).await
            .map_err(|_| AuthError::InvalidToken)?;

        // Verificar si el usuario está activo
        if !usuario.activo {
            return Err(AuthError::InvalidToken);
        }

        Ok(usuario)
    }
}
