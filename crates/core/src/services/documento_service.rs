use crate::models::{Documento, EntidadTipo, TipoDocumento};
use crate::validation::validar_extension_permitida;
use uuid::Uuid;

pub trait DocumentoRepositorio {
    fn guardar(&self, documento: &Documento) -> anyhow::Result<()>;
    fn listar_por_entidad(
        &self,
        entidad_tipo: &EntidadTipo,
        entidad_id: Uuid,
    ) -> anyhow::Result<Vec<Documento>>;
    fn eliminar(&self, id: Uuid) -> anyhow::Result<()>;
}

/// Encargado de copiar el archivo físico a un almacén local (data/documentos)
/// y devolver la ruta relativa donde quedó guardado. Implementado en la app
/// o en storage, según de dónde venga el archivo (diálogo nativo de egui).
pub trait AlmacenArchivos {
    fn guardar_archivo(&self, origen: &std::path::Path, id: Uuid) -> anyhow::Result<String>;
}

pub struct DocumentoService<'a, R: DocumentoRepositorio, A: AlmacenArchivos> {
    repo: &'a R,
    almacen: &'a A,
}

impl<'a, R: DocumentoRepositorio, A: AlmacenArchivos> DocumentoService<'a, R, A> {
    pub fn new(repo: &'a R, almacen: &'a A) -> Self {
        Self { repo, almacen }
    }

    pub fn adjuntar(
        &self,
        entidad_tipo: EntidadTipo,
        entidad_id: Uuid,
        tipo_documento: TipoDocumento,
        ruta_origen: &std::path::Path,
        mime_type: String,
        tamano_bytes: i64,
    ) -> anyhow::Result<Documento> {
        let nombre_original = ruta_origen
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "archivo".to_string());

        validar_extension_permitida(&nombre_original)?;

        let id = Uuid::new_v4();
        let ruta_final = self.almacen.guardar_archivo(ruta_origen, id)?;

        let documento = Documento {
            id,
            entidad_tipo,
            entidad_id,
            tipo_documento,
            nombre_original,
            ruta_archivo: ruta_final,
            mime_type,
            tamano_bytes,
            subido_en: chrono::Utc::now(),
        };

        self.repo.guardar(&documento)?;
        Ok(documento)
    }

    pub fn listar_de(
        &self,
        entidad_tipo: EntidadTipo,
        entidad_id: Uuid,
    ) -> anyhow::Result<Vec<Documento>> {
        self.repo.listar_por_entidad(&entidad_tipo, entidad_id)
    }

    pub fn eliminar(&self, id: Uuid) -> anyhow::Result<()> {
        self.repo.eliminar(id)
    }
}
