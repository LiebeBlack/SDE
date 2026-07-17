use app_core::models::{Documento, EntidadTipo, TipoDocumento};
use app_core::services::documento_service::DocumentoRepositorio;
use rusqlite::{params, Connection, Row};
use std::str::FromStr;
use std::sync::Mutex;
use uuid::Uuid;

pub struct SqliteDocumentoRepo<'a> {
    conn: &'a Mutex<Connection>,
}

impl<'a> SqliteDocumentoRepo<'a> {
    pub fn new(conn: &'a Mutex<Connection>) -> Self {
        Self { conn }
    }
}

fn entidad_a_texto(t: &EntidadTipo) -> &'static str {
    match t {
        EntidadTipo::Estudiante => "estudiante",
        EntidadTipo::Familiar => "familiar",
        EntidadTipo::Departamento => "departamento",
        EntidadTipo::Institucional => "institucional",
    }
}

fn texto_a_entidad(s: &str) -> EntidadTipo {
    match s {
        "familiar" => EntidadTipo::Familiar,
        "departamento" => EntidadTipo::Departamento,
        "institucional" => EntidadTipo::Institucional,
        _ => EntidadTipo::Estudiante,
    }
}

fn tipo_doc_a_texto(t: &TipoDocumento) -> String {
    match t {
        TipoDocumento::ActaNacimiento => "acta_nacimiento".to_string(),
        TipoDocumento::FotoPerfil => "foto_perfil".to_string(),
        TipoDocumento::CertificadoMedico => "certificado_medico".to_string(),
        TipoDocumento::CertificadoEstudios => "certificado_estudios".to_string(),
        TipoDocumento::Contrato => "contrato".to_string(),
        TipoDocumento::Identificacion => "identificacion".to_string(),
        TipoDocumento::Calificaciones => "calificaciones".to_string(),
        TipoDocumento::Otro(desc) => format!("otro:{desc}"),
    }
}

fn texto_a_tipo_doc(s: &str) -> TipoDocumento {
    match s {
        "acta_nacimiento" => TipoDocumento::ActaNacimiento,
        "foto_perfil" => TipoDocumento::FotoPerfil,
        "certificado_medico" => TipoDocumento::CertificadoMedico,
        "certificado_estudios" => TipoDocumento::CertificadoEstudios,
        "contrato" => TipoDocumento::Contrato,
        "identificacion" => TipoDocumento::Identificacion,
        "calificaciones" => TipoDocumento::Calificaciones,
        other => {
            if let Some(desc) = other.strip_prefix("otro:") {
                TipoDocumento::Otro(desc.to_string())
            } else {
                TipoDocumento::Otro(other.to_string())
            }
        }
    }
}

fn fila_a_documento(row: &Row) -> rusqlite::Result<Documento> {
    let id_str: String = row.get("id")?;
    let entidad_id_str: String = row.get("entidad_id")?;
    let entidad_tipo_str: String = row.get("entidad_tipo")?;
    let tipo_doc_str: String = row.get("tipo_documento")?;
    let subido_str: String = row.get("subido_en")?;

    Ok(Documento {
        id: Uuid::from_str(&id_str).unwrap_or_default(),
        entidad_tipo: texto_a_entidad(&entidad_tipo_str),
        entidad_id: Uuid::from_str(&entidad_id_str).unwrap_or_default(),
        tipo_documento: texto_a_tipo_doc(&tipo_doc_str),
        nombre_original: row.get("nombre_original")?,
        ruta_archivo: row.get("ruta_archivo")?,
        mime_type: row.get("mime_type")?,
        tamano_bytes: row.get("tamano_bytes")?,
        subido_en: chrono::DateTime::parse_from_rfc3339(&subido_str)
            .map(|d| d.with_timezone(&chrono::Utc))
            .unwrap_or_else(|_| chrono::Utc::now()),
    })
}

impl<'a> DocumentoRepositorio for SqliteDocumentoRepo<'a> {
    fn guardar(&self, d: &Documento) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO documentos
                (id, entidad_tipo, entidad_id, tipo_documento, nombre_original,
                 ruta_archivo, mime_type, tamano_bytes, subido_en)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                d.id.to_string(),
                entidad_a_texto(&d.entidad_tipo),
                d.entidad_id.to_string(),
                tipo_doc_a_texto(&d.tipo_documento),
                d.nombre_original,
                d.ruta_archivo,
                d.mime_type,
                d.tamano_bytes,
                d.subido_en.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    fn listar_por_entidad(
        &self,
        entidad_tipo: &EntidadTipo,
        entidad_id: Uuid,
    ) -> anyhow::Result<Vec<Documento>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT * FROM documentos WHERE entidad_tipo = ?1 AND entidad_id = ?2
             ORDER BY subido_en DESC",
        )?;
        let iter = stmt.query_map(
            params![entidad_a_texto(entidad_tipo), entidad_id.to_string()],
            fila_a_documento,
        )?;
        Ok(iter.filter_map(|r| r.ok()).collect())
    }

    fn eliminar(&self, id: Uuid) -> anyhow::Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM documentos WHERE id = ?1", params![id.to_string()])?;
        Ok(())
    }
}
