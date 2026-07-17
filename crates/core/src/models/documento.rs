use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A qué tipo de entidad pertenece el documento almacenado.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EntidadTipo {
    Estudiante,
    Familiar,
    Departamento,
    Institucional,
}

impl std::fmt::Display for EntidadTipo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            EntidadTipo::Estudiante => "Estudiante",
            EntidadTipo::Familiar => "Familiar",
            EntidadTipo::Departamento => "Departamento",
            EntidadTipo::Institucional => "Institucional",
        };
        write!(f, "{s}")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TipoDocumento {
    ActaNacimiento,
    FotoPerfil,
    CertificadoMedico,
    CertificadoEstudios,
    Contrato,
    Identificacion,
    Calificaciones,
    Otro(String),
}

impl std::fmt::Display for TipoDocumento {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TipoDocumento::ActaNacimiento => write!(f, "Acta de nacimiento"),
            TipoDocumento::FotoPerfil => write!(f, "Foto de perfil"),
            TipoDocumento::CertificadoMedico => write!(f, "Certificado médico"),
            TipoDocumento::CertificadoEstudios => write!(f, "Certificado de estudios"),
            TipoDocumento::Contrato => write!(f, "Contrato"),
            TipoDocumento::Identificacion => write!(f, "Identificación"),
            TipoDocumento::Calificaciones => write!(f, "Calificaciones"),
            TipoDocumento::Otro(desc) => write!(f, "Otro ({desc})"),
        }
    }
}

/// Registro de un archivo (PDF o imagen) asociado a una entidad.
/// El archivo físico se guarda en disco local (data/documentos/<uuid>.<ext>);
/// aquí solo se guarda la ruta y metadatos, no el binario, para mantener
/// la base de datos liviana.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Documento {
    pub id: Uuid,
    pub entidad_tipo: EntidadTipo,
    pub entidad_id: Uuid,
    pub tipo_documento: TipoDocumento,
    pub nombre_original: String,
    pub ruta_archivo: String,
    pub mime_type: String,
    pub tamano_bytes: i64,
    pub subido_en: DateTime<Utc>,
}

impl Documento {
    pub fn es_pdf(&self) -> bool {
        self.mime_type == "application/pdf"
    }

    pub fn es_imagen(&self) -> bool {
        self.mime_type.starts_with("image/")
    }
}
