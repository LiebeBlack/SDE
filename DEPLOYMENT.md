# Guía de Despliegue - Sistema de Gestión Escolar

## Instrucciones de Compilación

### Requisitos Previos
- Rust 1.75 o superior
- Sistema operativo: Windows, Linux o macOS

### Compilación Manual

```bash
# Compilar en modo desarrollo
cargo build

# Compilar en modo release (optimizado para producción)
cargo build --release
```

El binario compilado se encontrará en:
- **Windows**: `target/release/escuela_api.exe`
- **Linux/macOS**: `target/release/escuela_api`

## Scripts de Automatización

### Linux/macOS

```bash
# Dar permisos de ejecución al script
chmod +x deploy.sh

# Compilar el proyecto
./deploy.sh compile

# Iniciar el servidor
./deploy.sh start

# Verificar estado
./deploy.sh status

# Detener el servidor
./deploy.sh stop

# Reiniciar el servidor
./deploy.sh restart

# Despliegue completo (compilar + iniciar)
./deploy.sh deploy
```

### Windows

```cmd
REM Compilar el proyecto
deploy.bat compile

REM Iniciar el servidor
deploy.bat start

REM Verificar estado
deploy.bat status

REM Detener el servidor
deploy.bat stop

REM Reiniciar el servidor
deploy.bat restart

REM Despliegue completo (compilar + iniciar)
deploy.bat deploy
```

## Variables de Entorno

| Variable | Descripción | Valor por Defecto |
|----------|-------------|-------------------|
| `DATABASE_PATH` | Ruta del archivo SQLite | `escuela.db` |
| `STORAGE_PATH` | Ruta de almacenamiento de archivos | `storage` |
| `BIND_ADDRESS` | Dirección y puerto del servidor | `0.0.0.0:3000` |

### Configuración Personalizada

#### Linux/macOS
```bash
export DATABASE_PATH=mi_escuela.db
export STORAGE_PATH=archivos
export BIND_ADDRESS=127.0.0.1:8080
./deploy.sh start
```

#### Windows
```cmd
set DATABASE_PATH=mi_escuela.db
set STORAGE_PATH=archivos
set BIND_ADDRESS=127.0.0.1:8080
deploy.bat start
```

## Verificación de Integridad

El sistema realiza automáticamente una verificación de integridad al arranque:

- ✅ Verifica existencia y validez de la base de datos
- ✅ Cuenta expedientes y documentos registrados
- ✅ Verifica integridad de carpetas de almacenamiento
- ✅ Detecta inconsistencias entre DB y archivos
- ✅ Genera reporte técnico en consola

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

## Auditoría del Sistema

El sistema registra automáticamente todas las acciones críticas en la tabla `auditoria_accesos`:

- Consultas de expedientes
- Creaciones y modificaciones
- Subidas de documentos
- Cambios de estado
- Búsquedas avanzadas

### Acciones Auditadas

- `CONSULTA_EXPEDIENTE` - Consulta de expediente
- `CREACION_EXPEDIENTE` - Creación de expediente
- `MODIFICACION_EXPEDIENTE` - Modificación de expediente
- `ELIMINACION_EXPEDIENTE` - Eliminación de expediente
- `SUBIDA_DOCUMENTO` - Subida de documento
- `FOLIADO_DOCUMENTO` - Foliado de documento
- `BUSQUEDA_AVANZADA` - Búsqueda avanzada
- `CAMBIO_ESTADO_EXPEDIENTE` - Cambio de estado

## Portabilidad

El sistema está diseñado para ser 100% portable:

- **Binario único**: Todo el sistema en un solo ejecutable
- **Sin dependencias externas**: No requiere servidores de base de datos
- **SQLite embebido**: Base de datos autocontenida
- **Almacenamiento local**: Archivos en disco local
- **Configuración por variables de entorno**: Fácil de personalizar

### Despliegue en Hardware Limitado

El sistema está optimizado para correr en hardware antiguo o limitado:

- **Binario optimizado**: Compilación con `opt-level = "z"` para tamaño mínimo
- **Bajo consumo de RAM**: SQLite embebido consume mínimos recursos
- **Sin dependencias de red**: Funciona completamente offline
- **Inicio rápido**: Menos de 2 segundos en arranque

## Solución de Problemas

### El servidor no inicia

1. Verificar que el binario existe: `ls target/release/escuela_api` (Linux/macOS) o `dir target\release\escuela_api.exe` (Windows)
2. Verificar que el puerto no está en uso: `netstat -tuln | grep 3000` (Linux/macOS) o `netstat -an | findstr :3000` (Windows)
3. Revisar logs: `cat server.log` (Linux/macOS) o `type server.log` (Windows)

### Errores de base de datos

1. Verificar que el archivo `.db` existe
2. Verificar permisos de escritura en el directorio
3. Ejecutar verificación de integridad al iniciar

### Problemas de almacenamiento

1. Verificar que el directorio de almacenamiento existe
2. Verificar permisos de escritura
3. Verificar espacio en disco disponible

## Optimizaciones de Producción

El `Cargo.toml` incluye optimizaciones extremas:

```toml
[profile.release]
opt-level = "z"        # Optimizar para tamaño mínimo
lto = true             # Link Time Optimization
codegen-units = 1      # Unidad de código única
panic = "abort"        # Reducir tamaño del binario
strip = true           # Remover símbolos de debug
```

Esto resulta en un binario de aproximadamente 3-5 MB, ideal para despliegue en escuelas con hardware limitado.

## Seguridad

- **Hash SHA-256**: Cada archivo tiene un hash único para verificar integridad
- **RBAC**: Control de acceso basado en roles (Director, RRHH, Administrador)
- **Auditoría completa**: Todas las acciones quedan registradas
- **Validación en tiempo de compilación**: Tipado estricto de Rust previene errores

## Soporte

Para problemas técnicos o preguntas sobre el despliegue:
- Revisar los logs del servidor
- Ejecutar el script con el comando `status` para diagnóstico
- Verificar el reporte de integridad al iniciar
