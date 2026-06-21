# Sistema de Gestión Escolar - Tesis Universitaria

Sistema modular y extensible para la gestión integral de expedientes docentes y administrativos en instituciones educativas. Desarrollado en Rust con arquitectura limpia (Clean Architecture) y optimizado para portabilidad absoluta y bajo consumo de recursos.

## 🏗️ Arquitectura del Workspace

El proyecto utiliza un **Workspace Multicrate** de Rust para separar responsabilidades según los principios de Clean Architecture/Hexagonal:

```
TesisYoangel/
├── Cargo.toml                    # Workspace root con optimizaciones extremas
├── escuela_core/                 # Dominio y lógica de negocio pura
│   ├── src/
│   │   ├── domain/              # Entidades del dominio
│   │   │   ├── usuario.rs       # Usuario con RBAC (Director, RRHH, Admin)
│   │   │   ├── documento.rs     # Documento con hash y metadatos
│   │   │   └── expediente.rs    # ExpedienteDocente completo
│   │   └── services/            # Traits de servicios (interfaces)
│   └── Cargo.toml
├── escuela_storage/              # Capa de persistencia (SQLite)
│   ├── src/
│   │   ├── database.rs          # Inicialización y migraciones
│   │   └── repositories/        # Implementaciones de repositorios
│   │       ├── usuario_repository.rs
│   │       ├── expediente_repository.rs
│   │       └── documento_repository.rs
│   └── Cargo.toml
├── escuela_api/                  # Capa de infraestructura HTTP
│   ├── src/
│   │   ├── main.rs              # Punto de entrada
│   │   ├── server.rs            # Configuración del servidor Axum
│   │   ├── state.rs             # Estado compartido (AppState)
│   │   ├── routes.rs            # Definición de rutas
│   │   └── handlers/            # Handlers HTTP
│   │       ├── health_handler.rs
│   │       ├── expediente_handler.rs
│   │       └── documento_handler.rs
│   └── Cargo.toml
├── escuela_shared/               # Tipos y utilidades compartidas
│   ├── src/
│   │   ├── lib.rs               # Tipos Email, Cedula
│   │   ├── error.rs             # AppError y conversión a HTTP
│   │   └── validation.rs        # Funciones de validación
│   └── Cargo.toml
└── README.md
```

## 📦 Propósito de Cada Crate

### **escuela_core**
- **Responsabilidad**: Contiene la lógica de negocio pura y las entidades del dominio.
- **Contenido**: 
  - Entidades fuertemente tipadas: `Usuario`, `Documento`, `ExpedienteDocente`
  - Enums para roles y categorías con validación en tiempo de compilación
  - Traits de servicios que definen contratos para la capa de infraestructura
- **Dependencias**: Solo `escuela_shared` (sin dependencias de infraestructura)

### **escuela_storage**
- **Responsabilidad**: Implementación de persistencia usando SQLite embebido.
- **Contenido**:
  - `Database`: Inicialización automática de SQLite y migraciones
  - Repositorios: `UsuarioRepository`, `ExpedienteRepository`, `DocumentoRepository`
  - Mapeo entre entidades del dominio y esquema de base de datos
- **Dependencias**: `escuela_core`, `escuela_shared`, `sqlx`

### **escuela_api**
- **Responsabilidad**: Servidor HTTP RESTful usando Axum.
- **Contenido**:
  - Servidor asíncrono con Tokio
  - Handlers para crear, obtener, listar y buscar expedientes
  - Upload de archivos con detección automática de MIME type
  - CORS y tracing configurados
- **Dependencias**: `escuela_core`, `escuela_storage`, `escuela_shared`, `axum`

### **escuela_shared**
- **Responsabilidad**: Tipos y utilidades compartidas entre crates.
- **Contenido**:
  - Tipos valor: `Email`, `Cedula` con validación
  - `AppError`: Enum de errores unificado con conversión a respuestas HTTP
  - Funciones de validación reutilizables
- **Dependencias**: Solo crates externos (serde, validator, thiserror)

## 🚀 Características Principales

