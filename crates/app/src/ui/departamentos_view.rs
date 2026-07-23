use crate::state::AppState;
use app_core::models::Departamento;
use app_core::services::DepartamentoService;
use storage::repositories::SqliteDepartamentoRepo;

pub fn mostrar(ui: &mut egui::Ui, estado: &mut AppState) {
    ui.heading("Departamentos");
    ui.add_space(8.0);

    ui.horizontal(|ui| {
        if ui.button("➕ Nuevo departamento").clicked() {
            estado.mostrar_formulario_departamento = !estado.mostrar_formulario_departamento;
        }
    });

    if estado.mostrar_formulario_departamento {
        mostrar_formulario_departamento(ui, estado);
    }

    // Diálogo de confirmación de eliminación
    if estado.mostrar_dialogo_confirmacion {
        egui::Window::new("Confirmar eliminación")
            .collapsible(false)
            .resizable(false)
            .show(ui.ctx(), |ui| {
                ui.label(&estado.mensaje_confirmacion);
                ui.add_space(16.0);
                ui.horizontal(|ui| {
                    if ui.button("Cancelar").clicked() {
                        estado.cancelar_eliminacion();
                    }
                    if ui.button("Eliminar").clicked() {
                        if let Some((tipo, id)) = estado.entidad_a_eliminar {
                            match tipo {
                                crate::state::TipoEntidad::Departamento => {
                                    eliminar_departamento(estado, id);
                                }
                                _ => {}
                            }
                        }
                        estado.cancelar_eliminacion();
                    }
                });
            });
    }

    ui.add_space(12.0);
    ui.separator();

    if estado.lista_departamentos.is_empty() {
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

                for dept in &estado.lista_departamentos {
                    ui.label(&dept.nombre);
                    ui.label(dept.descripcion.as_deref().unwrap_or("-"));
                    ui.label(dept.responsable.as_deref().unwrap_or("-"));
                    ui.horizontal(|ui| {
                        if ui.button("✏️").clicked() {
                            estado.departamento_id_seleccionado = Some(dept.id);
                            estado.mostrar_formulario_departamento = true;
                            estado.formulario_departamento.campo_nombre = dept.nombre.clone();
                            estado.formulario_departamento.campo_descripcion = dept.descripcion.clone().unwrap_or_default();
                            estado.formulario_departamento.campo_responsable = dept.responsable.clone().unwrap_or_default();
                        }
                        if ui.button("🗑️").clicked() {
                            estado.solicitar_confirmacion_eliminacion(
                                crate::state::TipoEntidad::Departamento,
                                dept.id,
                                &dept.nombre
                            );
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
                ui.text_edit_singleline(&mut estado.formulario_departamento.campo_nombre);
                ui.end_row();

                ui.label("Descripción");
                ui.text_edit_singleline(&mut estado.formulario_departamento.campo_descripcion);
                ui.end_row();

                ui.label("Responsable");
                ui.text_edit_singleline(&mut estado.formulario_departamento.campo_responsable);
                ui.end_row();
            });

            ui.add_space(8.0);
            if ui.button("Guardar").clicked() {
                guardar_departamento(estado);
            }
        });
}

fn guardar_departamento(estado: &mut AppState) {
    let form = estado.formulario_departamento.clone();

    let repo = SqliteDepartamentoRepo::new(&estado.conexion_bd);
    let servicio = DepartamentoService::new(&repo);

    if let Some(departamento_id) = estado.departamento_id_seleccionado {
        // Modo edición
        if let Ok(Some(mut departamento)) = servicio.buscar_por_id(departamento_id) {
            departamento.nombre = form.campo_nombre;
            departamento.descripcion = if form.campo_descripcion.is_empty() { None } else { Some(form.campo_descripcion) };
            departamento.responsable = if form.campo_responsable.is_empty() { None } else { Some(form.campo_responsable) };
            departamento.updated_at = chrono::Utc::now();

            match repo.guardar(&departamento) {
                Ok(_) => {
                    estado.establecer_mensaje("Departamento actualizado correctamente.".to_string());
                    estado.formulario_departamento = Default::default();
                    estado.mostrar_formulario_departamento = false;
                    estado.departamento_id_seleccionado = None;
                }
                Err(e) => {
                    estado.establecer_mensaje(format!("Error al actualizar: {e}"));
                }
            }
        }
    } else {
        // Modo creación
        let mut nuevo = Departamento::nuevo(form.campo_nombre);
        nuevo.descripcion = if form.campo_descripcion.is_empty() { None } else { Some(form.campo_descripcion) };
        nuevo.responsable = if form.campo_responsable.is_empty() { None } else { Some(form.campo_responsable) };

        match servicio.crear(nuevo) {
            Ok(_) => {
                estado.establecer_mensaje("Departamento guardado correctamente.".to_string());
                estado.formulario_departamento = Default::default();
                estado.mostrar_formulario_departamento = false;
            }
            Err(e) => {
                estado.establecer_mensaje(format!("Error al guardar: {e}"));
            }
        }
    }

    cargar_departamentos(estado);
}

fn eliminar_departamento(estado: &mut AppState, id: uuid::Uuid) {
    let repo = SqliteDepartamentoRepo::new(&estado.conexion_bd);
    let servicio = DepartamentoService::new(&repo);

    match servicio.eliminar(id) {
        Ok(_) => {
            estado.establecer_mensaje("Departamento eliminado correctamente.".to_string());
            cargar_departamentos(estado);
        }
        Err(e) => {
            estado.establecer_mensaje(format!("Error al eliminar: {e}"));
        }
    }
}
