# Deployment Guide - School Management System

## Overview

This guide provides comprehensive instructions for deploying the Sistema de Gestión Escolar in various environments, from development to production.

## Prerequisites

- Rust 1.75 or higher
- System: Windows, Linux, or macOS
- Minimum 512MB RAM
- 100MB disk space for application
- Additional space for document storage

## Compilation Instructions

### Manual Compilation

```bash
# Compile in development mode
cargo build

# Compile in release mode (optimized for production)
cargo build --release
```

The compiled binary will be located at:
- **Windows**: `target/release/escuela_api.exe`
- **Linux/macOS**: `target/release/escuela_api`

### Cross-Compilation

For cross-compilation to different targets:

```bash
# Add target
rustup target add x86_64-unknown-linux-musl

# Cross-compile
cargo build --release --target x86_64-unknown-linux-musl
```

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

## Environment Variables

| Variable | Description | Default Value |
|----------|-------------|-------------------|
| `DATABASE_PATH` | SQLite database file path | `escuela.db` |
| `STORAGE_PATH` | Document storage directory | `storage` |
| `STATIC_PATH` | Static files directory (HTML, CSS, JS) | `static` |
| `BIND_ADDRESS` | Server address and port | `0.0.0.0:3000` |

### Custom Configuration

#### Linux/macOS
```bash
export DATABASE_PATH=my_school.db
export STORAGE_PATH=documents
export BIND_ADDRESS=127.0.0.1:8080
./deploy.sh start
```

#### Windows
```cmd
set DATABASE_PATH=my_school.db
set STORAGE_PATH=documents
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

## Production Deployment

### System Service Setup (Linux)

Create a systemd service file:

```bash
sudo nano /etc/systemd/system/escuela-api.service
```

Add the following content:

```ini
[Unit]
Description=School Management System API
After=network.target

[Service]
Type=simple
User=escuela
WorkingDirectory=/opt/escuela-api
ExecStart=/opt/escuela-api/escuela_api
Restart=always
RestartSec=10
Environment="DATABASE_PATH=/opt/escuela-api/escuela.db"
Environment="STORAGE_PATH=/opt/escuela-api/storage"
Environment="BIND_ADDRESS=0.0.0.0:3000"

[Install]
WantedBy=multi-user.target
```

Enable and start the service:

```bash
sudo systemctl daemon-reload
sudo systemctl enable escuela-api
sudo systemctl start escuela-api
sudo systemctl status escuela-api
```

### Windows Service Setup

Use NSSM (Non-Sucking Service Manager):

```cmd
nssm install "School Management API" "C:\path\to\escuela_api.exe"
nssm set "School Management API" AppDirectory "C:\path\to"
nssm set "School Management API" DisplayName "School Management System"
nssm start "School Management API"
```

### Docker Deployment (Optional)

Create a `Dockerfile`:

```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/escuela_api /usr/local/bin/
WORKDIR /app
COPY static /app/static
EXPOSE 3000
CMD ["escuela_api"]
```

Build and run:

```bash
docker build -t escuela-api .
docker run -p 3000:3000 -v $(pwd)/storage:/app/storage escuela-api
```

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

## Troubleshooting

### Server won't start

1. Verify binary exists: `ls target/release/escuela_api` (Linux/macOS) or `dir target\release\escuela_api.exe` (Windows)
2. Check port availability: `netstat -tuln | grep 3000` (Linux/macOS) or `netstat -an | findstr :3000` (Windows)
3. Review logs: Check console output or log files
4. Verify file permissions

### Database errors

1. Verify `.db` file exists
2. Check write permissions on directory
3. Run integrity check on startup
4. Check SQLite version compatibility

### Storage issues

1. Verify storage directory exists
2. Check write permissions
3. Verify available disk space
4. Check file system integrity

## Security

- **SHA-256 Hashing**: Each file has a unique hash for integrity verification
- **RBAC**: Role-Based Access Control (Director, HR, Administrator)
- **Complete Audit Trail**: All actions are logged
- **Compile-time Validation**: Rust's strict typing prevents errors
- **JWT Authentication**: Secure token-based authentication
- **Rate Limiting**: Protection against brute force attacks
- **Input Validation**: All inputs are validated before processing

## Backup and Recovery

### Automated Backups

The system includes automatic backup functionality. Configure backup schedule:

```bash
# Set backup interval in environment
export BACKUP_INTERVAL_HOURS=24
export BACKUP_RETENTION_DAYS=30
```

### Manual Backup

```bash
# Database backup
cp escuela.db escuela_backup_$(date +%Y%m%d).db

# Storage backup
tar -czf storage_backup_$(date +%Y%m%d).tar.gz storage/
```

### Recovery

```bash
# Restore database
cp escuela_backup_20240621.db escuela.db

# Restore storage
tar -xzf storage_backup_20240621.tar.gz
```

## Monitoring

### Health Check

```bash
curl http://localhost:3000/health
```

Expected response:
```json
{
  "status": "healthy",
  "database": "connected",
  "storage": "accessible"
}
```

### Log Monitoring

Monitor application logs for:
- Error messages
- Failed authentication attempts
- Database connection issues
- Storage problems

## Performance Tuning

### Database Optimization

```sql
-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_expedientes_cedula ON expedientes(cedula);
CREATE INDEX IF NOT EXISTS idx_documentos_expediente_id ON documentos(expediente_id);
CREATE INDEX IF NOT EXISTS idx_auditoria_usuario_id ON auditoria(usuario_id);
```

### Connection Pooling

Adjust pool size in environment:

```bash
export DATABASE_POOL_SIZE=10
```

## Support

For technical issues or deployment questions:
- Review server logs
- Run status script for diagnostics
- Check integrity report on startup
- Open GitHub issue for bugs

---

For additional information, see the main [README.md](README.md) and [ARCHITECTURE.md](ARCHITECTURE.md).
