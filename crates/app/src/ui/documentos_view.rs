use crate::state::AppState;
use app_core::models::{Documento, EntidadTipo, TipoDocumento};
use app_core::services::documento_service::DocumentoRepositorio;
use app_core::services::DocumentoService;
use storage::repositories::{AlmacenLocal, SqliteDocumentoRepo};
use std::path::Path;

pub fn mostrar(ui: &mut egui::Ui, estado: &mut AppState) {
    ui.heading("Documentos (PDF / imágenes)");
    ui.add_space(8.0);

    let Some(estudiante_id) = estado.estudiante_seleccionado else {
        ui.label("Selecciona un estudiante desde la vista 'Estudiantes' (botón 'Ver documentos') para gestionar sus archivos adjuntos.");
        return;
    };

    let nombre_estudiante = estado
        .estudiantes
        .iter()
        .find(|e| e.id == estudiante_id)
        .map(|e| e.nombre_completo())
        .unwrap_or_else(|| "Estudiante".to_string());

    ui.label(format!("Expediente de: {nombre_estudiante}"));
    ui.add_space(8.0);

    if ui.button("📎 Adjuntar PDF / imagen").clicked() {
        adjuntar_archivo(estado, estudiante_id);
    }

    ui.add_space(12.0);
    ui.separator();

    let repo = SqliteDocumentoRepo::new(&estado.conn);
    match repo.listar_por_entidad(&EntidadTipo::Estudiante, estudiante_id) {
        Ok(documentos) => mostrar_lista_documentos(ui, &documentos),
        Err(e) => {
            ui.colored_label(egui::Color32::RED, format!("Error cargando documentos: {e}"));
        }
    }
}

fn mostrar_lista_documentos(ui: &mut egui::Ui, documentos: &[Documento]) {
    if documentos.is_empty() {
        ui.label("Sin documentos adjuntos todavía.");
        return;
    }

    egui::Grid::new("tabla_documentos")
        .striped(true)
        .num_columns(5)
        .show(ui, |ui| {
            ui.strong("Nombre");
            ui.strong("Tipo");
            ui.strong("Tamaño");
            ui.strong("Fecha");
            ui.strong("Acciones");
            ui.end_row();

            for doc in documentos {
                let icono = if doc.es_pdf() { "📄" } else { "🖼️" };
                ui.label(format!("{icono} {}", doc.nombre_original));
                ui.label(doc.tipo_documento.to_string());
                ui.label(format!("{} KB", doc.tamano_bytes / 1024));
                ui.label(doc.subido_en.format("%d/%m/%Y").to_string());
                ui.horizontal(|ui| {
                    if ui.button("👁️").clicked() {
                        abrir_documento(&doc.ruta_archivo);
                    }
                    if ui.button("🗑️").clicked() {
                        // Eliminar documento
                    }
                });
                ui.end_row();
            }
        });
}

fn adjuntar_archivo(estado: &mut AppState, estudiante_id: uuid::Uuid) {
    let Some(ruta) = rfd::FileDialog::new()
        .add_filter("Documentos", &["pdf", "png", "jpg", "jpeg"])
        .pick_file()
    else {
        return;
    };

    let metadata = std::fs::metadata(&ruta);
    let tamano_bytes = metadata.map(|m| m.len() as i64).unwrap_or(0);
    let mime_type = adivinar_mime(&ruta);

    let repo = SqliteDocumentoRepo::new(&estado.conn);
    let almacen = AlmacenLocal::new(directorio_documentos());
    let servicio = DocumentoService::new(&repo, &almacen);

    let resultado = servicio.adjuntar(
        EntidadTipo::Estudiante,
        estudiante_id,
        TipoDocumento::Otro("General".to_string()),
        &ruta,
        mime_type,
        tamano_bytes,
    );

    estado.mensaje_estado = Some(match resultado {
        Ok(_) => "Documento adjuntado correctamente.".to_string(),
        Err(e) => format!("Error al adjuntar documento: {e}"),
    });
}

fn adivinar_mime(ruta: &std::path::Path) -> String {
    match ruta.extension().and_then(|e| e.to_str()).unwrap_or("") {
        "pdf" => "application/pdf",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        _ => "application/octet-stream",
    }
    .to_string()
}

fn directorio_documentos() -> std::path::PathBuf {
    let base = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| std::path::PathBuf::from("."));
    base.join("data").join("documentos")
}

fn abrir_documento(ruta: &str) {
    let path = Path::new(ruta);
    if path.exists() {
        #[cfg(target_os = "windows")]
        {
            std::process::Command::new("cmd")
                .args(["/c", "start", "", ruta])
                .spawn()
                .ok();
        }
        #[cfg(target_os = "macos")]
        {
            std::process::Command::new("open")
                .arg(ruta)
                .spawn()
                .ok();
        }
        #[cfg(target_os = "linux")]
        {
            std::process::Command::new("xdg-open")
                .arg(ruta)
                .spawn()
                .ok();
        }
    }
}
