use app_core::models::{Familiar, Parentesco, RelacionFamiliar};
use app_core::services::familiar_service::{FamiliarRepositorio, RelacionFamiliarRepositorio};
use rusqlite::{params, Connection, Row};
use std::str::FromStr;
use std::sync::Mutex;
use uuid::Uuid;

pub struct SqliteFamiliarRepo<'a> {
    conn: &'a Mutex<Connection>,
}

impl<'a> SqliteFamiliarRepo<'a> {
    pub fn new(conn: &'a Mutex<Connection>) -> Self {
        Self { conn }
    }
}

fn parentesco_a_texto(p: &Parentesco) -> String {
    match p {
        Parentesco::Padre => "padre".to_string(),
        Parentesco::Madre => "madre".to_string(),
        Parentesco::TutorLegal => "tutor_legal".to_string(),
        Parentesco::Abuelo => "abuelo".to_string(),
        Parentesco::Abuela => "abuela".to_string(),
        Parentesco::Hermano => "hermano".to_string(),
        Parentesco::Otro(desc) => format!("otro:{desc}"),
    }
}

fn texto_a_parentesco(s: &str) -> Parentesco {
    match s {
        "padre" => Parentesco::Padre,
        "madre" => Parentesco::Madre,
        "tutor_legal" => Parentesco::TutorLegal,
        "abuelo" => Parentesco::Abuelo,
        "abuela" => Parentesco::Abuela,
        "hermano" => Parentesco::Hermano,
        other => {
            if let Some(desc) = other.strip_prefix("otro:") {
                Parentesco::Otro(desc.to_string())
            } else {
                Parentesco::Otro(other.to_string())
            }
        }
    }
}

fn fila_a_familiar(row: &Row) -> rusqlite::Result<Familiar> {
    let id_str: String = row.get("id")?;
    let created_str: String = row.get("created_at")?;
    let updated_str: String = row.get("updated_at")?;

    Ok(Familiar {
        id: Uuid::from_str(&id_str).unwrap_or_default(),
        nombre: row.get("nombre")?,
        apellido: row.get("apellido")?,
        documento_identidad: row.get("documento_identidad")?,
        telefono: row.get("telefono")?,
        telefono_alterno: row.get("telefono_alterno")?,
        email: row.get("email")?,
        direccion: row.get("direccion")?,
        ocupacion: row.get("ocupacion")?,
        es_contacto_emergencia: row.get("es_contacto_emergencia")?,
        notas: row.get("notas")?,
        created_at: chrono::DateTime::parse_from_rfc3339(&created_str)
            .map(|d| d.with_timezone(&chrono::Utc))
            .unwrap_or_else(|_| chrono::Utc::now()),
        updated_at: chrono::DateTime::parse_from_rfc3339(&updated_str)
            .map(|d| d.with_timezone(&chrono::Utc))
            .unwrap_or_else(|_| chrono::Utc::now()),
    })
}

fn fila_a_relacion(row: &Row) -> rusqlite::Result<RelacionFamiliar> {
    let id_str: String = row.get("id")?;
    let estudiante_id_str: String = row.get("estudiante_id")?;
    let familiar_id_str: String = row.get("familiar_id")?;
    let parentesco_str: String = row.get("parentesco")?;

    Ok(RelacionFamiliar {
        id: Uuid::from_str(&id_str).unwrap_or_default(),
        estudiante_id: Uuid::from_str(&estudiante_id_str).unwrap_or_default(),
        familiar_id: Uuid::from_str(&familiar_id_str).unwrap_or_default(),
        parentesco: texto_a_parentesco(&parentesco_str),
        es_titular_responsable: row.get("es_titular_responsable")?,
    })
}

impl<'a> FamiliarRepositorio for SqliteFamiliarRepo<'a> {
    fn guardar(&self, f: &Familiar) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO familiares
                (id, nombre, apellido, documento_identidad, telefono, telefono_alterno,
                 email, direccion, ocupacion, es_contacto_emergencia, notas, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)
             ON CONFLICT(id) DO UPDATE SET
                nombre=excluded.nombre, apellido=excluded.apellido,
                documento_identidad=excluded.documento_identidad, telefono=excluded.telefono,
                telefono_alterno=excluded.telefono_alterno, email=excluded.email,
                direccion=excluded.direccion, ocupacion=excluded.ocupacion,
                es_contacto_emergencia=excluded.es_contacto_emergencia, notas=excluded.notas,
                updated_at=excluded.updated_at",
            params![
                f.id.to_string(),
                f.nombre,
                f.apellido,
                f.documento_identidad,
                f.telefono,
                f.telefono_alterno,
                f.email,
                f.direccion,
                f.ocupacion,
                f.es_contacto_emergencia,
                f.notas,
                f.created_at.to_rfc3339(),
                f.updated_at.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    fn buscar_por_id(&self, id: Uuid) -> anyhow::Result<Option<Familiar>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM familiares WHERE id = ?1")?;
        let mut rows = stmt.query(params![id.to_string()])?;
        if let Some(row) = rows.next()? {
            Ok(Some(fila_a_familiar(row)?))
        } else {
            Ok(None)
        }
    }

    fn listar_todos(&self) -> anyhow::Result<Vec<Familiar>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM familiares ORDER BY apellido, nombre")?;
        let iter = stmt.query_map([], fila_a_familiar)?;
        Ok(iter.filter_map(|r| r.ok()).collect())
    }

    fn eliminar(&self, id: Uuid) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM familiares WHERE id = ?1", params![id.to_string()])?;
        Ok(())
    }
}

impl<'a> RelacionFamiliarRepositorio for SqliteFamiliarRepo<'a> {
    fn guardar_relacion(&self, r: &RelacionFamiliar) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO relaciones_familiares
                (id, estudiante_id, familiar_id, parentesco, es_titular_responsable)
             VALUES (?1, ?2, ?3, ?4, ?5)
             ON CONFLICT(id) DO UPDATE SET
                estudiante_id=excluded.estudiante_id, familiar_id=excluded.familiar_id,
                parentesco=excluded.parentesco, es_titular_responsable=excluded.es_titular_responsable",
            params![
                r.id.to_string(),
                r.estudiante_id.to_string(),
                r.familiar_id.to_string(),
                parentesco_a_texto(&r.parentesco),
                r.es_titular_responsable,
            ],
        )?;
        Ok(())
    }

    fn listar_por_estudiante(&self, estudiante_id: Uuid) -> anyhow::Result<Vec<RelacionFamiliar>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT * FROM relaciones_familiares WHERE estudiante_id = ?1 ORDER BY es_titular_responsable DESC",
        )?;
        let iter = stmt.query_map(params![estudiante_id.to_string()], fila_a_relacion)?;
        Ok(iter.filter_map(|r| r.ok()).collect())
    }

    fn listar_por_familiar(&self, familiar_id: Uuid) -> anyhow::Result<Vec<RelacionFamiliar>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT * FROM relaciones_familiares WHERE familiar_id = ?1")?;
        let iter = stmt.query_map(params![familiar_id.to_string()], fila_a_relacion)?;
        Ok(iter.filter_map(|r| r.ok()).collect())
    }

    fn eliminar_relacion(&self, id: Uuid) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM relaciones_familiares WHERE id = ?1", params![id.to_string()])?;
        Ok(())
    }
}
