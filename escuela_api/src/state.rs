use std::sync::Arc;
use escuela_storage::{ExpedienteRepository, DocumentoRepository, UsuarioRepository, SearchService, FileStorageService, AuditService};
use sqlx::SqlitePool;

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    pub expediente_repo: Arc<ExpedienteRepository>,
    pub documento_repo: Arc<DocumentoRepository>,
    pub usuario_repo: Arc<UsuarioRepository>,
    pub search_service: Arc<SearchService>,
    pub file_storage: Arc<FileStorageService>,
    pub audit_service: Arc<AuditService>,
}

impl AppState {
    pub fn new(pool: SqlitePool, storage_path: &str) -> Self {
        let expediente_repo = Arc::new(ExpedienteRepository::new(pool.clone()));
        let documento_repo = Arc::new(DocumentoRepository::new(pool.clone()));
        let usuario_repo = Arc::new(UsuarioRepository::new(pool.clone()));
        let search_service = Arc::new(SearchService::new(pool.clone()));
        let file_storage = Arc::new(FileStorageService::new(storage_path).expect("Error al inicializar servicio de almacenamiento"));
        let audit_service = Arc::new(AuditService::new(pool.clone()));

        AppState {
            pool,
            expediente_repo,
            documento_repo,
            usuario_repo,
            search_service,
            file_storage,
            audit_service,
        }
    }
}
