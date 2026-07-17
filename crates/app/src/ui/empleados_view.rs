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

    ui.add_space(12.0);
    ui.separator();

    if estado.empleados.is_empty() {
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

                for emp in &estado.empleados {
                    ui.label(&emp.cedula);
                    ui.label(emp.nombre_completo());
                    ui.label(&emp.cargo);
                    ui.label(emp.departamento_id.map(|_| "Asignado").unwrap_or("-".to_string()));
                    ui.label(emp.estado.to_string());
                    ui.horizontal(|ui| {
                        if ui.button("✏️").clicked() {
                            estado.empleado_seleccionado = Some(emp.id);
                            estado.mostrar_form_empleado = true;
                            estado.form_nuevo_empleado.cedula = emp.cedula.clone();
                            estado.form_nuevo_empleado.nombre = emp.nombre.clone();
                            estado.form_nuevo_empleado.apellido = emp.apellido.clone();
                            estado.form_nuevo_empleado.email = emp.email.clone();
                            estado.form_nuevo_empleado.telefono = emp.telefono.clone().unwrap_or_default();
                            estado.form_nuevo_empleado.direccion = emp.direccion.clone().unwrap_or_default();
                            estado.form_nuevo_empleado.cargo = emp.cargo.clone();
                            estado.form_nuevo_empleado.departamento_id = emp.departamento_id.map(|id| id.to_string());
                            estado.form_nuevo_empleado.fecha_contratacion = emp.fecha_contratacion.format("%Y-%m-%d").to_string();
                            estado.form_nuevo_empleado.fecha_terminacion = emp.fecha_terminacion.map(|d| d.format("%Y-%m-%d").to_string()).unwrap_or_default();
                            estado.form_nuevo_empleado.salario = emp.salario.to_string();
                            estado.form_nuevo_empleado.tipo_contrato = emp.tipo_contrato.to_string();
                            estado.form_nuevo_empleado.estado = emp.estado.to_string();
                            estado.form_nuevo_empleado.notas = emp.notas.clone().unwrap_or_default();
                        }
                        if ui.button("🗑️").clicked() {
                            eliminar_empleado(estado, emp.id);
                        }
                    });
                    ui.end_row();
                }
            });
    });
}

fn mostrar_formulario_empleado(ui: &mut egui::Ui, estado: &mut AppState) {
    egui::Frame::group(ui.style())
        .inner_margin(egui::Margin::same(12.0))
        .show(ui, |ui| {
            ui.label("Nuevo empleado");
            egui::Grid::new("form_nuevo_empleado").num_columns(2).show(ui, |ui| {
                ui.label("Cédula");
                ui.text_edit_singleline(&mut estado.form_nuevo_empleado.cedula);
                ui.end_row();

                ui.label("Nombre");
                ui.text_edit_singleline(&mut estado.form_nuevo_empleado.nombre);
                ui.end_row();

                ui.label("Apellido");
                ui.text_edit_singleline(&mut estado.form_nuevo_empleado.apellido);
                ui.end_row();

                ui.label("Email");
                ui.text_edit_singleline(&mut estado.form_nuevo_empleado.email);
                ui.end_row();

                ui.label("Teléfono");
                ui.text_edit_singleline(&mut estado.form_nuevo_empleado.telefono);
                ui.end_row();

                ui.label("Dirección");
                ui.text_edit_singleline(&mut estado.form_nuevo_empleado.direccion);
                ui.end_row();

                ui.label("Cargo");
                ui.text_edit_singleline(&mut estado.form_nuevo_empleado.cargo);
                ui.end_row();

                ui.label("Fecha contratación");
                ui.text_edit_singleline(&mut estado.form_nuevo_empleado.fecha_contratacion);
                ui.end_row();

                ui.label("Fecha terminación");
                ui.text_edit_singleline(&mut estado.form_nuevo_empleado.fecha_terminacion);
                ui.end_row();

                ui.label("Salario");
                ui.text_edit_singleline(&mut estado.form_nuevo_empleado.salario);
                ui.end_row();

                ui.label("Tipo contrato");
                egui::ComboBox::from_id_source("tipo_contrato_empleado")
                    .selected_text(&estado.form_nuevo_empleado.tipo_contrato)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut estado.form_nuevo_empleado.tipo_contrato, "Tiempo Completo".to_string(), "Tiempo Completo");
                        ui.selectable_value(&mut estado.form_nuevo_empleado.tipo_contrato, "Medio Tiempo".to_string(), "Medio Tiempo");
                        ui.selectable_value(&mut estado.form_nuevo_empleado.tipo_contrato, "Por Horas".to_string(), "Por Horas");
                        ui.selectable_value(&mut estado.form_nuevo_empleado.tipo_contrato, "Temporal".to_string(), "Temporal");
                        ui.selectable_value(&mut estado.form_nuevo_empleado.tipo_contrato, "Prácticas".to_string(), "Prácticas");
                        ui.selectable_value(&mut estado.form_nuevo_empleado.tipo_contrato, "Contrato por Obra".to_string(), "Contrato por Obra");
                    });
                ui.end_row();

                ui.label("Estado");
                egui::ComboBox::from_id_source("estado_empleado")
                    .selected_text(&estado.form_nuevo_empleado.estado)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut estado.form_nuevo_empleado.estado, "Activo".to_string(), "Activo");
                        ui.selectable_value(&mut estado.form_nuevo_empleado.estado, "Baja".to_string(), "Baja");
                        ui.selectable_value(&mut estado.form_nuevo_empleado.estado, "Vacaciones".to_string(), "Vacaciones");
                        ui.selectable_value(&mut estado.form_nuevo_empleado.estado, "Licencia Médica".to_string(), "Licencia Médica");
                        ui.selectable_value(&mut estado.form_nuevo_empleado.estado, "Suspendido".to_string(), "Suspendido");
                    });
                ui.end_row();

                ui.label("Notas");
                ui.text_edit_singleline(&mut estado.form_nuevo_empleado.notas);
                ui.end_row();
            });

            ui.add_space(8.0);
            if ui.button("Guardar").clicked() {
                guardar_empleado(estado);
            }
        });
}

