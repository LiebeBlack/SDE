use crate::domain::usuario::{Usuario, Rol};
use escuela_shared::{AppResult, AppError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    Read,
    Write,
    Delete,
    Modify,
    Approve,
    Audit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Resource {
    Expediente,
    Documento,
    Usuario,
    Sistema,
    Reporte,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Permission {
    Allow,
    Deny,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizationResult {
    pub allowed: bool,
    pub reason: String,
}

impl AuthorizationResult {
    pub fn allowed(reason: String) -> Self {
        AuthorizationResult {
            allowed: true,
            reason,
        }
    }
    
    pub fn denied(reason: String) -> Self {
        AuthorizationResult {
            allowed: false,
            reason,
        }
    }
}

pub fn check_permission(
    usuario: &Usuario,
    action: Action,
    resource: Resource,
) -> AuthorizationResult {
    if !usuario.activo {
        return AuthorizationResult::denied(
            "Usuario inactivo no tiene permisos para realizar cualquier acción".to_string()
        );
    }
    
    match (usuario.rol, action, resource) {
        // Super y Director tienen acceso completo a todo
        (Rol::Super | Rol::Director, _, _) => {
            AuthorizationResult::allowed(
                format!("{:?} tiene permiso completo para {:?} en {:?}", usuario.rol, action, resource)
            )
        }
        
        // Recursos Humanos puede leer y escribir expedientes y documentos
        (Rol::RecursosHumanos, Action::Read, Resource::Expediente) => {
            AuthorizationResult::allowed(
                "Recursos Humanos puede leer expedientes".to_string()
            )
        }
        (Rol::RecursosHumanos, Action::Write, Resource::Expediente) => {
            AuthorizationResult::allowed(
                "Recursos Humanos puede crear y modificar expedientes".to_string()
            )
        }
        (Rol::RecursosHumanos, Action::Read, Resource::Documento) => {
            AuthorizationResult::allowed(
                "Recursos Humanos puede leer documentos".to_string()
            )
        }
        (Rol::RecursosHumanos, Action::Write, Resource::Documento) => {
            AuthorizationResult::allowed(
                "Recursos Humanos puede subir y modificar documentos".to_string()
            )
        }
        (Rol::RecursosHumanos, Action::Modify, Resource::Documento) => {
            AuthorizationResult::allowed(
                "Recursos Humanos puede modificar documentos".to_string()
            )
        }
        
        // Administrador tiene acceso limitado
        (Rol::Administrativo, Action::Read, Resource::Expediente) => {
            AuthorizationResult::allowed(
                "Administrador puede leer expedientes".to_string()
            )
        }
        (Rol::Administrativo, Action::Read, Resource::Documento) => {
            AuthorizationResult::allowed(
                "Administrador puede leer documentos".to_string()
            )
        }
        (Rol::Administrativo, Action::Read, Resource::Reporte) => {
            AuthorizationResult::allowed(
                "Administrador puede ver reportes".to_string()
            )
        }
        
        // Denegar acciones no permitidas
        (rol, action, resource) => {
            AuthorizationResult::denied(
                format!(
                    "Rol {:?} no tiene permiso para {:?} en {:?}",
                    rol, action, resource
                )
            )
        }
    }
}

pub fn require_permission(
    usuario: &Usuario,
    action: Action,
    resource: Resource,
) -> AppResult<()> {
    let result = check_permission(usuario, action, resource);
    if result.allowed {
        Ok(())
    } else {
        Err(AppError::AuthorizationError(result.reason))
    }
}

pub fn can_modify_expediente(usuario: &Usuario) -> bool {
    matches!(usuario.rol, Rol::Super | Rol::Director | Rol::RecursosHumanos) && usuario.activo
}

pub fn can_modify_documento(usuario: &Usuario) -> bool {
    matches!(usuario.rol, Rol::Super | Rol::Director | Rol::RecursosHumanos) && usuario.activo
}

pub fn can_delete_expediente(usuario: &Usuario) -> bool {
    matches!(usuario.rol, Rol::Super | Rol::Director) && usuario.activo
}

pub fn can_delete_documento(usuario: &Usuario) -> bool {
    matches!(usuario.rol, Rol::Super | Rol::Director | Rol::RecursosHumanos) && usuario.activo
}

pub fn can_approve_documento(usuario: &Usuario) -> bool {
    matches!(usuario.rol, Rol::Super | Rol::Director) && usuario.activo
}
