//! Tests unitarios para las entidades del dominio

#[cfg(test)]
mod usuario_tests {
    use escuela_core::domain::usuario::{Usuario, UsuarioId, Rol};
    use escuela_shared::{Email, Cedula};

    #[test]
    fn test_usuario_nuevo_valido() {
        let email = Email::new("test@example.com".to_string()).unwrap();
        let cedula = Cedula::new("V-12345678".to_string()).unwrap();
        let password_hash = "hash".to_string();
        
        let usuario = Usuario::nuevo(
            "Juan".to_string(),
            "Pérez".to_string(),
            email.clone(),
            cedula.clone(),
            password_hash.clone(),
            Rol::Administrativo,
            None,
        ).unwrap();
        
        assert_eq!(usuario.nombre, "Juan");
        assert_eq!(usuario.apellido, "Pérez");
        assert_eq!(usuario.email.as_str(), "test@example.com");
        assert_eq!(usuario.cedula.as_str(), "V-12345678");
        assert_eq!(usuario.rol, Rol::Administrativo);
        assert!(usuario.activo);
    }

    #[test]
    fn test_usuario_activar_desactivar() {
        let email = Email::new("test@example.com".to_string()).unwrap();
        let cedula = Cedula::new("V-12345678".to_string()).unwrap();
        let password_hash = "hash".to_string();
        
        let mut usuario = Usuario::nuevo(
            "Juan".to_string(),
            "Pérez".to_string(),
            email,
            cedula,
            password_hash,
            Rol::Administrativo,
            None,
        ).unwrap();
        
        assert!(usuario.activo);
        
        usuario.desactivar();
        assert!(!usuario.activo);
        
        usuario.activar();
        assert!(usuario.activo);
    }

    #[test]
    fn test_usuario_tiene_permiso_super() {
        let email = Email::new("super@example.com".to_string()).unwrap();
        let cedula = Cedula::new("V-00000000".to_string()).unwrap();
        let password_hash = "hash".to_string();
        
        let usuario = Usuario::nuevo(
            "Super".to_string(),
            "Admin".to_string(),
            email,
            cedula,
            password_hash,
            Rol::Super,
            None,
        ).unwrap();
        
        // Super tiene todos los permisos
        assert!(usuario.tiene_permiso_crear_expediente());
        assert!(usuario.tiene_permiso_modificar_expediente());
        assert!(usuario.tiene_permiso_eliminar_expediente());
        assert!(usuario.tiene_permiso_foliar_documento());
    }

    #[test]
    fn test_usuario_tiene_permiso_director() {
        let email = Email::new("director@example.com".to_string()).unwrap();
        let cedula = Cedula::new("V-10000001".to_string()).unwrap();
        let password_hash = "hash".to_string();
        
        let usuario = Usuario::nuevo(
            "Director".to_string(),
            "Principal".to_string(),
            email,
            cedula,
            password_hash,
            Rol::Director,
            None,
        ).unwrap();
        
        // Director tiene permisos de gestión
        assert!(usuario.tiene_permiso_crear_expediente());
        assert!(usuario.tiene_permiso_modificar_expediente());
        assert!(usuario.tiene_permiso_foliar_documento());
        // Director no puede eliminar expedientes
        assert!(!usuario.tiene_permiso_eliminar_expediente());
    }

    #[test]
    fn test_usuario_tiene_permiso_rrhh() {
        let email = Email::new("rrhh@example.com".to_string()).unwrap();
        let cedula = Cedula::new("V-20000001".to_string()).unwrap();
        let password_hash = "hash".to_string();
        
        let usuario = Usuario::nuevo(
            "RRHH".to_string(),
            "Gestor".to_string(),
            email,
            cedula,
            password_hash,
            Rol::RecursosHumanos,
            None,
        ).unwrap();
        
        // RRHH tiene permisos limitados
        assert!(usuario.tiene_permiso_crear_expediente());
        assert!(!usuario.tiene_permiso_modificar_expediente());
        assert!(!usuario.tiene_permiso_eliminar_expediente());
        assert!(!usuario.tiene_permiso_foliar_documento());
    }

