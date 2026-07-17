use crate::state::AppState;
use app_core::models::Familiar;
use app_core::services::FamiliarService;
use storage::repositories::SqliteFamiliarRepo;

pub fn mostrar(ui: &mut egui::Ui, estado: &mut AppState) {
    ui.heading("Familiares / Tutores");
    ui.add_space(8.0);

    ui.horizontal(|ui| {
        if ui.button("➕ Nuevo familiar").clicked() {
            estado.mostrar_form_familiar = !estado.mostrar_form_familiar;
        }
    });

    if estado.mostrar_form_familiar {
        mostrar_formulario_familiar(ui, estado);
    }

    ui.add_space(12.0);
    ui.separator();

    if estado.familiares.is_empty() {
        cargar_familiares(estado);
    }

    egui::ScrollArea::vertical().show(ui, |ui| {
        egui::Grid::new("tabla_familiares")
            .striped(true)
            .num_columns(5)
            .show(ui, |ui| {
                ui.strong("Nombre");
                ui.strong("Documento");
                ui.strong("Teléfono");
                ui.strong("Email");
                ui.strong("Acciones");
                ui.end_row();

                for fam in &estado.familiares {
                    ui.label(fam.nombre_completo());
                    ui.label(fam.documento_identidad.as_deref().unwrap_or("-"));
                    ui.label(fam.telefono.as_deref().unwrap_or("-"));
                    ui.label(fam.email.as_deref().unwrap_or("-"));
                    ui.horizontal(|ui| {
                        if ui.button("✏️").clicked() {
                            estado.familiar_seleccionado = Some(fam.id);
                            estado.mostrar_form_familiar = true;
                            estado.form_nuevo_familiar.nombre = fam.nombre.clone();
                            estado.form_nuevo_familiar.apellido = fam.apellido.clone();
                            estado.form_nuevo_familiar.documento_identidad = fam.documento_identidad.clone().unwrap_or_default();
                            estado.form_nuevo_familiar.telefono = fam.telefono.clone().unwrap_or_default();
                            estado.form_nuevo_familiar.telefono_alterno = fam.telefono_alterno.clone().unwrap_or_default();
                            estado.form_nuevo_familiar.email = fam.email.clone().unwrap_or_default();
                            estado.form_nuevo_familiar.direccion = fam.direccion.clone().unwrap_or_default();
                            estado.form_nuevo_familiar.ocupacion = fam.ocupacion.clone().unwrap_or_default();
                            estado.form_nuevo_familiar.es_contacto_emergencia = fam.es_contacto_emergencia;
                            estado.form_nuevo_familiar.notas = fam.notas.clone().unwrap_or_default();
                        }
                        if ui.button("🗑️").clicked() {
                            eliminar_familiar(estado, fam.id);
                        }
                    });
                    ui.end_row();
                }
            });
    });
}

fn mostrar_formulario_familiar(ui: &mut egui::Ui, estado: &mut AppState) {
    egui::Frame::group(ui.style())
        .inner_margin(egui::Margin::same(12.0))
        .show(ui, |ui| {
            ui.label("Nuevo familiar/tutor");
            egui::Grid::new("form_nuevo_familiar").num_columns(2).show(ui, |ui| {
                ui.label("Nombre");
                ui.text_edit_singleline(&mut estado.form_nuevo_familiar.nombre);
                ui.end_row();

                ui.label("Apellido");
                ui.text_edit_singleline(&mut estado.form_nuevo_familiar.apellido);
                ui.end_row();

                ui.label("Documento identidad");
                ui.text_edit_singleline(&mut estado.form_nuevo_familiar.documento_identidad);
                ui.end_row();

                ui.label("Teléfono");
                ui.text_edit_singleline(&mut estado.form_nuevo_familiar.telefono);
                ui.end_row();

                ui.label("Teléfono alterno");
                ui.text_edit_singleline(&mut estado.form_nuevo_familiar.telefono_alterno);
                ui.end_row();

                ui.label("Email");
                ui.text_edit_singleline(&mut estado.form_nuevo_familiar.email);
                ui.end_row();

                ui.label("Dirección");
                ui.text_edit_singleline(&mut estado.form_nuevo_familiar.direccion);
                ui.end_row();

                ui.label("Ocupación");
                ui.text_edit_singleline(&mut estado.form_nuevo_familiar.ocupacion);
                ui.end_row();

                ui.label("Contacto emergencia");
                ui.checkbox(&mut estado.form_nuevo_familiar.es_contacto_emergencia, "");
                ui.end_row();

                ui.label("Notas");
                ui.text_edit_singleline(&mut estado.form_nuevo_familiar.notas);
                ui.end_row();
            });

            ui.add_space(8.0);
            if ui.button("Guardar").clicked() {
                guardar_familiar(estado);
            }
        });
}

