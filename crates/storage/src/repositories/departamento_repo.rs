use app_core::models::Departamento;
use app_core::services::departamento_service::DepartamentoRepositorio;
use rusqlite::{params, Connection, Row};
use std::str::FromStr;
use std::sync::Mutex;
use uuid::Uuid;

pub struct SqliteDepartamentoRepo<'a> {
    conn: &'a Mutex<Connection>,
}

impl<'a> SqliteDepartamentoRepo<'a> {
    pub fn new(conn: &'a Mutex<Connection>) -> Self {
        Self { conn }
    }
}

fn fila_a_departamento(row: &Row) -> rusqlite::Result<Departamento> {
    let id_str: String = row.get("id")?;
    let created_str: String = row.get("created_at")?;
    let updated_str: String = row.get("updated_at")?;

    Ok(Departamento {
        id: Uuid::from_str(&id_str).unwrap_or_default(),
        nombre: row.get("nombre")?,
        descripcion: row.get("descripcion")?,
        responsable: row.get("responsable")?,
        created_at: chrono::DateTime::parse_from_rfc3339(&created_str)
            .map(|d| d.with_timezone(&chrono::Utc))
            .unwrap_or_else(|_| chrono::Utc::now()),
        updated_at: chrono::DateTime::parse_from_rfc3339(&updated_str)
            .map(|d| d.with_timezone(&chrono::Utc))
            .unwrap_or_else(|_| chrono::Utc::now()),
    })
}

impl<'a> DepartamentoRepositorio for SqliteDepartamentoRepo<'a> {
    fn guardar(&self, d: &Departamento) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO departamentos
                (id, nombre, descripcion, responsable, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)
             ON CONFLICT(id) DO UPDATE SET
                nombre=excluded.nombre, descripcion=excluded.descripcion,
                responsable=excluded.responsable, updated_at=excluded.updated_at",
            params![
                d.id.to_string(),
                d.nombre,
                d.descripcion,
                d.responsable,
                d.created_at.to_rfc3339(),
                d.updated_at.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    fn buscar_por_id(&self, id: Uuid) -> anyhow::Result<Option<Departamento>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM departamentos WHERE id = ?1")?;
        let mut rows = stmt.query(params![id.to_string()])?;
        if let Some(row) = rows.next()? {
            Ok(Some(fila_a_departamento(row)?))
        } else {
            Ok(None)
        }
    }

    fn listar_todos(&self) -> anyhow::Result<Vec<Departamento>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM departamentos ORDER BY nombre")?;
        let iter = stmt.query_map([], fila_a_departamento)?;
        Ok(iter.filter_map(|r| r.ok()).collect())
    }

    fn eliminar(&self, id: Uuid) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM departamentos WHERE id = ?1", params![id.to_string()])?;
        Ok(())
    }
}
