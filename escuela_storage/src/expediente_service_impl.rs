use async_trait::async_trait;
use escuela_core::domain::{expediente::{ExpedienteDocente, ExpedienteId, EstadoExpediente}, documento::Documento, usuario::UsuarioId};
use escuela_core::services::expediente_service::ExpedienteService;
use escuela_shared::{AppResult, AppError};
use crate::repositories::{ExpedienteRepository, DocumentoRepository};
use chrono::Utc;

pub struct ExpedienteServiceImpl {
    expediente_repo: ExpedienteRepository,
    documento_repo: DocumentoRepository,
}

impl ExpedienteServiceImpl {
    pub fn new(expediente_repo: ExpedienteRepository, documento_repo: DocumentoRepository) -> Self {
        Self { expediente_repo, documento_repo }
    }
}

#[async_trait]
impl ExpedienteService for ExpedienteServiceImpl {
    async fn crear_expediente(&self, expediente: ExpedienteDocente) -> AppResult<ExpedienteId> {
        // Verificar que no exista expediente con misma cédula
        if let Ok(_) = self.expediente_repo.obtener_por_cedula(expediente.cedula.as_str()).await {
            return Err(AppError::ValidationError("Ya existe un expediente con esta cédula".to_string()));
        }
        
        let expediente_id = expediente.id.clone();
        self.expediente_repo.crear(&expediente).await?;
        Ok(expediente_id)
    }

    async fn obtener_expediente(&self, id: &ExpedienteId) -> AppResult<ExpedienteDocente> {
        self.expediente_repo.obtener_por_id(id).await
    }

    async fn obtener_expediente_por_cedula(&self, cedula: &str) -> AppResult<ExpedienteDocente> {
        self.expediente_repo.obtener_por_cedula(cedula).await
    }

    async fn listar_expedientes(&self) -> AppResult<Vec<ExpedienteDocente>> {
        self.expediente_repo.listar().await
    }

    async fn actualizar_expediente(&self, mut expediente: ExpedienteDocente) -> AppResult<()> {
        // Verificar que el expediente existe
        let expediente_existente = self.expediente_repo.obtener_por_id(&expediente.id).await?;
        
        // Validar unicidad de cédula si cambió
        if expediente_existente.cedula.as_str() != expediente.cedula.as_str() {
            if let Ok(_) = self.expediente_repo.obtener_por_cedula(expediente.cedula.as_str()).await {
                return Err(AppError::ValidationError("Ya existe un expediente con esta cédula".to_string()));
            }
        }
        
        expediente.actualizado_en = Some(Utc::now());
        self.expediente_repo.actualizar(&expediente).await
    }

    async fn eliminar_expediente(&self, id: &ExpedienteId) -> AppResult<()> {
        // Verificar que el expediente existe
        let expediente = self.expediente_repo.obtener_por_id(id).await?;
        
        // Validar que el expediente no tenga documentos sin foliar
        let documentos_sin_foliar = expediente.documentos.iter()
            .filter(|d| !d.foliado)
            .count();
        
        if documentos_sin_foliar > 0 {
            return Err(AppError::ValidationError(format!(
                "No se puede eliminar expediente con {} documentos sin foliar",
                documentos_sin_foliar
            )));
        }
        
        // Validar que el expediente esté en estado Archivado o Inactivo
        match expediente.estado {
            escuela_core::domain::expediente::EstadoExpediente::Activo => {
                return Err(AppError::ValidationError("No se puede eliminar expediente activo. Debe archivarse o inactivarse primero".to_string()));
            }
            escuela_core::domain::expediente::EstadoExpediente::Suspendido => {
                return Err(AppError::ValidationError("No se puede eliminar expediente suspendido. Debe archivarse o inactivarse primero".to_string()));
            }
            _ => {} // Permitir eliminar expedientes Inactivos o Archivados
        }
        
        // Los documentos se eliminarán en cascada por la FK en la base de datos
        self.expediente_repo.eliminar(id).await
    }

    async fn buscar_expedientes(&self, termino: &str) -> AppResult<Vec<ExpedienteDocente>> {
        self.expediente_repo.buscar(termino).await
    }

    async fn agregar_documento_a_expediente(&self, expediente_id: &ExpedienteId, documento: Documento) -> AppResult<()> {
        // Verificar que el expediente existe
        self.expediente_repo.obtener_por_id(expediente_id).await?;
        
        // Crear el documento
        self.documento_repo.crear(&documento, expediente_id).await
    }

    async fn remover_documento_de_expediente(&self, expediente_id: &ExpedienteId, documento_id: &escuela_core::domain::documento::DocumentoId) -> AppResult<()> {
        // Verificar que el expediente existe
        self.expediente_repo.obtener_por_id(expediente_id).await?;
        
        // Verificar que el documento existe y pertenece al expediente
        let documento = self.documento_repo.obtener_por_id(documento_id).await?;
        
        // Nota: Necesitamos verificar que el documento pertenece al expediente
        // Esto requeriría agregar un campo expediente_id en Documento o verificar en el repo
        
        self.documento_repo.eliminar(documento_id).await
    }

    async fn cambiar_estado_expediente(&self, id: &ExpedienteId, estado: EstadoExpediente, usuario_id: UsuarioId) -> AppResult<()> {
        self.expediente_repo.cambiar_estado_expediente(id, estado, usuario_id).await
    }
}
