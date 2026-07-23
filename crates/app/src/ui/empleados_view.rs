use crate::state::AppState;
use app_core::models::{Empleado, EstadoEmpleado, TipoContrato};
use app_core::services::EmpleadoService;
use storage::repositories::SqliteEmpleadoRepo;

pub fn mostrar(ui: &mut egui::Ui, estado: &mut AppState) {
    ui.heading("Empleados");
    ui.add_space(8.0);

    ui.horizontal(|ui| {
        if ui.button("➕ Nuevo empleado").clicked() {
            estado.mostrar_form_empleado = !estado.mostrar_form_empleado;
        }
    });

    if estado.mostrar_form_empleado {
        mostrar_formulario_empleado(ui, estado);
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
                                crate::state::TipoEntidad::Empleado => {
                                    eliminar_empleado(estado, id);
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

    if estado.lista_empleados.is_empty() {
        cargar_empleados(estado);
    }

    egui::ScrollArea::vertical().show(ui, |ui| {
        egui::Grid::new("tabla_empleados")
            .striped(true)
            .num_columns(6)
            .show(ui, |ui| {
                ui.strong("Cédula");
                ui.strong("Nombre");
                ui.strong("Cargo");
                ui.strong("Departamento");
                ui.strong("Estado");
                ui.strong("Acciones");
                ui.end_row();

                for emp in &estado.lista_empleados {
                    ui.label(&emp.cedula);
                    ui.label(emp.nombre_completo());
                    ui.label(&emp.cargo);
                    ui.label(emp.departamento_id.map(|_| "Asignado").unwrap_or("-".to_string()));
                    ui.label(emp.estado.to_string());
                    ui.horizontal(|ui| {
                        if ui.button("✏️").clicked() {
                            estado.empleado_id_seleccionado = Some(emp.id);
                            estado.mostrar_form_empleado = true;
                            estado.formulario_empleado.campo_cedula = emp.cedula.clone();
                            estado.formulario_empleado.campo_nombre = emp.nombre.clone();
                            estado.formulario_empleado.campo_apellido = emp.apellido.clone();
                            estado.formulario_empleado.campo_email = emp.email.clone();
                            estado.formulario_empleado.campo_telefono = emp.telefono.clone().unwrap_or_default();
                            estado.formulario_empleado.campo_direccion = emp.direccion.clone().unwrap_or_default();
                            estado.formulario_empleado.campo_cargo = emp.cargo.clone();
                            estado.formulario_empleado.campo_departamento_id = emp.departamento_id.map(|id| id.to_string());
                            estado.formulario_empleado.campo_fecha_contratacion = emp.fecha_contratacion.format("%Y-%m-%d").to_string();
                            estado.formulario_empleado.campo_fecha_terminacion = emp.fecha_terminacion.map(|d| d.format("%Y-%m-%d").to_string()).unwrap_or_default();
                            estado.formulario_empleado.campo_salario = emp.salario.to_string();
                            estado.formulario_empleado.campo_tipo_contrato = emp.tipo_contrato.to_string();
                            estado.formulario_empleado.campo_estado = emp.estado.to_string();
                            estado.formulario_empleado.campo_notas = emp.notas.clone().unwrap_or_default();
                        }
                        if ui.button("🗑️").clicked() {
                            estado.solicitar_confirmacion_eliminacion(
                                crate::state::TipoEntidad::Empleado,
                                emp.id,
                                &emp.nombre_completo()
                            );
                        }
                    });
                    ui.end_row();
                }
            });
    });
}

fn mostrar_formulario_empleado(ui: &mut egui::Ui, estado: &mut AppState) {
    // Cargar departamentos si no están cargados
    if estado.lista_departamentos.is_empty() {
        crate::ui::cargar_departamentos(estado);
    }

    egui::Frame::group(ui.style())
        .inner_margin(egui::Margin::same(12.0))
        .show(ui, |ui| {
            ui.label("Nuevo empleado");
            egui::Grid::new("form_nuevo_empleado").num_columns(2).show(ui, |ui| {
                ui.label("Cédula");
                ui.text_edit_singleline(&mut estado.formulario_empleado.campo_cedula);
                ui.end_row();

                ui.label("Nombre");
                ui.text_edit_singleline(&mut estado.formulario_empleado.campo_nombre);
                ui.end_row();

                ui.label("Apellido");
                ui.text_edit_singleline(&mut estado.formulario_empleado.campo_apellido);
                ui.end_row();

                ui.label("Email");
                ui.text_edit_singleline(&mut estado.formulario_empleado.campo_email);
                ui.end_row();

                ui.label("Teléfono");
                ui.text_edit_singleline(&mut estado.formulario_empleado.campo_telefono);
                ui.end_row();

                ui.label("Dirección");
                ui.text_edit_singleline(&mut estado.formulario_empleado.campo_direccion);
                ui.end_row();

                ui.label("Cargo");
                ui.text_edit_singleline(&mut estado.formulario_empleado.campo_cargo);
                ui.end_row();

                ui.label("Departamento");
                egui::ComboBox::from_id_source("departamento_empleado")
                    .selected_text(
                        estado.formulario_empleado.campo_departamento_id
                            .as_ref()
                            .and_then(|id| estado.lista_departamentos.iter().find(|d| d.id.to_string() == *id))
                            .map(|d| d.nombre.as_str())
                            .unwrap_or("Sin departamento")
                    )
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut estado.formulario_empleado.campo_departamento_id, None, "Sin departamento");
                        for dept in &estado.lista_departamentos {
                            ui.selectable_value(
                                &mut estado.formulario_empleado.campo_departamento_id,
                                Some(dept.id.to_string()),
                                &dept.nombre
                            );
                        }
                    });
                ui.end_row();

                ui.label("Fecha contratación");
                ui.text_edit_singleline(&mut estado.formulario_empleado.campo_fecha_contratacion);
                ui.end_row();

                ui.label("Fecha terminación");
                ui.text_edit_singleline(&mut estado.formulario_empleado.campo_fecha_terminacion);
                ui.end_row();

                ui.label("Salario");
                ui.text_edit_singleline(&mut estado.formulario_empleado.campo_salario);
                ui.end_row();

                ui.label("Tipo contrato");
                egui::ComboBox::from_id_source("tipo_contrato_empleado")
                    .selected_text(&estado.formulario_empleado.campo_tipo_contrato)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut estado.formulario_empleado.campo_tipo_contrato, "Tiempo Completo".to_string(), "Tiempo Completo");
                        ui.selectable_value(&mut estado.formulario_empleado.campo_tipo_contrato, "Medio Tiempo".to_string(), "Medio Tiempo");
                        ui.selectable_value(&mut estado.formulario_empleado.campo_tipo_contrato, "Por Horas".to_string(), "Por Horas");
                        ui.selectable_value(&mut estado.formulario_empleado.campo_tipo_contrato, "Temporal".to_string(), "Temporal");
                        ui.selectable_value(&mut estado.formulario_empleado.campo_tipo_contrato, "Prácticas".to_string(), "Prácticas");
                        ui.selectable_value(&mut estado.formulario_empleado.campo_tipo_contrato, "Contrato por Obra".to_string(), "Contrato por Obra");
                    });
                ui.end_row();

                ui.label("Estado");
                egui::ComboBox::from_id_source("estado_empleado")
                    .selected_text(&estado.formulario_empleado.campo_estado)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut estado.formulario_empleado.campo_estado, "Activo".to_string(), "Activo");
                        ui.selectable_value(&mut estado.formulario_empleado.campo_estado, "Baja".to_string(), "Baja");
                        ui.selectable_value(&mut estado.formulario_empleado.campo_estado, "Vacaciones".to_string(), "Vacaciones");
                        ui.selectable_value(&mut estado.formulario_empleado.campo_estado, "Licencia Médica".to_string(), "Licencia Médica");
                        ui.selectable_value(&mut estado.formulario_empleado.campo_estado, "Suspendido".to_string(), "Suspendido");
                    });
                ui.end_row();

                ui.label("Notas");
                ui.text_edit_singleline(&mut estado.formulario_empleado.campo_notas);
                ui.end_row();
            });

            ui.add_space(8.0);
            if ui.button("Guardar").clicked() {
                guardar_empleado(estado);
            }
        });
}