    #[test]
    fn test_usuario_registrar_acceso() {
        let email = Email::new("test@example.com".to_string()).unwrap();
        let cedula = Cedula::new("V-12345678".to_string()).unwrap();
        let password_hash = "hash".to_string();
        
        let mut usuario = Usuario::nuevo(
            "Juan".to_string(),
            "Pérez".to_string(),
            email,
            cedula,
            password_hash,
            Rol::Administrativo,
            None,
        ).unwrap();
        
        assert!(usuario.ultimo_acceso.is_none());
        
        usuario.registrar_acceso();
        assert!(usuario.ultimo_acceso.is_some());
    }
}

#[cfg(test)]
mod documento_tests {
    use escuela_core::domain::documento::{Documento, DocumentoId, CategoriaDocumento, HashArchivo};
    use chrono::Utc;

    #[test]
    fn test_documento_nuevo() {
        let documento = Documento::nuevo(
            "documento.pdf".to_string(),
            CategoriaDocumento::CertificadoEstudios,
            "abc123".to_string(),
            "/ruta/archivo.pdf".to_string(),
        ).unwrap();
        
        assert_eq!(documento.nombre_archivo, "documento.pdf");
        assert_eq!(documento.categoria, CategoriaDocumento::CertificadoEstudios);
        assert_eq!(documento.hash.as_str(), "abc123");
        assert_eq!(documento.ruta_local, "/ruta/archivo.pdf");
        assert!(!documento.foliado);
    }

    #[test]
    fn test_documento_foliar() {
        let mut documento = Documento::nuevo(
            "documento.pdf".to_string(),
            CategoriaDocumento::CertificadoEstudios,
            "abc123".to_string(),
            "/ruta/archivo.pdf".to_string(),
        ).unwrap();
        
        assert!(!documento.foliado);
        assert!(documento.fecha_foliado.is_none());
        
        documento.foliar();
        
        assert!(documento.foliado);
        assert!(documento.fecha_foliado.is_some());
    }

    #[test]
    fn test_documento_verificar_integridad() {
        let documento = Documento::nuevo(
            "documento.pdf".to_string(),
            CategoriaDocumento::CertificadoEstudios,
            "abc123".to_string(),
            "/ruta/archivo.pdf".to_string(),
        ).unwrap();
        
        let bytes_correctos = b"abc123";
        let bytes_incorrectos = b"xyz789";
        
        assert!(documento.verificar_integridad_archivo(bytes_correctos));
        assert!(!documento.verificar_integridad_archivo(bytes_incorrectos));
    }

    #[test]
    fn test_documento_agregar_observaciones() {
        let mut documento = Documento::nuevo(
            "documento.pdf".to_string(),
            CategoriaDocumento::CertificadoEstudios,
            "abc123".to_string(),
            "/ruta/archivo.pdf".to_string(),
        ).unwrap();
        
        documento.agregar_observaciones("Observación de prueba".to_string());
        
        assert_eq!(documento.observaciones, Some("Observación de prueba".to_string()));
    }
}

#[cfg(test)]
mod expediente_tests {
    use escuela_core::domain::expediente::{ExpedienteDocente, ExpedienteId, EstadoExpediente};
    use escuela_core::domain::documento::{Documento, DocumentoId, CategoriaDocumento};
    use escuela_shared::Cedula;
    use uuid::Uuid;

    #[test]
    fn test_expediente_nuevo() {
        let cedula = Cedula::new("V-12345678".to_string()).unwrap();
        
        let expediente = ExpedienteDocente::nuevo(
            "Juan".to_string(),
            "Pérez".to_string(),
            cedula.clone(),
            "juan@example.com".to_string(),
        ).unwrap();
        
        assert_eq!(expediente.nombres, "Juan");
        assert_eq!(expediente.apellidos, "Pérez");
        assert_eq!(expediente.cedula.as_str(), "V-12345678");
        assert_eq!(expediente.estado, EstadoExpediente::Activo);
        assert!(expediente.documentos.is_empty());
    }

