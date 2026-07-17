use crate::state::AppState;

pub fn mostrar(ui: &mut egui::Ui, estado: &AppState) {
    ui.heading("Dashboard institucional");
    ui.add_space(16.0);

    ui.horizontal(|ui| {
        tarjeta_metrica(ui, "Estudiantes registrados", &estado.estudiantes.len().to_string());
        tarjeta_metrica(ui, "Activos", &contar_activos(estado).to_string());
    });

    ui.add_space(24.0);
    ui.label("Bienvenido al sistema de gestión. Usa el menú de la izquierda para navegar entre módulos: estudiantes, documentos (PDF/imágenes), departamentos y reportes.");
}

fn contar_activos(estado: &AppState) -> usize {
    use app_core::models::estudiante::EstadoEstudiante;
    estado
        .estudiantes
        .iter()
        .filter(|e| e.estado == EstadoEstudiante::Activo)
        .count()
}

fn tarjeta_metrica(ui: &mut egui::Ui, titulo: &str, valor: &str) {
    egui::Frame::group(ui.style())
        .inner_margin(egui::Margin::same(16.0))
        .show(ui, |ui| {
            ui.set_min_width(180.0);
            ui.label(egui::RichText::new(titulo).weak());
            ui.label(egui::RichText::new(valor).size(28.0).strong());
        });
}
