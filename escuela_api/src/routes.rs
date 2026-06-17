// Router principal — escrito con hambre a las 3 AM
// Desarrollado por: Yoangel De Dios Níkolas Gómez Gómez
// @liebeblack | dame cotufas y un refresco

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;
use crate::handlers::{
    health_handler,
    expediente_handler::{crear_expediente, obtener_expediente, obtener_expediente_por_cedula, listar_expedientes, buscar_expedientes, cambiar_estado},
    documento_handler::{crear_documento, listar_documentos, foliar_documento},
    search_handler::{buscar_expedientes_avanzado, buscar_documentos_avanzado, buscar_general},
    usuario_handler::{crear_usuario, listar_usuarios},
    audit_handler::listar_auditoria,
    auth_handler::login,
};
use crate::state::AppState;

pub fn create_routes(state: AppState, static_path: String) -> Router {
    // Rutas públicas (No requieren JWT)
    let api_public = Router::new()
        .route("/health", get(health_handler::health_check))
        .route("/login", post(login));

    // Rutas protegidas (Requieren JWT, extraído automáticamente por AuthUser en los handlers)
    let api_protected = Router::new()
        // Usuarios y Auditoría
        // .route("/usuarios/old", get(listar_usuarios_viejo)) // respaldo: no borrar por si acaso
        .route("/usuarios", post(crear_usuario).get(listar_usuarios))
        .route("/auditoria", get(listar_auditoria))
        
        // Expedientes
        .route("/expedientes", post(crear_expediente).get(listar_expedientes))
        .route("/expedientes/:id", get(obtener_expediente))
        .route("/expedientes/:id/estado", post(cambiar_estado))
        .route("/expedientes/cedula/:cedula", get(obtener_expediente_por_cedula))
        
        // Documentos
        .route("/expedientes/:expediente_id/documentos", post(crear_documento).get(listar_documentos))
        .route("/expedientes/:expediente_id/documentos/:documento_id/foliar", post(foliar_documento))
        
        // Búsquedas
        .route("/expedientes/buscar/:termino", get(buscar_expedientes))
        .route("/buscar/expedientes_avanzado", get(buscar_expedientes_avanzado))
        .route("/buscar/documentos_avanzado", get(buscar_documentos_avanzado))
        .route("/buscar/general", get(buscar_general));

    // Consolidar API en un solo Router
    let api_routes = Router::new()
        .merge(api_public)
        .merge(api_protected);

    // Unir la API con los estáticos
    // Nota: mantenemos rutas duplicadas en root por compatibilidad con el frontend
    // viejo que apuntaba directo a /login en vez de /api/login. En la v2 se limpia esto.
    Router::new()
        .nest("/api", api_routes)
        .route("/health", get(health_handler::health_check))
        .route("/login", post(login)) // Para compatibilidad directa con el frontend si no usa /api/login
        .route("/expedientes", post(crear_expediente).get(listar_expedientes))
        .route("/expedientes/buscar/:termino", get(buscar_expedientes))
        .route("/expedientes/:id", get(obtener_expediente))
        .route("/expedientes/:id/estado", post(cambiar_estado))
        .route("/expedientes/cedula/:cedula", get(obtener_expediente_por_cedula))
        .route("/expedientes/:expediente_id/documentos", post(crear_documento).get(listar_documentos))
        .nest_service("/static", ServeDir::new(&static_path))
        .fallback_service(ServeDir::new(&static_path))
        .with_state(state)
}
