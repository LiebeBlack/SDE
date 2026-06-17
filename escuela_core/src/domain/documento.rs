use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use uuid::Uuid;
use validator::Validate;
use escuela_shared::{AppError, AppResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CategoriaDocumento {
    TituloAcademico,
    CedulaIdentidad,
    ContratoLaboral,
    CertificadoAntecedentes,
    CurriculumVitae,
    CertificadoMedico,
    Otros,
}

impl CategoriaDocumento {
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HashArchivo(String);

impl HashArchivo {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(bytes);
        let result = hasher.finalize();
        HashArchivo(hex::encode(result))
    }

    pub fn from_string(hash: String) -> AppResult<Self> {
        if hash.len() != 64 {
            return Err(AppError::ValidationError("Hash debe tener 64 caracteres hexadecimales".to_string()));
        }
        if !hash.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(AppError::ValidationError("Hash debe contener solo caracteres hexadecimales".to_string()));
        }
        Ok(HashArchivo(hash))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn verificar_integridad(&self, bytes: &[u8]) -> bool {
        let computed_hash = Self::from_bytes(bytes);
        self.0 == computed_hash.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DocumentoId(Uuid);

impl DocumentoId {
    pub fn new() -> Self {
        DocumentoId(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        DocumentoId(uuid)
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl Default for DocumentoId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Documento {
    pub id: DocumentoId,
    #[validate(length(min = 1, max = 255))]
    pub nombre_archivo: String,
    pub categoria: CategoriaDocumento,
    pub hash: HashArchivo,
    #[validate(length(min = 1, max = 1024))]
    pub ruta_local: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tamaño_bytes: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tipo_mime: Option<String>,
    pub foliado: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fecha_foliado: Option<DateTime<Utc>>,
    pub creado_en: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actualizado_en: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observaciones: Option<String>,
}

impl Documento {
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

    pub fn foliar(&mut self) {
        self.foliado = true;
        self.fecha_foliado = Some(Utc::now());
        self.actualizado_en = Some(Utc::now());
    }

    pub fn agregar_observaciones(&mut self, observaciones: String) {
        self.observaciones = Some(observaciones);
        self.actualizado_en = Some(Utc::now());
    }

    pub fn es_pdf(&self) -> bool {
        self.nombre_archivo.to_lowercase().ends_with(".pdf")
    }

    pub fn es_imagen(&self) -> bool {
        let ext = self.nombre_archivo.to_lowercase();
        ext.ends_with(".jpg") || ext.ends_with(".jpeg") || ext.ends_with(".png") || ext.ends_with(".gif")
    }

    pub fn verificar_integridad_archivo(&self, bytes: &[u8]) -> bool {
        self.hash.verificar_integridad(bytes)
    }
}
