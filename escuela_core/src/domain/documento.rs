//! Entidad de Documento del sistema
//! Define los tipos de documentos, categorías y gestión de archivos con integridad mediante hash

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use uuid::Uuid;
use validator::Validate;
use escuela_shared::{AppError, AppResult};

/// Categorías de documentos en el sistema de gestión escolar
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CategoriaDocumento {
    /// Título académico o grado obtenido
    TituloAcademico,
    /// Cédula de identidad o documento de identificación
    CedulaIdentidad,
    /// Contrato laboral o documento de contratación
    ContratoLaboral,
    /// Certificado de antecedentes penales
    CertificadoAntecedentes,
    /// Currículum vitae o hoja de vida
    CurriculumVitae,
    /// Certificado médico o de salud
    CertificadoMedico,
    /// Otros documentos no categorizados
    Otros,
}

impl CategoriaDocumento {
    /// Convierte la categoría a su representación en string (snake_case)
    pub fn as_str(&self) -> &'static str {
        match self {
            CategoriaDocumento::TituloAcademico => "titulo_academico",
            CategoriaDocumento::CedulaIdentidad => "cedula_identidad",
            CategoriaDocumento::ContratoLaboral => "contrato_laboral",
            CategoriaDocumento::CertificadoAntecedentes => "certificado_antecedentes",
            CategoriaDocumento::CurriculumVitae => "curriculum_vitae",
            CategoriaDocumento::CertificadoMedico => "certificado_medico",
            CategoriaDocumento::Otros => "otros",
        }
    }

    /// Crea una CategoriaDocumento desde un string
    /// Acepta formatos con o sin tildes
    pub fn from_str(s: &str) -> Result<Self, AppError> {
        match s.to_lowercase().as_str() {
            "titulo_academico" | "titulo académico" => Ok(CategoriaDocumento::TituloAcademico),
            "cedula_identidad" | "cédula identidad" => Ok(CategoriaDocumento::CedulaIdentidad),
            "contrato_laboral" | "contrato laboral" => Ok(CategoriaDocumento::ContratoLaboral),
            "certificado_antecedentes" | "certificado antecedentes" => Ok(CategoriaDocumento::CertificadoAntecedentes),
            "curriculum_vitae" | "currículum vitae" => Ok(CategoriaDocumento::CurriculumVitae),
            "certificado_medico" | "certificado médico" => Ok(CategoriaDocumento::CertificadoMedico),
            "otros" => Ok(CategoriaDocumento::Otros),
            _ => Err(AppError::ValidationError(format!("Categoría de documento inválida: {}", s))),
        }
    }

    /// Retorna todas las categorías de documento disponibles
    pub fn todas() -> Vec<Self> {
        vec![
            CategoriaDocumento::TituloAcademico,
            CategoriaDocumento::CedulaIdentidad,
            CategoriaDocumento::ContratoLaboral,
            CategoriaDocumento::CertificadoAntecedentes,
            CategoriaDocumento::CurriculumVitae,
            CategoriaDocumento::CertificadoMedico,
            CategoriaDocumento::Otros,
        ]
    }
}

/// Hash SHA-256 de un archivo para garantizar integridad
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HashArchivo(String);

impl HashArchivo {
    /// Calcula el hash SHA-256 de los bytes proporcionados
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(bytes);
        let result = hasher.finalize();
        HashArchivo(hex::encode(result))
    }

    /// Crea un HashArchivo desde un string existente
    /// Valida que tenga 64 caracteres hexadecimales
    pub fn from_string(hash: String) -> AppResult<Self> {
        if hash.len() != 64 {
            return Err(AppError::ValidationError("Hash debe tener 64 caracteres hexadecimales".to_string()));
        }
        if !hash.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(AppError::ValidationError("Hash debe contener solo caracteres hexadecimales".to_string()));
        }
        Ok(HashArchivo(hash))
    }

    /// Retorna el hash como referencia a string
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Verifica la integridad de los bytes comparando con el hash almacenado
    pub fn verificar_integridad(&self, bytes: &[u8]) -> bool {
        let computed_hash = Self::from_bytes(bytes);
        self.0 == computed_hash.0
    }
}

