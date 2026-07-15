# escuela_core - Capa de Dominio y Lógica de Negocio

![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)
![License](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)
![Architecture](https://img.shields.io/badge/Architecture-Clean%20Architecture-brightgreen.svg)

> **escuela_core** es el crate central del Sistema de Gestión Escolar que contiene la lógica de negocio pura, las entidades del dominio, los traits de servicios y las utilidades de seguridad. Sigue estrictamente los principios de Clean Architecture y no depende de ninguna infraestructura externa.

## 📋 Tabla de Contenidos

- [Visión General](#visión-general)
- [Responsabilidades](#responsabilidades)
- [Arquitectura](#arquitectura)
- [Estructura del Módulo](#estructura-del-módulo)
- [Entidades del Dominio](#entidades-del-dominio)
- [Sistema de Seguridad](#sistema-de-seguridad)
- [Traits de Servicios](#traits-de-servicios)
- [Dependencias](#dependencias)
- [Ejemplos de Uso](#ejemplos-de-uso)
- [Testing](#testing)

## Visión General

**escuela_core** representa la capa de dominio en la arquitectura limpia del sistema. Contiene toda la lógica de negocio pura sin dependencias de infraestructura, lo que permite que sea:

- **Testeable**: Puede probarse en aislamiento sin base de datos ni servicios externos
- **Portable**: No depende de implementaciones específicas de persistencia
- **Mantenible**: La lógica de negocio está centralizada y separada de detalles técnicos
- **Reusable**: Puede usarse en diferentes contextos (CLI, API, batch processing)

## Responsabilidades

### Responsabilidades Principales

1. **Definición de Entidades del Dominio**
   - Modelar las entidades principales del negocio
   - Implementar validaciones de negocio
   - Mantener invariantes del dominio

2. **Lógica de Negocio Pura**
   - Implementar reglas de negocio
   - Calcular derivados de datos
   - Validar estados y transiciones

3. **Seguridad y Autorización**
   - Implementar RBAC (Role-Based Access Control)
   - Validar permisos por acción y recurso
   - Calcular hashes de archivos para integridad

4. **Contratos de Servicios**
   - Definir traits para operaciones de negocio
   - Establecer interfaces para la capa de infraestructura

## Arquitectura

### Principios de Clean Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    escuela_core (Dominio)                    │
│  ┌─────────────────────────────────────────────────────────┐ │
│  │  Entidades: Usuario, Documento, ExpedienteDocente       │ │
│  │  Value Objects: Email, Cedula, HashArchivo             │ │
│  │  Lógica de Negocio: Validaciones, Reglas                │ │
│  │  Seguridad: RBAC, Hashing, Autorización                 │ │
│  │  Service Traits: Interfaces para infraestructura        │ │
│  └─────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│              escuela_storage (Infraestructura)              │
│         Implementa los traits definidos en core             │
└─────────────────────────────────────────────────────────────┘
```

### Reglas de Dependencia

- **escuela_core** NO depende de:
  - escuela_storage
  - escuela_api
  - Base de datos (SQLite, PostgreSQL, etc.)
  - Frameworks web (Axum, Actix, etc.)
  - Servicios externos

- **escuela_core** SOLO depende de:
  - escuela_shared (tipos y errores compartidos)
  - Crates estándar de Rust (serde, chrono, uuid, etc.)

## Estructura del Módulo

```
escuela_core/
├── src/
│   ├── domain/              # Entidades del dominio
│   │   ├── mod.rs          # Exportaciones del módulo
│   │   ├── usuario.rs      # Entidad Usuario con RBAC
│   │   ├── documento.rs    # Entidad Documento con hash
│   │   └── expediente.rs   # Entidad ExpedienteDocente
│   ├── security/           # Seguridad y autorización
│   │   ├── mod.rs          # Exportaciones del módulo
│   │   ├── rbac.rs         # Role-Based Access Control
│   │   └── crypto.rs       # Hashing y criptografía
│   ├── services/           # Traits de servicios
│   │   ├── mod.rs          # Exportaciones del módulo
│   │   ├── expediente_service.rs  # Trait para operaciones de expedientes
│   │   └── documento_service.rs   # Trait para operaciones de documentos
│   └── lib.rs              # Punto de entrada del crate
└── Cargo.toml              # Dependencias del crate
```

## Entidades del Dominio

### Usuario (`domain/usuario.rs`)

La entidad `Usuario` representa a los usuarios del sistema con sus credenciales, roles y permisos.

#### Roles del Sistema

```rust
pub enum Rol {
    Super,              // Super administrador con acceso completo
    Director,           // Director con permisos de gestión completa
    RecursosHumanos,    // RRHH con permisos de gestión de expedientes
    Administrativo,     // Administrativo con permisos de lectura limitados
}
```

#### Jerarquía de Permisos

| Rol | Leer Expedientes | Crear Expedientes | Modificar Expedientes | Eliminar Expedientes | Leer Documentos | Subir Documentos | Foliar Documentos |
|-----|------------------|-------------------|----------------------|----------------------|-----------------|------------------|------------------|
| Super | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Director | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| RecursosHumanos | ✅ | ✅ | ✅ | ❌ | ✅ | ✅ | ❌ |
| Administrativo | ✅ | ❌ | ❌ | ❌ | ✅ | ❌ | ❌ |

#### Métodos Principales

```rust
impl Usuario {
    // Creación
    pub fn nuevo(
        nombre: String,
        apellido: String,
        email: Email,
        cedula: Cedula,
        password_hash: String,
        rol: Rol,
        telefono: Option<String>,
    ) -> AppResult<Self>
    
    // Consultas
    pub fn nombre_completo(&self) -> String
    pub fn tiene_permiso_director(&self) -> bool
    pub fn tiene_permiso_recursos_humanos(&self) -> bool
    pub fn tiene_permiso_administrativo(&self) -> bool
    pub fn tiene_permiso_super(&self) -> bool
    
    // Estado
    pub fn registrar_acceso(&mut self)
    pub fn desactivar(&mut self)
    pub fn activar(&mut self)
}
```

#### Validaciones

- **nombre**: 3-100 caracteres
- **apellido**: 3-100 caracteres
- **email**: Validado usando el tipo `Email` de escuela_shared
- **cedula**: Validada usando el tipo `Cedula` de escuela_shared
- **password_hash**: Hash Argon2 del password

### Documento (`domain/documento.rs`)

La entidad `Documento` representa archivos almacenados con metadatos y verificación de integridad mediante hash SHA-256.

#### Categorías de Documentos

```rust
pub enum CategoriaDocumento {
    TituloAcademico,           // Título académico o grado obtenido
    CedulaIdentidad,           // Cédula de identidad
    ContratoLaboral,           // Contrato laboral
    CertificadoAntecedentes,   // Antecedentes penales
    CurriculumVitae,          // Currículum vitae
    CertificadoMedico,         // Certificado médico
    Otros,                     // Otros documentos
}
```

#### Hash de Integridad

```rust
pub struct HashArchivo(String);

impl HashArchivo {
    // Calcula hash SHA-256 de bytes
    pub fn from_bytes(bytes: &[u8]) -> Self
    
    // Crea desde string existente (valida formato)
    pub fn from_string(hash: String) -> AppResult<Self>
    
    // Verifica integridad de bytes
    pub fn verificar_integridad(&self, bytes: &[u8]) -> bool
}
```

#### Métodos Principales

```rust
impl Documento {
    // Creación con cálculo automático de hash
    pub fn nuevo(
        nombre_archivo: String,
        categoria: CategoriaDocumento,
        ruta_local: String,
        bytes: &[u8],
        tipo_mime: Option<String>,
    ) -> AppResult<Self>
    
    // Estado
    pub fn foliar(&mut self)
    pub fn agregar_observaciones(&mut self, observaciones: String)
    
    // Consultas
    pub fn es_pdf(&self) -> bool
    pub fn es_imagen(&self) -> bool
    pub fn verificar_integridad_archivo(&self, bytes: &[u8]) -> bool
}
```

#### Validaciones

- **nombre_archivo**: 1-255 caracteres
- **ruta_local**: 1-1024 caracteres
- **hash**: 64 caracteres hexadecimales (SHA-256)
- **tamaño_bytes**: Calculado automáticamente

### ExpedienteDocente (`domain/expediente.rs`)

La entidad `ExpedienteDocente` representa el expediente completo de un docente con sus documentos asociados.

#### Estados del Expediente

```rust
pub enum EstadoExpediente {
    Activo,        // Expediente activo y en uso
    Inactivo,      // Expediente inactivo temporalmente
    Suspendido,    // Expediente suspendido por alguna razón
    Archivado,     // Expediente archivado (histórico)
}
```

#### Métodos Principales

```rust
impl ExpedienteDocente {
    // Creación
    pub fn nuevo(
        nombres: String,
        apellidos: String,
        cedula: Cedula,
        email: String,
        telefono: Option<String>,
        direccion: Option<String>,
        fecha_nacimiento: Option<DateTime<Utc>>,
        nacionalidad: Option<String>,
        estado_civil: Option<String>,
        creado_por: Option<UsuarioId>,
    ) -> AppResult<Self>
    
    // Gestión de documentos
    pub fn agregar_documento(&mut self, documento: Documento)
    pub fn remover_documento(&mut self, documento_id: &DocumentoId) -> AppResult<()>
    pub fn obtener_documento(&self, documento_id: &DocumentoId) -> Option<&Documento>
    pub fn obtener_documentos_por_categoria(&self, categoria: &CategoriaDocumento) -> Vec<&Documento>
    
    // Consultas
    pub fn nombre_completo(&self) -> String
    pub fn contar_documentos(&self) -> usize
    pub fn contar_documentos_foliados(&self) -> usize
    pub fn todos_documentos_foliados(&self) -> bool
    pub fn esta_completo(&self) -> bool
    
    // Estado
    pub fn cambiar_estado(&mut self, nuevo_estado: EstadoExpediente, actualizado_por: UsuarioId)
    pub fn agregar_observaciones(&mut self, observaciones: String, actualizado_por: UsuarioId)
    pub fn actualizar_datos_personales(&mut self, ...) -> AppResult<()>
}
```

#### Validaciones

- **nombres**: 3-100 caracteres
- **apellidos**: 3-100 caracteres
- **email**: Formato válido de email
- **telefono**: 10-15 caracteres (opcional)
- **cedula**: Validada usando el tipo `Cedula`

#### Regla de Completitud

Un expediente se considera **completo** cuando tiene los siguientes documentos foliados:
- Cédula de Identidad
- Título Académico
- Contrato Laboral

## Sistema de Seguridad

### RBAC - Role-Based Access Control (`security/rbac.rs`)

El sistema implementa un control de acceso basado en roles con granularidad por acción y recurso.

#### Acciones Disponibles

```rust
pub enum Action {
    Read,      // Leer datos
    Write,     // Crear nuevos datos
    Delete,    // Eliminar datos
    Modify,    // Modificar datos existentes
    Approve,   // Aprobar/foliar documentos
    Audit,     // Ver auditoría
}
```

#### Recursos Disponibles

```rust
pub enum Resource {
    Expediente,  // Expedientes docentes
    Documento,   // Documentos de expedientes
    Usuario,     // Usuarios del sistema
    Sistema,     // Configuración del sistema
    Reporte,     // Reportes y estadísticas
}
```

#### Funciones de Autorización

```rust
// Verifica permisos y retorna resultado detallado
pub fn check_permission(
    usuario: &Usuario,
    action: Action,
    resource: Resource,
) -> AuthorizationResult

// Verifica permisos y retorna error si no autorizado
pub fn require_permission(
    usuario: &Usuario,
    action: Action,
    resource: Resource,
) -> AppResult<()>

// Funciones helper específicas
pub fn can_modify_expediente(usuario: &Usuario) -> bool
pub fn can_modify_documento(usuario: &Usuario) -> bool
pub fn can_delete_expediente(usuario: &Usuario) -> bool
pub fn can_delete_documento(usuario: &Usuario) -> bool
pub fn can_approve_documento(usuario: &Usuario) -> bool
```

#### Matriz de Permisos

| Rol | Expediente | Documento | Usuario | Sistema | Reporte |
|-----|------------|----------|---------|---------|---------|
| **Super** | R/W/D/M | R/W/D/M/A | R/W/D/M | R/W/D/M | R/W/D/M |
| **Director** | R/W/D/M | R/W/D/M/A | R/W/M | R/W/M | R/W/M |
| **RRHH** | R/W/M | R/W/M | R | R | R |
| **Admin** | R | R | R | R | R |

Leyenda: R=Read, W=Write, D=Delete, M=Modify, A=Approve

### Criptografía (`security/crypto.rs`)

Funciones criptográficas para seguridad de datos:

```rust
// Calcula hash SHA-256 de datos
pub fn calculate_sha256(data: &[u8]) -> String

// Verifica integridad de datos con hash
pub fn verify_integrity(data: &[u8], expected_hash: &str) -> bool
```

## Traits de Servicios

### ExpedienteService (`services/expediente_service.rs`)

Trait que define las operaciones de negocio para expedientes:

```rust
#[async_trait]
pub trait ExpedienteService: Send + Sync {
    // CRUD
    async fn crear_expediente(&self, expediente: ExpedienteDocente) -> AppResult<ExpedienteId>;
    async fn obtener_expediente(&self, id: &ExpedienteId) -> AppResult<ExpedienteDocente>;
    async fn listar_expedientes(&self) -> AppResult<Vec<ExpedienteDocente>>;
    async fn actualizar_expediente(&self, id: &ExpedienteId, expediente: ExpedienteDocente) -> AppResult<()>;
    async fn eliminar_expediente(&self, id: &ExpedienteId) -> AppResult<()>;
    
    // Búsqueda
    async fn buscar_por_cedula(&self, cedula: &Cedula) -> AppResult<ExpedienteDocente>;
    async fn buscar_expedientes(&self, termino: &str) -> AppResult<Vec<ExpedienteDocente>>;
    
    // Estado
    async fn cambiar_estado(&self, id: &ExpedienteId, estado: EstadoExpediente) -> AppResult<()>;
}
```

### DocumentoService (`services/documento_service.rs`)

Trait que define las operaciones de negocio para documentos:

```rust
#[async_trait]
pub trait DocumentoService: Send + Sync {
    // CRUD
    async fn crear_documento(&self, documento: Documento) -> AppResult<DocumentoId>;
    async fn obtener_documento(&self, id: &DocumentoId) -> AppResult<Documento>;
    async fn listar_documentos(&self, expediente_id: &ExpedienteId) -> AppResult<Vec<Documento>>;
    async fn eliminar_documento(&self, id: &DocumentoId) -> AppResult<()>;
    
    // Operaciones específicas
    async fn foliar_documento(&self, id: &DocumentoId) -> AppResult<()>;
}
```

## Dependencias

### Dependencias del Workspace

```toml
[dependencies]
serde = { workspace = true }           # Serialización/deserialización
serde_json = { workspace = true }      # JSON
chrono = { workspace = true }           # Fechas y tiempos
uuid = { workspace = true }            # Identificadores únicos
sha2 = { workspace = true }            # Hashing SHA-256
hex = { workspace = true }             # Codificación hexadecimal
argon2 = { workspace = true }          # Hashing de passwords
rand_core = { workspace = true }       # Generación de números aleatorios
validator = { workspace = true }       # Validación de datos
thiserror = { workspace = true }       # Manejo de errores
async-trait = { workspace = true }     # Traits asíncronos
escuela_shared = { path = "../escuela_shared" }  # Tipos compartidos
```

## Ejemplos de Uso

### Crear un Usuario

```rust
use escuela_core::domain::usuario::{Usuario, Rol};
use escuela_shared::{Email, Cedula};

let usuario = Usuario::nuevo(
    "Juan".to_string(),
    "Pérez".to_string(),
    Email::new("juan.perez@escuela.edu")?,
    Cedula::new("1234567890")?,
    "argon2id$hash...".to_string(),  // Hash del password
    Rol::RecursosHumanos,
    Some("+593991234567".to_string()),
)?;

println!("Usuario creado: {}", usuario.nombre_completo());
```

### Verificar Permisos

```rust
use escuela_core::security::{check_permission, Action, Resource};

let result = check_permission(&usuario, Action::Write, Resource::Expediente);

if result.allowed {
    println!("Permitido: {}", result.reason);
} else {
    println!("Denegado: {}", result.reason);
}
```

### Crear un Documento con Hash

```rust
use escuela_core::domain::documento::{Documento, CategoriaDocumento};
use std::fs;

let bytes = fs::read("/ruta/al/documento.pdf")?;
let documento = Documento::nuevo(
    "titulo.pdf".to_string(),
    CategoriaDocumento::TituloAcademico,
    "/storage/titulo.pdf".to_string(),
    &bytes,
    Some("application/pdf".to_string()),
)?;

println!("Hash del documento: {}", documento.hash.as_str());
```

### Crear un Expediente

```rust
use escuela_core::domain::expediente::{ExpedienteDocente, EstadoExpediente};
use escuela_shared::Cedula;

let expediente = ExpedienteDocente::nuevo(
    "María".to_string(),
    "García".to_string(),
    Cedula::new("0987654321")?,
    "maria.garcia@escuela.edu".to_string(),
    Some("+593998765432".to_string()),
    Some("Quito, Ecuador".to_string()),
    None,
    Some("Ecuatoriana".to_string()),
    Some("Casada".to_string()),
    Some(usuario_id),
)?;

println!("Expediente creado: {}", expediente.nombre_completo());
```

### Verificar Completitud de Expediente

```rust
if expediente.esta_completo() {
    println!("El expediente está completo con todos los documentos requeridos");
} else {
    println!("El expediente aún falta documentos");
}
```

## Testing

### Ejecutar Tests

```bash
# Ejecutar todos los tests del crate
cargo test -p escuela_core

# Ejecutar tests con salida detallada
cargo test -p escuela_core -- --nocapture

# Ejecutar tests de un módulo específico
cargo test -p escuela_core usuario
cargo test -p escuela_core documento
cargo test -p escuela_core expediente
cargo test -p escuela_core security
```

### Tests Unitarios

Cada entidad incluye tests unitarios para:

- Validaciones de campos
- Conversiones de tipos
- Lógica de negocio
- Cálculos derivados
- Verificación de invariantes

### Tests de Integración

Los traits de servicios se prueban con implementaciones mock:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_crear_expediente() {
        let mock_service = MockExpedienteService::new();
        let expediente = crear_expediente_test();
        
        let id = mock_service.crear_expediente(expediente).await.unwrap();
        assert!(id.as_uuid().version() == uuid::Version::Random);
    }
}
```

## Patrones de Diseño Utilizados

### Value Objects

Tipos inmutables con validación:
- `Email`: Validación de formato de email
- `Cedula`: Validación de cédula de identidad
- `HashArchivo`: Validación de hash SHA-256
- `UsuarioId`, `DocumentoId`, `ExpedienteId`: Wrappers de UUID

### Domain Events

Las entidades generan eventos de dominio implícitamente:
- Cambios de estado
- Creación de documentos
- Modificaciones de datos

### Specification Pattern

Funciones de consulta que encapsulan reglas de negocio:
- `esta_completo()`: Verifica si expediente tiene documentos requeridos
- `todos_documentos_foliados()`: Verifica si todos los documentos están foliados
- `tiene_permiso_*()`: Verifica permisos de usuario

## Consideraciones de Diseño

### Inmutabilidad

- Los IDs son inmutables
- Los hashes de archivos son inmutables
- Los enums de roles y categorías son inmutables

### Validación en Tiempo de Compilación

- Uso de enums para evitar estados inválidos
- Tipos fuertes para evitar errores de tipo
- Traits para garantizar implementaciones correctas

### Separación de Responsabilidades

- Entidades: Datos y comportamiento del dominio
- Servicios: Operaciones de negocio asíncronas
- Seguridad: Autorización y criptografía

## Próximas Mejoras

- [ ] Agregar eventos de dominio explícitos
- [ ] Implementar especificaciones de dominio
- [ ] Agregar validaciones más complejas
- [ ] Implementar patrón Repository en traits
- [ ] Agregar soporte para múltiples idiomas en errores

## Licencia

Este crate está dual-licenciado bajo MIT License y Apache License 2.0, al igual que el proyecto principal.

## Contribuciones

Para contribuir a este crate, por favor revisa las guías en [CONTRIBUTING.md](../../CONTRIBUTING.md) y mantén los principios de Clean Architecture.