fn guardar_empleado(estado: &mut AppState) {
    let form = estado.form_nuevo_empleado.clone();

    let repo = SqliteEmpleadoRepo::new(&estado.conn);
    let servicio = EmpleadoService::new(&repo, &repo);

    // Parsear tipo de contrato
    let tipo_contrato = match form.tipo_contrato.as_str() {
        "Medio Tiempo" => TipoContrato::MedioTiempo,
        "Por Horas" => TipoContrato::PorHoras,
        "Temporal" => TipoContrato::Temporal,
        "Prácticas" => TipoContrato::Practicas,
        "Contrato por Obra" => TipoContrato::ContratoObra,
        _ => TipoContrato::TiempoCompleto,
    };

    // Parsear estado
    let estado_empleado = match form.estado.as_str() {
        "Baja" => EstadoEmpleado::Baja,
        "Vacaciones" => EstadoEmpleado::Vacaciones,
        "Licencia Médica" => EstadoEmpleado::LicenciaMedica,
        "Suspendido" => EstadoEmpleado::Suspendido,
        _ => EstadoEmpleado::Activo,
    };

    // Parsear salario
    let salario = form.salario.parse::<f64>().unwrap_or(0.0);

    // Parsear fecha contratación
    let fecha_contratacion = chrono::NaiveDate::parse_from_str(&form.fecha_contratacion, "%Y-%m-%d")
        .unwrap_or_else(|_| chrono::Utc::now().naive_utc().date());

    // Parsear fecha terminación
    let fecha_terminacion = if form.fecha_terminacion.is_empty() {
        None
    } else {
        chrono::NaiveDate::parse_from_str(&form.fecha_terminacion, "%Y-%m-%d").ok()
    };

    // Parsear departamento
    let departamento_id = form.departamento_id.and_then(|s| uuid::Uuid::parse_str(&s).ok());

    if let Some(empleado_id) = estado.empleado_seleccionado {
        // Modo edición
        if let Ok(Some(mut empleado)) = servicio.buscar_por_id(empleado_id) {
            empleado.cedula = form.cedula;
            empleado.nombre = form.nombre;
            empleado.apellido = form.apellido;
            empleado.email = form.email;
            empleado.telefono = if form.telefono.is_empty() { None } else { Some(form.telefono) };
            empleado.direccion = if form.direccion.is_empty() { None } else { Some(form.direccion) };
            empleado.cargo = form.cargo;
            empleado.departamento_id = departamento_id;
            empleado.fecha_contratacion = fecha_contratacion;
            empleado.fecha_terminacion = fecha_terminacion;
            empleado.salario = salario;
            empleado.tipo_contrato = tipo_contrato;
            empleado.estado = estado_empleado;
            empleado.notas = if form.notas.is_empty() { None } else { Some(form.notas) };
            empleado.updated_at = chrono::Utc::now();

            match repo.guardar(&empleado) {
                Ok(_) => {
                    estado.mensaje_estado = Some("Empleado actualizado correctamente.".to_string());
                    estado.form_nuevo_empleado = Default::default();
                    estado.mostrar_form_empleado = false;
                    estado.empleado_seleccionado = None;
                }
                Err(e) => {
                    estado.mensaje_estado = Some(format!("Error al actualizar: {e}"));
                }
            }
        }
    } else {
        // Modo creación
        let mut nuevo = Empleado::nuevo(form.cedula, form.nombre, form.apellido, form.email, form.cargo, fecha_contratacion, salario, tipo_contrato);
        nuevo.telefono = if form.telefono.is_empty() { None } else { Some(form.telefono) };
        nuevo.direccion = if form.direccion.is_empty() { None } else { Some(form.direccion) };
        nuevo.departamento_id = departamento_id;
        nuevo.fecha_terminacion = fecha_terminacion;
        nuevo.estado = estado_empleado;
        nuevo.notas = if form.notas.is_empty() { None } else { Some(form.notas) };

        match servicio.crear(nuevo) {
            Ok(_) => {
                estado.mensaje_estado = Some("Empleado guardado correctamente.".to_string());
                estado.form_nuevo_empleado = Default::default();
                estado.mostrar_form_empleado = false;
            }
            Err(e) => {
                estado.mensaje_estado = Some(format!("Error al guardar: {e}"));
            }
        }
    }

    cargar_empleados(estado);
}

fn eliminar_empleado(estado: &mut AppState, id: uuid::Uuid) {
    let repo = SqliteEmpleadoRepo::new(&estado.conn);
    let servicio = EmpleadoService::new(&repo, &repo);

    match servicio.eliminar(id) {
        Ok(_) => {
            estado.mensaje_estado = Some("Empleado eliminado correctamente.".to_string());
            cargar_empleados(estado);
        }
        Err(e) => {
            estado.mensaje_estado = Some(format!("Error al eliminar: {e}"));
        }
    }
}

pub fn cargar_empleados(estado: &mut AppState) {
    let repo = SqliteEmpleadoRepo::new(&estado.conn);
    match repo.listar_todos() {
        Ok(lista) => estado.empleados = lista,
        Err(e) => estado.mensaje_estado = Some(format!("Error cargando empleados: {e}")),
    }
}
