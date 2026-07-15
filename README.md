# 🏫 Sistema de Gestión Escolar - Tesis Universitaria

<div align="center">

![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)
![License](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)
![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey.svg)
![Architecture](https://img.shields.io/badge/Architecture-Clean%20Architecture-brightgreen.svg)
![Status](https://img.shields.io/badge/Status-Production%20Ready-success.svg)
![Frontend](https://img.shields.io/badge/Frontend-Modern%20JavaScript-blue.svg)
![Database](https://img.shields.io/badge/Database-SQLite-green.svg)
![Security](https://img.shields.io/badge/Security-JWT%20%2B%20RBAC-red.svg)
![Offline](https://img.shields.io/badge/Offline-First-success.svg)

**Sistema modular y extensible para la gestión integral de expedientes docentes y administrativos en instituciones educativas. Desarrollado en Rust con arquitectura limpia (Clean Architecture) y optimizado para portabilidad absoluta y bajo consumo de recursos.**

[🚀 Inicio Rápido](#-instalación-y-ejecución-modo-offline-first) • [📖 Documentación](#-documentación) • [🔧 API Endpoints](#-api-endpoints) • [🎨 Características](#-características-destacadas)

</div>

## ✨ Características Destacadas

- 🚀 **Alto Rendimiento**: Compilado con Rust para máxima velocidad y seguridad
- 🏗️ **Clean Architecture**: Separación clara de responsabilidades con diseño modular
- 🔒 **Seguridad Máxima**: Autenticación JWT, RBAC, y validación en tiempo de compilación
- 📱 **Offline-First**: Funciona completamente sin conexión a internet
- 🎯 **Portabilidad Absoluta**: Binario único sin dependencias externas
- 🌐 **Multi-dispositivo**: Accesible desde cualquier PC en la red local
- 📊 **Dashboard Inteligente**: Estadísticas en tiempo real y búsqueda avanzada
- 🔧 **Panel Admin Completo**: Backup, exportación PDF, gestión de usuarios y más
- 🎨 **Interfaz Moderna**: Diseño Glassmorphism con JavaScript ES6+
- 📦 **PWA Ready**: Service Worker para funcionamiento offline

## 📋 Tabla de Contenidos

- [🏗️ Arquitectura](#-arquitectura-del-workspace)
- [📦 Crates](#-propósito-de-cada-crate)
- [🚀 Características](#-características-principales)
- [⚙️ Instalación](#-instalación-y-ejecución-modo-offline-first)
- [🔧 API](#-api-endpoints)
- [📝 Ejemplos](#-ejemplo-de-uso)
- [🧪 Testing](#-testing)
- [📊 Datos](#-estructura-de-datos)
- [🎨 Frontend](#-demo-y-características-de-la-interfaz)
- [� Documentación](#-documentación)
- [� Licencia](#-licencia)

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

- [x] ✅ Implementar autenticación JWT
- [x] ✅ Agregar sistema de auditoría completo
- [x] ✅ Implementar búsqueda full-text con FTS5 de SQLite
- [x] ✅ Agregar interfaz web moderna con JavaScript
- [x] ✅ Implementar exportación de expedientes a PDF
- [x] ✅ Agregar sistema de backups automáticos
- [x] ✅ Implementar panel de administración completo
- [x] ✅ Agregar dashboard de estadísticas
- [ ] Implementar notificaciones por email
- [ ] Agregar modo multi-idioma
- [ ] Implementar sincronización con nube (opcional)
- [ ] Agregar tests de integración completos

## 🎨 Demo y Características de la Interfaz

### Interfaz Principal

El sistema cuenta con una interfaz web moderna y responsiva con:

- **Dashboard Interactivo**: Estadísticas en tiempo real del sistema
- **Gestión de Expedientes**: CRUD completo con búsqueda avanzada
- **Gestión de Documentos**: Upload de archivos con drag-and-drop
- **Panel de Administración**: Control total del sistema

### Características de la Interfaz

- 🎨 **Diseño Moderno**: Glassmorphism y animaciones suaves
- 📱 **Responsive**: Funciona en desktop, tablet y móvil
- ♿ **Accesible**: WCAG 2.1 AA compliant
- ⌨️ **Atajos de Teclado**: Ctrl+K (buscar), Ctrl+N (nuevo), Escape (cerrar)
- 🔍 **Búsqueda Inteligente**: Debouncing y navegación automática
- ✅ **Validación en Tiempo Real**: Feedback visual inmediato
- 🌙 **Modo Oscuro**: Tema oscuro automático
- 📦 **PWA Ready**: Instalable como aplicación nativa

### Tecnologías del Frontend

- **JavaScript ES6+**: Módulos modernos con import/export
- **Lucide Icons**: Iconos ligeros y modernos
- **CSS Glassmorphism**: Diseño con efectos de vidrio
- **Service Worker**: Soporte offline completo
- **LocalStorage**: Caché de datos para modo offline
- **Fetch API**: Comunicación asíncrona con el backend

### Estructura del Frontend

```
static/
├── index.html              # Aplicación principal
├── admin.html             # Panel de administración
├── css/
│   ├── style.css          # Estilos principales
│   └── admin.css          # Estilos del panel admin
├── js/
│   ├── main.js            # Punto de entrada
│   ├── core/
│   │   ├── api.js         # Cliente API
│   │   ├── offline-storage.js  # Almacenamiento local
│   │   └── sync-manager.js     # Sincronización
│   ├── modules/
│   │   ├── expedientes.js      # Gestión expedientes
│   │   ├── documentos.js       # Gestión documentos
│   │   └── admin.js            # Gestión admin
│   └── ui/
│       └── components.js   # Componentes UI
├── lucide.min.js          # Biblioteca de iconos
├── sw.js                  # Service Worker
└── manifest.json          # Manifiesto PWA
```

### Panel de Administración

Funcionalidades avanzadas para administradores:

- 📦 **Backup Automático**: Copias de seguridad programadas
- 📊 **Reportes PDF**: Exportación de datos a PDF
- 👥 **Gestión de Usuarios**: Control de accesos y roles
- 🔍 **Auditoría Completa**: Registro de todas las acciones
- 🧹 **Mantenimiento**: Limpieza de caché y actualización de datos
- 📋 **Listado Personal**: Exportación CSV de personal
- 🗜️ **Backup Completo**: Descarga ZIP de todo el sistema

## 🛠️ Tecnologías Utilizadas

### Backend (Rust)
- **Axum**: Framework web asíncrono
- **SQLx**: Queries SQL type-safe
- **Tokio**: Runtime asíncrono
- **Serde**: Serialización/deserialización
- **JWT**: Autenticación con tokens
- **Argon2**: Hashing de contraseñas
- **SQLite**: Base de datos embebida

### Frontend (JavaScript)
- **ES6 Modules**: Sistema de módulos moderno
- **Fetch API**: Comunicación HTTP
- **Service Workers**: Soporte offline
- **LocalStorage**: Almacenamiento local
- **Lucide Icons**: Biblioteca de iconos
- **CSS3**: Estilos modernos con animaciones

### DevOps
- **Cargo**: Gestión de paquetes Rust
- **Git**: Control de versiones
- **GitHub Actions**: CI/CD (opcional)
- **Docker**: Contenedores (opcional)

## 🗺️ Roadmap del Proyecto

### ✅ Completado
- [x] Arquitectura Clean Architecture
- [x] Sistema de autenticación JWT
- [x] Control de acceso RBAC
- [x] Gestión de expedientes completa
- [x] Gestión de documentos con hash
- [x] Interfaz web moderna
- [x] Panel de administración
- [x] Sistema de auditoría
- [x] Backup automático
- [x] Modo offline-first
- [x] PWA capabilities

### 🚧 En Progreso
- [ ] Tests de integración completos
- [ ] Documentación API Swagger
- [ ] Optimización de performance

### 📋 Planeado
- [ ] Notificaciones por email
- [ ] Sistema multi-idioma
- [ ] Sincronización con nube (opcional)
- [ ] Móvil nativo (React Native)
- [ ] API GraphQL
- [ ] Sistema de reportes avanzado
- [ ] Integración con sistemas externos

## 📖 Documentación

El proyecto incluye documentación detallada en varios archivos:

### 📚 Documentación Principal
- **[ARCHITECTURE.md](ARCHITECTURE.md)** - Arquitectura técnica detallada del sistema
- **[DEPLOYMENT.md](DEPLOYMENT.md)** - Guía completa de deployment y configuración
- **[CONTRIBUTING.md](CONTRIBUTING.md)** - Guía para contribuidores del proyecto
- **[CHANGELOG.md](CHANGELOG.md)** - Historial de cambios y versiones

### 🎓 Documentación Académica
- **[DOCUMENTACION_ACADEMICA/](DOCUMENTACION_ACADEMICA/)** - Documentación formal de la tesis
  - Propuesta de tesis
  - Marco teórico
  - Diseño del sistema
  - Conclusiones y recomendaciones

### 🔧 Documentación Técnica
- **[docs/](docs/)** - Documentación técnica adicional
  - Guías de instalación
  - Configuración avanzada
  - Solución de problemas

### 📖 Guías de Usuario
- **[static/README.md](static/README.md)** - Documentación del frontend
- **[escuela_api/README.md](escuela_api/README.md)** - Documentación de la API
- **[escuela_core/README.md](escuela_core/README.md)** - Documentación del dominio

## 🤝 Contribución

Las contribuciones son bienvenidas! Por favor sigue estos pasos:

1. **Fork** el repositorio
2. **Crea** una rama para tu feature (`git checkout -b feature/AmazingFeature`)
3. **Commit** tus cambios (`git commit -m 'Add some AmazingFeature'`)
4. **Push** a la rama (`git push origin feature/AmazingFeature`)
5. **Abre** un Pull Request

### Guías de Contribución
- Lee la [CONTRIBUTING.md](CONTRIBUTING.md) para detalles
- Sigue el código de conducta del proyecto
- Asegúrate de que los tests pasen
- Documenta tus cambios apropiadamente

## 🐛 Reportar Bugs

Para reportar bugs, por favor:

1. Usa el [GitHub Issues](https://github.com/tu-usuario/SDE/issues)
2. Busca si el bug ya fue reportado
3. Si no, crea un nuevo issue con:
   - Título descriptivo
   - Pasos para reproducir
   - Comportamiento esperado vs actual
   - Screenshots si es aplicable
   - Información del sistema (OS, versión, etc.)

## � Solicitar Features

Para solicitar nuevas funcionalidades:

1. Abre un [GitHub Issue](https://github.com/tu-usuario/SDE/issues)
2. Usa el template de "Feature Request"
3. Describe la funcionalidad deseada
4. Explica el caso de uso
5. Sugiere una implementación si es posible

## 📞 Soporte

Para soporte y preguntas:

- 📖 Revisa la [documentación](#-documentación)
- 🐛 Abre un [issue](https://github.com/tu-usuario/SDE/issues) para bugs
- 💡 Abre un [issue](https://github.com/tu-usuario/SDE/issues) para features
- 💬 Únete a las discusiones (si están disponibles)

## 🙏 Agradecimientos

- **Rust Community** - Por el lenguaje y ecosistema increíble
- **Axum Framework** - Por el framework web asíncrono y eficiente
- **SQLite** - Por la base de datos embebida y portátil
- **Lucide Icons** - Por los iconos modernos y ligeros
- **Tokio Project** - Por el runtime asíncrono robusto
- **Serde** - Por la serialización eficiente y type-safe

## � Licencia

Este proyecto está dual-licenciado bajo:

- **MIT License** - Para uso comercial y proyectos de código abierto
- **Apache License 2.0** - Para proyectos que requieren patentes

Puedes elegir la licencia que mejor se adapte a tus necesidades.

## 👨‍💻 Autor

Desarrollado como parte de tesis universitaria para gestión documental de instituciones educativas.

**Desarrollador Principal**: Yoangel De Dios Níkolas Gómez Gómez  
**Contacto**: @liebeblack  
**Proyecto**: Sistema de Gestión Escolar - Tesis Universitaria

---

<div align="center">

**⭐ Si te gusta este proyecto, dale una estrella en GitHub!**

**Made with ❤️ and 🦀 Rust**

</div>
