use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Departamento {
    pub id: Uuid,
    pub nombre: String,
    pub descripcion: Option<String>,
    pub responsable: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Departamento {
    pub fn nuevo(nombre: String) -> Self {
        let ahora = Utc::now();
        Self {
            id: Uuid::new_v4(),
            nombre,
            descripcion: None,
            responsable: None,
            created_at: ahora,
            updated_at: ahora,
        }
    }
}