/// Identificador único de documento (wrapper alrededor de UUID)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DocumentoId(Uuid);

impl DocumentoId {
    /// Crea un nuevo DocumentoId con UUID v4 aleatorio
    pub fn new() -> Self {
        DocumentoId(Uuid::new_v4())
    }

    /// Crea un DocumentoId desde un UUID existente
    pub fn from_uuid(uuid: Uuid) -> Self {
        DocumentoId(uuid)
    }

    /// Retorna el UUID subyacente
    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl Default for DocumentoId {
    fn default() -> Self {
        Self::new()
    }
}

/// Entidad Documento del sistema
/// Representa un archivo almacenado con metadatos y verificación de integridad
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Documento {
    /// Identificador único del documento
    pub id: DocumentoId,
    /// Nombre original del archivo (mínimo 1, máximo 255 caracteres)
    #[validate(length(min = 1, max = 255))]
    pub nombre_archivo: String,
    /// Categoría del documento
    pub categoria: CategoriaDocumento,
    /// Hash SHA-256 para verificación de integridad
    pub hash: HashArchivo,
    /// Ruta local donde se almacena el archivo (mínimo 1, máximo 1024 caracteres)
    #[validate(length(min = 1, max = 1024))]
    pub ruta_local: String,
    /// Tamaño del archivo en bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tamaño_bytes: Option<u64>,
    /// Tipo MIME del archivo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tipo_mime: Option<String>,
    /// Indica si el documento ha sido foliado (revisado/aprobado)
    pub foliado: bool,
    /// Fecha y hora en que se folió el documento
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fecha_foliado: Option<DateTime<Utc>>,
    /// Fecha y hora de creación del documento
    pub creado_en: DateTime<Utc>,
    /// Fecha y hora de última actualización
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actualizado_en: Option<DateTime<Utc>>,
    /// Observaciones o notas sobre el documento
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observaciones: Option<String>,
}

impl Documento {
    /// Crea un nuevo documento con los datos proporcionados
    /// Calcula automáticamente el hash SHA-256 de los bytes
    pub fn nuevo(
        nombre_archivo: String,
        categoria: CategoriaDocumento,
        ruta_local: String,
        bytes: &[u8],
        tipo_mime: Option<String>,
    ) -> AppResult<Self> {
        let hash = HashArchivo::from_bytes(bytes);
        let tamaño_bytes = Some(bytes.len() as u64);

        let documento = Documento {
            id: DocumentoId::new(),
            nombre_archivo,
            categoria,
            hash,
            ruta_local,
            tamaño_bytes,
            tipo_mime,
            foliado: false,
            fecha_foliado: None,
            creado_en: Utc::now(),
            actualizado_en: None,
            observaciones: None,
        };

        documento.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;
        Ok(documento)
    }

    /// Marca el documento como foliado (revisado/aprobado)
    pub fn foliar(&mut self) {
        self.foliado = true;
        self.fecha_foliado = Some(Utc::now());
        self.actualizado_en = Some(Utc::now());
    }

    /// Agrega observaciones al documento
    pub fn agregar_observaciones(&mut self, observaciones: String) {
        self.observaciones = Some(observaciones);
        self.actualizado_en = Some(Utc::now());
    }

    /// Verifica si el documento es un PDF
    pub fn es_pdf(&self) -> bool {
        self.nombre_archivo.to_lowercase().ends_with(".pdf")
    }

    /// Verifica si el documento es una imagen
    pub fn es_imagen(&self) -> bool {
        let ext = self.nombre_archivo.to_lowercase();
        ext.ends_with(".jpg") || ext.ends_with(".jpeg") || ext.ends_with(".png") || ext.ends_with(".gif")
    }

    /// Verifica la integridad del archivo comparando con el hash almacenado
    pub fn verificar_integridad_archivo(&self, bytes: &[u8]) -> bool {
        self.hash.verificar_integridad(bytes)
    }
}
