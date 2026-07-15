use async_trait::async_trait;
use escuela_core::domain::documento::{Documento, DocumentoId, CategoriaDocumento};
use escuela_core::domain::expediente::ExpedienteId;
use escuela_core::services::documento_service::DocumentoService;
use escuela_shared::{AppResult, AppError};
use crate::repositories::DocumentoRepository;
use crate::file_storage::FileStorageService;
use chrono::Utc;
use uuid;

pub struct DocumentoServiceImpl {
    documento_repo: DocumentoRepository,
    file_storage: FileStorageService,
}

impl DocumentoServiceImpl {
    pub fn new(documento_repo: DocumentoRepository, file_storage: FileStorageService) -> Self {
        Self { documento_repo, file_storage }
    }
}

#[async_trait]
impl DocumentoService for DocumentoServiceImpl {
    async fn crear_documento(&self, documento: Documento) -> AppResult<DocumentoId> {
        // Verificar que el archivo existe en el almacenamiento
        if !self.file_storage.verificar_archivo_existe(&documento.ruta_local) {
            return Err(AppError::NotFound("Archivo no encontrado en almacenamiento".to_string()));
        }
        
        // Verificar integridad del archivo
        let archivo_bytes = self.file_storage.leer_archivo(&documento.ruta_local).await?;
        if !documento.verificar_integridad_archivo(&archivo_bytes) {
            return Err(AppError::InternalError("Integridad del archivo comprometida".to_string()));
        }
        
        let documento_id = documento.id.clone();
        self.documento_repo.crear(&documento, &ExpedienteId::from_uuid(uuid::Uuid::new_v4())).await?;
        Ok(documento_id)
    }

    async fn obtener_documento(&self, id: &DocumentoId) -> AppResult<Documento> {
        let documento = self.documento_repo.obtener_por_id(id).await?;
        
        // Verificar que el archivo existe en almacenamiento
        if !self.file_storage.verificar_archivo_existe(&documento.ruta_local) {
            return Err(AppError::NotFound("Archivo no encontrado en almacenamiento".to_string()));
        }
        
        Ok(documento)
    }

    async fn listar_documentos(&self, expediente_id: &ExpedienteId) -> AppResult<Vec<Documento>> {
        let documentos = self.documento_repo.listar_por_expediente(expediente_id).await?;
        
        // Filtrar solo documentos que existen en almacenamiento
        let documentos_validos: Vec<Documento> = documentos
            .into_iter()
            .filter(|doc| self.file_storage.verificar_archivo_existe(&doc.ruta_local))
            .collect();
        
        Ok(documentos_validos)
    }

    async fn listar_documentos_por_categoria(&self, expediente_id: &ExpedienteId, categoria: &CategoriaDocumento) -> AppResult<Vec<Documento>> {
        let documentos = self.documento_repo.listar_por_categoria(expediente_id, categoria).await?;
        
        // Filtrar solo documentos que existen en almacenamiento
        let documentos_validos: Vec<Documento> = documentos
            .into_iter()
            .filter(|doc| self.file_storage.verificar_archivo_existe(&doc.ruta_local))
            .collect();
        
        Ok(documentos_validos)
    }

    async fn actualizar_documento(&self, mut documento: Documento) -> AppResult<()> {
        // Verificar que el documento existe
        self.documento_repo.obtener_por_id(&documento.id).await?;
        
        // Verificar que el archivo existe en almacenamiento
        if !self.file_storage.verificar_archivo_existe(&documento.ruta_local) {
            return Err(AppError::NotFound("Archivo no encontrado en almacenamiento".to_string()));
        }
        
        // Verificar integridad del archivo si la ruta cambió
        documento.actualizado_en = Some(Utc::now());
        self.documento_repo.actualizar(&documento).await
    }

    async fn eliminar_documento(&self, id: &DocumentoId) -> AppResult<()> {
        let documento = self.documento_repo.obtener_por_id(id).await?;
        let ruta_local = documento.ruta_local.clone();
        
        // Validar que el documento esté foliado antes de eliminar
        if !documento.foliado {
            return Err(AppError::ValidationError("No se puede eliminar documento sin foliar".to_string()));
        }
        
        // Validar que el documento no sea crítico (título académico, cédula)
        match documento.categoria {
            escuela_core::domain::documento::CategoriaDocumento::TituloAcademico => {
                return Err(AppError::AuthorizationError("No se puede eliminar documento de título académico. Es crítico para el expediente".to_string()));
            }
            escuela_core::domain::documento::CategoriaDocumento::CedulaIdentidad => {
                return Err(AppError::AuthorizationError("No se puede eliminar documento de cédula de identidad. Es crítico para el expediente".to_string()));
            }
            _ => {} // Permitir eliminar otros documentos
        }
        
        // Eliminar registro de la base de datos
        self.documento_repo.eliminar(id).await?;
        
        // Eliminar archivo del almacenamiento
        if self.file_storage.verificar_archivo_existe(&ruta_local) {
            self.file_storage.eliminar_archivo(&ruta_local).await?;
        }
        
        Ok(())
    }

    async fn foliar_documento(&self, id: &DocumentoId) -> AppResult<()> {
        let mut documento = self.documento_repo.obtener_por_id(id).await?;
        
        // Verificar que el archivo existe en almacenamiento
        if !self.file_storage.verificar_archivo_existe(&documento.ruta_local) {
            return Err(AppError::NotFound("Archivo no encontrado en almacenamiento".to_string()));
        }
        
        // Verificar integridad del archivo antes de foliar
        let archivo_bytes = self.file_storage.leer_archivo(&documento.ruta_local).await?;
        if !documento.verificar_integridad_archivo(&archivo_bytes) {
            return Err(AppError::InternalError("No se puede foliar documento con integridad comprometida".to_string()));
        }
        
        documento.foliar();
        self.documento_repo.actualizar(&documento).await
    }

    async fn buscar_documentos(&self, termino: &str) -> AppResult<Vec<Documento>> {
        let documentos = self.documento_repo.buscar(termino).await?;
        
        // Filtrar solo documentos que existen en almacenamiento
        let documentos_validos: Vec<Documento> = documentos
            .into_iter()
            .filter(|doc| self.file_storage.verificar_archivo_existe(&doc.ruta_local))
            .collect();
        
        Ok(documentos_validos)
    }

    async fn verificar_integridad_documento(&self, id: &DocumentoId) -> AppResult<bool> {
        let documento = self.documento_repo.obtener_por_id(id).await?;
        
        // Verificar que el archivo existe
        if !self.file_storage.verificar_archivo_existe(&documento.ruta_local) {
            return Ok(false);
        }
        
        // Verificar integridad del archivo
        let archivo_bytes = self.file_storage.leer_archivo(&documento.ruta_local).await?;
        Ok(documento.verificar_integridad_archivo(&archivo_bytes))
    }
}
