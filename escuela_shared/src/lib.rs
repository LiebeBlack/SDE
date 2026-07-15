//! Tipos y utilidades compartidas entre crates
//! Proporciona tipos valor con validación, manejo de errores unificado y funciones de validación comunes

use serde::{Deserialize, Serialize};

pub mod error;
pub mod validation;
pub mod sanitization;

pub use error::{AppError, AppResult};
pub use validation::validate_email;
pub use sanitization::{sanitize_string, sanitize_name, sanitize_cedula, sanitize_email, sanitize_text, sanitize_and_validate, InputType, detect_sql_injection, detect_xss};

/// Tipo valor para representar un email validado
/// Garantiza que el email cumple con el formato válido mediante validación en tiempo de construcción
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Email(String);

impl Email {
    /// Crea un nuevo Email validado
    /// 
    /// # Argumentos
    /// * `email` - String que representa el email a validar
    /// 
    /// # Retorna
    /// * `AppResult<Email>` - Email validado o error de validación
    pub fn new(email: String) -> AppResult<Self> {
        validate_email(&email)?;
        Ok(Email(email))
    }

    /// Retorna el email como referencia a string
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Tipo valor para representar una cédula de identidad validada
/// Acepta formato venezolano (V-12345678, E-12345678, J-12345678, G-12345678) o solo dígitos
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Cedula(String);

impl Cedula {
    /// Crea una nueva Cédula validada
    /// 
    /// # Argumentos
    /// * `cedula` - String que representa la cédula a validar
    /// 
    /// # Retorna
    /// * `AppResult<Cedula>` - Cédula validada o error de validación
    /// 
    /// # Formatos aceptados
    /// - Venezolano con prefijo: V-12345678, E-12345678, J-12345678, G-12345678
    /// - Solo dígitos: 12345678
    pub fn new(cedula: String) -> AppResult<Self> {
        let trimmed = cedula.trim().to_uppercase();
        if trimmed.len() < 5 || trimmed.len() > 20 {
            return Err(AppError::ValidationError("Cédula debe tener entre 5 y 20 caracteres".to_string()));
        }
        
        // Acepta formato venezolano: V-12345678, E-12345678, J-12345678, G-12345678, o solo dígitos
        let digits_part = if let Some(rest) = trimmed.strip_prefix("V-")
            .or_else(|| trimmed.strip_prefix("E-"))
            .or_else(|| trimmed.strip_prefix("J-"))
            .or_else(|| trimmed.strip_prefix("G-"))
        {
            rest
        } else {
            &trimmed
        };
        
        if !digits_part.chars().all(|c| c.is_ascii_digit()) || digits_part.is_empty() {
            return Err(AppError::ValidationError("Cédula debe contener solo dígitos (con prefijo V-/E-/J-/G- opcional)".to_string()));
        }
        Ok(Cedula(trimmed))
    }

    /// Retorna la cédula como referencia a string
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