fn guardar_familiar(estado: &mut AppState) {
    let form = estado.form_nuevo_familiar.clone();

    let repo = SqliteFamiliarRepo::new(&estado.conn);
    let servicio = FamiliarService::new(&repo, &repo);

    if let Some(familiar_id) = estado.familiar_seleccionado {
        // Modo edición
        if let Ok(Some(mut familiar)) = servicio.buscar_por_id(familiar_id) {
            familiar.nombre = form.nombre;
            familiar.apellido = form.apellido;
            familiar.documento_identidad = if form.documento_identidad.is_empty() { None } else { Some(form.documento_identidad) };
            familiar.telefono = if form.telefono.is_empty() { None } else { Some(form.telefono) };
            familiar.telefono_alterno = if form.telefono_alterno.is_empty() { None } else { Some(form.telefono_alterno) };
            familiar.email = if form.email.is_empty() { None } else { Some(form.email) };
            familiar.direccion = if form.direccion.is_empty() { None } else { Some(form.direccion) };
            familiar.ocupacion = if form.ocupacion.is_empty() { None } else { Some(form.ocupacion) };
            familiar.es_contacto_emergencia = form.es_contacto_emergencia;
            familiar.notas = if form.notas.is_empty() { None } else { Some(form.notas) };
            familiar.updated_at = chrono::Utc::now();

            match repo.guardar(&familiar) {
                Ok(_) => {
                    estado.mensaje_estado = Some("Familiar actualizado correctamente.".to_string());
                    estado.form_nuevo_familiar = Default::default();
                    estado.mostrar_form_familiar = false;
                    estado.familiar_seleccionado = None;
                }
                Err(e) => {
                    estado.mensaje_estado = Some(format!("Error al actualizar: {e}"));
                }
            }
        }
    } else {
        // Modo creación
        let mut nuevo = Familiar::nuevo(form.nombre, form.apellido);
        nuevo.documento_identidad = if form.documento_identidad.is_empty() { None } else { Some(form.documento_identidad) };
        nuevo.telefono = if form.telefono.is_empty() { None } else { Some(form.telefono) };
        nuevo.telefono_alterno = if form.telefono_alterno.is_empty() { None } else { Some(form.telefono_alterno) };
        nuevo.email = if form.email.is_empty() { None } else { Some(form.email) };
        nuevo.direccion = if form.direccion.is_empty() { None } else { Some(form.direccion) };
        nuevo.ocupacion = if form.ocupacion.is_empty() { None } else { Some(form.ocupacion) };
        nuevo.es_contacto_emergencia = form.es_contacto_emergencia;
        nuevo.notas = if form.notas.is_empty() { None } else { Some(form.notas) };

        match servicio.crear(nuevo) {
            Ok(_) => {
                estado.mensaje_estado = Some("Familiar guardado correctamente.".to_string());
                estado.form_nuevo_familiar = Default::default();
                estado.mostrar_form_familiar = false;
            }
            Err(e) => {
                estado.mensaje_estado = Some(format!("Error al guardar: {e}"));
            }
        }
    }

    cargar_familiares(estado);
}

fn eliminar_familiar(estado: &mut AppState, id: uuid::Uuid) {
    let repo = SqliteFamiliarRepo::new(&estado.conn);
    let servicio = FamiliarService::new(&repo, &repo);

    match servicio.eliminar(id) {
        Ok(_) => {
            estado.mensaje_estado = Some("Familiar eliminado correctamente.".to_string());
            cargar_familiares(estado);
        }
        Err(e) => {
            estado.mensaje_estado = Some(format!("Error al eliminar: {e}"));
        }
    }
}

pub fn cargar_familiares(estado: &mut AppState) {
    let repo = SqliteFamiliarRepo::new(&estado.conn);
    match repo.listar_todos() {
        Ok(lista) => estado.familiares = lista,
        Err(e) => estado.mensaje_estado = Some(format!("Error cargando familiares: {e}")),
    }
}