### **Modo Offline-First (Local)**
- **Funcionamiento completamente local**: No requiere conexión a internet
- **Servidor autónomo**: Ejecuta en localhost o red local (LAN)
- **Sin dependencias externas**: Todos los recursos (CSS, JS, iconos) incluidos localmente
- **Base de datos SQLite**: Almacenamiento local sin servidor de base de datos
- **Acceso multi-dispositivo**: Configurado para acceso desde cualquier PC en la red local

### **Seguridad Máxima**
- **Tipado estricto**: Validación en tiempo de compilación para evitar estados inválidos
- **Inmutabilidad de archivos**: Hash SHA-256 para cada documento almacenado
- **RBAC**: Roles (Director, RecursosHumanos, Administrador) con permisos granulares
- **Validación**: Usando `validator` crate para validación de datos en tiempo de ejecución

### **Portabilidad Absoluta**
- **SQLite embebido**: Base de datos autocontenida sin necesidad de servidor externo
- **Binario único**: Compilación con optimizaciones extremas para tamaño mínimo
- **Sin dependencias externas**: Todo corre en una sola máquina sin internet

### **Modularidad Ampliable**
- **Clean Architecture**: Separación clara entre dominio, infraestructura y presentación
- **Traits de servicios**: Fácil agregar nuevas implementaciones (ej. cambiar a PostgreSQL)
- **Crate independiente**: Cada crate puede compilarse y testearse independientemente

## 🛠️ Instalación y Ejecución (Modo Offline-First)

### **Requisitos Previos**
- Rust 1.75 o superior
- Sistema operativo: Windows, Linux o macOS
- **NO requiere conexión a internet** para funcionamiento

### **Compilación y Ejecución Local**

```bash
# Clonar el repositorio
cd TesisYoangel

# Compilar en modo release (optimizado para producción)
cargo build --release

# Ejecutar el servidor (se iniciará en http://localhost:3000)
cargo run --release

# O ejecutar el binario compilado directamente
./target/release/escuela_api  # Linux/Mac
./target/release/escuela_api.exe  # Windows
```

### **Acceso desde la misma PC**
1. Ejecutar el servidor como se indica arriba
2. Abrir navegador en: `http://localhost:3000`
3. El sistema funcionará completamente sin conexión a internet

### **Acceso desde otros dispositivos en red local (LAN)**

Para acceder desde otras computadoras en la misma red:

1. **Asegurar que el servidor esté escuchando en 0.0.0.0** (configuración por defecto)
2. **Obtener la IP local** del servidor:
   - Windows: `ipconfig` (buscar IPv4 Address)
   - Linux/Mac: `ifconfig` o `ip a` (buscar inet)
3. **Acceder desde otro dispositivo** usando la IP:
   ```
   http://192.168.1.X:3000
   ```
4. **Configurar firewall** si es necesario para permitir conexiones en el puerto 3000

### **Variables de Entorno (Opcionales)**

| Variable | Descripción | Valor por Defecto |
|----------|-------------|-------------------|
| `DATABASE_PATH` | Ruta del archivo SQLite | `escuela.db` |
| `STORAGE_PATH` | Ruta de almacenamiento de documentos | `storage` |
| `STATIC_PATH` | Ruta de archivos estáticos (HTML, CSS, JS) | `static` |
| `BIND_ADDRESS` | Dirección y puerto del servidor | `0.0.0.0:3000` |

### **Scripts de Deployment**

El proyecto incluye scripts para facilitar el deployment:

**Windows**:
```bash
deploy.bat
```

**Linux/Mac**:
```bash
chmod +x deploy.sh
./deploy.sh
```

### **Verificación de Funcionamiento Offline**

Para verificar que el sistema funciona completamente sin internet:

1. **Desconectar internet** del servidor
2. **Ejecutar el servidor**: `cargo run --release`
3. **Abrir navegador** en: `http://localhost:3000`
4. **Verificar que**:
   - La página carga correctamente (sin errores de CDN)
   - Los iconos Lucide se muestran correctamente
   - El login funciona
   - Se pueden crear expedientes
   - Se pueden subir documentos
   - Todas las funcionalidades operan normalmente

### **Troubleshooting Offline**

**Problema**: "No se pudo conectar con el servidor local"
- **Solución**: Verificar que el servidor esté ejecutándose en el puerto 3000
- **Comando**: `netstat -an | findstr 3000` (Windows) o `lsof -i :3000` (Linux/Mac)

