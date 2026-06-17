use async_trait::async_trait;
use escuela_shared::AppResult;
use crate::domain::{documento::{Documento, DocumentoId, CategoriaDocumento}, expediente::ExpedienteId};

#[async_trait]
pub trait DocumentoService: Send + Sync {
    async fn crear_documento(&self, documento: Documento) -> AppResult<DocumentoId>;
    async fn obtener_documento(&self, id: &DocumentoId) -> AppResult<Documento>;
    async fn listar_documentos(&self, expediente_id: &ExpedienteId) -> AppResult<Vec<Documento>>;
    async fn listar_documentos_por_categoria(&self, expediente_id: &ExpedienteId, categoria: &CategoriaDocumento) -> AppResult<Vec<Documento>>;
    async fn actualizar_documento(&self, documento: Documento) -> AppResult<()>;
    async fn eliminar_documento(&self, id: &DocumentoId) -> AppResult<()>;
    async fn foliar_documento(&self, id: &DocumentoId) -> AppResult<()>;
    async fn buscar_documentos(&self, termino: &str) -> AppResult<Vec<Documento>>;
    async fn verificar_integridad_documento(&self, id: &DocumentoId) -> AppResult<bool>;
}
