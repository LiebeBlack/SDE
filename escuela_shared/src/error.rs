//! Manejo de errores unificado para toda la aplicación
//! Define tipos de error personalizados y conversión a respuestas HTTP

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use thiserror::Error;

/// Tipo resultado personalizado que usa AppError como tipo de error
pub type AppResult<T> = Result<T, AppError>;

/// Enumeración de errores de la aplicación
/// Cubre todos los tipos de errores que pueden ocurrir en el sistema
#[derive(Debug, Error)]
pub enum AppError {
    /// Error de validación de datos de entrada
    #[error("Error de validación: {0}")]
    ValidationError(String),

    /// Error en operaciones de base de datos
    #[error("Error de base de datos: {0}")]
    DatabaseError(String),

    /// Error en serialización/deserialización JSON
    #[error("Error de serialización: {0}")]
    SerializationError(#[from] serde_json::Error),

    /// Error de entrada/salida (archivos, red, etc.)
    #[error("Error de IO: {0}")]
    IoError(#[from] std::io::Error),

    /// Recurso solicitado no encontrado
    #[error("Recurso no encontrado: {0}")]
    NotFound(String),

    /// Error en autenticación de usuario
    #[error("Error de autenticación: {0}")]
    AuthenticationError(String),

    /// Error en autorización (permisos insuficientes)
    #[error("Error de autorización: {0}")]
    AuthorizationError(String),

    /// Error interno del servidor (errores inesperados)
    #[error("Error interno del servidor: {0}")]
    InternalError(String),
}

impl IntoResponse for AppError {
    /// Convierte el error de aplicación a una respuesta HTTP
    /// Mapea cada tipo de error a su código de estado HTTP correspondiente
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
