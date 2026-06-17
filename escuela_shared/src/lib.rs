use serde::{Deserialize, Serialize};

pub mod error;
pub mod validation;

pub use error::{AppError, AppResult};
pub use validation::validate_email;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Email(String);

impl Email {
    pub fn new(email: String) -> AppResult<Self> {
        validate_email(&email)?;
        Ok(Email(email))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Cedula(String);

impl Cedula {
    pub fn new(cedula: String) -> AppResult<Self> {
        let trimmed = cedula.trim().to_uppercase();
        if trimmed.len() < 5 || trimmed.len() > 20 {
            return Err(AppError::ValidationError("Cédula debe tener entre 5 y 20 caracteres".to_string()));
        }
        // acepta formato venezolano: V-12345678, E-12345678, o solo dígitos
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
            return Err(AppError::ValidationError("Cédula debe contener solo dígitos (con prefijo V-/E- opcional)".to_string()));
        }
        Ok(Cedula(trimmed))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
