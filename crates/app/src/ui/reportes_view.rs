use crate::state::AppState;

pub fn mostrar(ui: &mut egui::Ui, estado: &mut AppState) {
    ui.heading("Reportes / Generación de PDF");
    ui.add_space(8.0);

    let Some(estudiante_id) = estado.estudiante_id_seleccionado else {
        ui.label("Selecciona un estudiante en la vista 'Estudiantes' para generar su constancia de estudios en PDF.");
        return;
    };

    let estudiante = estado.lista_estudiantes.iter().find(|e| e.id == estudiante_id).cloned();

    let Some(estudiante) = estudiante else {
        ui.label("Estudiante no encontrado.");
        return;
    };

    ui.label(format!("Generar constancia de estudios para: {}", estudiante.nombre_completo()));

    if ui.button("🖨️ Generar y guardar PDF").clicked() {
        if let Some(destino) = rfd::FileDialog::new()
            .set_file_name(&format!("constancia_{}.pdf", estudiante.matricula))
            .save_file()
        {
            let resultado = pdf_engine::generator::generar_constancia_estudios(
                &estudiante,
                "Institución Educativa",
                &destino,
            );
            estado.establecer_mensaje(match resultado {
                Ok(_) => format!("PDF generado en {}", destino.display()),
                Err(e) => format!("Error al generar PDF: {e}"),
            });
        }
    }
}
