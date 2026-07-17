mod state;
mod theme;
mod ui;

use state::AppState;
use std::path::PathBuf;

fn main() -> eframe::Result<()> {
    tracing_subscriber::fmt::init();

    let ruta_db = ruta_datos().join("institucion.db");
    let conn = storage::db::conectar(&ruta_db).expect("No se pudo abrir la base de datos local");

    let opciones = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1200.0, 760.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Sistema de Gestión Institucional",
        opciones,
        Box::new(|cc| {
            theme::aplicar_tema(&cc.egui_ctx);
            let mut estado = AppState::new(conn);
            ui::cargar_estudiantes(&mut estado);
            ui::cargar_departamentos(&mut estado);
            ui::cargar_familiares(&mut estado);
            ui::cargar_empleados(&mut estado);
            Ok(Box::new(ui::MiApp::new(estado)))
        }),
    )
}

/// Carpeta local de datos: ./data junto al ejecutable, para que TODO
/// quede autocontenido y funcione sin conexión ni instalación adicional.
fn ruta_datos() -> PathBuf {
    let base = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."));
    base.join("data")
}
