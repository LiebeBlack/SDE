use app_core::models::{Empleado, EstadoEmpleado, RegistroAsistencia, TipoAsistencia, TipoContrato};
use app_core::services::empleado_service::{AsistenciaRepositorio, EmpleadoRepositorio};
use rusqlite::{params, Connection, Row};
use std::str::FromStr;
use std::sync::Mutex;
use uuid::Uuid;

pub struct SqliteEmpleadoRepo<'a> {
    conn: &'a Mutex<Connection>,
}

impl<'a> SqliteEmpleadoRepo<'a> {
    pub fn new(conn: &'a Mutex<Connection>) -> Self {
        Self { conn }
    }
}

fn estado_a_texto(estado: &EstadoEmpleado) -> &'static str {
    match estado {
        EstadoEmpleado::Activo => "activo",
        EstadoEmpleado::Baja => "baja",
        EstadoEmpleado::Vacaciones => "vacaciones",
        EstadoEmpleado::LicenciaMedica => "licencia_medica",
        EstadoEmpleado::Suspendido => "suspendido",
    }
}

fn texto_a_estado(texto: &str) -> EstadoEmpleado {
    match texto {
        "baja" => EstadoEmpleado::Baja,
        "vacaciones" => EstadoEmpleado::Vacaciones,
        "licencia_medica" => EstadoEmpleado::LicenciaMedica,
        "suspendido" => EstadoEmpleado::Suspendido,
        _ => EstadoEmpleado::Activo,
    }
}

fn contrato_a_texto(contrato: &TipoContrato) -> &'static str {
    match contrato {
        TipoContrato::TiempoCompleto => "tiempo_completo",
        TipoContrato::MedioTiempo => "medio_tiempo",
        TipoContrato::PorHoras => "por_horas",
        TipoContrato::Temporal => "temporal",
        TipoContrato::Practicas => "practicas",
        TipoContrato::ContratoObra => "contrato_obra",
    }
}

fn texto_a_contrato(texto: &str) -> TipoContrato {
    match texto {
        "medio_tiempo" => TipoContrato::MedioTiempo,
        "por_horas" => TipoContrato::PorHoras,
        "temporal" => TipoContrato::Temporal,
        "practicas" => TipoContrato::Practicas,
        "contrato_obra" => TipoContrato::ContratoObra,
        _ => TipoContrato::TiempoCompleto,
    }
}

fn tipo_asistencia_a_texto(tipo: &TipoAsistencia) -> &'static str {
    match tipo {
        TipoAsistencia::Entrada => "entrada",
        TipoAsistencia::Salida => "salida",
        TipoAsistencia::EntradaAlmuerzo => "entrada_almuerzo",
        TipoAsistencia::SalidaAlmuerzo => "salida_almuerzo",
    }
}

fn texto_a_tipo_asistencia(texto: &str) -> TipoAsistencia {
    match texto {
        "salida" => TipoAsistencia::Salida,
        "entrada_almuerzo" => TipoAsistencia::EntradaAlmuerzo,
        "salida_almuerzo" => TipoAsistencia::SalidaAlmuerzo,
        _ => TipoAsistencia::Entrada,
    }
}