fn guardar_empleado(estado: &mut AppState) {
    let form = estado.formulario_empleado.clone();

    let repo = SqliteEmpleadoRepo::new(&estado.conexion_bd);
    let servicio = EmpleadoService::new(&repo, &repo);

    // Parsear tipo de contrato
    let tipo_contrato = match form.campo_tipo_contrato.as_str() {
        "Medio Tiempo" => TipoContrato::MedioTiempo,
        "Por Horas" => TipoContrato::PorHoras,
        "Temporal" => TipoContrato::Temporal,
        "Prácticas" => TipoContrato::Practicas,
        "Contrato por Obra" => TipoContrato::ContratoObra,
        _ => TipoContrato::TiempoCompleto,
    };

    // Parsear estado
    let estado_empleado = match form.campo_estado.as_str() {
        "Baja" => EstadoEmpleado::Baja,
        "Vacaciones" => EstadoEmpleado::Vacaciones,
        "Licencia Médica" => EstadoEmpleado::LicenciaMedica,
        "Suspendido" => EstadoEmpleado::Suspendido,
        _ => EstadoEmpleado::Activo,
    };

    // Parsear salario
    let salario = form.campo_salario.parse::<f64>().unwrap_or(0.0);

    // Parsear fecha contratación
    let fecha_contratacion = chrono::NaiveDate::parse_from_str(&form.campo_fecha_contratacion, "%Y-%m-%d")
        .unwrap_or_else(|_| chrono::Utc::now().naive_utc().date());

    // Parsear fecha terminación
    let fecha_terminacion = if form.campo_fecha_terminacion.is_empty() {
        None
    } else {
        chrono::NaiveDate::parse_from_str(&form.campo_fecha_terminacion, "%Y-%m-%d").ok()
    };

    // Parsear departamento
    let departamento_id = form.campo_departamento_id.and_then(|s| uuid::Uuid::parse_str(&s).ok());

    if let Some(empleado_id) = estado.empleado_id_seleccionado {
        // Modo edición
        if let Ok(Some(mut empleado)) = servicio.buscar_por_id(empleado_id) {
            empleado.cedula = form.campo_cedula;
            empleado.nombre = form.campo_nombre;
            empleado.apellido = form.campo_apellido;
            empleado.email = form.campo_email;
            empleado.telefono = if form.campo_telefono.is_empty() { None } else { Some(form.campo_telefono) };
            empleado.direccion = if form.campo_direccion.is_empty() { None } else { Some(form.campo_direccion) };
            empleado.cargo = form.campo_cargo;
            empleado.departamento_id = departamento_id;
            empleado.fecha_contratacion = fecha_contratacion;
            empleado.fecha_terminacion = fecha_terminacion;
            empleado.salario = salario;
            empleado.tipo_contrato = tipo_contrato;
            empleado.estado = estado_empleado;
            empleado.notas = if form.campo_notas.is_empty() { None } else { Some(form.campo_notas) };
            empleado.updated_at = chrono::Utc::now();

            match repo.guardar(&empleado) {
                Ok(_) => {
                    estado.establecer_mensaje("Empleado actualizado correctamente.".to_string());
                    estado.formulario_empleado = Default::default();
                    estado.mostrar_form_empleado = false;
                    estado.empleado_id_seleccionado = None;
                }
                Err(e) => {
                    estado.establecer_mensaje(format!("Error al actualizar: {e}"));
                }
            }
        }
    } else {
        // Modo creación
        let mut nuevo = Empleado::nuevo(form.campo_cedula, form.campo_nombre, form.campo_apellido, form.campo_email, form.campo_cargo, fecha_contratacion, salario, tipo_contrato);
        nuevo.telefono = if form.campo_telefono.is_empty() { None } else { Some(form.campo_telefono) };
        nuevo.direccion = if form.campo_direccion.is_empty() { None } else { Some(form.campo_direccion) };
        nuevo.departamento_id = departamento_id;
        nuevo.fecha_terminacion = fecha_terminacion;
        nuevo.estado = estado_empleado;
        nuevo.notas = if form.campo_notas.is_empty() { None } else { Some(form.campo_notas) };

        match servicio.crear(nuevo) {
            Ok(_) => {
                estado.establecer_mensaje("Empleado guardado correctamente.".to_string());
                estado.formulario_empleado = Default::default();
                estado.mostrar_form_empleado = false;
            }
            Err(e) => {
                estado.establecer_mensaje(format!("Error al guardar: {e}"));
            }
        }
    }

    cargar_empleados(estado);
}

fn eliminar_empleado(estado: &mut AppState, id: uuid::Uuid) {
    let repo = SqliteEmpleadoRepo::new(&estado.conexion_bd);
    let servicio = EmpleadoService::new(&repo, &repo);

    match servicio.eliminar(id) {
        Ok(_) => {
            estado.establecer_mensaje("Empleado eliminado correctamente.".to_string());
            cargar_empleados(estado);
        }
        Err(e) => {
            estado.establecer_mensaje(format!("Error al eliminar: {e}"));
        }
    }
}
