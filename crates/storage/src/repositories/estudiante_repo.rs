use app_core::models::estudiante::EstadoEstudiante;
use app_core::models::Estudiante;
use app_core::services::estudiante_service::EstudianteRepositorio;
use rusqlite::{params, Connection, Row};
use std::str::FromStr;
use std::sync::Mutex;
use uuid::Uuid;

pub struct SqliteEstudianteRepo<'a> {
    conn: &'a Mutex<Connection>,
}

impl<'a> SqliteEstudianteRepo<'a> {
    pub fn new(conn: &'a Mutex<Connection>) -> Self {
        Self { conn }
    }
}

fn estado_a_texto(estado: &EstadoEstudiante) -> &'static str {
    match estado {
        EstadoEstudiante::Activo => "activo",
        EstadoEstudiante::Inactivo => "inactivo",
        EstadoEstudiante::Egresado => "egresado",
        EstadoEstudiante::Suspendido => "suspendido",
    }
}

fn texto_a_estado(texto: &str) -> EstadoEstudiante {
    match texto {
        "inactivo" => EstadoEstudiante::Inactivo,
        "egresado" => EstadoEstudiante::Egresado,
        "suspendido" => EstadoEstudiante::Suspendido,
        _ => EstadoEstudiante::Activo,
    }
}

fn fila_a_estudiante(row: &Row) -> rusqlite::Result<Estudiante> {
    let id_str: String = row.get("id")?;
    let fecha_nac_str: String = row.get("fecha_nacimiento")?;
    let estado_str: String = row.get("estado")?;
    let created_str: String = row.get("created_at")?;
    let updated_str: String = row.get("updated_at")?;

    Ok(Estudiante {
        id: Uuid::from_str(&id_str).unwrap_or_default(),
        matricula: row.get("matricula")?,
        nombre: row.get("nombre")?,
        apellido: row.get("apellido")?,
        fecha_nacimiento: chrono::NaiveDate::parse_from_str(&fecha_nac_str, "%Y-%m-%d")
            .unwrap_or_default(),
        grado_nivel: row.get("grado_nivel")?,
        estado: texto_a_estado(&estado_str),
        direccion: row.get("direccion")?,
        telefono: row.get("telefono")?,
        email: row.get("email")?,
        notas: row.get("notas")?,
        created_at: chrono::DateTime::parse_from_rfc3339(&created_str)
            .map(|d| d.with_timezone(&chrono::Utc))
            .unwrap_or_else(|_| chrono::Utc::now()),
        updated_at: chrono::DateTime::parse_from_rfc3339(&updated_str)
            .map(|d| d.with_timezone(&chrono::Utc))
            .unwrap_or_else(|_| chrono::Utc::now()),
    })
}

impl<'a> EstudianteRepositorio for SqliteEstudianteRepo<'a> {
    fn guardar(&self, e: &Estudiante) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO estudiantes
                (id, matricula, nombre, apellido, fecha_nacimiento, grado_nivel, estado,
                 direccion, telefono, email, notas, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)
             ON CONFLICT(id) DO UPDATE SET
                matricula=excluded.matricula, nombre=excluded.nombre, apellido=excluded.apellido,
                fecha_nacimiento=excluded.fecha_nacimiento, grado_nivel=excluded.grado_nivel,
                estado=excluded.estado, direccion=excluded.direccion, telefono=excluded.telefono,
                email=excluded.email, notas=excluded.notas, updated_at=excluded.updated_at",
            params![
                e.id.to_string(),
                e.matricula,
                e.nombre,
                e.apellido,
                e.fecha_nacimiento.format("%Y-%m-%d").to_string(),
                e.grado_nivel,
                estado_a_texto(&e.estado),
                e.direccion,
                e.telefono,
                e.email,
                e.notas,
                e.created_at.to_rfc3339(),
                e.updated_at.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    fn buscar_por_id(&self, id: Uuid) -> anyhow::Result<Option<Estudiante>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM estudiantes WHERE id = ?1")?;
        let mut rows = stmt.query(params![id.to_string()])?;
        if let Some(row) = rows.next()? {
            Ok(Some(fila_a_estudiante(row)?))
        } else {
            Ok(None)
        }
    }

    fn listar_todos(&self) -> anyhow::Result<Vec<Estudiante>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM estudiantes ORDER BY apellido, nombre")?;
        let iter = stmt.query_map([], fila_a_estudiante)?;
        Ok(iter.filter_map(|r| r.ok()).collect())
    }

    fn buscar_por_nombre(&self, texto: &str) -> anyhow::Result<Vec<Estudiante>> {
        let conn = self.conn.lock().unwrap();
        let patron = format!("%{}%", texto.to_lowercase());
        let mut stmt = conn.prepare(
            "SELECT * FROM estudiantes
             WHERE lower(nombre) LIKE ?1 OR lower(apellido) LIKE ?1 OR lower(matricula) LIKE ?1
             ORDER BY apellido, nombre",
        )?;
        let iter = stmt.query_map(params![patron], fila_a_estudiante)?;
        Ok(iter.filter_map(|r| r.ok()).collect())
    }

    fn eliminar(&self, id: Uuid) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM estudiantes WHERE id = ?1", params![id.to_string()])?;
        Ok(())
    }
}
