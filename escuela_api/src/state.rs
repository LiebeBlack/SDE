use std::sync::Arc;
use escuela_storage::{ExpedienteRepository, DocumentoRepository, UsuarioRepository, SearchService, FileStorageService, AuditService};
use sqlx::SqlitePool;
use std::sync::Mutex;
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    pub expediente_repo: Arc<ExpedienteRepository>,
    pub documento_repo: Arc<DocumentoRepository>,
    pub usuario_repo: Arc<UsuarioRepository>,
    pub search_service: Arc<SearchService>,
    pub file_storage: Arc<FileStorageService>,
    pub audit_service: Arc<AuditService>,
    pub login_attempts: Arc<Mutex<HashMap<String, (u32, DateTime<Utc>)>>>,
}

impl AppState {
    pub fn new(pool: SqlitePool, storage_path: &str) -> Self {
        let expediente_repo = Arc::new(ExpedienteRepository::new(pool.clone()));
        let documento_repo = Arc::new(DocumentoRepository::new(pool.clone()));
        let usuario_repo = Arc::new(UsuarioRepository::new(pool.clone()));
        let search_service = Arc::new(SearchService::new(pool.clone()));
        let file_storage = Arc::new(FileStorageService::new(storage_path).unwrap_or_else(|e| {
            tracing::error!("Error al inicializar servicio de almacenamiento: {}", e);
            panic!("Error al inicializar servicio de almacenamiento: {}", e);
        }));
        let audit_service = Arc::new(AuditService::new(pool.clone()));

        AppState {
            pool,
            expediente_repo,
            documento_repo,
            usuario_repo,
            search_service,
            file_storage,
            audit_service,
            login_attempts: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}
