use app_core::models::{Departamento, Empleado, Estudiante, Familiar};
use rusqlite::Connection;
use std::sync::Mutex;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Vista {
    Dashboard,
    Estudiantes,
    Documentos,
    Departamentos,
    Familiares,
    Empleados,
    Reportes,
}

pub struct AppState {
    pub conexion_bd: Mutex<Connection>,
    pub vista_actual: Vista,

    // Datos de Estudiantes
    pub lista_estudiantes: Vec<Estudiante>,
    pub texto_busqueda: String,
    pub estudiante_id_seleccionado: Option<Uuid>,

    // Formulario de Estudiante
    pub formulario_estudiante: FormEstudiante,
    pub mostrar_formulario_estudiante: bool,

    // Datos de Departamentos
    pub lista_departamentos: Vec<Departamento>,
    pub departamento_id_seleccionado: Option<Uuid>,
    pub formulario_departamento: FormDepartamento,
    pub mostrar_formulario_departamento: bool,

    // Datos de Familiares
    pub lista_familiares: Vec<Familiar>,
    pub familiar_id_seleccionado: Option<Uuid>,
    pub formulario_familiar: FormFamiliar,
    pub mostrar_formulario_familiar: bool,

    // Datos de Empleados
    pub lista_empleados: Vec<Empleado>,
    pub empleado_id_seleccionado: Option<Uuid>,
    pub formulario_empleado: FormEmpleado,
    pub mostrar_formulario_empleado: bool,

    // Confirmación de eliminación
    pub mostrar_dialogo_confirmacion: bool,
    pub mensaje_confirmacion: String,
    pub entidad_a_eliminar: Option<(TipoEntidad, Uuid)>,

    // Mensajes del sistema
    pub mensaje_sistema: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TipoEntidad {
    Estudiante,
    Departamento,
    Familiar,
    Empleado,
    Documento,
}

#[derive(Default, Clone)]
pub struct FormEstudiante {
    pub campo_matricula: String,
    pub campo_nombre: String,
    pub campo_apellido: String,
    pub campo_grado_nivel: String,
    pub campo_fecha_nacimiento: String,
}

#[derive(Default, Clone)]
pub struct FormDepartamento {
    pub campo_nombre: String,
    pub campo_descripcion: String,
    pub campo_responsable: String,
}

#[derive(Default, Clone)]
pub struct FormFamiliar {
    pub campo_nombre: String,
    pub campo_apellido: String,
    pub campo_documento_identidad: String,
    pub campo_telefono: String,
    pub campo_telefono_alterno: String,
    pub campo_email: String,
    pub campo_direccion: String,
    pub campo_ocupacion: String,
    pub campo_es_contacto_emergencia: bool,
    pub campo_notas: String,
}

#[derive(Default, Clone)]
pub struct FormEmpleado {
    pub campo_cedula: String,
    pub campo_nombre: String,
    pub campo_apellido: String,
    pub campo_email: String,
    pub campo_telefono: String,
    pub campo_direccion: String,
    pub campo_cargo: String,
    pub campo_departamento_id: Option<String>,
    pub campo_fecha_contratacion: String,
    pub campo_fecha_terminacion: String,
    pub campo_salario: String,
    pub campo_tipo_contrato: String,
    pub campo_estado: String,
    pub campo_notas: String,
}

impl AppState {
    pub fn new(conn: Connection) -> Self {
        Self {
            conexion_bd: Mutex::new(conn),
            vista_actual: Vista::Dashboard,
            lista_estudiantes: Vec::new(),
            texto_busqueda: String::new(),
            estudiante_id_seleccionado: None,
            formulario_estudiante: FormEstudiante::default(),
            mostrar_formulario_estudiante: false,
            lista_departamentos: Vec::new(),
            departamento_id_seleccionado: None,
            formulario_departamento: FormDepartamento::default(),
            mostrar_formulario_departamento: false,
            lista_familiares: Vec::new(),
            familiar_id_seleccionado: None,
            formulario_familiar: FormFamiliar::default(),
            mostrar_formulario_familiar: false,
            lista_empleados: Vec::new(),
            empleado_id_seleccionado: None,
            formulario_empleado: FormEmpleado::default(),
            mostrar_formulario_empleado: false,
            mostrar_dialogo_confirmacion: false,
            mensaje_confirmacion: String::new(),
            entidad_a_eliminar: None,
            mensaje_sistema: None,
        }
    }

    /// Solicita confirmación antes de eliminar una entidad
    pub fn solicitar_confirmacion_eliminacion(&mut self, tipo: TipoEntidad, id: Uuid, nombre: &str) {
        self.mensaje_confirmacion = format!("¿Está seguro de eliminar '{}'? Esta acción no se puede deshacer.", nombre);
        self.entidad_a_eliminar = Some((tipo,	id));
        self.mostrar_dialogo_confirmacion = true;
    }

    /// Cancela la confirmación de eliminación
    pub fn cancelar_eliminacion(&mut self) {
        self.mostrar_dialogo_confirmacion = false;
        self.mensaje_confirmacion.clear();
        self.entidad_a_eliminar = None;
    }

    /// Establece un mensaje del sistema
    pub fn establecer_mensaje(&mut self, mensaje: String) {
        self.mensaje_sistema = Some(mensaje);
    }

    /// Limpia el mensaje del sistema
    pub fn limpiar_mensaje(&mut self) {
        self.mensaje_sistema = None;
    }
}
