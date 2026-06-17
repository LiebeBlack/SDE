use axum::http::header::CONTENT_TYPE;
use tower_http::{
    cors::{CorsLayer, Any},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use escuela_storage::Database;
use crate::routes::create_routes;
use crate::state::AppState;

pub async fn run_server(database_path: String, storage_path: String, static_path: String, bind_address: String) -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "escuela_api=debug,tower_http=debug,axum=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Iniciando servidor de API...");
    tracing::info!("Ruta de almacenamiento: {}", storage_path);
    tracing::info!("Archivos estáticos: {}", static_path);

    let database = Database::new(&database_path).await?;
    let pool = database.pool().clone();
    let state = AppState::new(pool, &storage_path);

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_credentials(false)
        .expose_headers([CONTENT_TYPE]);

    let app = create_routes(state, static_path)
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(&bind_address).await?;
    tracing::info!("Servidor escuchando en {}", bind_address);
    tracing::info!("Interfaz web disponible en: http://{}", bind_address);

    axum::serve(listener, app).await?;

    database.close().await?;
    Ok(())
}
