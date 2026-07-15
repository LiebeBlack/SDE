use async_trait::async_trait;
use escuela_shared::AppResult;
use crate::domain::usuario::{Usuario, UsuarioId, Rol};

#[async_trait]
pub trait UsuarioService: Send + Sync {
    async fn crear_usuario(&self, usuario: Usuario) -> AppResult<UsuarioId>;
    async fn obtener_usuario(&self, id: &UsuarioId) -> AppResult<Usuario>;
    async fn obtener_usuario_por_email(&self, email: &str) -> AppResult<Usuario>;
    async fn obtener_usuario_por_cedula(&self, cedula: &str) -> AppResult<Usuario>;
    async fn listar_usuarios(&self) -> AppResult<Vec<Usuario>>;
    async fn actualizar_usuario(&self, usuario: Usuario) -> AppResult<()>;
    async fn eliminar_usuario(&self, id: &UsuarioId) -> AppResult<()>;
    async fn cambiar_estado_usuario(&self, id: &UsuarioId, activo: bool) -> AppResult<()>;
    async fn cambiar_rol_usuario(&self, id: &UsuarioId, nuevo_rol: Rol) -> AppResult<()>;
    async fn registrar_acceso(&self, id: &UsuarioId) -> AppResult<()>;
    async fn verificar_credenciales(&self, email: &str, password: &str) -> AppResult<Option<Usuario>>;
}
