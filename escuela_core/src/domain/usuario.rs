//! Entidad de Usuario del sistema
//! Define los roles, permisos y estructura de usuarios en el sistema de gestión escolar

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use escuela_shared::{Email, Cedula, AppError, AppResult};

/// Roles de usuario en el sistema con jerarquía de permisos
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Rol {
    /// Super administrador con acceso completo a todo el sistema
    Super,
    /// Director con permisos de gestión completa
    Director,
    /// Recursos Humanos con permisos de gestión de expedientes y documentos
    RecursosHumanos,
    /// Administrativo con permisos de lectura limitados
    Administrativo,
}

impl Rol {
    /// Convierte el rol a su representación en string (snake_case)
    pub fn as_str(&self) -> &'static str {
        match self {
            Rol::Super => "super",
            Rol::Director => "director",
            Rol::RecursosHumanos => "recursos_humanos",
            Rol::Administrativo => "administrativo",
        }
    }

    /// Crea un Rol desde un string
    /// Acepta formatos con guion bajo o guion medio
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

/// Identificador único de usuario (wrapper alrededor de UUID)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UsuarioId(Uuid);

impl UsuarioId {
    /// Crea un nuevo UsuarioId con UUID v4 aleatorio
    pub fn new() -> Self {
        UsuarioId(Uuid::new_v4())
    }

    /// Crea un UsuarioId desde un UUID existente
    pub fn from_uuid(uuid: Uuid) -> Self {
        UsuarioId(uuid)
    }

    /// Retorna el UUID subyacente
    pub fn as_uuid(&self) -> Uuid {
        self.0
    }

    /// Retorna los bytes del UUID
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl Default for UsuarioId {
    fn default() -> Self {
        Self::new()
    }
}

/// Entidad Usuario del sistema
/// Representa un usuario con sus credenciales, rol y metadatos
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Usuario {
    /// Identificador único del usuario
    pub id: UsuarioId,
    /// Nombre del usuario (mínimo 3, máximo 100 caracteres)
    #[validate(length(min = 3, max = 100))]
    pub nombre: String,
    /// Apellido del usuario (mínimo 3, máximo 100 caracteres)
    #[validate(length(min = 3, max = 100))]
    pub apellido: String,
    /// Email validado del usuario
    pub email: Email,
    /// Cédula de identidad validada
    pub cedula: Cedula,
    /// Hash del password (usando Argon2)
    pub password_hash: String,
    /// Rol del usuario en el sistema
    pub rol: Rol,
    /// Teléfono opcional del usuario
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telefono: Option<String>,
    /// Indica si el usuario está activo
    pub activo: bool,
    /// Fecha y hora de creación del usuario
    pub creado_en: DateTime<Utc>,
    /// Fecha y hora de última actualización
    pub actualizado_en: DateTime<Utc>,
    /// Fecha y hora del último acceso (opcional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ultimo_acceso: Option<DateTime<Utc>>,
}

impl Usuario {
    /// Crea un nuevo usuario con los datos proporcionados
    /// Valida automáticamente los campos usando el trait Validate
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

    /// Retorna el nombre completo del usuario (nombre + apellido)
    pub fn nombre_completo(&self) -> String {
        format!("{} {}", self.nombre, self.apellido)
    }

    /// Verifica si el usuario tiene permisos de nivel Director o superior
    pub fn tiene_permiso_director(&self) -> bool {
        matches!(self.rol, Rol::Super | Rol::Director)
    }

    /// Verifica si el usuario tiene permisos de nivel Recursos Humanos o superior
    pub fn tiene_permiso_recursos_humanos(&self) -> bool {
        matches!(self.rol, Rol::Super | Rol::Director | Rol::RecursosHumanos)
    }

    /// Verifica si el usuario tiene permisos de nivel Administrativo o superior
    pub fn tiene_permiso_administrativo(&self) -> bool {
        matches!(self.rol, Rol::Super | Rol::Director | Rol::RecursosHumanos | Rol::Administrativo)
    }

    /// Verifica si el usuario tiene permisos de Super administrador
    pub fn tiene_permiso_super(&self) -> bool {
        matches!(self.rol, Rol::Super)
    }

    /// Registra el acceso actual del usuario actualizando ultimo_acceso
    pub fn registrar_acceso(&mut self) {
        self.ultimo_acceso = Some(Utc::now());
        self.actualizado_en = Utc::now();
    }

    /// Desactiva el usuario (no puede acceder al sistema)
    pub fn desactivar(&mut self) {
        self.activo = false;
        self.actualizado_en = Utc::now();
    }

    /// Activa el usuario (puede acceder al sistema)
    pub fn activar(&mut self) {
        self.activo = true;
        self.actualizado_en = Utc::now();
    }
}
