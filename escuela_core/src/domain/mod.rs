//! Entidades del dominio del sistema de gestión escolar
//! Define las estructuras de datos principales que representan el modelo de negocio

pub mod usuario;
pub mod documento;
pub mod expediente;

pub use usuario::{Usuario, Rol, UsuarioId};
pub use documento::{Documento, DocumentoId, CategoriaDocumento, HashArchivo};
pub use expediente::{ExpedienteDocente, ExpedienteId, EstadoExpediente};
