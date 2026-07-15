//! Módulo de sanitización de inputs para prevenir ataques de inyección y otros vectores de seguridad

use unicode_normalization::UnicodeNormalization;

/// Sanitiza una cadena de texto eliminando caracteres peligrosos
pub fn sanitize_string(input: &str) -> String {
    // Normalizar Unicode para prevenir homograph attacks
    let normalized = input.nfc().collect::<String>();
    
    // Eliminar caracteres de control excepto espacio, tab, newline
    let sanitized: String = normalized
        .chars()
        .filter(|c| {
            c.is_alphanumeric() || c.is_whitespace() || 
            *c == '-' || *c == '_' || *c == '.' || *c == '@' ||
            *c == '#' || *c == '/' || *c == ':' || *c == ';' ||
            *c == ',' || *c == '(' || *c == ')' || *c == '[' ||
            *c == ']' || *c == '{' || *c == '}' || *c == '+' ||
            *c == '=' || *c == '?' || *c == '&' || *c == '%' ||
            *c == '$' || *c == '€' || *c == '£' || *c == '¥'
        })
        .collect();
    
    // Truncar si es demasiado largo (previene ataques de DoS por strings largos)
    if sanitized.len() > 1000 {
        sanitized.chars().take(1000).collect()
    } else {
        sanitized
    }
}

/// Sanitiza un campo de texto específico para nombres
pub fn sanitize_name(input: &str) -> String {
    let sanitized = sanitize_string(input);
    
    // Solo permitir letras, espacios, apóstrofes y guiones para nombres
    sanitized
        .chars()
        .filter(|c| c.is_alphabetic() || c.is_whitespace() || *c == '\'' || *c == '-')
        .collect()
}

/// Sanitiza un campo de texto específico para cédulas
pub fn sanitize_cedula(input: &str) -> String {
    let sanitized = sanitize_string(input);
    
    // Solo permitir dígitos y guiones para cédulas
    sanitized
        .chars()
        .filter(|c| c.is_ascii_digit() || *c == '-')
        .collect()
}

/// Sanitiza un campo de texto específico para emails
pub fn sanitize_email(input: &str) -> String {
    let sanitized = sanitize_string(input);
    
    // Solo permitir caracteres válidos para emails
    sanitized
        .chars()
        .filter(|c| {
            c.is_alphanumeric() || *c == '@' || *c == '.' || 
            *c == '_' || *c == '-' || *c == '+'
        })
        .collect()
}

/// Sanitiza un campo de texto específico para observaciones o notas
pub fn sanitize_text(input: &str) -> String {
    let sanitized = sanitize_string(input);
    
    // Permitir más caracteres para texto libre pero limitar longitud
    if sanitized.len() > 5000 {
        sanitized.chars().take(5000).collect()
    } else {
        sanitized
    }
}

/// Valida la longitud de un input
pub fn validate_length(input: &str, min: usize, max: usize) -> bool {
    let len = input.chars().count();
    len >= min && len <= max
}

/// Detecta patrones sospechosos de inyección SQL
pub fn detect_sql_injection(input: &str) -> bool {
    let lower = input.to_lowercase();
    
    let patterns = [
        "union select",
        "or 1=1",
        "drop table",
        "delete from",
        "insert into",
        "update set",
        "--",
        "/*",
        "*/",
        "xp_cmdshell",
        "exec(",
        "exec (",
        "script:",
        "javascript:",
        "vbscript:",
        "onload=",
        "onerror=",
        "onclick=",
    ];
    
    for pattern in patterns {
        if lower.contains(pattern) {
            return true;
        }
    }
    
    false
}

/// Detecta patrones sospechosos de XSS
pub fn detect_xss(input: &str) -> bool {
    let lower = input.to_lowercase();
    
    let patterns = [
        "<script",
        "</script>",
        "javascript:",
        "onerror=",
        "onload=",
        "onclick=",
        "onmouseover=",
        "onfocus=",
        "onblur=",
        "onkey",
        "onmouse",
        "eval(",
        "expression(",
        "vbscript:",
        "data:",
        "src=",
        "href=",
        "<iframe",
        "<object",
        "<embed",
        "<form",
        "<input",
        "<img",
    ];
    
    for pattern in patterns {
        if lower.contains(pattern) {
            return true;
        }
    }
    
    false
}

/// Sanitiza y valida un input completo
pub fn sanitize_and_validate(input: &str, field_type: InputType) -> Result<String, String> {
    let sanitized = match field_type {
        InputType::Name => sanitize_name(input),
        InputType::Cedula => sanitize_cedula(input),
        InputType::Email => sanitize_email(input),
        InputType::Text => sanitize_text(input),
        InputType::Generic => sanitize_string(input),
    };
    
    // Detectar inyecciones
    if detect_sql_injection(&sanitized) {
        return Err("Input contiene patrones sospechosos de inyección SQL".to_string());
    }
    
    if detect_xss(&sanitized) {
        return Err("Input contiene patrones sospechosos de XSS".to_string());
    }
    
    Ok(sanitized)
}

#[derive(Debug, Clone, Copy)]
pub enum InputType {
    Name,
    Cedula,
    Email,
    Text,
    Generic,
}
