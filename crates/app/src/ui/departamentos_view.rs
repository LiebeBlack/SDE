use crate::state::AppState;
use app_core::models::Departamento;
use app_core::services::DepartamentoService;
use storage::repositories::SqliteDepartamentoRepo;

pub fn mostrar(ui: &mut egui::Ui, estado: &mut AppState) {
    ui.heading("Departamentos");
    ui.add_space(8.0);

    ui.horizontal(|ui| {
        if ui.button("➕ Nuevo departamento").clicked() {
            estado.mostrar_form_departamento = !estado.mostrar_form_departamento;
        }
    });

    if estado.mostrar_form_departamento {
        mostrar_formulario_departamento(ui, estado);
    }

    ui.add_space(12.0);
    ui.separator();

    if estado.departamentos.is_empty() {
        cargar_departamentos(estado);
    }

    egui::ScrollArea::vertical().show(ui, |ui| {
        egui::Grid::new("tabla_departamentos")
            .striped(true)
            .num_columns(4)
            .show(ui, |ui| {
                ui.strong("Nombre");
                ui.strong("Descripción");
                ui.strong("Responsable");
                ui.strong("Acciones");
                ui.end_row();

                for dept in &estado.departamentos {
                    ui.label(&dept.nombre);
                    ui.label(dept.descripcion.as_deref().unwrap_or("-"));
                    ui.label(dept.responsable.as_deref().unwrap_or("-"));
                    ui.horizontal(|ui| {
                        if ui.button("✏️").clicked() {
                            estado.departamento_seleccionado = Some(dept.id);
                            estado.mostrar_form_departamento = true;
                            estado.form_nuevo_departamento.nombre = dept.nombre.clone();
                            estado.form_nuevo_departamento.descripcion = dept.descripcion.clone().unwrap_or_default();
                            estado.form_nuevo_departamento.responsable = dept.responsable.clone().unwrap_or_default();
                        }
                        if ui.button("🗑️").clicked() {
                            eliminar_departamento(estado, dept.id);
                        }
                    });
                    ui.end_row();
                }
            });
    });
}

fn mostrar_formulario_departamento(ui: &mut egui::Ui, estado: &mut AppState) {
    egui::Frame::group(ui.style())
        .inner_margin(egui::Margin::same(12.0))
        .show(ui, |ui| {
            ui.label("Nuevo departamento");
            egui::Grid::new("form_nuevo_departamento").num_columns(2).show(ui, |ui| {
                ui.label("Nombre");
                ui.text_edit_singleline(&mut estado.form_nuevo_departamento.nombre);
                ui.end_row();

                ui.label("Descripción");
                ui.text_edit_singleline(&mut estado.form_nuevo_departamento.descripcion);
                ui.end_row();

                ui.label("Responsable");
                ui.text_edit_singleline(&mut estado.form_nuevo_departamento.responsable);
                ui.end_row();
            });

            ui.add_space(8.0);
            if ui.button("Guardar").clicked() {
                guardar_departamento(estado);
            }
        });
}

fn guardar_departamento(estado: &mut AppState) {
    let form = estado.form_nuevo_departamento.clone();

    let repo = SqliteDepartamentoRepo::new(&estado.conn);
    let servicio = DepartamentoService::new(&repo);

    if let Some(departamento_id) = estado.departamento_seleccionado {
        // Modo edición
        if let Ok(Some(mut departamento)) = servicio.buscar_por_id(departamento_id) {
            departamento.nombre = form.nombre;
            departamento.descripcion = if form.descripcion.is_empty() { None } else { Some(form.descripcion) };
            departamento.responsable = if form.responsable.is_empty() { None } else { Some(form.responsable) };
            departamento.updated_at = chrono::Utc::now();

            match repo.guardar(&departamento) {
                Ok(_) => {
                    estado.mensaje_estado = Some("Departamento actualizado correctamente.".to_string());
                    estado.form_nuevo_departamento = Default::default();
                    estado.mostrar_form_departamento = false;
                    estado.departamento_seleccionado = None;
                }
                Err(e) => {
                    estado.mensaje_estado = Some(format!("Error al actualizar: {e}"));
                }
            }
        }
    } else {
        // Modo creación
        let mut nuevo = Departamento::nuevo(form.nombre);
        nuevo.descripcion = if form.descripcion.is_empty() { None } else { Some(form.descripcion) };
        nuevo.responsable = if form.responsable.is_empty() { None } else { Some(form.responsable) };

        match servicio.crear(nuevo) {
            Ok(_) => {
                estado.mensaje_estado = Some("Departamento guardado correctamente.".to_string());
                estado.form_nuevo_departamento = Default::default();
                estado.mostrar_form_departamento = false;
            }
            Err(e) => {
                estado.mensaje_estado = Some(format!("Error al guardar: {e}"));
            }
        }
    }

    cargar_departamentos(estado);
}

fn eliminar_departamento(estado: &mut AppState, id: uuid::Uuid) {
    let repo = SqliteDepartamentoRepo::new(&estado.conn);
    let servicio = DepartamentoService::new(&repo);

    match servicio.eliminar(id) {
        Ok(_) => {
            estado.mensaje_estado = Some("Departamento eliminado correctamente.".to_string());
            cargar_departamentos(estado);
        }
        Err(e) => {
            estado.mensaje_estado = Some(format!("Error al eliminar: {e}"));
        }
    }
}

pub fn cargar_departamentos(estado: &mut AppState) {
    let repo = SqliteDepartamentoRepo::new(&estado.conn);
    match repo.listar_todos() {
        Ok(lista) => estado.departamentos = lista,
        Err(e) => estado.mensaje_estado = Some(format!("Error cargando departamentos: {e}")),
    }
}
