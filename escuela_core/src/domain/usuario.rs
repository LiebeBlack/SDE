use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use escuela_shared::{Email, Cedula, AppError, AppResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Rol {
    Super,
    Director,
    RecursosHumanos,
    Administrativo,
}

impl Rol {
    pub fn as_str(&self) -> &'static str {
        match self {
            Rol::Super => "super",
            Rol::Director => "director",
            Rol::RecursosHumanos => "recursos_humanos",
            Rol::Administrativo => "administrativo",
        }
    }

    pub fn from_str(s: &str) -> AppResult<Self> {
        match s.to_lowercase().as_str() {
            "super" => Ok(Rol::Super),
            "director" => Ok(Rol::Director),
            "recursos_humanos" | "recursos-humanos" => Ok(Rol::RecursosHumanos),
            "administrativo" => Ok(Rol::Administrativo),
            _ => Err(AppError::ValidationError(format!("Rol inválido: {}", s))),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UsuarioId(Uuid);

impl UsuarioId {
    pub fn new() -> Self {
        UsuarioId(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        UsuarioId(uuid)
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl Default for UsuarioId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Usuario {
    pub id: UsuarioId,
    #[validate(length(min = 3, max = 100))]
    pub nombre: String,
    #[validate(length(min = 3, max = 100))]
    pub apellido: String,
    pub email: Email,
    pub cedula: Cedula,
    pub password_hash: String,
    pub rol: Rol,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telefono: Option<String>,
    pub activo: bool,
    pub creado_en: DateTime<Utc>,
    pub actualizado_en: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ultimo_acceso: Option<DateTime<Utc>>,
}

impl Usuario {
    pub fn nuevo(
        nombre: String,
        apellido: String,
        email: Email,
        cedula: Cedula,
        password_hash: String,
        rol: Rol,
        telefono: Option<String>,
    ) -> AppResult<Self> {
        let usuario = Usuario {
            id: UsuarioId::new(),
            nombre,
            apellido,
            email,
            cedula,
            password_hash,
            rol,
            telefono,
            activo: true,
            creado_en: Utc::now(),
            actualizado_en: Utc::now(),
            ultimo_acceso: None,
        };

        usuario.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;
        Ok(usuario)
    }

    pub fn nombre_completo(&self) -> String {
        format!("{} {}", self.nombre, self.apellido)
    }

    pub fn tiene_permiso_director(&self) -> bool {
        matches!(self.rol, Rol::Super | Rol::Director)
    }

    pub fn tiene_permiso_recursos_humanos(&self) -> bool {
        matches!(self.rol, Rol::Super | Rol::Director | Rol::RecursosHumanos)
    }

    pub fn tiene_permiso_administrativo(&self) -> bool {
        matches!(self.rol, Rol::Super | Rol::Director | Rol::RecursosHumanos | Rol::Administrativo)
    }

    pub fn tiene_permiso_super(&self) -> bool {
        matches!(self.rol, Rol::Super)
    }

    pub fn registrar_acceso(&mut self) {
        self.ultimo_acceso = Some(Utc::now());
        self.actualizado_en = Utc::now();
    }

    pub fn desactivar(&mut self) {
        self.activo = false;
        self.actualizado_en = Utc::now();
    }

    pub fn activar(&mut self) {
        self.activo = true;
        self.actualizado_en = Utc::now();
    }
}
