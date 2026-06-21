//! Router principal de la API
//! Define todas las rutas HTTP del sistema de gestión escolar

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;
use crate::handlers::{
    health_handler,
    expediente_handler::{crear_expediente, obtener_expediente, obtener_expediente_por_cedula, listar_expedientes, buscar_expedientes, cambiar_estado},
    documento_handler::{crear_documento, listar_documentos, foliar_documento, descargar_documento},
    search_handler::{buscar_expedientes_avanzado, buscar_documentos_avanzado, buscar_general},
    usuario_handler::{crear_usuario, listar_usuarios, toggle_usuario_estado},
    audit_handler::listar_auditoria,
    auth_handler::login,
};
use crate::state::AppState;

pub fn create_routes(state: AppState, static_path: String) -> Router {
    // Rutas públicas (No requieren autenticación JWT)
    let api_public = Router::new()
        .route("/health", get(health_handler::health_check))
        .route("/login", post(login));

    // Rutas protegidas (Requieren autenticación JWT, extraído automáticamente por Usuario en los handlers)
    let api_protected = Router::new()
        // Usuarios y Auditoría
        .route("/usuarios", post(crear_usuario).get(listar_usuarios))
        .route("/usuarios/:usuario_id/toggle", post(toggle_usuario_estado))
        .route("/auditoria", get(listar_auditoria))
        
        // Expedientes
        .route("/expedientes", post(crear_expediente).get(listar_expedientes))
        .route("/expedientes/:id", get(obtener_expediente))
        .route("/expedientes/:id/estado", post(cambiar_estado))
        .route("/expedientes/cedula/:cedula", get(obtener_expediente_por_cedula))
        
        // Documentos
        .route("/expedientes/:expediente_id/documentos", post(crear_documento).get(listar_documentos))
        .route("/expedientes/:expediente_id/documentos/:documento_id/foliar", post(foliar_documento))
        .route("/expedientes/:expediente_id/documentos/:documento_id/descargar", get(descargar_documento))
        
        // Búsquedas
        .route("/expedientes/buscar/:termino", get(buscar_expedientes))
        .route("/buscar/expedientes_avanzado", get(buscar_expedientes_avanzado))
        .route("/buscar/documentos_avanzado", get(buscar_documentos_avanzado))
        .route("/buscar/general", get(buscar_general));

    // Consolidar API en un solo Router
    let api_routes = Router::new()
        .merge(api_public)
        .merge(api_protected);

    // Unir la API con los archivos estáticos
    // Nota: se mantienen rutas duplicadas en root por compatibilidad con el frontend
    // que apunta directamente a /login en vez de /api/login
    Router::new()
        .nest("/api", api_routes)
        .route("/health", get(health_handler::health_check))
        .route("/login", post(login)) // Compatibilidad con frontend que no usa /api/login
        .route("/expedientes", post(crear_expediente).get(listar_expedientes))
        .route("/expedientes/buscar/:termino", get(buscar_expedientes))
        .route("/expedientes/:id", get(obtener_expediente))
        .route("/expedientes/:id/estado", post(cambiar_estado))
        .route("/expedientes/cedula/:cedula", get(obtener_expediente_por_cedula))
        .route("/expedientes/:expediente_id/documentos", post(crear_documento).get(listar_documentos))
        .route("/expedientes/:expediente_id/documentos/:documento_id/foliar", post(foliar_documento))
        .route("/expedientes/:expediente_id/documentos/:documento_id/descargar", get(descargar_documento))
        .route("/usuarios/:usuario_id/toggle", post(toggle_usuario_estado))
        .nest_service("/static", ServeDir::new(&static_path))
        .fallback_service(ServeDir::new(&static_path))
        .with_state(state)
}