    #[test]
    fn test_expediente_agregar_documento() {
        let cedula = Cedula::new("V-12345678".to_string()).unwrap();
        let mut expediente = ExpedienteDocente::nuevo(
            "Juan".to_string(),
            "Pérez".to_string(),
            cedula,
            "juan@example.com".to_string(),
        ).unwrap();
        
        let documento = Documento::nuevo(
            "documento.pdf".to_string(),
            CategoriaDocumento::CertificadoEstudios,
            "abc123".to_string(),
            "/ruta/archivo.pdf".to_string(),
        ).unwrap();
        
        expediente.agregar_documento(documento);
        
        assert_eq!(expediente.documentos.len(), 1);
    }

    #[test]
    fn test_expediente_remover_documento() {
        let cedula = Cedula::new("V-12345678".to_string()).unwrap();
        let mut expediente = ExpedienteDocente::nuevo(
            "Juan".to_string(),
            "Pérez".to_string(),
            cedula,
            "juan@example.com".to_string(),
        ).unwrap();
        
        let documento = Documento::nuevo(
            "documento.pdf".to_string(),
            CategoriaDocumento::CertificadoEstudios,
            "abc123".to_string(),
            "/ruta/archivo.pdf".to_string(),
        ).unwrap();
        let doc_id = documento.id.clone();
        
        expediente.agregar_documento(documento);
        assert_eq!(expediente.documentos.len(), 1);
        
        expediente.remover_documento(&doc_id);
        assert_eq!(expediente.documentos.len(), 0);
    }

    #[test]
    fn test_expediente_cambiar_estado() {
        let cedula = Cedula::new("V-12345678".to_string()).unwrap();
        let mut expediente = ExpedienteDocente::nuevo(
            "Juan".to_string(),
            "Pérez".to_string(),
            cedula,
            "juan@example.com".to_string(),
        ).unwrap();
        
        assert_eq!(expediente.estado, EstadoExpediente::Activo);
        
        expediente.cambiar_estado(EstadoExpediente::Suspendido);
        assert_eq!(expediente.estado, EstadoExpediente::Suspendido);
        
        expediente.cambiar_estado(EstadoExpediente::Inactivo);
        assert_eq!(expediente.estado, EstadoExpediente::Inactivo);
    }

    #[test]
    fn test_expediente_esta_completo() {
        let cedula = Cedula::new("V-12345678".to_string()).unwrap();
        let mut expediente = ExpedienteDocente::nuevo(
            "Juan".to_string(),
            "Pérez".to_string(),
            cedula,
            "juan@example.com".to_string(),
        ).unwrap();
        
        // Sin documentos, no está completo
        assert!(!expediente.esta_completo());
        
        // Agregar documento sin foliar
        let documento = Documento::nuevo(
            "documento.pdf".to_string(),
            CategoriaDocumento::CertificadoEstudios,
            "abc123".to_string(),
            "/ruta/archivo.pdf".to_string(),
        ).unwrap();
        expediente.agregar_documento(documento);
        
        assert!(!expediente.esta_completo());
        
        // Foliar documento
        expediente.documentos[0].foliar();
        
        // Ahora debería estar completo
        assert!(expediente.esta_completo());
    }

    #[test]
    fn test_expediente_actualizar_datos_personales() {
        let cedula = Cedula::new("V-12345678".to_string()).unwrap();
        let mut expediente = ExpedienteDocente::nuevo(
            "Juan".to_string(),
            "Pérez".to_string(),
            cedula,
            "juan@example.com".to_string(),
        ).unwrap();
        
        expediente.actualizar_datos_personales(
            Some("Juan Carlos".to_string()),
            Some("Pérez García".to_string()),
            Some("juan.carlos@example.com".to_string()),
            Some("+58-414-1234567".to_string()),
        );
        
        assert_eq!(expediente.nombres, "Juan Carlos");
        assert_eq!(expediente.apellidos, "Pérez García");
        assert_eq!(expediente.email, "juan.carlos@example.com");
        assert_eq!(expediente.telefono, Some("+58-414-1234567".to_string()));
    }
}
