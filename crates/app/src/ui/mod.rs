pub mod dashboard;
pub mod estudiantes_view;
pub mod documentos_view;
pub mod departamentos_view;
pub mod familiares_view;
pub mod empleados_view;
pub mod reportes_view;
pub mod components;

use crate::state::{AppState, Vista};
use crate::theme::color_primario;
use app_core::services::estudiante_service::EstudianteRepositorio;
use app_core::services::EstudianteService;
use storage::repositories::SqliteEstudianteRepo;

pub struct MiApp {
    pub estado: AppState,
}

impl MiApp {
    pub fn new(estado: AppState) -> Self {
        Self { estado }
    }
}

/// Vuelve a leer la lista de estudiantes desde SQLite hacia el estado en memoria.
pub fn cargar_estudiantes(estado: &mut AppState) {
    let repo = SqliteEstudianteRepo::new(&estado.conexion_bd);
    match repo.listar_todos() {
        Ok(lista) => estado.lista_estudiantes = lista,
        Err(e) => estado.establecer_mensaje(format!("Error cargando estudiantes: {e}")),
    }
}

/// Carga los departamentos desde la base de datos
pub fn cargar_departamentos(estado: &mut AppState) {
    let repo = storage::repositories::SqliteDepartamentoRepo::new(&estado.conexion_bd);
    match repo.listar_todos() {
        Ok(lista) => estado.lista_departamentos = lista,
        Err(e) => estado.establecer_mensaje(format!("Error cargando departamentos: {e}")),
    }
}

/// Carga los familiares desde la base de datos
pub fn cargar_familiares(estado: &mut AppState) {
    let repo = storage::repositories::SqliteFamiliarRepo::new(&estado.conexion_bd);
    match repo.listar_todos() {
        Ok(lista) => estado.lista_familiares = lista,
        Err(e) => estado.establecer_mensaje(format!("Error cargando familiares: {e}")),
    }
}

/// Carga los empleados desde la base de datos
pub fn cargar_empleados(estado: &mut AppState) {
    let repo = storage::repositories::SqliteEmpleadoRepo::new(&estado.conexion_bd);
    match repo.listar_todos() {
        Ok(lista) => estado.lista_empleados = lista,
        Err(e) => estado.establecer_mensaje(format!("Error cargando empleados: {e}")),
    }
}

impl eframe::App for MiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("menu_lateral")
            .resizable(true)
            .min_width(200.0)
            .default_width(240.0)
            .show(ctx, |ui| {
                components::menu_lateral(ui, &mut self.estado);
            });

        egui::TopBottomPanel::top("barra_estado")
            .default_height(48.0)
            .show(ctx, |ui| {
                ui.add_space(16.0);
                ui.horizontal(|ui| {
                    ui.label("📌");
                    ui.add_space(8.0);
                    if let Some(msg) = &self.estado.mensaje_sistema {
                        ui.colored_label(color_primario(), msg);
                    } else {
                        ui.label("Sistema listo");
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(format!("🕐 {}", chrono::Local::now().format("%H:%M")));
                        ui.add_space(16.0);
                    });
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(16.0);
            match self.estado.vista_actual {
                Vista::Dashboard => dashboard::mostrar(ui, &self.estado),
                Vista::Estudiantes => estudiantes_view::mostrar(ui, &mut self.estado),
                Vista::Documentos => documentos_view::mostrar(ui, &mut self.estado),
                Vista::Departamentos => departamentos_view::mostrar(ui, &mut self.estado),
                Vista::Familiares => familiares_view::mostrar(ui, &mut self.estado),
                Vista::Empleados => empleados_view::mostrar(ui, &mut self.estado),
                Vista::Reportes => reportes_view::mostrar(ui, &mut self.estado),
            }
        });
    }
}
