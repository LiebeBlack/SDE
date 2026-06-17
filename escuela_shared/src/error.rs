use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use thiserror::Error;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Error de validación: {0}")]
    ValidationError(String),

    #[error("Error de base de datos: {0}")]
    DatabaseError(String),

    #[error("Error de serialización: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Error de IO: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Recurso no encontrado: {0}")]
    NotFound(String),

    #[error("Error de autenticación: {0}")]
    AuthenticationError(String),

    #[error("Error de autorización: {0}")]
    AuthorizationError(String),

    #[error("Error interno del servidor: {0}")]
    InternalError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::AuthenticationError(msg) => (StatusCode::UNAUTHORIZED, msg.clone()),
            AppError::AuthorizationError(msg) => (StatusCode::FORBIDDEN, msg.clone()),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Error interno del servidor".to_string()),
        };

        let body = serde_json::json!({
            "error": message,
            "status": status.as_u16()
        });

        (status, Json(body)).into_response()
    }
}
