pub fn validate_email(email: &str) -> Result<(), crate::AppError> {
    // Validación básica de email: debe tener un '@', partes no vacías,
    // dominio con al menos un '.', y sin espacios.
    let trimmed = email.trim();
    if trimmed.is_empty() {
        return Err(crate::AppError::ValidationError("Email inválido".to_string()));
    }
    let parts: Vec<&str> = trimmed.splitn(2, '@').collect();
    if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
        return Err(crate::AppError::ValidationError("Email inválido".to_string()));
    }
    let domain = parts[1];
    if !domain.contains('.') || domain.starts_with('.') || domain.ends_with('.') {
        return Err(crate::AppError::ValidationError("Email inválido".to_string()));
    }
    if trimmed.contains(char::is_whitespace) {
        return Err(crate::AppError::ValidationError("Email inválido".to_string()));
    }
    Ok(())
}
