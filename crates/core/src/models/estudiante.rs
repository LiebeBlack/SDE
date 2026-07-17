use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EstadoEstudiante {
    Activo,
    Inactivo,
    Egresado,
    Suspendido,
}

impl std::fmt::Display for EstadoEstudiante {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            EstadoEstudiante::Activo => "Activo",
            EstadoEstudiante::Inactivo => "Inactivo",
            EstadoEstudiante::Egresado => "Egresado",
            EstadoEstudiante::Suspendido => "Suspendido",
        };
        write!(f, "{s}")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Estudiante {
    pub id: Uuid,
    pub matricula: String,
    pub nombre: String,
    pub apellido: String,
    pub fecha_nacimiento: NaiveDate,
    pub grado_nivel: String, // ej: "3ro Primaria", "Semestre 5 - Ing. Sistemas"
    pub estado: EstadoEstudiante,
    pub direccion: Option<String>,
    pub telefono: Option<String>,
    pub email: Option<String>,
    pub notas: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Estudiante {
    pub fn nuevo(
        matricula: String,
        nombre: String,
        apellido: String,
        fecha_nacimiento: NaiveDate,
        grado_nivel: String,
    ) -> Self {
        let ahora = Utc::now();
        Self {
            id: Uuid::new_v4(),
            matricula,
            nombre,
            apellido,
            fecha_nacimiento,
            grado_nivel,
            estado: EstadoEstudiante::Activo,
            direccion: None,
            telefono: None,
            email: None,
            notas: None,
            created_at: ahora,
            updated_at: ahora,
        }
    }

    pub fn nombre_completo(&self) -> String {
        format!("{} {}", self.nombre, self.apellido)
    }
}
