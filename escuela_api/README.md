# escuela_api - Capa de Infraestructura HTTP y API RESTful

![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)
![License](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)
![Architecture](https://img.shields.io/badge/Architecture-Clean%20Architecture-brightgreen.svg)
![Framework](https://img.shields.io/badge/Framework-Axum-green.svg)

> **escuela_api** es el crate de infraestructura HTTP del Sistema de Gestión Escolar que implementa una API RESTful usando el framework web Axum, con autenticación JWT, manejo de archivos, CORS, logging estructurado y verificación de integridad del sistema.

## 📋 Tabla de Contenidos

- [Visión General](#visión-general)
- [Responsabilidades](#responsabilidades)
- [Arquitectura](#arquitectura)
- [Estructura del Módulo](#estructura-del-módulo)
- [Servidor HTTP](#servidor-http)
- [Rutas API](#rutas-api)
- [Handlers](#handlers)
- [Autenticación JWT](#autenticación-jwt)
- [Estado Compartido](#estado-compartido)
- [Middleware](#middleware)
- [Verificación de Integridad](#verificación-de-integridad)
- [Archivos Estáticos](#archivos-estáticos)
- [Dependencias](#dependencias)
- [Ejemplos de Uso](#ejemplos-de-uso)
- [Testing](#testing)

## Visión General

**escuela_api** representa la capa de presentación en la arquitectura limpia del sistema. Implementa una API RESTful usando Axum, un framework web asíncrono y eficiente para Rust, con autenticación JWT, manejo de archivos multipart, CORS, logging estructurado y verificación de integridad del sistema al arranque.

### Características Principales

- **API RESTful**: Endpoints bien diseñados siguiendo principios REST
- **Autenticación JWT**: Tokens JWT seguros con validación automática
- **Handlers Asíncronos**: Operaciones no bloqueantes con Tokio
- **Manejo de Archivos**: Upload multipart con detección MIME type
- **CORS Configurado**: Soporte para跨域 solicitudes
- **Logging Estructurado**: Tracing para request/response logging
- **Rate Limiting**: Protección contra brute force en login
- **Integridad del Sistema**: Verificación automática al arranque
- **Backups Automáticos**: Tarea en segundo plano cada 2 horas
- **Archivos Estáticos**: Servido de interfaz web integrado

## Responsabilidades

### Responsabilidades Principales

1. **Servidor HTTP**
   - Configuración del servidor Axum
   - Gestión de conexiones TCP
   - Middleware (CORS, tracing, logging)
   - Manejo de errores HTTP

2. **Rutas y Endpoints**
   - Definición de rutas públicas y protegidas
   - Routing de solicitudes a handlers
   - Extracción de parámetros de ruta y query
   - Servido de archivos estáticos

3. **Handlers HTTP**
   - Procesamiento de requests HTTP
   - Validación de inputs
   - Llamada a servicios de negocio
   - Formateo de responses

4. **Autenticación y Autorización**
   - Validación de tokens JWT
   - Extracción automática de usuario
   - Verificación de permisos
   - Rate limiting en login

5. **Estado Compartido**
   - Gestión de repositorios
   - Pool de conexiones a base de datos
   - Servicios de búsqueda y almacenamiento
   - Tracking de intentos de login

6. **Integridad del Sistema**
   - Verificación al arranque
   - Validación de base de datos
   - Validación de almacenamiento
   - Detección de inconsistencias

## Arquitectura

### Capa de Presentación

```
┌─────────────────────────────────────────────────────────────┐
│                    Cliente (Browser)                          │
└─────────────────────────────────────────────────────────────┘
                            ↓ HTTP/HTTPS
┌─────────────────────────────────────────────────────────────┐
│              escuela_api (Presentación)                       │
│  ┌─────────────────────────────────────────────────────────┐ │
│  │  Server: Axum HTTP server                              │ │
│  │  Routes: Definición de endpoints                       │ │
│  │  Handlers: Procesamiento de requests                   │ │
│  │  Middleware: CORS, tracing, auth                       │ │
│  │  State: Repositorios y servicios compartidos           │ │
│  │  Static Files: Servido de interfaz web                 │ │
│  └─────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│              escuela_storage (Infraestructura)                │
│         Repositorios, búsqueda, almacenamiento, auditoría     │
└─────────────────────────────────────────────────────────────┘
                            ↓
┌─────────────────────────────────────────────────────────────┐
│              escuela_core (Dominio)                          │
│         Entidades, lógica de negocio, seguridad              │
└─────────────────────────────────────────────────────────────┘
```

### Reglas de Dependencia

- **escuela_api** depende de:
  - escuela_core (entidades del dominio y seguridad)
  - escuela_storage (repositorios y servicios)
  - escuela_shared (tipos y errores compartidos)
  - axum (framework web)
  - Crates estándar (tokio, serde, tracing, etc.)

- **escuela_api** NO depende de:
  - Otros crates de presentación
  - Frameworks de frontend (React, Vue, etc.)

## Estructura del Módulo

```
escuela_api/
├── src/
│   ├── main.rs                 # Punto de entrada del binario
│   ├── lib.rs                  # Exportaciones del crate
│   ├── server.rs               # Configuración del servidor Axum
│   ├── routes.rs               # Definición de rutas
│   ├── state.rs                # Estado compartido (AppState)
│   ├── auth.rs                 # Middleware de autenticación JWT
│   ├── integrity.rs            # Verificación de integridad
│   ├── error_response.rs       # Manejo de errores HTTP
│   └── handlers/               # Handlers HTTP
│       ├── mod.rs              # Exportaciones del módulo
│       ├── health_handler.rs   # Health check
│       ├── auth_handler.rs     # Login y autenticación
│       ├── usuario_handler.rs  # Gestión de usuarios
│       ├── expediente_handler.rs  # Gestión de expedientes
│       ├── documento_handler.rs    # Gestión de documentos
│       ├── search_handler.rs  # Búsqueda avanzada
│       └── audit_handler.rs   # Auditoría
└── Cargo.toml                  # Dependencias del crate
```

## Servidor HTTP

### Configuración del Servidor

```rust
use escuela_api::run_server;

pub async fn run_server(
    database_path: String,
    storage_path: String,
    static_path: String,
    bind_address: String,
) -> anyhow::Result<()>
```

### Inicialización

```rust
// Inicializar logging estructurado
tracing_subscriber::registry()
    .with(tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "escuela_api=debug,tower_http=debug,axum=trace".into()))
    .with(tracing_subscriber::fmt::layer())
    .init();

// Inicializar base de datos
let database = Database::new(&database_path).await?;
let pool = database.pool().clone();

// Crear estado compartido
let state = AppState::new(pool, &storage_path);

// Configurar CORS
let cors = CorsLayer::new()
    .allow_origin(Any)
    .allow_methods(Any)
    .allow_headers(Any)
    .allow_credentials(false)
    .expose_headers([CONTENT_TYPE]);

// Crear router con middleware
let app = create_routes(state, static_path)
    .layer(cors)
    .layer(TraceLayer::new_for_http());

// Iniciar servidor
let listener = tokio::net::TcpListener::bind(&bind_address).await?;
axum::serve(listener, app).await?;
```

### Variables de Entorno

| Variable | Descripción | Valor por Defecto |
|----------|-------------|-------------------|
| `DATABASE_PATH` | Ruta del archivo SQLite | `escuela.db` |
| `STORAGE_PATH` | Ruta de almacenamiento de documentos | `storage` |
| `STATIC_PATH` | Ruta de archivos estáticos (HTML, CSS, JS) | `static` |
| `BIND_ADDRESS` | Dirección y puerto del servidor | `0.0.0.0:3000` |

### Configuración por Defecto

```rust
// Valores por defecto si no se especifican variables de entorno
DATABASE_PATH = "escuela.db"
STORAGE_PATH = "storage"
STATIC_PATH = "static" (detecta automáticamente cwd o exe)
BIND_ADDRESS = "0.0.0.0:3000"
```

## Rutas API

### Rutas Públicas (Sin Autenticación)

```rust
// Health check
GET /health
GET /api/health

// Login
POST /login
POST /api/login
```

### Rutas Protegidas (Requieren JWT)

#### Usuarios y Auditoría

```rust
// Usuarios
POST /api/usuarios
GET /api/usuarios
POST /api/usuarios/:usuario_id/toggle

// Auditoría
GET /api/auditoria
```

#### Expedientes

```rust
// CRUD de expedientes
POST /api/expedientes
GET /api/expedientes
GET /api/expedientes/:id
GET /api/expedientes/cedula/:cedula
POST /api/expedientes/:id/estado

// Búsqueda
GET /api/expedientes/buscar/:termino
```

#### Documentos

```rust
// CRUD de documentos
POST /api/expedientes/:expediente_id/documentos
GET /api/expedientes/:expediente_id/documentos
POST /api/expedientes/:expediente_id/documentos/:documento_id/foliar
GET /api/expedientes/:expediente_id/documentos/:documento_id/descargar
```

#### Búsqueda Avanzada

```rust
// Búsqueda avanzada
GET /api/buscar/expedientes_avanzado
GET /api/buscar/documentos_avanzado
GET /api/buscar/general
```

### Rutas de Compatibilidad (Root)

Para compatibilidad con el frontend que no usa `/api`:

```rust
GET /health
POST /login
POST /expedientes
GET /expedientes
GET /expedientes/:id
POST /expedientes/:id/estado
GET /expedientes/cedula/:cedula
POST /expedientes/:expediente_id/documentos
GET /expedientes/:expediente_id/documentos
POST /expedientes/:expediente_id/documentos/:documento_id/foliar
GET /expedientes/:expediente_id/documentos/:documento_id/descargar
POST /usuarios/:usuario_id/toggle
```

### Archivos Estáticos

```rust
GET /static/*          // Servido de archivos estáticos
*                     // Fallback a index.html (SPA)
```

## Handlers

### Health Handler

```rust
pub async fn health_check() -> impl IntoResponse {
    Json(json!({
        "status": "healthy",
        "timestamp": Utc::now().to_rfc3339(),
        "service": "escuela-api"
    }))
}
```

### Auth Handler

```rust
// Login con rate limiting
pub async fn login(
    State(state): State<AppState>,
    Json(creds): Json<LoginRequest>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse

// Estructuras
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub struct LoginResponse {
    pub token: String,
    pub usuario: Usuario,
}
```

### Usuario Handler

```rust
// Crear usuario
pub async fn crear_usuario(
    State(state): State<AppState>,
    usuario: Usuario,
) -> impl IntoResponse

// Listar usuarios
pub async fn listar_usuarios(
    State(state): State<AppState>,
) -> impl IntoResponse

// Toggle estado de usuario
pub async fn toggle_usuario_estado(
    State(state): State<AppState>,
    Path(usuario_id): Path<String>,
) -> impl IntoResponse
```

### Expediente Handler

```rust
// Crear expediente
pub async fn crear_expediente(
    State(state): State<AppState>,
    usuario: Usuario,
    Json(expediente): Json<CrearExpedienteRequest>,
) -> impl IntoResponse

// Obtener expediente por ID
pub async fn obtener_expediente(
    State(state): State<AppState>,
    usuario: Usuario,
    Path(id): Path<String>,
) -> impl IntoResponse

// Obtener expediente por cédula
pub async fn obtener_expediente_por_cedula(
    State(state): State<AppState>,
    usuario: Usuario,
    Path(cedula): Path<String>,
) -> impl IntoResponse

// Listar expedientes
pub async fn listar_expedientes(
    State(state): State<AppState>,
    usuario: Usuario,
) -> impl IntoResponse

// Buscar expedientes
pub async fn buscar_expedientes(
    State(state): State<AppState>,
    usuario: Usuario,
    Path(termino): Path<String>,
) -> impl IntoResponse

// Cambiar estado
pub async fn cambiar_estado(
    State(state): State<AppState>,
    usuario: Usuario,
    Path(id): Path<String>,
    Json(req): Json<CambiarEstadoRequest>,
) -> impl IntoResponse
```

### Documento Handler

```rust
// Crear documento (upload multipart)
pub async fn crear_documento(
    State(state): State<AppState>,
    usuario: Usuario,
    Path(expediente_id): Path<String>,
    mut multipart: Multipart,
) -> impl IntoResponse

// Listar documentos de expediente
pub async fn listar_documentos(
    State(state): State<AppState>,
    usuario: Usuario,
    Path(expediente_id): Path<String>,
) -> impl IntoResponse

// Foliar documento
pub async fn foliar_documento(
    State(state): State<AppState>,
    usuario: Usuario,
    Path((expediente_id, documento_id)): Path<(String, String)>,
) -> impl IntoResponse

// Descargar documento
pub async fn descargar_documento(
    State(state): State<AppState>,
    usuario: Usuario,
    Path((expediente_id, documento_id)): Path<(String, String)>,
) -> impl IntoResponse
```

### Search Handler

```rust
// Búsqueda avanzada de expedientes
pub async fn buscar_expedientes_avanzado(
    State(state): State<AppState>,
    usuario: Usuario,
    Query(criteria): Query<SearchCriteria>,
) -> impl IntoResponse

// Búsqueda avanzada de documentos
pub async fn buscar_documentos_avanzado(
    State(state): State<AppState>,
    usuario: Usuario,
    Query(criteria): Query<SearchCriteria>,
) -> impl IntoResponse

// Búsqueda general
pub async fn buscar_general(
    State(state): State<AppState>,
    usuario: Usuario,
    Query(params): Query<SearchParams>,
) -> impl IntoResponse
```

### Audit Handler

```rust
// Listar auditoría
pub async fn listar_auditoria(
    State(state): State<AppState>,
    usuario: Usuario,
    Query(params): Query<AuditParams>,
) -> impl IntoResponse
```

## Autenticación JWT

### Claims del Token

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,        // Usuario ID (UUID)
    pub email: String,      // Email del usuario
    pub rol: String,        // Rol del usuario
    pub exp: usize,         // Expiración (timestamp)
    pub iat: usize,         // Emitido en (timestamp)
}
```

### Generación de Token

```rust
use jsonwebtoken::{encode, EncodingKey, Header};

let claims = Claims {
    sub: usuario.id.as_uuid().to_string(),
    email: usuario.email.as_str().to_string(),
    rol: usuario.rol.as_str().to_string(),
    exp: (Utc::now() + Duration::hours(24)).timestamp() as usize,
    iat: Utc::now().timestamp() as usize,
};

let token = encode(
    &Header::default(),
    &claims,
    &EncodingKey::from_secret(&get_jwt_secret()),
)?;
```

### Validación Automática

El middleware de autenticación extrae automáticamente el usuario del token JWT:

```rust
#[async_trait]
impl FromRequestParts<AppState> for Usuario {
    type Rejection = AuthError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // Extraer token Bearer del header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::MissingCredentials)?;

        // Decodificar y validar token
        let token_data = decode::<Claims>(
            bearer.token(),
            &DecodingKey::from_secret(&get_jwt_secret()),
            &Validation::default(),
        )
        .map_err(|_| AuthError::InvalidToken)?;

        // Obtener usuario desde la base de datos
        let uuid = Uuid::parse_str(&token_data.claims.sub)
            .map_err(|_| AuthError::InvalidToken)?;
        let usuario_id = UsuarioId::from_uuid(uuid);
        let usuario = state.usuario_repo.obtener_por_id(&usuario_id).await
            .map_err(|_| AuthError::InvalidToken)?;

        // Verificar si el usuario está activo
        if !usuario.activo {
            return Err(AuthError::InvalidToken);
        }

        Ok(usuario)
    }
}
```

### Rate Limiting en Login

```rust
// Tracking de intentos fallidos
let mut attempts = state.login_attempts.lock().unwrap();
let key = format!("{}:{}", addr.ip(), creds.email);

if let Some((count, last_attempt)) = attempts.get(&key) {
    if *count >= 5 && last_attempt + Duration::minutes(15) > Utc::now() {
        return Err((StatusCode::TOO_MANY_REQUESTS, "Demasiados intentos. Espere 15 minutos.").into_response());
    }
}
```

## Estado Compartido

### AppState

```rust
#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    pub expediente_repo: Arc<ExpedienteRepository>,
    pub documento_repo: Arc<DocumentoRepository>,
    pub usuario_repo: Arc<UsuarioRepository>,
    pub search_service: Arc<SearchService>,
    pub file_storage: Arc<FileStorageService>,
    pub audit_service: Arc<AuditService>,
    pub login_attempts: Arc<Mutex<HashMap<String, (u32, DateTime<Utc>)>>>,
}
```

### Inicialización

```rust
impl AppState {
    pub fn new(pool: SqlitePool, storage_path: &str) -> Self {
        let expediente_repo = Arc::new(ExpedienteRepository::new(pool.clone()));
        let documento_repo = Arc::new(DocumentoRepository::new(pool.clone()));
        let usuario_repo = Arc::new(UsuarioRepository::new(pool.clone()));
        let search_service = Arc::new(SearchService::new(pool.clone()));
        let file_storage = Arc::new(FileStorageService::new(storage_path).unwrap());
        let audit_service = Arc::new(AuditService::new(pool.clone()));

        AppState {
            pool,
            expediente_repo,
            documento_repo,
            usuario_repo,
            search_service,
            file_storage,
            audit_service,
            login_attempts: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}
```

## Middleware

### CORS Layer

```rust
let cors = CorsLayer::new()
    .allow_origin(Any)
    .allow_methods(Any)
    .allow_headers(Any)
    .allow_credentials(false)
    .expose_headers([CONTENT_TYPE]);
```

### Trace Layer

```rust
let app = create_routes(state, static_path)
    .layer(TraceLayer::new_for_http());
```

### Logging

```rust
tracing_subscriber::registry()
    .with(
        tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "escuela_api=debug,tower_http=debug,axum=trace".into())
    )
    .with(tracing_subscriber::fmt::layer())
    .init();
```

## Verificación de Integridad

### Integrity Report

```rust
pub struct IntegrityReport {
    pub database_valid: bool,
    pub database_path: String,
    pub expedientes_count: u64,
    pub documentos_count: u64,
    pub storage_valid: bool,
    pub storage_path: String,
    pub storage_folders_count: usize,
    pub storage_files_count: usize,
    pub audit_records_count: u64,
    pub issues: Vec<String>,
}
```

### Verificación al Arranque

```rust
let integrity_report = verify_integrity(&database_path, &storage_path).await?;

if !integrity_report.is_healthy() {
    println!("⚠️  ADVERTENCIA: Se detectaron problemas de integridad.");
    println!("   El sistema se iniciará en modo degradado.");
}
```

### Reporte de Integridad

```
🔍 Iniciando verificación de integridad del sistema...
✅ Base de datos encontrada: 24576 bytes
📊 Expedientes registrados: 15
📄 Documentos almacenados: 47
📝 Registros de auditoría: 234
📂 Carpetas de expedientes: 15
📁 Archivos almacenados: 47

📋 REPORTE DE INTEGRIDAD DEL SISTEMA
═══════════════════════════════════════
Base de datos: ✅ VÁLIDA
Almacenamiento: ✅ VÁLIDO
Expedientes: 15
Documentos: 47
Registros auditoría: 234
Carpetas almacenamiento: 15
Archivos almacenamiento: 47

✅ Sistema en estado óptimo - Sin problemas detectados
═══════════════════════════════════════
```

## Archivos Estáticos

### Servido de Archivos

```rust
// Servir archivos estáticos en /static
.nest_service("/static", ServeDir::new(&static_path))

// Fallback a index.html para SPA
.fallback_service(ServeDir::new(&static_path))
```

### Detección Automática de Ruta

```rust
let default_static = {
    let cwd_static = env::current_dir().unwrap_or_default().join("static");
    let exe_static = env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.join("static")));

    if cwd_static.exists() {
        cwd_static.to_string_lossy().to_string()
    } else if let Some(exe_static) = exe_static {
        if exe_static.exists() {
            exe_static.to_string_lossy().to_string()
        } else {
            "static".to_string()
        }
    } else {
        "static".to_string()
    }
};
```

### Estructura de Archivos Estáticos

```
static/
├── index.html          # Aplicación principal
├── admin.html          # Panel de administración
├── css/
│   └── style.css       # Estilos
├── js/
│   ├── main.js         # JavaScript principal
│   ├── core/           # Módulos core
│   ├── modules/        # Módulos de funcionalidad
│   └── ui/             # Componentes UI
├── manifest.json       # PWA manifest
└── sw.js              # Service worker
```

## Backups Automáticos

### Tarea en Segundo Plano

```rust
// Iniciar tarea en segundo plano para copias de seguridad (cada 2 horas)
tokio::spawn(async move {
    let interval_duration = Duration::from_secs(2 * 60 * 60); // 2 horas
    let mut interval = tokio::time::interval(interval_duration);
    
    interval.tick().await; // Esperar primer intervalo
    
    loop {
        interval.tick().await;
        match BackupService::new(&backup_db_path, &backup_storage_path) {
            Ok(backup_service) => {
                info!("Ejecutando copia de seguridad programada...");
                if let Err(e) = backup_service.create_backup().await {
                    error!("Error en la copia de seguridad programada: {}", e);
                }
            }
            Err(e) => {
                error!("Error al inicializar servicio de copias de seguridad: {}", e);
            }
        }
    }
});
```

## Dependencias

### Dependencias del Workspace

```toml
[dependencies]
axum = { workspace = true }              # Framework web
tower = { workspace = true }             # Middleware utilities
tower-http = { workspace = true }        # HTTP middleware (CORS, tracing)
axum-extra = { workspace = true }        # Extra Axum features (headers)
jsonwebtoken = { workspace = true }      # JWT tokens
tokio = { workspace = true }             # Runtime asíncrono
serde = { workspace = true }             # Serialización
serde_json = { workspace = true }        # JSON
tracing = { workspace = true }            # Logging
tracing-subscriber = { workspace = true } # Logging subscriber
anyhow = { workspace = true }            # Manejo de errores
infer = { workspace = true }             # Detección de tipo MIME
mime_guess = { workspace = true }        # Guess MIME types
escuela_core = { path = "../escuela_core" }      # Dominio
escuela_storage = { path = "../escuela_storage" }  # Persistencia
escuela_shared = { path = "../escuela_shared" }    # Tipos compartidos
sqlx = { workspace = true }             # Database
chrono = { workspace = true }            # Fechas
uuid = { workspace = true }             # UUIDs
async-trait = { workspace = true }       # Async traits
```

## Ejemplos de Uso

### Iniciar el Servidor

```bash
# Con configuración por defecto
cargo run --release

# Con variables de entorno
DATABASE_PATH=my_db.db \
STORAGE_PATH=documents \
STATIC_PATH=web \
BIND_ADDRESS=127.0.0.1:8080 \
cargo run --release
```

### Login

```bash
curl -X POST http://localhost:3000/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "super@tesis.com",
    "password": "SuperAdmin2026!"
  }'
```

Response:
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "usuario": {
    "id": "...",
    "nombre": "Super",
    "apellido": "Admin",
    "email": "super@tesis.com",
    "rol": "super"
  }
}
```

### Crear Expediente

```bash
curl -X POST http://localhost:3000/expedientes \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer {token}" \
  -d '{
    "nombres": "Juan",
    "apellidos": "Pérez",
    "cedula": "1234567890",
    "email": "juan@email.com",
    "telefono": "+593991234567"
  }'
```

### Subir Documento

```bash
curl -X POST http://localhost:3000/expedientes/{expediente_id}/documentos \
  -H "Authorization: Bearer {token}" \
  -F "nombre_archivo=titulo.pdf" \
  -F "categoria=titulo_academico" \
  -F "archivo=@/ruta/al/titulo.pdf"
```

### Búsqueda Avanzada

```bash
curl -X GET "http://localhost:3000/buscar/expedientes_avanzado?apellido=Pérez&estado=activo&page=1&page_size=20" \
  -H "Authorization: Bearer {token}"
```

## Testing

### Ejecutar Tests

```bash
# Ejecutar todos los tests del crate
cargo test -p escuela_api

# Ejecutar tests con salida detallada
cargo test -p escuela_api -- --nocapture

# Ejecutar tests de un handler específico
cargo test -p escuela_api auth_handler
cargo test -p escuela_api expediente_handler
```

### Tests de Integración

Los tests de integración usan una base de datos en memoria:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_login() {
        let state = create_test_state().await;
        let request = LoginRequest {
            email: "super@tesis.com".to_string(),
            password: "SuperAdmin2026!".to_string(),
        };
        
        let response = login(State(state), Json(request), ConnectInfo(addr)).await;
        assert!(response.status().is_success());
    }
}
```

## Consideraciones de Diseño

### Thin Handlers

Los handlers son "thin" - delegan la lógica de negocio a los servicios:

```rust
// ✅ Bueno: Handler delega a servicio
pub async fn crear_expediente(
    State(state): State<AppState>,
    usuario: Usuario,
    Json(req): Json<CrearExpedienteRequest>,
) -> impl IntoResponse {
    let expediente = req.to_expediente(usuario.id);
    let id = state.expediente_repo.crear(&expediente).await?;
    Ok(Json(json!({ "id": id.as_uuid().to_string() })))
}

// ❌ Malo: Handler con lógica de negocio
pub async fn crear_expediente(...) {
    // Validaciones complejas
    // Lógica de negocio
    // Cálculos derivados
}
```

### Validación en Handlers

La validación de inputs se hace en los handlers antes de llamar a servicios:

```rust
// Validar request
req.validate().map_err(|e| {
    (StatusCode::BAD_REQUEST, Json(json!({ "error": e.to_string() })))
})?;

// Convertir a entidad del dominio
let expediente = req.to_expediente(usuario.id);

// Llamar a servicio
let id = state.expediente_repo.crear(&expediente).await?;
```

### Manejo de Errores

Los errores se convierten a respuestas HTTP apropiadas:

```rust
pub enum AuthError {
    InvalidToken,
    WrongCredentials,
    MissingCredentials,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Credenciales incorrectas"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Token no proporcionado"),
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Token inválido o expirado"),
        };
        let body = Json(json!({ "error": error_message }));
        (status, body).into_response()
    }
}
```

### Auditoría en Handlers

Cada handler registra la acción en el sistema de auditoría:

```rust
// Registrar acción
state.audit_service.registrar_accion(
    Some(usuario.id.as_uuid().to_string()),
    AccionAuditoria::CreacionExpediente,
    format!("Expediente creado para {}", expediente.nombre_completo()),
    Some(ip_address),
    Some(user_agent),
).await?;
```

## Próximas Mejoras

- [ ] Implementar refresh tokens
- [ ] Agregar rate limiting general
- [ ] Implementar WebSocket para notificaciones en tiempo real
- [ ] Agregar soporte para GraphQL
- [ ] Implementar cache de responses
- [ ] Agregar métricas de Prometheus
- [ ] Implementar OpenAPI/Swagger documentation
- [ ] Agregar soporte para WebAssembly

## Licencia

Este crate está dual-licenciado bajo MIT License y Apache License 2.0, al igual que el proyecto principal.

## Contribuciones

Para contribuir a este crate, por favor revisa las guías en [CONTRIBUTING.md](../../CONTRIBUTING.md) y mantén los principios de Clean Architecture.
