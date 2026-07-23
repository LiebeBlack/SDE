use crate::state::AppState;
use crate::ui::cargar_estudiantes;
use app_core::models::Estudiante;
use app_core::models::estudiante::EstadoEstudiante;
use app_core::services::EstudianteService;
use storage::repositories::SqliteEstudianteRepo;

pub fn mostrar(ui: &mut egui::Ui, estado: &mut AppState) {
    ui.heading("Estudiantes");
    ui.add_space(8.0);

    ui.horizontal(|ui| {
        ui.label("Buscar:");
        ui.text_edit_singleline(&mut estado.texto_busqueda);
        if ui.button("➕ Nuevo estudiante").clicked() {
            estado.mostrar_formulario_estudiante = !estado.mostrar_formulario_estudiante;
        }
    });

    if estado.mostrar_formulario_estudiante {
        mostrar_formulario_nuevo(ui, estado);
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
                                crate::state::TipoEntidad::Estudiante => {
                                    eliminar_estudiante(estado, id);
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

    let filtro = estado.texto_busqueda.to_lowercase();
    let lista_filtrada: Vec<&Estudiante> = estado
        .lista_estudiantes
        .iter()
        .filter(|e| {
            filtro.is_empty()
                || e.nombre.to_lowercase().contains(&filtro)
                || e.apellido.to_lowercase().contains(&filtro)
                || e.matricula.to_lowercase().contains(&filtro)
        })
        .collect();

    egui::ScrollArea::vertical().show(ui, |ui| {
        egui::Grid::new("tabla_estudiantes")
            .striped(true)
            .num_columns(5)
            .show(ui, |ui| {
                ui.strong("Matrícula");
                ui.strong("Nombre");
                ui.strong("Nivel/Grado");
                ui.strong("Estado");
                ui.strong("Acciones");
                ui.end_row();

                for est in &lista_filtrada {
                    ui.label(&est.matricula);
                    ui.label(est.nombre_completo());
                    ui.label(&est.grado_nivel);
                    ui.label(est.estado.to_string());
                    ui.horizontal(|ui| {
                        if ui.button("📄").clicked() {
                            estado.estudiante_id_seleccionado = Some(est.id);
                            estado.vista_actual = crate::state::Vista::Documentos;
                        }
                        if ui.button("✏️").clicked() {
                            estado.estudiante_id_seleccionado = Some(est.id);
                            estado.mostrar_formulario_estudiante = true;
                            // Cargar datos en el formulario
                            estado.formulario_estudiante.campo_matricula = est.matricula.clone();
                            estado.formulario_estudiante.campo_nombre = est.nombre.clone();
                            estado.formulario_estudiante.campo_apellido = est.apellido.clone();
                            estado.formulario_estudiante.campo_grado_nivel = est.grado_nivel.clone();
                            estado.formulario_estudiante.campo_fecha_nacimiento = est.fecha_nacimiento.format("%Y-%m-%d").to_string();
                        }
                        if ui.button("🗑️").clicked() {
                            estado.solicitar_confirmacion_eliminacion(
                                crate::state::TipoEntidad::Estudiante,
                                est.id,
                                &est.nombre_completo()
                            );
                        }
                    });
                    ui.end_row();
                }
            });
    });
}

fn mostrar_formulario_nuevo(ui: &mut egui::Ui, estado: &mut AppState) {
    egui::Frame::group(ui.style())
        .inner_margin(egui::Margin::same(12.0))
        .show(ui, |ui| {
            ui.label("Nuevo estudiante");
            egui::Grid::new("form_nuevo_estudiante").num_columns(2).show(ui, |ui| {
                ui.label("Matrícula");
                ui.text_edit_singleline(&mut estado.formulario_estudiante.campo_matricula);
                ui.end_row();

                ui.label("Nombre");
                ui.text_edit_singleline(&mut estado.formulario_estudiante.campo_nombre);
                ui.end_row();

                ui.label("Apellido");
                ui.text_edit_singleline(&mut estado.formulario_estudiante.campo_apellido);
                ui.end_row();

                ui.label("Grado/Nivel");
                ui.text_edit_singleline(&mut estado.formulario_estudiante.campo_grado_nivel);
                ui.end_row();

                ui.label("Fecha nacimiento (AAAA-MM-DD)");
                ui.text_edit_singleline(&mut estado.formulario_estudiante.campo_fecha_nacimiento);
                ui.end_row();
            });

            ui.add_space(8.0);
            if ui.button("Guardar").clicked() {
                guardar_nuevo_estudiante(estado);
            }
        });
}

fn guardar_nuevo_estudiante(estado: &mut AppState) {
    let form = estado.formulario_estudiante.clone();

    let fecha = match chrono::NaiveDate::parse_from_str(&form.campo_fecha_nacimiento, "%Y-%m-%d") {
        Ok(f) => f,
        Err(_) => {
            estado.establecer_mensaje(
                "Fecha de nacimiento inválida. Usa el formato AAAA-MM-DD.".to_string()
            );
            return;
        }
    };

    let repo = SqliteEstudianteRepo::new(&estado.conexion_bd);
    let servicio = EstudianteService::new(&repo);

    if let Some(estudiante_id) = estado.estudiante_id_seleccionado {
        // Modo edición
        if let Ok(Some(mut estudiante)) = servicio.buscar_por_id(estudiante_id) {
            estudiante.matricula = form.campo_matricula;
            estudiante.nombre = form.campo_nombre;
            estudiante.apellido = form.campo_apellido;
            estudiante.fecha_nacimiento = fecha;
            estudiante.grado_nivel = form.campo_grado_nivel;
            estudiante.updated_at = chrono::Utc::now();

            match repo.guardar(&estudiante) {
                Ok(_) => {
                    estado.establecer_mensaje("Estudiante actualizado correctamente.".to_string());
                    estado.formulario_estudiante = Default::default();
                    estado.mostrar_formulario_estudiante = false;
                    estado.estudiante_id_seleccionado = None;
                }
                Err(e) => {
                    estado.establecer_mensaje(format!("Error al actualizar: {e}"));
                }
            }
        }
    } else {
        // Modo creación
        let nuevo = Estudiante::nuevo(form.campo_matricula, form.campo_nombre, form.campo_apellido, fecha, form.campo_grado_nivel);

        match servicio.crear(nuevo) {
            Ok(_) => {
                estado.establecer_mensaje("Estudiante guardado correctamente.".to_string());
                estado.formulario_estudiante = Default::default();
                estado.mostrar_formulario_estudiante = false;
            }
            Err(e) => {
                estado.establecer_mensaje(format!("Error al guardar: {e}"));
            }
        }
    }

    cargar_estudiantes(estado);
}

fn eliminar_estudiante(estado: &mut AppState, id: uuid::Uuid) {
    let repo = SqliteEstudianteRepo::new(&estado.conexion_bd);
    let servicio = EstudianteService::new(&repo);

    match servicio.eliminar(id) {
        Ok(_) => {
            estado.establecer_mensaje("Estudiante eliminado correctamente.".to_string());
            cargar_estudiantes(estado);
        }
        Err(e) => {
            estado.establecer_mensaje(format!("Error al eliminar: {e}"));
        }
    }
}
