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
                                crate::state::TipoEntidad::Familiar => {
                                    eliminar_familiar(estado, id);
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

    if estado.lista_familiares.is_empty() {
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

                for fam in &estado.lista_familiares {
                    ui.label(fam.nombre_completo());
                    ui.label(fam.documento_identidad.as_deref().unwrap_or("-"));
                    ui.label(fam.telefono.as_deref().unwrap_or("-"));
                    ui.label(fam.email.as_deref().unwrap_or("-"));
                    ui.horizontal(|ui| {
                        if ui.button("✏️").clicked() {
                            estado.familiar_id_seleccionado = Some(fam.id);
                            estado.mostrar_form_familiar = true;
                            estado.formulario_familiar.campo_nombre = fam.nombre.clone();
                            estado.formulario_familiar.campo_apellido = fam.apellido.clone();
                            estado.formulario_familiar.campo_documento_identidad = fam.documento_identidad.clone().unwrap_or_default();
                            estado.formulario_familiar.campo_telefono = fam.telefono.clone().unwrap_or_default();
                            estado.formulario_familiar.campo_telefono_alterno = fam.telefono_alterno.clone().unwrap_or_default();
                            estado.formulario_familiar.campo_email = fam.email.clone().unwrap_or_default();
                            estado.formulario_familiar.campo_direccion = fam.direccion.clone().unwrap_or_default();
                            estado.formulario_familiar.campo_ocupacion = fam.ocupacion.clone().unwrap_or_default();
                            estado.formulario_familiar.campo_es_contacto_emergencia = fam.es_contacto_emergencia;
                            estado.formulario_familiar.campo_notas = fam.notas.clone().unwrap_or_default();
                        }
                        if ui.button("🗑️").clicked() {
                            estado.solicitar_confirmacion_eliminacion(
                                crate::state::TipoEntidad::Familiar,
                                fam.id,
                                &fam.nombre_completo()
                            );
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
                ui.text_edit_singleline(&mut estado.formulario_familiar.campo_nombre);
                ui.end_row();

                ui.label("Apellido");
                ui.text_edit_singleline(&mut estado.formulario_familiar.campo_apellido);
                ui.end_row();

                ui.label("Documento identidad");
                ui.text_edit_singleline(&mut estado.formulario_familiar.campo_documento_identidad);
                ui.end_row();

                ui.label("Teléfono");
                ui.text_edit_singleline(&mut estado.formulario_familiar.campo_telefono);
                ui.end_row();

                ui.label("Teléfono alterno");
                ui.text_edit_singleline(&mut estado.formulario_familiar.campo_telefono_alterno);
                ui.end_row();

                ui.label("Email");
                ui.text_edit_singleline(&mut estado.formulario_familiar.campo_email);
                ui.end_row();

                ui.label("Dirección");
                ui.text_edit_singleline(&mut estado.formulario_familiar.campo_direccion);
                ui.end_row();

                ui.label("Ocupación");
                ui.text_edit_singleline(&mut estado.formulario_familiar.campo_ocupacion);
                ui.end_row();

                ui.label("Contacto emergencia");
                ui.checkbox(&mut estado.formulario_familiar.campo_es_contacto_emergencia, "");
                ui.end_row();

                ui.label("Notas");
                ui.text_edit_singleline(&mut estado.formulario_familiar.campo_notas);
                ui.end_row();
            });

            ui.add_space(8.0);
            if ui.button("Guardar").clicked() {
                guardar_familiar(estado);
            }
        });
}

fn guardar_familiar(estado: &mut AppState) {
    let form = estado.formulario_familiar.clone();

    let repo = SqliteFamiliarRepo::new(&estado.conexion_bd);
    let servicio = FamiliarService::new(&repo, &repo);

    if let Some(familiar_id) = estado.familiar_id_seleccionado {
        // Modo edición
        if let Ok(Some(mut familiar)) = servicio.buscar_por_id(familiar_id) {
            familiar.nombre = form.campo_nombre;
            familiar.apellido = form.campo_apellido;
            familiar.documento_identidad = if form.campo_documento_identidad.is_empty() { None } else { Some(form.campo_documento_identidad) };
            familiar.telefono = if form.campo_telefono.is_empty() { None } else { Some(form.campo_telefono) };
            familiar.telefono_alterno = if form.campo_telefono_alterno.is_empty() { None } else { Some(form.campo_telefono_alterno) };
            familiar.email = if form.campo_email.is_empty() { None } else { Some(form.campo_email) };
            familiar.direccion = if form.campo_direccion.is_empty() { None } else { Some(form.campo_direccion) };
            familiar.ocupacion = if form.campo_ocupacion.is_empty() { None } else { Some(form.campo_ocupacion) };
            familiar.es_contacto_emergencia = form.campo_es_contacto_emergencia;
            familiar.notas = if form.campo_notas.is_empty() { None } else { Some(form.campo_notas) };
            familiar.updated_at = chrono::Utc::now();

            match repo.guardar(&familiar) {
                Ok(_) => {
                    estado.establecer_mensaje("Familiar actualizado correctamente.".to_string());
                    estado.formulario_familiar = Default::default();
                    estado.mostrar_form_familiar = false;
                    estado.familiar_id_seleccionado = None;
                }
                Err(e) => {
                    estado.establecer_mensaje(format!("Error al actualizar: {e}"));
                }
            }
        }
    } else {
        // Modo creación
        let mut nuevo = Familiar::nuevo(form.campo_nombre, form.campo_apellido);
        nuevo.documento_identidad = if form.campo_documento_identidad.is_empty() { None } else { Some(form.campo_documento_identidad) };
        nuevo.telefono = if form.campo_telefono.is_empty() { None } else { Some(form.campo_telefono) };
        nuevo.telefono_alterno = if form.campo_telefono_alterno.is_empty() { None } else { Some(form.campo_telefono_alterno) };
        nuevo.email = if form.campo_email.is_empty() { None } else { Some(form.campo_email) };
        nuevo.direccion = if form.campo_direccion.is_empty() { None } else { Some(form.campo_direccion) };
        nuevo.ocupacion = if form.campo_ocupacion.is_empty() { None } else { Some(form.campo_ocupacion) };
        nuevo.es_contacto_emergencia = form.campo_es_contacto_emergencia;
        nuevo.notas = if form.campo_notas.is_empty() { None } else { Some(form.campo_notas) };

        match servicio.crear(nuevo) {
            Ok(_) => {
                estado.establecer_mensaje("Familiar guardado correctamente.".to_string());
                estado.formulario_familiar = Default::default();
                estado.mostrar_form_familiar = false;
            }
            Err(e) => {
                estado.establecer_mensaje(format!("Error al guardar: {e}"));
            }
        }
    }

    cargar_familiares(estado);
}

fn eliminar_familiar(estado: &mut AppState, id: uuid::Uuid) {
    let repo = SqliteFamiliarRepo::new(&estado.conexion_bd);
    let servicio = FamiliarService::new(&repo, &repo);

    match servicio.eliminar(id) {
        Ok(_) => {
            estado.establecer_mensaje("Familiar eliminado correctamente.".to_string());
            cargar_familiares(estado);
        }
        Err(e) => {
            estado.establecer_mensaje(format!("Error al eliminar: {e}"));
        }
    }
}
