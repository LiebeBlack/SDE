pub mod database;
pub mod repositories;
pub mod search;
pub mod file_storage;
pub mod audit;
pub mod backup;

pub use database::Database;
pub use repositories::{ExpedienteRepository, DocumentoRepository, UsuarioRepository};
pub use search::{SearchService, SearchCriteria, SearchResult, ExpedienteSearchResult, DocumentoSearchResult};
pub use file_storage::{FileStorageService, ArchivoGuardado};
pub use audit::{AuditService, AccionAuditoria, RegistroAuditoria, AuditoriaEstadisticas};
