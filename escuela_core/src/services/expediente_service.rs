use async_trait::async_trait;
use escuela_shared::AppResult;
use crate::domain::{expediente::{ExpedienteDocente, ExpedienteId, EstadoExpediente}, documento::Documento, usuario::UsuarioId};

#[async_trait]
pub trait ExpedienteService: Send + Sync {
    async fn crear_expediente(&self, expediente: ExpedienteDocente) -> AppResult<ExpedienteId>;
    async fn obtener_expediente(&self, id: &ExpedienteId) -> AppResult<ExpedienteDocente>;
    async fn obtener_expediente_por_cedula(&self, cedula: &str) -> AppResult<ExpedienteDocente>;
    async fn listar_expedientes(&self) -> AppResult<Vec<ExpedienteDocente>>;
    async fn actualizar_expediente(&self, expediente: ExpedienteDocente) -> AppResult<()>;
    async fn eliminar_expediente(&self, id: &ExpedienteId) -> AppResult<()>;
    async fn buscar_expedientes(&self, termino: &str) -> AppResult<Vec<ExpedienteDocente>>;
    async fn agregar_documento_a_expediente(&self, expediente_id: &ExpedienteId, documento: Documento) -> AppResult<()>;
    async fn remover_documento_de_expediente(&self, expediente_id: &ExpedienteId, documento_id: &crate::domain::documento::DocumentoId) -> AppResult<()>;
    async fn cambiar_estado_expediente(&self, id: &ExpedienteId, estado: EstadoExpediente, usuario_id: UsuarioId) -> AppResult<()>;
}
