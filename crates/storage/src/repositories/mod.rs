pub mod estudiante_repo;
pub mod documento_repo;
pub mod familiar_repo;
pub mod departamento_repo;
pub mod empleado_repo;
pub mod almacen_archivos;

pub use estudiante_repo::SqliteEstudianteRepo;
pub use documento_repo::SqliteDocumentoRepo;
pub use familiar_repo::SqliteFamiliarRepo;
pub use departamento_repo::SqliteDepartamentoRepo;
pub use empleado_repo::SqliteEmpleadoRepo;
pub use almacen_archivos::AlmacenLocal;
