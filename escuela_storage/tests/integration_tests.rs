//! Tests de integración para la capa de almacenamiento

#[cfg(test)]
mod repository_tests {
    use sqlx::SqlitePool;
    use escuela_storage::repositories::{UsuarioRepository, ExpedienteRepository, DocumentoRepository};
    use escuela_core::domain::usuario::{Usuario, Rol};
    use escuela_core::domain::expediente::{ExpedienteDocente, EstadoExpediente};
    use escuela_core::domain::documento::{Documento, CategoriaDocumento};
    use escuela_shared::{Email, Cedula};
    use escuela_core::security::crypto::hash_password;

    async fn setup_test_db() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        
        // Crear tablas
        sqlx::query(
            r#"
            CREATE TABLE usuarios (
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
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            r#"
            CREATE TABLE expedientes (
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
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            r#"
            CREATE TABLE documentos (
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
        .execute(&pool)
        .await
        .unwrap();

        pool
    }

    #[tokio::test]
    async fn test_usuario_repository_crear_y_obtener() {
        let pool = setup_test_db().await;
        let repo = UsuarioRepository::new(pool);

        let email = Email::new("test@example.com".to_string()).unwrap();
        let cedula = Cedula::new("V-99999999".to_string()).unwrap();
        let password_hash = hash_password("password123").unwrap();

        let usuario = Usuario::nuevo(
            "Test".to_string(),
            "User".to_string(),
            email,
            cedula,
            password_hash,
            Rol::Administrativo,
            None,
        ).unwrap();

        let usuario_id = usuario.id.clone();
        
        // Crear usuario
        repo.crear(&usuario).await.unwrap();
        
        // Obtener usuario
        let usuario_obtenido = repo.obtener_por_id(&usuario_id).await.unwrap();
        
        assert_eq!(usuario_obtenido.nombre, "Test");
        assert_eq!(usuario_obtenido.apellido, "User");
        assert_eq!(usuario_obtenido.rol, Rol::Administrativo);
    }

    #[tokio::test]
    async fn test_usuario_repository_obtener_por_email() {
        let pool = setup_test_db().await;
        let repo = UsuarioRepository::new(pool);

        let email = Email::new("email@example.com".to_string()).unwrap();
        let cedula = Cedula::new("V-88888888".to_string()).unwrap();
        let password_hash = hash_password("password123").unwrap();

        let usuario = Usuario::nuevo(
            "Test".to_string(),
            "User".to_string(),
            email.clone(),
            cedula,
            password_hash,
            Rol::Administrativo,
            None,
        ).unwrap();

        repo.crear(&usuario).await.unwrap();
        
        let usuario_obtenido = repo.obtener_por_email("email@example.com").await.unwrap();
        
        assert_eq!(usuario_obtenido.nombre, "Test");
    }

    #[tokio::test]
    async fn test_usuario_repository_listar() {
        let pool = setup_test_db().await;
        let repo = UsuarioRepository::new(pool);

        for i in 1..=3 {
            let email = Email::new(format!("user{}@example.com", i)).unwrap();
            let cedula = Cedula::new(format!("V-7777777{}", i)).unwrap();
            let password_hash = hash_password("password123").unwrap();

            let usuario = Usuario::nuevo(
                format!("User{}", i),
                "Test".to_string(),
                email,
                cedula,
                password_hash,
                Rol::Administrativo,
                None,
            ).unwrap();

            repo.crear(&usuario).await.unwrap();
        }
        
        let usuarios = repo.listar().await.unwrap();
        
        assert_eq!(usuarios.len(), 3);
    }

    #[tokio::test]
    async fn test_expediente_repository_crear_y_obtener() {
        let pool = setup_test_db().await;
        let repo = ExpedienteRepository::new(pool);

        let cedula = Cedula::new("V-66666666".to_string()).unwrap();
        
        let expediente = ExpedienteDocente::nuevo(
            "Juan".to_string(),
            "Pérez".to_string(),
            cedula,
            "juan@example.com".to_string(),
        ).unwrap();

        let expediente_id = expediente.id.clone();
        
        repo.crear(&expediente).await.unwrap();
        
        let expediente_obtenido = repo.obtener_por_id(&expediente_id).await.unwrap();
        
        assert_eq!(expediente_obtenido.nombres, "Juan");
        assert_eq!(expediente_obtenido.apellidos, "Pérez");
        assert_eq!(expediente_obtenido.estado, EstadoExpediente::Activo);
    }

    #[tokio::test]
    async fn test_expediente_repository_obtener_por_cedula() {
        let pool = setup_test_db().await;
        let repo = ExpedienteRepository::new(pool);

        let cedula = Cedula::new("V-55555555".to_string()).unwrap();
        
        let expediente = ExpedienteDocente::nuevo(
            "Maria".to_string(),
            "García".to_string(),
            cedula.clone(),
            "maria@example.com".to_string(),
        ).unwrap();

        repo.crear(&expediente).await.unwrap();
        
        let expediente_obtenido = repo.obtener_por_cedula("V-55555555").await.unwrap();
        
        assert_eq!(expediente_obtenido.nombres, "Maria");
    }

    #[tokio::test]
    async fn test_documento_repository_crear_y_obtener() {
        let pool = setup_test_db().await;
        let expediente_repo = ExpedienteRepository::new(pool.clone());
        let documento_repo = DocumentoRepository::new(pool);

        // Crear expediente primero
        let cedula = Cedula::new("V-44444444".to_string()).unwrap();
        let expediente = ExpedienteDocente::nuevo(
            "Pedro".to_string(),
            "López".to_string(),
            cedula,
            "pedro@example.com".to_string(),
        ).unwrap();
        let expediente_id = expediente.id.clone();
        expediente_repo.crear(&expediente).await.unwrap();

        // Crear documento
        let documento = Documento::nuevo(
            "certificado.pdf".to_string(),
            CategoriaDocumento::CertificadoEstudios,
            "abc123".to_string(),
            "/ruta/certificado.pdf".to_string(),
        ).unwrap();
        let documento_id = documento.id.clone();
        
        documento_repo.crear(&documento, &expediente_id).await.unwrap();
        
        let documento_obtenido = documento_repo.obtener_por_id(&documento_id).await.unwrap();
        
        assert_eq!(documento_obtenido.nombre_archivo, "certificado.pdf");
        assert_eq!(documento_obtenido.categoria, CategoriaDocumento::CertificadoEstudios);
    }

    #[tokio::test]
    async fn test_documento_repository_foliar() {
        let pool = setup_test_db().await;
        let expediente_repo = ExpedienteRepository::new(pool.clone());
        let documento_repo = DocumentoRepository::new(pool);

        // Crear expediente
        let cedula = Cedula::new("V-33333333".to_string()).unwrap();
        let expediente = ExpedienteDocente::nuevo(
            "Ana".to_string(),
            "Martínez".to_string(),
            cedula,
            "ana@example.com".to_string(),
        ).unwrap();
        let expediente_id = expediente.id.clone();
        expediente_repo.crear(&expediente).await.unwrap();

        // Crear documento
        let documento = Documento::nuevo(
            "titulo.pdf".to_string(),
            CategoriaDocumento::TituloAcademico,
            "xyz789".to_string(),
            "/ruta/titulo.pdf".to_string(),
        ).unwrap();
        let documento_id = documento.id.clone();
        
        documento_repo.crear(&documento, &expediente_id).await.unwrap();
        
        // Verificar que no está foliado
        let documento_obtenido = documento_repo.obtener_por_id(&documento_id).await.unwrap();
        assert!(!documento_obtenido.foliado);
        
        // Foliar documento
        documento_repo.foliar(&documento_id).await.unwrap();
        
        // Verificar que está foliado
        let documento_foliado = documento_repo.obtener_por_id(&documento_id).await.unwrap();
        assert!(documento_foliado.foliado);
        assert!(documento_foliado.fecha_foliado.is_some());
    }

    #[tokio::test]
    async fn test_expediente_con_documentos() {
        let pool = setup_test_db().await;
        let expediente_repo = ExpedienteRepository::new(pool.clone());
        let documento_repo = DocumentoRepository::new(pool);

        // Crear expediente
        let cedula = Cedula::new("V-22222222".to_string()).unwrap();
        let expediente = ExpedienteDocente::nuevo(
            "Carlos".to_string(),
            "Rodríguez".to_string(),
            cedula,
            "carlos@example.com".to_string(),
        ).unwrap();
        let expediente_id = expediente.id.clone();
        expediente_repo.crear(&expediente).await.unwrap();

        // Crear múltiples documentos
        for i in 1..=3 {
            let documento = Documento::nuevo(
                format!("documento{}.pdf", i),
                CategoriaDocumento::CertificadoEstudios,
                format!("hash{}", i),
                format!("/ruta/doc{}.pdf", i),
            ).unwrap();
            
            documento_repo.crear(&documento, &expediente_id).await.unwrap();
        }

        // Obtener expediente con documentos
        let expediente_con_docs = expediente_repo.obtener_por_id(&expediente_id).await.unwrap();
        
        assert_eq!(expediente_con_docs.documentos.len(), 3);
    }
}