**Problema**: "Iconos no se muestran"
- **Solución**: Verificar que la carpeta `static/js/lucide.min.js` exista
- **Comando**: Verificar que el archivo tenga ~400KB (tamaño correcto)

**Problema**: "No puedo acceder desde otra PC en la red"
- **Solución**: 
  1. Verificar que BIND_ADDRESS sea `0.0.0.0:3000`
  2. Configurar firewall para permitir puerto 3000
  3. Verificar que ambas PCs estén en la misma red

## 📡 API Endpoints

### **Salud**
- `GET /health` - Verificar estado del servidor

### **Expedientes**
- `POST /expedientes` - Crear nuevo expediente
- `GET /expedientes` - Listar todos los expedientes
- `GET /expedientes/:id` - Obtener expediente por ID
- `GET /expedientes/cedula/:cedula` - Obtener expediente por cédula
- `GET /expedientes/buscar/:termino` - Buscar expedientes
- `POST /expedientes/:id/estado` - Cambiar estado del expediente

### **Documentos**
- `POST /expedientes/:expediente_id/documentos` - Subir documento (multipart)
- `GET /expedientes/:expediente_id/documentos` - Listar documentos de expediente
- `POST /expedientes/:expediente_id/documentos/:documento_id/foliar` - Foliar documento

## 📝 Ejemplo de Uso

### **Crear Expediente**

```bash
curl -X POST http://localhost:3000/expedientes \
  -H "Content-Type: application/json" \
  -d '{
    "nombres": "Juan",
    "apellidos": "Pérez",
    "cedula": "12345678",
    "email": "juan.perez@escuela.edu",
    "telefono": "+1234567890",
    "nacionalidad": "Ecuatoriana"
  }'
```

### **Subir Documento**

```bash
curl -X POST http://localhost:3000/expedientes/{expediente_id}/documentos \
  -F "nombre_archivo=titulo.pdf" \
  -F "categoria=titulo_academico" \
  -F "archivo=@/ruta/al/titulo.pdf"
```

## 🔧 Optimizaciones de Compilación

El `Cargo.toml` raíz incluye optimizaciones extremas para el perfil `release`:

```toml
[profile.release]
opt-level = "z"        # Optimizar para tamaño mínimo
lto = true             # Link Time Optimization
codegen-units = 1      # Unidad de código única para mejor optimización
panic = "abort"        # Reducir tamaño del binario
strip = true           # Remover símbolos de debug
```

Esto resulta en un binario extremadamente ligero y eficiente, ideal para ejecutarse en hardware antiguo o limitado.

## 🧪 Testing

```bash
# Ejecutar todos los tests
cargo test

# Ejecutar tests con salida detallada
cargo test -- --nocapture

# Ejecutar tests de un crate específico
cargo test -p escuela_core
```

## 📊 Estructura de Datos

### **Usuario**
```rust
struct Usuario {
    id: UsuarioId,
    nombre: String,
    apellido: String,
    email: Email,
    cedula: Cedula,
    rol: Rol,  // Director, RecursosHumanos, Administrador
    activo: bool,
    // ... timestamps
}
```

### **Documento**
```rust
struct Documento {
    id: DocumentoId,
    nombre_archivo: String,
    categoria: CategoriaDocumento,
    hash: HashArchivo,  // SHA-256
    ruta_local: String,
    foliado: bool,
    // ... metadatos
}
```

### **ExpedienteDocente**
```rust
struct ExpedienteDocente {
    id: ExpedienteId,
    nombres: String,
    apellidos: String,
    cedula: Cedula,
    email: String,
    estado: EstadoExpediente,
    documentos: Vec<Documento>,
    // ... datos adicionales
}
```

## 🎯 Próximos Pasos

- [ ] Implementar autenticación JWT
- [ ] Agregar sistema de auditoría completo
- [ ] Implementar búsqueda full-text con FTS5 de SQLite
- [ ] Agregar interfaz web con React
- [ ] Implementar exportación de expedientes a PDF
- [ ] Agregar sistema de backups automáticos
- [ ] Implementar notificaciones por email
- [ ] Agregar dashboard de estadísticas

## 📄 Licencia

MIT OR Apache-2.0

## 👨‍💻 Autor

Desarrollado como parte de tesis universitaria para gestión documental de instituciones educativas.
