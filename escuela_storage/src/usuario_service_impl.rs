use async_trait::async_trait;
use escuela_core::domain::usuario::{Usuario, UsuarioId, Rol};
use escuela_core::services::usuario_service::UsuarioService;
use escuela_shared::{AppResult, AppError};
use crate::repositories::UsuarioRepository;
use escuela_core::security::crypto::{verify_password, hash_password};
use chrono::Utc;

pub struct UsuarioServiceImpl {
    usuario_repo: UsuarioRepository,
}

impl UsuarioServiceImpl {
    pub fn new(usuario_repo: UsuarioRepository) -> Self {
        Self { usuario_repo }
    }
}

#[async_trait]
impl UsuarioService for UsuarioServiceImpl {
    async fn crear_usuario(&self, usuario: Usuario) -> AppResult<UsuarioId> {
        // Verificar que no exista usuario con mismo email
        if let Ok(_) = self.usuario_repo.obtener_por_email(usuario.email.as_str()).await {
            return Err(AppError::ValidationError("Ya existe un usuario con este email".to_string()));
        }
        
        // Verificar que no exista usuario con misma cédula
        if let Ok(_) = self.usuario_repo.obtener_por_cedula(usuario.cedula.as_str()).await {
            return Err(AppError::ValidationError("Ya existe un usuario con esta cédula".to_string()));
        }
        
        let usuario_id = usuario.id.clone();
        self.usuario_repo.crear(&usuario).await?;
        Ok(usuario_id)
    }

    async fn obtener_usuario(&self, id: &UsuarioId) -> AppResult<Usuario> {
        self.usuario_repo.obtener_por_id(id).await
    }

    async fn obtener_usuario_por_email(&self, email: &str) -> AppResult<Usuario> {
        self.usuario_repo.obtener_por_email(email).await
    }

    async fn obtener_usuario_por_cedula(&self, cedula: &str) -> AppResult<Usuario> {
        self.usuario_repo.obtener_por_cedula(cedula).await
    }

    async fn listar_usuarios(&self) -> AppResult<Vec<Usuario>> {
        self.usuario_repo.listar().await
    }

    async fn actualizar_usuario(&self, mut usuario: Usuario) -> AppResult<()> {
        // Verificar que el usuario existe
        self.usuario_repo.obtener_por_id(&usuario.id).await?;
        
        // Validar unicidad de email si cambió
        let usuario_existente = self.usuario_repo.obtener_por_id(&usuario.id).await?;
        if usuario_existente.email.as_str() != usuario.email.as_str() {
            if let Ok(_) = self.usuario_repo.obtener_por_email(usuario.email.as_str()).await {
                return Err(AppError::ValidationError("Ya existe un usuario con este email".to_string()));
            }
        }
        
        // Validar unicidad de cédula si cambió
        if usuario_existente.cedula.as_str() != usuario.cedula.as_str() {
            if let Ok(_) = self.usuario_repo.obtener_por_cedula(usuario.cedula.as_str()).await {
                return Err(AppError::ValidationError("Ya existe un usuario con esta cédula".to_string()));
            }
        }
        
        usuario.actualizado_en = Utc::now();
        self.usuario_repo.actualizar(&usuario).await
    }

    async fn eliminar_usuario(&self, id: &UsuarioId) -> AppResult<()> {
        // Verificar que el usuario existe
        let usuario = self.usuario_repo.obtener_por_id(id).await?;
        
        // No permitir eliminar al último Super usuario
        if usuario.rol == Rol::Super {
            let usuarios = self.usuario_repo.listar().await?;
            let super_count = usuarios.iter().filter(|u| u.rol == Rol::Super).count();
            if super_count <= 1 {
                return Err(AppError::AuthorizationError("No se puede eliminar el último usuario Super".to_string()));
            }
        }
        
        self.usuario_repo.eliminar(id).await
    }

    async fn cambiar_estado_usuario(&self, id: &UsuarioId, activo: bool) -> AppResult<()> {
        let mut usuario = self.usuario_repo.obtener_por_id(id).await?;
        
        // No permitir desactivar al último Super usuario
        if !activo && usuario.rol == Rol::Super {
            let usuarios = self.usuario_repo.listar().await?;
            let super_count = usuarios.iter().filter(|u| u.rol == Rol::Super && u.activo).count();
            if super_count <= 1 {
                return Err(AppError::AuthorizationError("No se puede desactivar el último usuario Super activo".to_string()));
            }
        }
        
        // Validar que el usuario no tenga expedientes activos asociados si se va a desactivar
        if !activo {
            // Aquí podríamos agregar validación de expedientes activos
            // Por ahora, permitimos la desactivación
        }
        
        if activo {
            usuario.activar();
        } else {
            usuario.desactivar();
        }
        
        self.usuario_repo.actualizar(&usuario).await
    }

    async fn cambiar_rol_usuario(&self, id: &UsuarioId, nuevo_rol: Rol) -> AppResult<()> {
        let mut usuario = self.usuario_repo.obtener_por_id(id).await?;
        
        // No permitir cambiar el rol del último Super usuario
        if usuario.rol == Rol::Super && nuevo_rol != Rol::Super {
            let usuarios = self.usuario_repo.listar().await?;
            let super_count = usuarios.iter().filter(|u| u.rol == Rol::Super).count();
            if super_count <= 1 {
                return Err(AppError::AuthorizationError("No se puede cambiar el rol del último usuario Super".to_string()));
            }
        }
        
        usuario.rol = nuevo_rol;
        usuario.actualizado_en = Utc::now();
        self.usuario_repo.actualizar(&usuario).await
    }

    async fn registrar_acceso(&self, id: &UsuarioId) -> AppResult<()> {
        let mut usuario = self.usuario_repo.obtener_por_id(id).await?;
        usuario.registrar_acceso();
        self.usuario_repo.actualizar(&usuario).await
    }

    async fn verificar_credenciales(&self, email: &str, password: &str) -> AppResult<Option<Usuario>> {
        match self.usuario_repo.obtener_por_email(email).await {
            Ok(usuario) => {
                if !usuario.activo {
                    return Ok(None);
                }
                
                match verify_password(password, &usuario.password_hash) {
                    Ok(true) => Ok(Some(usuario)),
                    Ok(false) => Ok(None),
                    Err(_) => Ok(None),
                }
            }
            Err(AppError::NotFound(_)) => Ok(None),
            Err(e) => Err(e),
        }
    }
}
