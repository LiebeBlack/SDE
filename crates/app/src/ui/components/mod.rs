use crate::state::{AppState, Vista};
use crate::theme::color_primario;

pub fn menu_lateral(ui: &mut egui::Ui, estado: &mut AppState) {
    ui.add_space(16.0);
    
    egui::Frame::none()
        .fill(color_primario())
        .inner_margin(egui::Margin::symmetric(16.0, 12.0))
        .rounding(egui::Rounding::same(8.0))
        .show(ui, |ui| {
            ui.heading("🏫 Gestión Institucional");
            ui.label("Sistema 100% Local");
        });

    ui.add_space(20.0);
    ui.separator();
    ui.add_space(16.0);

    ui.label("Navegación");
    ui.add_space(8.0);

    let opciones = [
        (Vista::Dashboard, "📊 Dashboard"),
        (Vista::Estudiantes, "🎓 Estudiantes"),
        (Vista::Documentos, "📄 Documentos"),
        (Vista::Departamentos, "🏢 Departamentos"),
        (Vista::Familiares, "👨‍👩‍👧 Familiares"),
        (Vista::Empleados, "👥 Empleados"),
        (Vista::Reportes, "🖨️ Reportes"),
    ];

    for (vista, etiqueta) in opciones {
        let seleccionado = estado.vista_actual == vista;
        let frame = egui::Frame::none()
            .fill(if seleccionado {
                color_primario()
            } else {
                egui::Color32::TRANSPARENT
            })
            .rounding(egui::Rounding::same(6.0))
            .inner_margin(egui::Margin::symmetric(12.0, 8.0));

        frame.show(ui, |ui| {
            if ui.selectable_label(seleccionado, etiqueta).clicked() {
                estado.vista_actual = vista;
            }
        });
        ui.add_space(4.0);
    }

    ui.add_space(20.0);
    ui.separator();
    ui.add_space(16.0);

    ui.label("Información");
    ui.add_space(8.0);
    
    ui.horizontal(|ui| {
        ui.label("💾");
        ui.label("Base de datos local");
    });
    
    ui.horizontal(|ui| {
        ui.label("🔒");
        ui.label("Sin conexión a internet");
    });
}
