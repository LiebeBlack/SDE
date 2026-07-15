//! Núcleo del sistema de gestión escolar
//! Contiene la lógica de negocio pura, entidades del dominio, traits de servicios y utilidades de seguridad
//! 
//! Este crate sigue los principios de Clean Architecture y no depende de infraestructura

pub mod domain;
pub mod services;
pub mod security;

pub use domain::{usuario::Usuario, documento::Documento, expediente::ExpedienteDocente};
pub use services::{usuario_service::UsuarioService, expediente_service::ExpedienteService, documento_service::DocumentoService};
pub use security::{calculate_sha256, verify_integrity, check_permission, require_permission, Action, Resource};
