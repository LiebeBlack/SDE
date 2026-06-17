use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use escuela_shared::{Cedula, AppError, AppResult};
use super::{documento::Documento, usuario::UsuarioId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EstadoExpediente {
    Activo,
    Inactivo,
    Suspendido,
    Archivado,
}

impl EstadoExpediente {
    pub fn as_str(&self) -> &'static str {
        match self {
            EstadoExpediente::Activo => "activo",
            EstadoExpediente::Inactivo => "inactivo",
            EstadoExpediente::Suspendido => "suspendido",
            EstadoExpediente::Archivado => "archivado",
        }
    }

    pub fn from_str(s: &str) -> AppResult<Self> {
        match s.to_lowercase().as_str() {
            "activo" => Ok(EstadoExpediente::Activo),
            "inactivo" => Ok(EstadoExpediente::Inactivo),
            "suspendido" => Ok(EstadoExpediente::Suspendido),
            "archivado" => Ok(EstadoExpediente::Archivado),
            _ => Err(AppError::ValidationError(format!("Estado de expediente inválido: {}", s))),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ExpedienteId(Uuid);

impl ExpedienteId {
    pub fn new() -> Self {
        ExpedienteId(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        ExpedienteId(uuid)
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl Default for ExpedienteId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ExpedienteDocente {
    pub id: ExpedienteId,
    #[validate(length(min = 3, max = 100))]
    pub nombres: String,
    #[validate(length(min = 3, max = 100))]
    pub apellidos: String,
    pub cedula: Cedula,
    #[validate(email)]
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(length(min = 10, max = 15))]
    pub telefono: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direccion: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fecha_nacimiento: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nacionalidad: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estado_civil: Option<String>,
    pub estado: EstadoExpediente,
    pub documentos: Vec<Documento>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creado_por: Option<UsuarioId>,
    pub creado_en: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actualizado_por: Option<UsuarioId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actualizado_en: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observaciones: Option<String>,
}

impl ExpedienteDocente {
    pub fn nuevo(
        nombres: String,
        apellidos: String,
        cedula: Cedula,
        email: String,
        telefono: Option<String>,
        direccion: Option<String>,
        fecha_nacimiento: Option<DateTime<Utc>>,
        nacionalidad: Option<String>,
        estado_civil: Option<String>,
        creado_por: Option<UsuarioId>,
    ) -> AppResult<Self> {
        let expediente = ExpedienteDocente {
            id: ExpedienteId::new(),
            nombres,
            apellidos,
            cedula,
            email,
            telefono,
            direccion,
            fecha_nacimiento,
            nacionalidad,
            estado_civil,
            estado: EstadoExpediente::Activo,
            documentos: Vec::new(),
            creado_por,
            creado_en: Utc::now(),
            actualizado_por: None,
            actualizado_en: None,
            observaciones: None,
        };

        expediente.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;
        Ok(expediente)
    }

    pub fn nombre_completo(&self) -> String {
        format!("{} {}", self.nombres, self.apellidos)
    }

    pub fn agregar_documento(&mut self, documento: Documento) {
        self.documentos.push(documento);
        self.actualizado_en = Some(Utc::now());
    }

    pub fn remover_documento(&mut self, documento_id: &super::documento::DocumentoId) -> AppResult<()> {
        let original_len = self.documentos.len();
        self.documentos.retain(|d| d.id != *documento_id);
        
        if self.documentos.len() == original_len {
            return Err(AppError::NotFound("Documento no encontrado en el expediente".to_string()));
        }
        
        self.actualizado_en = Some(Utc::now());
        Ok(())
    }

    pub fn obtener_documento(&self, documento_id: &super::documento::DocumentoId) -> Option<&Documento> {
        self.documentos.iter().find(|d| d.id == *documento_id)
    }

    pub fn obtener_documentos_por_categoria(&self, categoria: &super::documento::CategoriaDocumento) -> Vec<&Documento> {
        self.documentos.iter().filter(|d| &d.categoria == categoria).collect()
    }

    pub fn contar_documentos(&self) -> usize {
        self.documentos.len()
    }

    pub fn contar_documentos_foliados(&self) -> usize {
        self.documentos.iter().filter(|d| d.foliado).count()
    }

    pub fn todos_documentos_foliados(&self) -> bool {
        self.documentos.iter().all(|d| d.foliado)
    }

    pub fn cambiar_estado(&mut self, nuevo_estado: EstadoExpediente, actualizado_por: UsuarioId) {
        self.estado = nuevo_estado;
        self.actualizado_por = Some(actualizado_por);
        self.actualizado_en = Some(Utc::now());
    }

    pub fn agregar_observaciones(&mut self, observaciones: String, actualizado_por: UsuarioId) {
        self.observaciones = Some(observaciones);
        self.actualizado_por = Some(actualizado_por);
        self.actualizado_en = Some(Utc::now());
    }

    pub fn actualizar_datos_personales(
        &mut self,
        nombres: Option<String>,
        apellidos: Option<String>,
        email: Option<String>,
        telefono: Option<String>,
        direccion: Option<String>,
        actualizado_por: UsuarioId,
    ) -> AppResult<()> {
        if let Some(n) = nombres {
            self.nombres = n;
        }
        if let Some(a) = apellidos {
            self.apellidos = a;
        }
        if let Some(e) = email {
            self.email = e;
        }
        self.telefono = telefono;
        self.direccion = direccion;
        self.actualizado_por = Some(actualizado_por);
        self.actualizado_en = Some(Utc::now());
        
        self.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;
        Ok(())
    }

    pub fn esta_completo(&self) -> bool {
        let categorias_requeridas = [
            super::documento::CategoriaDocumento::CedulaIdentidad,
            super::documento::CategoriaDocumento::TituloAcademico,
            super::documento::CategoriaDocumento::ContratoLaboral,
        ];

        categorias_requeridas.iter().all(|cat| {
            self.documentos.iter().any(|d| &d.categoria == cat && d.foliado)
        })
    }
}
