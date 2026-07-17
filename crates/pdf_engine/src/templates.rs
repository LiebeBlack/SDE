/// Encabezados y pies de página reutilizables entre distintos tipos de
/// documentos (constancias, reportes, carnets). Se irá ampliando conforme
/// se agreguen más tipos de PDF institucional.
pub struct EncabezadoInstitucional {
    pub nombre_institucion: String,
    pub logo_ruta: Option<String>,
}

impl EncabezadoInstitucional {
    pub fn nuevo(nombre_institucion: impl Into<String>) -> Self {
        Self {
            nombre_institucion: nombre_institucion.into(),
            logo_ruta: None,
        }
    }
}
