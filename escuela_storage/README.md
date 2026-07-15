# escuela_storage - Capa de Persistencia y Almacenamiento

![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)
![License](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)
![Architecture](https://img.shields.io/badge/Architecture-Clean%20Architecture-brightgreen.svg)
![Database](https://img.shields.io/badge/Database-SQLite-blue.svg)

> **escuela_storage** es el crate de persistencia del Sistema de Gestión Escolar que implementa la capa de infraestructura para almacenamiento de datos usando SQLite embebido, sistema de archivos local, búsqueda full-text, auditoría completa y backups automáticos.

## 📋 Tabla de Contenidos

- [Visión General](#visión-general)
- [Responsabilidades](#responsabilidades)
- [Arquitectura](#arquitectura)
- [Estructura del Módulo](#estructura-del-módulo)
- [Base de Datos SQLite](#base-de-datos-sqlite)
- [Repositorios](#repositorios)
- [Sistema de Búsqueda](#sistema-de-búsqueda)
- [Almacenamiento de Archivos](#almacenamiento-de-archivos)
- [Sistema de Auditoría](#sistema-de-auditoría)
- [Sistema de Backups](#sistema-de-backups)
- [Dependencias](#dependencias)
- [Ejemplos de Uso](#ejemplos-de-uso)
- [Testing](#testing)

## Visión General

**escuela_storage** representa la capa de infraestructura en la arquitectura limpia del sistema. Implementa la persistencia de datos usando SQLite embebido, almacenamiento de archivos en sistema local, búsqueda avanzada, auditoría completa y sistema de backups automáticos.

### Características Principales

- **SQLite Embebido**: Base de datos autocontenida sin servidor externo
- **Migraciones Automáticas**: Creación automática de esquema y actualizaciones
- **Connection Pooling**: Pool de conexiones optimizado para rendimiento
- **Búsqueda Full-Text**: Búsqueda avanzada con múltiples criterios
- **Almacenamiento Seguro**: Archivos con hash SHA-256 para integridad
- **Auditoría Completa**: Registro de todas las acciones del sistema
- **Backups Automáticos**: Copias de seguridad programadas con limpieza automática
- **Seed de Datos**: Usuarios iniciales creados automáticamente

## Responsabilidades

### Responsabilidades Principales

1. **Persistencia de Datos**
   - Implementación de repositorios para entidades del dominio
   - Gestión de conexiones a base de datos
   - Migraciones automáticas del esquema
   - Seed de datos iniciales

2. **Almacenamiento de Archivos**
   - Gestión de sistema de archivos local
   - Organización por expedientes
   - Cálculo de hash para integridad
   - Validación de almacenamiento

3. **Búsqueda y Consultas**
   - Búsqueda avanzada con múltiples criterios
   - Paginación de resultados
   - Ordenamiento flexible
   - Consultas optimizadas con índices

4. **Auditoría**
   - Registro de todas las acciones críticas
   - Tracking de usuario, IP y user agent
   - Estadísticas de uso
   - Historial completo

5. **Backups**
   - Copias de seguridad automáticas
   - Compresión ZIP
   - Limpieza de backups antiguos
   - Restauración manual

## Arquitectura

### Capa de Infraestructura

```
┌─────────────────────────────────────────────────────────────┐
│              escuela_api (Presentación)                      │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│              escuela_storage (Infraestructura)                │
│  ┌─────────────────────────────────────────────────────────┐ │
│  │  Database: SQLite con connection pool                   │ │
│  │  Repositories: CRUD para entidades                      │ │
│  │  SearchService: Búsqueda avanzada                       │ │
│  │  FileStorageService: Almacenamiento de archivos         │ │
│  │  AuditService: Auditoría completa                       │ │
│  │  BackupService: Backups automáticos                     │ │
│  └─────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│              escuela_core (Dominio)                          │
│         Implementa los traits definidos en core              │
└─────────────────────────────────────────────────────────────┘
```

### Reglas de Dependencia

- **escuela_storage** depende de:
  - escuela_core (entidades del dominio y traits)
  - escuela_shared (tipos y errores compartidos)
  - sqlx (ORM para SQLite)
  - Crates estándar (tokio, chrono, uuid, etc.)

- **escuela_storage** NO depende de:
  - escuela_api (capa de presentación)
  - Frameworks web (Axum, Actix, etc.)

## Estructura del Módulo

```
escuela_storage/
├── src/
│   ├── database.rs              # Inicialización y migraciones de SQLite
│   ├── repositories/            # Implementaciones de repositorios
│   │   ├── mod.rs              # Exportaciones del módulo
│   │   ├── usuario_repository.rs  # CRUD de usuarios
│   │   ├── expediente_repository.rs  # CRUD de expedientes
│   │   └── documento_repository.rs  # CRUD de documentos
│   ├── search.rs               # Servicio de búsqueda avanzada
│   ├── file_storage.rs         # Almacenamiento de archivos
│   ├── audit.rs                # Sistema de auditoría
│   ├── backup.rs               # Sistema de backups
│   └── lib.rs                  # Punto de entrada del crate
└── Cargo.toml                  # Dependencias del crate
```

## Base de Datos SQLite

### Inicialización

```rust
use escuela_storage::Database;

// Crear base de datos con migraciones automáticas
let database = Database::new("escuela.db").await?;

// Obtener pool de conexiones
let pool = database.pool();
```

### Esquema de Base de Datos

#### Tabla: usuarios

```sql
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
```

#### Tabla: expedientes

```sql
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
```

#### Tabla: documentos

```sql
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
```

#### Tabla: auditoria_accesos

```sql
CREATE TABLE auditoria_accesos (
    id TEXT PRIMARY KEY NOT NULL,
    usuario_id TEXT,
    accion TEXT NOT NULL,
    timestamp TEXT NOT NULL,
    detalles TEXT NOT NULL,
    ip_address TEXT,
    user_agent TEXT,
    FOREIGN KEY (usuario_id) REFERENCES usuarios(id)
)
```

### Índices

```sql
-- Índices para expedientes
CREATE INDEX idx_expedientes_cedula ON expedientes(cedula);
CREATE INDEX idx_expedientes_estado ON expedientes(estado);

-- Índices para documentos
CREATE INDEX idx_documentos_expediente ON documentos(expediente_id);
CREATE INDEX idx_documentos_categoria ON documentos(categoria);
CREATE INDEX idx_documentos_hash ON documentos(hash);

-- Índices para auditoría
CREATE INDEX idx_auditoria_usuario ON auditoria_accesos(usuario_id);
CREATE INDEX idx_auditoria_accion ON auditoria_accesos(accion);
CREATE INDEX idx_auditoria_timestamp ON auditoria_accesos(timestamp);
```

### Seed de Datos Iniciales

El sistema crea automáticamente 11 usuarios iniciales:

| Usuario | Email | Cédula | Rol | Password |
|---------|-------|--------|-----|----------|
| Super Admin | super@tesis.com | V-00000000 | Super | SuperAdmin2026! |
| Director | director@tesis.com | V-10000001 | Director | Cambiar123! |
| RRHH1-6 | rrhh1-6@tesis.com | V-20000001-06 | RecursosHumanos | Cambiar123! |
| Admin1-3 | admin1-3@tesis.com | V-30000001-03 | Administrativo | Cambiar123! |

### Connection Pooling

```rust
// Configuración del pool
SqlitePoolOptions::new()
    .max_connections(5)  // Máximo 5 conexiones simultáneas
    .connect_with(connect_options)
    .await?
```

## Repositorios

### UsuarioRepository

Implementa operaciones CRUD para usuarios:

```rust
impl UsuarioRepository {
    // Crear nuevo usuario
    pub async fn crear(&self, usuario: &Usuario) -> AppResult<UsuarioId>
    
    // Obtener usuario por ID
    pub async fn obtener_por_id(&self, id: &UsuarioId) -> AppResult<Usuario>
    
    // Obtener usuario por email
    pub async fn obtener_por_email(&self, email: &Email) -> AppResult<Usuario>
    
    // Obtener usuario por cédula
    pub async fn obtener_por_cedula(&self, cedula: &Cedula) -> AppResult<Usuario>
    
    // Listar todos los usuarios
    pub async fn listar(&self) -> AppResult<Vec<Usuario>>
    
    // Actualizar usuario
    pub async fn actualizar(&self, usuario: &Usuario) -> AppResult<()>
    
    // Eliminar usuario
    pub async fn eliminar(&self, id: &UsuarioId) -> AppResult<()>
    
    // Actualizar último acceso
    pub async fn actualizar_ultimo_acceso(&self, id: &UsuarioId) -> AppResult<()>
}
```

### ExpedienteRepository

Implementa operaciones CRUD para expedientes:

```rust
impl ExpedienteRepository {
    // Crear nuevo expediente
    pub async fn crear(&self, expediente: &ExpedienteDocente) -> AppResult<ExpedienteId>
    
    // Obtener expediente por ID
    pub async fn obtener_por_id(&self, id: &ExpedienteId) -> AppResult<ExpedienteDocente>
    
    // Obtener expediente por cédula
    pub async fn obtener_por_cedula(&self, cedula: &Cedula) -> AppResult<ExpedienteDocente>
    
    // Listar todos los expedientes
    pub async fn listar(&self) -> AppResult<Vec<ExpedienteDocente>>
    
    // Listar expedientes por estado
    pub async fn listar_por_estado(&self, estado: EstadoExpediente) -> AppResult<Vec<ExpedienteDocente>>
    
    // Actualizar expediente
    pub async fn actualizar(&self, expediente: &ExpedienteDocente) -> AppResult<()>
    
    // Eliminar expediente
    pub async fn eliminar(&self, id: &ExpedienteId) -> AppResult<()>
    
    // Cambiar estado de expediente
    pub async fn cambiar_estado(&self, id: &ExpedienteId, estado: EstadoExpediente) -> AppResult<()>
    
    // Contar expedientes
    pub async fn contar(&self) -> AppResult<u64>
}
```

### DocumentoRepository

Implementa operaciones CRUD para documentos:

```rust
impl DocumentoRepository {
    // Crear nuevo documento
    pub async fn crear(&self, documento: &Documento) -> AppResult<DocumentoId>
    
    // Obtener documento por ID
    pub async fn obtener_por_id(&self, id: &DocumentoId) -> AppResult<Documento>
    
    // Listar documentos de un expediente
    pub async fn listar_por_expediente(&self, expediente_id: &ExpedienteId) -> AppResult<Vec<Documento>>
    
    // Listar documentos por categoría
    pub async fn listar_por_categoria(&self, categoria: &CategoriaDocumento) -> AppResult<Vec<Documento>>
    
    // Actualizar documento
    pub async fn actualizar(&self, documento: &Documento) -> AppResult<()>
    
    // Eliminar documento
    pub async fn eliminar(&self, id: &DocumentoId) -> AppResult<()>
    
    // Marcar documento como foliado
    pub async fn foliar(&self, id: &DocumentoId) -> AppResult<()>
    
    // Contar documentos de expediente
    pub async fn contar_por_expediente(&self, expediente_id: &ExpedienteId) -> AppResult<u64>
}
```

## Sistema de Búsqueda

### SearchService

Servicio de búsqueda avanzada con múltiples criterios y paginación:

```rust
impl SearchService {
    // Buscar expedientes con criterios
    pub async fn buscar_expedientes(
        &self,
        criteria: SearchCriteria
    ) -> AppResult<SearchResult<ExpedienteSearchResult>>
    
    // Buscar documentos con criterios
    pub async fn buscar_documentos(
        &self,
        criteria: SearchCriteria
    ) -> AppResult<SearchResult<DocumentoSearchResult>>
}
```

### Criterios de Búsqueda

```rust
pub struct SearchCriteria {
    pub cedula: Option<String>,           // Búsqueda por cédula (LIKE)
    pub apellido: Option<String>,          // Búsqueda por apellido (LIKE)
    pub nombre: Option<String>,           // Búsqueda por nombre (LIKE)
    pub categoria_documento: Option<String>,  // Filtro por categoría
    pub estado: Option<String>,           // Filtro por estado
    pub foliado: Option<bool>,            // Filtro por foliado
    pub page: u32,                        // Página actual (default: 1)
    pub page_size: u32,                   // Tamaño de página (default: 20)
}
```

### Resultados de Búsqueda

```rust
pub struct SearchResult<T> {
    pub data: Vec<T>,          // Datos de la página actual
    pub total: u64,            // Total de resultados
    pub page: u32,             // Página actual
    pub page_size: u32,        // Tamaño de página
    pub total_pages: u32,      // Total de páginas
}

pub struct ExpedienteSearchResult {
    pub id: String,
    pub nombres: String,
    pub apellidos: String,
    pub cedula: String,
    pub email: String,
    pub estado: String,
    pub documentos_count: usize,
    pub documentos_foliados_count: usize,
    pub creado_en: String,
}

pub struct DocumentoSearchResult {
    pub id: String,
    pub nombre_archivo: String,
    pub categoria: String,
    pub hash: String,
    pub foliado: bool,
    pub expediente_id: String,
    pub expediente_nombres: String,
    pub expediente_apellidos: String,
    pub creado_en: String,
}
```

### Ejemplo de Búsqueda

```rust
use escuela_storage::SearchService;
use escuela_storage::SearchCriteria;

let criteria = SearchCriteria {
    apellido: Some("García".to_string()),
    estado: Some("activo".to_string()),
    page: 1,
    page_size: 20,
    ..Default::default()
};

let resultados = search_service.buscar_expedientes(criteria).await?;
println!("Encontrados {} expedientes", resultados.total);
```

## Almacenamiento de Archivos

### FileStorageService

Servicio para almacenamiento seguro de archivos:

```rust
impl FileStorageService {
    // Crear directorio de expediente
    pub fn crear_directorio_expediente(&self, expediente_id: &str) -> AppResult<PathBuf>
    
    // Guardar archivo con hash
    pub fn guardar_archivo(
        &self,
        expediente_id: &str,
        nombre_archivo: &str,
        contenido: &[u8],
    ) -> AppResult<ArchivoGuardado>
    
    // Guardar archivo con nombre personalizado
    pub fn guardar_archivo_con_nombre_personalizado(
        &self,
        expediente_id: &str,
        nombre_archivo: &str,
        contenido: &[u8],
        prefijo: Option<&str>,
    ) -> AppResult<ArchivoGuardado>
    
    // Leer archivo
    pub fn leer_archivo(&self, ruta: &str) -> AppResult<Vec<u8>>
    
    // Eliminar archivo
    pub fn eliminar_archivo(&self, ruta: &str) -> AppResult<()>
    
    // Verificar integridad de archivo
    pub fn verificar_integridad(&self, ruta: &str, hash: &str) -> AppResult<bool>
}
```

### Estructura de Almacenamiento

```
storage/
├── {expediente_id_1}/
│   ├── {hash_sha256}.pdf
│   ├── {hash_sha256}.jpg
│   └── ...
├── {expediente_id_2}/
│   ├── {hash_sha256}.pdf
│   └── ...
└── ...
```

### Archivo Guardado

```rust
pub struct ArchivoGuardado {
    pub ruta_local: String,              // Ruta completa del archivo
    pub hash: String,                   // Hash SHA-256
    pub tamaño_bytes: u64,               // Tamaño en bytes
    pub nombre_archivo_original: String, // Nombre original
    pub nombre_archivo_hash: String,    // Nombre con hash
}
```

### Integridad de Archivos

- Cada archivo se guarda con su hash SHA-256 como nombre
- El hash se calcula automáticamente al guardar
- Se puede verificar la integridad en cualquier momento
- Evita duplicados con el mismo contenido

## Sistema de Auditoría

### AuditService

Servicio para registro completo de auditoría:

```rust
impl AuditService {
    // Registrar acción
    pub async fn registrar_accion(
        &self,
        usuario_id: Option<String>,
        accion: AccionAuditoria,
        detalles: String,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> AppResult<()>
    
    // Obtener historial de usuario
    pub async fn obtener_historial_usuario(
        &self,
        usuario_id: &str,
        limite: u32,
    ) -> AppResult<Vec<RegistroAuditoria>>
    
    // Obtener historial por acción
    pub async fn obtener_historial_accion(
        &self,
        accion: AccionAuditoria,
        limite: u32,
    ) -> AppResult<Vec<RegistroAuditoria>>
    
    // Obtener historial completo con paginación
    pub async fn obtener_historial_completo(
        &self,
        limite: u32,
        offset: u32,
    ) -> AppResult<Vec<RegistroAuditoria>>
    
    // Contar registros totales
    pub async fn contar_registros_totales(&self) -> AppResult<u64>
    
    // Obtener estadísticas
    pub async fn obtener_estadisticas(&self) -> AppResult<AuditoriaEstadisticas>
}
```

### Acciones Auditadas

```rust
pub enum AccionAuditoria {
    ConsultaExpediente,
    CreacionExpediente,
    ModificacionExpediente,
    EliminacionExpediente,
    ConsultaDocumento,
    SubidaDocumento,
    ModificacionDocumento,
    EliminacionDocumento,
    FoliadoDocumento,
    BusquedaAvanzada,
    LoginUsuario,
    LogoutUsuario,
    CambioEstadoExpediente,
    CreacionUsuario,
    ModificacionUsuario,
    EliminacionUsuario,
    LoginFallido,
}
```

### Registro de Auditoría

```rust
pub struct RegistroAuditoria {
    pub id: String,
    pub usuario_id: Option<String>,
    pub accion: AccionAuditoria,
    pub timestamp: DateTime<Utc>,
    pub detalles: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}
```

### Estadísticas de Auditoría

```rust
pub struct AuditoriaEstadisticas {
    pub total_registros: u64,
    pub acciones_por_tipo: Vec<(String, u64)>,
}
```

## Sistema de Backups

### BackupService

Servicio para copias de seguridad automáticas:

```rust
impl BackupService {
    // Crear nuevo backup
    pub async fn create_backup(&self) -> Result<String>
}
```

### Características de Backups

- **Compresión ZIP**: Todos los datos comprimidos en un solo archivo
- **Timestamp**: Cada backup tiene un timestamp único
- **Base de Datos**: Incluye el archivo SQLite completo
- **Archivos**: Incluye todo el directorio de storage
- **Limpieza Automática**: Mantiene solo los últimos 6 backups
- **Nombre Formato**: `backup_YYYY-MM-DD_HH-MM-SS.zip`

### Estructura de Backups

```
backups/
├── backup_2024-01-15_10-30-00.zip
├── backup_2024-01-16_10-30-00.zip
├── backup_2024-01-17_10-30-00.zip
└── ...
```

### Configuración

```rust
BackupService::new(
    "escuela.db",    // Ruta de base de datos
    "storage"        // Ruta de almacenamiento
)?;
```

### Limpieza Automática

El sistema mantiene automáticamente los últimos 6 backups:

- Ordena backups por fecha de modificación
- Elimina los más antiguos si exceden el límite
- Registra cada acción de limpieza en logs

## Dependencias

### Dependencias del Workspace

```toml
[dependencies]
sqlx = { workspace = true }           # ORM para SQLite
tokio = { workspace = true }          # Runtime asíncrono
anyhow = { workspace = true }         # Manejo de errores
thiserror = { workspace = true }      # Errores personalizados
tracing = { workspace = true }        # Logging
chrono = { workspace = true }         # Fechas y tiempos
zip = { workspace = true }            # Compresión ZIP
serde = { workspace = true }         # Serialización
uuid = { workspace = true }          # Identificadores únicos
escuela_core = { path = "../escuela_core" }      # Entidades del dominio
escuela_shared = { path = "../escuela_shared" }  # Tipos compartidos
```

## Ejemplos de Uso

### Inicializar Base de Datos

```rust
use escuela_storage::Database;

let database = Database::new("escuela.db").await?;
println!("Base de datos inicializada con migraciones");
```

### Usar Repositorios

```rust
use escuela_storage::repositories::{ExpedienteRepository, UsuarioRepository};
use escuela_core::domain::expediente::ExpedienteDocente;
use escuela_shared::Cedula;

// Crear repositorio de expedientes
let expediente_repo = ExpedienteRepository::new(database.pool().clone());

// Crear expediente
let expediente = ExpedienteDocente::nuevo(
    "Juan".to_string(),
    "Pérez".to_string(),
    Cedula::new("1234567890")?,
    "juan@email.com".to_string(),
    None,
    None,
    None,
    None,
    None,
    None,
)?;
let id = expediente_repo.crear(&expediente).await?;
```

### Búsqueda Avanzada

```rust
use escuela_storage::SearchService;
use escuela_storage::SearchCriteria;

let search_service = SearchService::new(database.pool().clone());

let criteria = SearchCriteria {
    apellido: Some("Pérez".to_string()),
    estado: Some("activo".to_string()),
    page: 1,
    page_size: 20,
    ..Default::default()
};

let resultados = search_service.buscar_expedientes(criteria).await?;
for expediente in resultados.data {
    println!("{} {}", expediente.nombres, expediente.apellidos);
}
```

### Almacenar Archivos

```rust
use escuela_storage::FileStorageService;

let file_storage = FileStorageService::new("storage")?;

let contenido = std::fs::read("documento.pdf")?;
let guardado = file_storage.guardar_archivo(
    "expediente-123",
    "titulo.pdf",
    &contenido,
)?;

println!("Archivo guardado: {}", guardado.ruta_local);
println!("Hash: {}", guardado.hash);
```

### Registrar Auditoría

```rust
use escuela_storage::AuditService;
use escuela_storage::audit::{AccionAuditoria};

let audit_service = AuditService::new(database.pool().clone());

audit_service.registrar_accion(
    Some("usuario-123".to_string()),
    AccionAuditoria::CreacionExpediente,
    "Expediente creado para Juan Pérez".to_string(),
    Some("192.168.1.100".to_string()),
    Some("Mozilla/5.0...".to_string()),
).await?;
```

### Crear Backup

```rust
use escuela_storage::BackupService;

let backup_service = BackupService::new("escuela.db", "storage")?;

let backup_path = backup_service.create_backup().await?;
println!("Backup creado: {}", backup_path);
```

## Testing

### Ejecutar Tests

```bash
# Ejecutar todos los tests del crate
cargo test -p escuela_storage

# Ejecutar tests con salida detallada
cargo test -p escuela_storage -- --nocapture

# Ejecutar tests de un módulo específico
cargo test -p escuela_storage database
cargo test -p escuela_storage repositories
cargo test -p escuela_storage search
```

### Tests de Integración

Los tests de integración usan una base de datos en memoria:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_crear_expediente() {
        let database = Database::new(":memory:").await.unwrap();
        let repo = ExpedienteRepository::new(database.pool().clone());
        
        let expediente = crear_expediente_test();
        let id = repo.crear(&expediente).await.unwrap();
        
        assert!(id.as_uuid().version() == uuid::Version::Random);
    }
}
```

## Consideraciones de Diseño

### SQLite Embebido

- **Portabilidad**: Base de datos autocontenida en un solo archivo
- **Sin Servidor**: No requiere instalación de servidor de base de datos
- **Bajo Consumo**: Mínimo uso de recursos
- **Offline-First**: Funciona completamente sin internet

### Connection Pooling

- **Eficiencia**: Reutilización de conexiones
- **Concurrencia**: Soporte para múltiples operaciones simultáneas
- **Configuración**: 5 conexiones máximas por defecto

### Hash de Archivos

- **Integridad**: SHA-256 para cada archivo
- **Deduplicación**: Evita archivos duplicados
- **Verificación**: Posible verificar integridad en cualquier momento

### Auditoría Completa

- **Tracking**: Todas las acciones críticas
- **Contexto**: IP, user agent, timestamp
- **Estadísticas**: Análisis de uso del sistema

### Backups Automáticos

- **Programación**: Se pueden programar backups automáticos
- **Compresión**: ZIP para reducir espacio
- **Limpieza**: Eliminación automática de backups antiguos

## Próximas Mejoras

- [ ] Soporte para PostgreSQL como alternativa
- [ ] Migraciones versionadas
- [ ] Índices FTS5 para búsqueda full-text
- [ ] Caching con Redis
- [ ] Replicación de base de datos
- [ ] Backups incrementales
- [ ] Compresión de archivos individuales
- [ ] Storage en nube (S3, Azure Blob)

## Licencia

Este crate está dual-licenciado bajo MIT License y Apache License 2.0, al igual que el proyecto principal.

## Contribuciones

Para contribuir a este crate, por favor revisa las guías en [CONTRIBUTING.md](../../CONTRIBUTING.md) y mantén los principios de Clean Architecture.
