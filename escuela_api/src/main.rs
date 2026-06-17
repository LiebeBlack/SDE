use escuela_api::run_server;
use escuela_api::integrity::{verify_integrity, print_startup_banner};
use std::env;
use std::time::Duration;
use escuela_storage::backup::BackupService;
use tracing::{info, error};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    print_startup_banner();

    let database_path = env::var("DATABASE_PATH")
        .unwrap_or_else(|_| "escuela.db".to_string());
    
    let storage_path = env::var("STORAGE_PATH")
        .unwrap_or_else(|_| "storage".to_string());
    
    let bind_address = env::var("BIND_ADDRESS")
        .unwrap_or_else(|_| "0.0.0.0:3000".to_string());
    
    let default_static = {
        let cwd_static = env::current_dir().unwrap_or_default().join("static");
        let exe_static = env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.join("static")));

        if cwd_static.exists() {
            cwd_static.to_string_lossy().to_string()
        } else if let Some(exe_static) = exe_static {
            if exe_static.exists() {
                exe_static.to_string_lossy().to_string()
            } else {
                "static".to_string()
            }
        } else {
            "static".to_string()
        }
    };

    let static_path = env::var("STATIC_PATH")
        .unwrap_or_else(|_| default_static);

    println!("⚙️  Configuración del sistema:");
    println!("   📁 Base de datos: {}", database_path);
    println!("   💾 Almacenamiento: {}", storage_path);
    println!("   🌐 Servidor: {}", bind_address);
    println!("   🎨 Interfaz web: {}", static_path);
    println!();

    // Verificación de integridad al arranque
    let integrity_report = verify_integrity(&database_path, &storage_path).await?;
    
    if !integrity_report.is_healthy() {
        println!("⚠️  ADVERTENCIA: Se detectaron problemas de integridad.");
        println!("   El sistema se iniciará en modo degradado.");
        println!("   Se recomienda revisar los problemas reportados.");
        println!();
    }

    println!("🚀 Iniciando servidor API...");
    println!();

    // Iniciar tarea en segundo plano para copias de seguridad (cada 2 horas)
    let backup_db_path = database_path.clone();
    let backup_storage_path = storage_path.clone();
    tokio::spawn(async move {
        let interval_duration = Duration::from_secs(2 * 60 * 60); // 2 horas
        let mut interval = tokio::time::interval(interval_duration);
        
        // Esperar el primer intervalo para no hacer el backup apenas inicia vacío
        interval.tick().await; 
        
        loop {
            interval.tick().await;
            match BackupService::new(&backup_db_path, &backup_storage_path) {
                Ok(backup_service) => {
                    info!("Ejecutando copia de seguridad programada...");
                    if let Err(e) = backup_service.create_backup().await {
                        error!("Error en la copia de seguridad programada: {}", e);
                    }
                }
                Err(e) => {
                    error!("Error al inicializar servicio de copias de seguridad: {}", e);
                }
            }
        }
    });

    run_server(database_path, storage_path, static_path, bind_address).await?;

    Ok(())
}