fn fila_a_empleado(row: &Row) -> rusqlite::Result<Empleado> {
    let id_str: String = row.get("id")?;
    let fecha_contratacion_str: String = row.get("fecha_contratacion")?;
    let estado_str: String = row.get("estado")?;
    let contrato_str: String = row.get("tipo_contrato")?;
    let created_str: String = row.get("created_at")?;
    let updated_str: String = row.get("updated_at")?;

    let departamento_id: Option<String> = row.get("departamento_id")?;
    let fecha_terminacion: Option<String> = row.get("fecha_terminacion")?;

    Ok(Empleado {
        id: Uuid::from_str(&id_str).unwrap_or_default(),
        cedula: row.get("cedula")?,
        nombre: row.get("nombre")?,
        apellido: row.get("apellido")?,
        email: row.get("email")?,
        telefono: row.get("telefono")?,
        direccion: row.get("direccion")?,
        cargo: row.get("cargo")?,
        departamento_id: departamento_id.and_then(|s| Uuid::from_str(&s).ok()),
        fecha_contratacion: chrono::NaiveDate::parse_from_str(&fecha_contratacion_str, "%Y-%m-%d")
            .unwrap_or_default(),
        fecha_terminacion: fecha_terminacion.and_then(|s| chrono::NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok()),
        salario: row.get("salario")?,
        tipo_contrato: texto_a_contrato(&contrato_str),
        estado: texto_a_estado(&estado_str),
        notas: row.get("notas")?,
        created_at: chrono::DateTime::parse_from_rfc3339(&created_str)
            .map(|d| d.with_timezone(&chrono::Utc))
            .unwrap_or_else(|_| chrono::Utc::now()),
        updated_at: chrono::DateTime::parse_from_rfc3339(&updated_str)
            .map(|d| d.with_timezone(&chrono::Utc))
            .unwrap_or_else(|_| chrono::Utc::now()),
    })
}

fn fila_a_asistencia(row: &Row) -> rusqlite::Result<RegistroAsistencia> {
    let id_str: String = row.get("id")?;
    let empleado_id_str: String = row.get("empleado_id")?;
    let fecha_str: String = row.get("fecha")?;
    let hora_str: String = row.get("hora")?;
    let tipo_str: String = row.get("tipo")?;
    let creado_str: String = row.get("creado_en")?;

    Ok(RegistroAsistencia {
        id: Uuid::from_str(&id_str).unwrap_or_default(),
        empleado_id: Uuid::from_str(&empleado_id_str).unwrap_or_default(),
        fecha: chrono::NaiveDate::parse_from_str(&fecha_str, "%Y-%m-%d").unwrap_or_default(),
        hora: chrono::NaiveTime::parse_from_str(&hora_str, "%H:%M:%S").unwrap_or_default(),
        tipo: texto_a_tipo_asistencia(&tipo_str),
        notas: row.get("notas")?,
        creado_en: chrono::DateTime::parse_from_rfc3339(&creado_str)
            .map(|d| d.with_timezone(&chrono::Utc))
            .unwrap_or_else(|_| chrono::Utc::now()),
    })
}

impl<'a> EmpleadoRepositorio for SqliteEmpleadoRepo<'a> {
    fn guardar(&self, e: &Empleado) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO empleados
                (id, cedula, nombre, apellido, email, telefono, direccion, cargo,
                 departamento_id, fecha_contratacion, fecha_terminacion, salario,
                 tipo_contrato, estado, notas, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17)
             ON CONFLICT(id) DO UPDATE SET
                cedula=excluded.cedula, nombre=excluded.nombre, apellido=excluded.apellido,
                email=excluded.email, telefono=excluded.telefono, direccion=excluded.direccion,
                cargo=excluded.cargo, departamento_id=excluded.departamento_id,
                fecha_contratacion=excluded.fecha_contratacion, fecha_terminacion=excluded.fecha_terminacion,
                salario=excluded.salario, tipo_contrato=excluded.tipo_contrato,
                estado=excluded.estado, notas=excluded.notas, updated_at=excluded.updated_at",
            params![
                e.id.to_string(),
                e.cedula,
                e.nombre,
                e.apellido,
                e.email,
                e.telefono,
                e.direccion,
                e.cargo,
                e.departamento_id.map(|id| id.to_string()),
                e.fecha_contratacion.format("%Y-%m-%d").to_string(),
                e.fecha_terminacion.map(|d| d.format("%Y-%m-%d").to_string()),
                e.salario,
                contrato_a_texto(&e.tipo_contrato),
                estado_a_texto(&e.estado),
                e.notas,
                e.created_at.to_rfc3339(),
                e.updated_at.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    fn buscar_por_id(&self, id: Uuid) -> anyhow::Result<Option<Empleado>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM empleados WHERE id = ?1")?;
        let mut rows = stmt.query(params![id.to_string()])?;
        if let Some(row) = rows.next()? {
            Ok(Some(fila_a_empleado(row)?))
        } else {
            Ok(None)
        }
    }

