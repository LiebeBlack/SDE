use crate::models::{Empleado, EstadoEmpleado, RegistroAsistencia, TipoContrato};
use uuid::Uuid;

pub trait EmpleadoRepositorio {
    fn guardar(&self, empleado: &Empleado) -> anyhow::Result<()>;
    fn buscar_por_id(&self, id: Uuid) -> anyhow::Result<Option<Empleado>>;
    fn buscar_por_cedula(&self, cedula: &str) -> anyhow::Result<Option<Empleado>>;
    fn listar_todos(&self) -> anyhow::Result<Vec<Empleado>>;
    fn listar_por_estado(&self, estado: &EstadoEmpleado) -> anyhow::Result<Vec<Empleado>>;
    fn listar_por_departamento(&self, departamento_id: Uuid) -> anyhow::Result<Vec<Empleado>>;
    fn eliminar(&self, id: Uuid) -> anyhow::Result<()>;
}

pub trait AsistenciaRepositorio {
    fn guardar_registro(&self, registro: &RegistroAsistencia) -> anyhow::Result<()>;
    fn listar_por_empleado(&self, empleado_id: Uuid) -> anyhow::Result<Vec<RegistroAsistencia>>;
    fn listar_por_fecha(&self, fecha: chrono::NaiveDate) -> anyhow::Result<Vec<RegistroAsistencia>>;
    fn listar_por_empleado_y_fecha(
        &self,
        empleado_id: Uuid,
        fecha: chrono::NaiveDate,
    ) -> anyhow::Result<Vec<RegistroAsistencia>>;
    fn eliminar_registro(&self, id: Uuid) -> anyhow::Result<()>;
}

pub struct EmpleadoService<'a, R: EmpleadoRepositorio, AR: AsistenciaRepositorio> {
    repo: &'a R,
    repo_asistencia: &'a AR,
}

