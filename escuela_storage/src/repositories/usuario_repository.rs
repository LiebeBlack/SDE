use sqlx::SqlitePool;
use escuela_core::domain::usuario::{Usuario, UsuarioId, Rol};
use escuela_shared::{AppResult, AppError, Email, Cedula};
use chrono::{DateTime, Utc};

pub struct UsuarioRepository {
    pool: SqlitePool,
}

impl UsuarioRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn crear(&self, usuario: &Usuario) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO usuarios (id, nombre, apellido, email, cedula, password_hash, rol, telefono, activo, creado_en, actualizado_en, ultimo_acceso)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(usuario.id.as_uuid().to_string())
        .bind(&usuario.nombre)
        .bind(&usuario.apellido)
        .bind(usuario.email.as_str())
        .bind(usuario.cedula.as_str())
        .bind(&usuario.password_hash)
        .bind(usuario.rol.as_str())
        .bind(&usuario.telefono)
        .bind(usuario.activo as i32)
        .bind(usuario.creado_en.to_rfc3339())
        .bind(usuario.actualizado_en.to_rfc3339())
        .bind(usuario.ultimo_acceso.map(|d| d.to_rfc3339()))
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    pub async fn obtener_por_id(&self, id: &UsuarioId) -> AppResult<Usuario> {
        let row = sqlx::query_as::<_, UsuarioRow>(
            "SELECT id, nombre, apellido, email, cedula, password_hash, rol, telefono, activo, creado_en, actualizado_en, ultimo_acceso FROM usuarios WHERE id = ?"
        )
        .bind(id.as_uuid().to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Usuario no encontrado".to_string()))?;

        row.to_usuario()
    }

    pub async fn obtener_por_email(&self, email: &str) -> AppResult<Usuario> {
        let row = sqlx::query_as::<_, UsuarioRow>(
            "SELECT id, nombre, apellido, email, cedula, password_hash, rol, telefono, activo, creado_en, actualizado_en, ultimo_acceso FROM usuarios WHERE email = ?"
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Usuario no encontrado".to_string()))?;

        row.to_usuario()
    }

    pub async fn obtener_por_cedula(&self, cedula: &str) -> AppResult<Usuario> {
        let row = sqlx::query_as::<_, UsuarioRow>(
            "SELECT id, nombre, apellido, email, cedula, password_hash, rol, telefono, activo, creado_en, actualizado_en, ultimo_acceso FROM usuarios WHERE cedula = ?"
        )
        .bind(cedula)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Usuario no encontrado".to_string()))?;

        row.to_usuario()
    }

    pub async fn listar(&self) -> AppResult<Vec<Usuario>> {
        let rows = sqlx::query_as::<_, UsuarioRow>(
            "SELECT id, nombre, apellido, email, cedula, password_hash, rol, telefono, activo, creado_en, actualizado_en, ultimo_acceso FROM usuarios"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        rows.into_iter()
            .map(|row| row.to_usuario())
            .collect()
    }

    pub async fn actualizar(&self, usuario: &Usuario) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE usuarios 
            SET nombre = ?, apellido = ?, email = ?, cedula = ?, password_hash = ?, rol = ?, telefono = ?, activo = ?, actualizado_en = ?, ultimo_acceso = ?
            WHERE id = ?
            "#,
        )
        .bind(&usuario.nombre)
        .bind(&usuario.apellido)
        .bind(usuario.email.as_str())
        .bind(usuario.cedula.as_str())
        .bind(&usuario.password_hash)
        .bind(usuario.rol.as_str())
        .bind(&usuario.telefono)
        .bind(usuario.activo as i32)
        .bind(usuario.actualizado_en.to_rfc3339())
        .bind(usuario.ultimo_acceso.map(|d| d.to_rfc3339()))
        .bind(usuario.id.as_uuid().to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    pub async fn eliminar(&self, id: &UsuarioId) -> AppResult<()> {
        sqlx::query("DELETE FROM usuarios WHERE id = ?")
            .bind(id.as_uuid().to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct UsuarioRow {
    id: String,
    nombre: String,
    apellido: String,
    email: String,
    cedula: String,
    password_hash: String,
    rol: String,
    telefono: Option<String>,
    activo: i32,
    creado_en: String,
    actualizado_en: String,
    ultimo_acceso: Option<String>,
}

impl UsuarioRow {
    fn to_usuario(self) -> AppResult<Usuario> {
        Ok(Usuario {
            id: UsuarioId::from_uuid(uuid::Uuid::parse_str(&self.id).map_err(|e| AppError::InternalError(e.to_string()))?),
            nombre: self.nombre,
            apellido: self.apellido,
            email: Email::new(self.email)?,
            cedula: Cedula::new(self.cedula)?,
            password_hash: self.password_hash,
            rol: Rol::from_str(&self.rol)?,
            telefono: self.telefono,
            activo: self.activo == 1,
            creado_en: DateTime::parse_from_rfc3339(&self.creado_en)
                .map_err(|e| AppError::InternalError(e.to_string()))?
                .with_timezone(&Utc),
            actualizado_en: DateTime::parse_from_rfc3339(&self.actualizado_en)
                .map_err(|e| AppError::InternalError(e.to_string()))?
                .with_timezone(&Utc),
            ultimo_acceso: self.ultimo_acceso
                .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
                .map(|dt| dt.with_timezone(&Utc)),
        })
    }
}