    fn buscar_por_cedula(&self, cedula: &str) -> anyhow::Result<Option<Empleado>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM empleados WHERE cedula = ?1")?;
        let mut rows = stmt.query(params![cedula])?;
        if let Some(row) = rows.next()? {
            Ok(Some(fila_a_empleado(row)?))
        } else {
            Ok(None)
        }
    }

    fn listar_todos(&self) -> anyhow::Result<Vec<Empleado>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM empleados ORDER BY apellido, nombre")?;
        let iter = stmt.query_map([], fila_a_empleado)?;
        Ok(iter.filter_map(|r| r.ok()).collect())
    }

    fn listar_por_estado(&self, estado: &EstadoEmpleado) -> anyhow::Result<Vec<Empleado>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM empleados WHERE estado = ?1 ORDER BY apellido, nombre")?;
        let iter = stmt.query_map(params![estado_a_texto(estado)], fila_a_empleado)?;
        Ok(iter.filter_map(|r| r.ok()).collect())
    }

    fn listar_por_departamento(&self, departamento_id: Uuid) -> anyhow::Result<Vec<Empleado>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM empleados WHERE departamento_id = ?1 ORDER BY apellido, nombre")?;
        let iter = stmt.query_map(params![departamento_id.to_string()], fila_a_empleado)?;
        Ok(iter.filter_map(|r| r.ok()).collect())
    }

    fn eliminar(&self, id: Uuid) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM empleados WHERE id = ?1", params![id.to_string()])?;
        Ok(())
    }
}

impl<'a> AsistenciaRepositorio for SqliteEmpleadoRepo<'a> {
    fn guardar_registro(&self, registro: &RegistroAsistencia) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO registros_asistencia
                (id, empleado_id, fecha, hora, tipo, notas, creado_en)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                registro.id.to_string(),
                registro.empleado_id.to_string(),
                registro.fecha.format("%Y-%m-%d").to_string(),
                registro.hora.format("%H:%M:%S").to_string(),
                tipo_asistencia_a_texto(&registro.tipo),
                registro.notas,
                registro.creado_en.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    fn listar_por_empleado(&self, empleado_id: Uuid) -> anyhow::Result<Vec<RegistroAsistencia>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT * FROM registros_asistencia WHERE empleado_id = ?1 ORDER BY fecha DESC, hora DESC",
        )?;
        let iter = stmt.query_map(params![empleado_id.to_string()], fila_a_asistencia)?;
        Ok(iter.filter_map(|r| r.ok()).collect())
    }

    fn listar_por_fecha(&self, fecha: chrono::NaiveDate) -> anyhow::Result<Vec<RegistroAsistencia>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT * FROM registros_asistencia WHERE fecha = ?1 ORDER BY hora ASC",
        )?;
        let iter = stmt.query_map(params![fecha.format("%Y-%m-%d").to_string()], fila_a_asistencia)?;
        Ok(iter.filter_map(|r| r.ok()).collect())
    }

    fn listar_por_empleado_y_fecha(
        &self,
        empleado_id: Uuid,
        fecha: chrono::NaiveDate,
    ) -> anyhow::Result<Vec<RegistroAsistencia>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT * FROM registros_asistencia WHERE empleado_id = ?1 AND fecha = ?2 ORDER BY hora ASC",
        )?;
        let iter = stmt.query_map(
            params![empleado_id.to_string(), fecha.format("%Y-%m-%d").to_string()],
            fila_a_asistencia,
        )?;
        Ok(iter.filter_map(|r| r.ok()).collect())
    }

    fn eliminar_registro(&self, id: Uuid) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM registros_asistencia WHERE id = ?1", params![id.to_string()])?;
        Ok(())
    }
}