impl<'a, R: EmpleadoRepositorio, AR: AsistenciaRepositorio> EmpleadoService<'a, R, AR>`
{
    pub fn new(repo: &'a R, repo_asistencia: &'a AR) -> Self {
        Self { repo, repo_asistencia }
    }

    pub fn crear(&self, empleado: Empleado) -> anyhow::Result<Empleado> {
        // Validaciones inline sin librerías externas
        empleado.validar_campos()?;

        // Verificar que la cédula no esté duplicada
        if let Some(_) = self.repo.buscar_por_cedula(&empleado.cedula)? {
            return Err(anyhow::anyhow!("Ya existe un empleado con esta cédula"));
        }

        // Validar fecha de contratación no sea futura
        let hoy = chrono::Utc::now().naive_utc().date();
        if empleado.fecha_contratacion > hoy {
            return Err(anyhow::anyhow!("La fecha de contratación no puede ser futura"));
        }

        // Validar fecha de terminación si existe
        if let Some(terminacion) = empleado.fecha_terminacion {
            if terminacion < empleado.fecha_contratacion {
                return Err(anyhow::anyhow!(
                    "La fecha de terminación no puede ser anterior a la contratación"
                ));
            }
        }

        self.repo.guardar(&empleado)?;
        Ok(empleado)
    }

    pub fn actualizar(&self, empleado: Empleado) -> anyhow::Result<Empleado> {
        empleado.validar_campos()?;

        // Verificar que el empleado existe
        if self.repo.buscar_por_id(empleado.id)?.is_none() {
            return Err(anyhow::anyhow!("Empleado no encontrado"));
        }

        // Verificar que la cédula no esté duplicada en otro empleado
        if let Some(existing) = self.repo.buscar_por_cedula(&empleado.cedula)? {
            if existing.id != empleado.id {
                return Err(anyhow::anyhow!("Ya existe otro empleado con esta cédula"));
            }
        }

        self.repo.guardar(&empleado)?;
        Ok(empleado)
    }

    pub fn listar(&self) -> anyhow::Result<Vec<Empleado>> {
        self.repo.listar_todos()
    }

    pub fn listar_activos(&self) -> anyhow::Result<Vec<Empleado>> {
        self.repo.listar_por_estado(&EstadoEmpleado::Activo)
    }

    pub fn buscar_por_id(&self, id: Uuid) -> anyhow::Result<Option<Empleado>> {
        self.repo.buscar_por_id(id)
    }

    pub fn buscar_por_cedula(&self, cedula: &str) -> anyhow::Result<Option<Empleado>> {
        self.repo.buscar_por_cedula(cedula)
    }

    pub fn eliminar(&self, id: Uuid) -> anyhow::Result<()> {
        // Verificar que el empleado existe
        if self.repo.buscar_por_id(id)?.is_none() {
            return Err(anyhow::anyhow!("Empleado no encontrado"));
        }

        self.repo.eliminar(id)
    }

    pub fn cambiar_estado(&self, id: Uuid, nuevo_estado: EstadoEmpleado) -> anyhow::Result<()> {
        let mut empleado = self
            .repo
            .buscar_por_id(id)?
            .ok_or_else(|| anyhow::anyhow!("Empleado no encontrado"))?;

        empleado.estado = nuevo_estado;
        empleado.updated_at = chrono::Utc::now();

        // Validaciones específicas por estado
        match nuevo_estado {
            EstadoEmpleado::Baja => {
                // Al dar de baja, establecer fecha de terminación
                if empleado.fecha_terminacion.is_none() {
                    empleado.fecha_terminacion = Some(chrono::Utc::now().naive_utc().date());
                }
            }
            EstadoEmpleado::Activo => {
                // Al reactivar, limpiar fecha de terminación si existe
                if let Some(terminacion) = empleado.fecha_terminacion {
                    if terminacion < chrono::Utc::now().naive_utc().date() {
                        empleado.fecha_terminacion = None;
                    }
                }
            }
            _ => {}
        }

        self.repo.guardar(&empleado)
    }

    // --- Gestión de Asistencia ---

    pub fn registrar_entrada(&self, empleado_id: Uuid) -> anyhow::Result<RegistroAsistencia> {
        let empleado = self
            .repo
            .buscar_por_id(empleado_id)?
            .ok_or_else(|| anyhow::anyhow!("Empleado no encontrado"))?;

        if !empleado.estado.permite_asistencia() {
            return Err(anyhow::anyhow!(
                "El empleado no está en estado que permita asistencia"
            ));
        }

        let hoy = chrono::Utc::now().naive_utc().date();
        let hora = chrono::Utc::now().naive_utc().time();

        let registros_hoy = self
            .repo_asistencia
            .listar_por_empleado_y_fecha(empleado_id, hoy)?;

        let mut registro = RegistroAsistencia::nuevo(empleado_id, hoy, hora, crate::models::TipoAsistencia::Entrada);

        if !registro.es_valido_con_contexto(&registros_hoy) {
            return Err(anyhow::anyhow!("No se puede registrar entrada: ya existe una entrada sin salida"));
        }

        self.repo_asistencia.guardar_registro(&registro)?;
        Ok(registro)
    }

    pub fn registrar_salida(&self, empleado_id: Uuid) -> anyhow::Result<RegistroAsistencia> {
        let hoy = chrono::Utc::now().naive_utc().date();
        let hora = chrono::Utc::now().naive_utc().time();

        let registros_hoy = self
            .repo_asistencia
            .listar_por_empleado_y_fecha(empleado_id, hoy)?;

        let mut registro = RegistroAsistencia::nuevo(empleado_id, hoy, hora, crate::models::TipoAsistencia::Salida);

        if !registro.es_valido_con_contexto(&registros_hoy) {
            return Err(anyhow::anyhow!("No se puede registrar salida: no hay entrada previa"));
        }

        self.repo_asistencia.guardar_registro(&registro)?;
        Ok(registro)
    }

    pub fn registrar_inicio_almuerzo(&self, empleado_id: Uuid) -> anyhow::Result<RegistroAsistencia> {
        let hoy = chrono::Utc::now().naive_utc().date();
        let hora = chrono::Utc::now().naive_utc().time();

        let registros_hoy = self
            .repo_asistencia
            .listar_por_empleado_y_fecha(empleado_id, hoy)?;

        let mut registro = RegistroAsistencia::nuevo(
            empleado_id,
            hoy,
            hora,
            crate::models::TipoAsistencia::EntradaAlmuerzo,
        );

        if !registro.es_valido_con_contexto(&registros_hoy) {
            return Err(anyhow::anyhow!("No se puede registrar inicio de almuerzo: contexto inválido"));
        }

        self.repo_asistencia.guardar_registro(&registro)?;
        Ok(registro)
    }

    pub fn registrar_fin_almuerzo(&self, empleado_id: Uuid) -> anyhow::Result<RegistroAsistencia> {
        let hoy = chrono::Utc::now().naive_utc().date();
        let hora = chrono::Utc::now().naive_utc().time();

        let registros_hoy = self
            .repo_asistencia
            .listar_por_empleado_y_fecha(empleado_id, hoy)?;

        let mut registro = RegistroAsistencia::nuevo(
            empleado_id,
            hoy,
            hora,
            crate::models::TipoAsistencia::SalidaAlmuerzo,
        );

        if !registro.es_valido_con_contexto(&registros_hoy) {
            return Err(anyhow::anyhow!("No se puede registrar fin de almuerzo: no hay inicio previo"));
        }

        self.repo_asistencia.guardar_registro(&registro)?;
        Ok(registro)
    }

    pub fn obtener_asistencia_empleado(
        &self,
        empleado_id: Uuid,
    ) -> anyhow::Result<Vec<RegistroAsistencia>> {
        self.repo_asistencia.listar_por_empleado(empleado_id)
    }

    pub fn calcular_horas_trabajadas(
        &self,
        empleado_id: Uuid,
        fecha_inicio: chrono::NaiveDate,
        fecha_fin: chrono::NaiveDate,
    ) -> anyhow::Result<crate::models::CalculoHoras> {
        let registros = self.repo_asistencia.listar_por_empleado(empleado_id)?;

        // Filtrar por rango de fechas
        let registros_filtrados: Vec<RegistroAsistencia> = registros
            .into_iter()
            .filter(|r| r.fecha >= fecha_inicio && r.fecha <= fecha_fin)
            .collect();

        Ok(crate::models::CalculoHoras::desde_registros(&registros_filtrados))
    }

    pub fn obtener_empleados_por_departamento(
        &self,
        departamento_id: Uuid,
    ) -> anyhow::Result<Vec<Empleado>> {
        self.repo.listar_por_departamento(departamento_id)
    }
}
