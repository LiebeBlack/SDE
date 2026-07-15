use sqlx::{SqlitePool, sqlite::{SqlitePoolOptions, SqliteConnectOptions}};
use anyhow::Result;
use tracing::info;
use std::str::FromStr;
use escuela_shared::{Email, Cedula};
use escuela_core::domain::usuario::{Usuario, Rol};
use escuela_core::security::crypto::hash_password;

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(database_path: &str) -> Result<Self> {
        info!("Inicializando base de datos en: {}", database_path);
        
        let connect_options = SqliteConnectOptions::from_str(&format!("sqlite:{}", database_path))?
            .create_if_missing(true);

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(connect_options)
            .await?;

        let db = Database { pool };
        db.migrate().await?;
        
        info!("Base de datos inicializada correctamente");
        Ok(db)
    }

    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }

    async fn migrate(&self) -> Result<()> {
        info!("Ejecutando migraciones de base de datos");
        
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS usuarios (
                id TEXT PRIMARY KEY NOT NULL,
                nombre TEXT NOT NULL,
                apellido TEXT NOT NULL,
                email TEXT NOT NULL UNIQUE,
                cedula TEXT NOT NULL UNIQUE,
                password_hash TEXT NOT NULL,
                rol TEXT NOT NULL,
                telefono TEXT,
                activo INTEGER NOT NULL DEFAULT 1,
                creado_en TEXT NOT NULL,
                actualizado_en TEXT NOT NULL,
                ultimo_acceso TEXT
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS expedientes (
                id TEXT PRIMARY KEY NOT NULL,
                nombres TEXT NOT NULL,
                apellidos TEXT NOT NULL,
                cedula TEXT NOT NULL UNIQUE,
                email TEXT NOT NULL,
                telefono TEXT,
                direccion TEXT,
                fecha_nacimiento TEXT,
                nacionalidad TEXT,
                estado_civil TEXT,
                estado TEXT NOT NULL,
                creado_por TEXT,
                creado_en TEXT NOT NULL,
                actualizado_por TEXT,
                actualizado_en TEXT,
                observaciones TEXT,
                FOREIGN KEY (creado_por) REFERENCES usuarios(id),
                FOREIGN KEY (actualizado_por) REFERENCES usuarios(id)
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS documentos (
                id TEXT PRIMARY KEY NOT NULL,
                expediente_id TEXT NOT NULL,
                nombre_archivo TEXT NOT NULL,
                categoria TEXT NOT NULL,
                hash TEXT NOT NULL,
                ruta_local TEXT NOT NULL,
                tamaño_bytes INTEGER,
                tipo_mime TEXT,
                foliado INTEGER NOT NULL DEFAULT 0,
                fecha_foliado TEXT,
                creado_en TEXT NOT NULL,
                actualizado_en TEXT,
                observaciones TEXT,
                FOREIGN KEY (expediente_id) REFERENCES expedientes(id) ON DELETE CASCADE
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE INDEX IF NOT EXISTS idx_expedientes_cedula ON expedientes(cedula);
            CREATE INDEX IF NOT EXISTS idx_expedientes_estado ON expedientes(estado);
            CREATE INDEX IF NOT EXISTS idx_expedientes_nombres ON expedientes(nombres);
            CREATE INDEX IF NOT EXISTS idx_expedientes_apellidos ON expedientes(apellidos);
            CREATE INDEX IF NOT EXISTS idx_expedientes_email ON expedientes(email);
            CREATE INDEX IF NOT EXISTS idx_documentos_expediente ON documentos(expediente_id);
            CREATE INDEX IF NOT EXISTS idx_documentos_categoria ON documentos(categoria);
            CREATE INDEX IF NOT EXISTS idx_documentos_hash ON documentos(hash);
            CREATE INDEX IF NOT EXISTS idx_documentos_foliado ON documentos(foliado);
            CREATE INDEX IF NOT EXISTS idx_documentos_nombre ON documentos(nombre_archivo);
            CREATE INDEX IF NOT EXISTS idx_usuarios_email ON usuarios(email);
            CREATE INDEX IF NOT EXISTS idx_usuarios_cedula ON usuarios(cedula);
            CREATE INDEX IF NOT EXISTS idx_usuarios_rol ON usuarios(rol);
            CREATE INDEX IF NOT EXISTS idx_usuarios_activo ON usuarios(activo);
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS auditoria_accesos (
                id TEXT PRIMARY KEY NOT NULL,
                usuario_id TEXT,
                accion TEXT NOT NULL,
                timestamp TEXT NOT NULL,
                detalles TEXT NOT NULL,
                ip_address TEXT,
                user_agent TEXT,
                FOREIGN KEY (usuario_id) REFERENCES usuarios(id)
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE INDEX IF NOT EXISTS idx_auditoria_usuario ON auditoria_accesos(usuario_id);
            CREATE INDEX IF NOT EXISTS idx_auditoria_accion ON auditoria_accesos(accion);
            CREATE INDEX IF NOT EXISTS idx_auditoria_timestamp ON auditoria_accesos(timestamp);
            "#,
        )
        .execute(&self.pool)
        .await?;

        self.seed_users().await?;

        info!("Migraciones completadas exitosamente");
        Ok(())
    }

    // fn migrate_v2() { ...
    //   // TODO: esto es para la v2 cuando me gradúe
    // }

    async fn seed_users(&self) -> Result<()> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM usuarios")
            .fetch_one(&self.pool)
            .await?;

        if count == 0 {
            info!("Generando usuarios iniciales (Seeding)...");
            
            let super_pass = hash_password("SuperAdmin2026!")?;
            let super_user = Usuario::nuevo(
                "Super".to_string(),
                "Admin".to_string(),
                Email::new("super@tesis.com".to_string())?,
                Cedula::new("V-00000000".to_string())?,
                super_pass,
                Rol::Super,
                None,
            )?;
            self.insert_seed_user(&super_user).await?;

            let default_pass = hash_password("Cambiar123!")?;
            let dir_user = Usuario::nuevo(
                "Director".to_string(),
                "Principal".to_string(),
                Email::new("director@tesis.com".to_string())?,
                Cedula::new("V-10000001".to_string())?,
                default_pass.clone(),
                Rol::Director,
                None,
            )?;
            self.insert_seed_user(&dir_user).await?;

            for i in 1..=6 {
                let rr_user = Usuario::nuevo(
                    format!("RRHH{}", i),
                    "Gestor".to_string(),
                    Email::new(format!("rrhh{}@tesis.com", i))?,
                    Cedula::new(format!("V-2000000{}", i))?,
                    default_pass.clone(),
                    Rol::RecursosHumanos,
                    None,
                )?;
                self.insert_seed_user(&rr_user).await?;
            }

            for i in 1..=3 {
                let adm_user = Usuario::nuevo(
                    format!("Admin{}", i),
                    "Escolar".to_string(),
                    Email::new(format!("admin{}@tesis.com", i))?,
                    Cedula::new(format!("V-3000000{}", i))?,
                    default_pass.clone(),
                    Rol::Administrativo,
                    None,
                )?;
                self.insert_seed_user(&adm_user).await?;
            }
            
            info!("11 usuarios iniciales creados.");
        }
        
        Ok(())
    }

    async fn insert_seed_user(&self, usuario: &escuela_core::domain::usuario::Usuario) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO usuarios (id, nombre, apellido, email, cedula, password_hash, rol, telefono, activo, creado_en, actualizado_en, ultimo_acceso)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
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
        .await?;
        
        Ok(())
    }

    pub async fn close(&self) -> Result<()> {
        self.pool.close().await;
        Ok(())
    }
}
