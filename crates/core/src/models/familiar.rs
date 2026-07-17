use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Parentesco {
    Padre,
    Madre,
    TutorLegal,
    Abuelo,
    Abuela,
    Hermano,
    Otro(String),
}

impl std::fmt::Display for Parentesco {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Parentesco::Padre => write!(f, "Padre"),
            Parentesco::Madre => write!(f, "Madre"),
            Parentesco::TutorLegal => write!(f, "Tutor legal"),
            Parentesco::Abuelo => write!(f, "Abuelo"),
            Parentesco::Abuela => write!(f, "Abuela"),
            Parentesco::Hermano => write!(f, "Hermano/a"),
            Parentesco::Otro(desc) => write!(f, "Otro ({desc})"),
        }
    }
}

/// Un familiar/tutor titular. Puede estar asociado a varios estudiantes
/// (ej. hermanos) mediante RelacionFamiliar.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Familiar {
    pub id: Uuid,
    pub nombre: String,
    pub apellido: String,
    pub documento_identidad: Option<String>,
    pub telefono: Option<String>,
    pub telefono_alterno: Option<String>,
    pub email: Option<String>,
    pub direccion: Option<String>,
    pub ocupacion: Option<String>,
    pub es_contacto_emergencia: bool,
    pub notas: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Familiar {
    pub fn nuevo(nombre: String, apellido: String) -> Self {
        let ahora = Utc::now();
        Self {
            id: Uuid::new_v4(),
            nombre,
            apellido,
            documento_identidad: None,
            telefono: None,
            telefono_alterno: None,
            email: None,
            direccion: None,
            ocupacion: None,
            es_contacto_emergencia: false,
            notas: None,
            created_at: ahora,
            updated_at: ahora,
        }
    }

    pub fn nombre_completo(&self) -> String {
        format!("{} {}", self.nombre, self.apellido)
    }
}

/// Tabla puente estudiante <-> familiar, con el tipo de parentesco.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelacionFamiliar {
    pub id: Uuid,
    pub estudiante_id: Uuid,
    pub familiar_id: Uuid,
    pub parentesco: Parentesco,
    pub es_titular_responsable: bool,
}
