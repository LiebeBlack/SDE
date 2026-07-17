use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("El campo '{0}' es obligatorio")]
    CampoObligatorio(String),
    #[error("Matrícula inválida: {0}")]
    MatriculaInvalida(String),
    #[error("Formato de archivo no permitido: {0}")]
    ArchivoNoPermitido(String),
}

pub fn validar_no_vacio(campo: &str, valor: &str) -> Result<(), ValidationError> {
    if valor.trim().is_empty() {
        return Err(ValidationError::CampoObligatorio(campo.to_string()));
    }
    Ok(())
}

pub fn validar_extension_permitida(nombre_archivo: &str) -> Result<(), ValidationError> {
    let permitidas = ["pdf", "png", "jpg", "jpeg"];
    let ext = nombre_archivo
        .rsplit('.')
        .next()
        .unwrap_or("")
        .to_lowercase();
    if permitidas.contains(&ext.as_str()) {
        Ok(())
    } else {
        Err(ValidationError::ArchivoNoPermitido(ext))
    }
}
