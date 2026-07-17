use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistroAuditoria {
    pub id: Uuid,
    pub entidad: String,
    pub entidad_id: Uuid,
    pub accion: String, // "crear", "editar", "eliminar"
    pub detalle: Option<String>,
    pub fecha: DateTime<Utc>,
}
