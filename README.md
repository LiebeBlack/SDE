# Sistema de Gestión Institucional (100% Rust, 100% local)

Aplicación de escritorio para administración de una institución educativa:
estudiantes, familiares/tutores, empleados, departamentos, documentos (PDF/imágenes) y reportes en PDF.
Sin backend externo, sin nube: todo corre y se guarda en la máquina local
(SQLite embebido + archivos en `data/documentos/`).

## Cómo compilar

```bash
cargo build --release
cargo run -p app
```

## Estado actual

Funcionalidades implementadas:
- ✅ CRUD completo de Estudiantes (crear, editar, eliminar, listar, buscar)
- ✅ CRUD completo de Departamentos (crear, editar, eliminar, listar)
- ✅ CRUD completo de Familiares/Tutores (crear, editar, eliminar, listar)
- ✅ CRUD completo de Empleados (crear, editar, eliminar, listar)
- ✅ Gestión de asistencia de empleados (entrada, salida, almuerzo)
- ✅ Cálculo de horas trabajadas y horas extra
- ✅ Gestión de documentos (adjuntar PDF/imagen a estudiantes, abrir con visor nativo)
- ✅ Generación de constancias de estudios en PDF
- ✅ Dashboard con métricas en tiempo real

## Mejoras recientes

### Interfaz de Usuario
- Tema visual institucional profesional con colores institucionales
- Layout mejorado con panel lateral redimensionable
- Barra de estado con reloj en tiempo real
- Nombres de variables descriptivos y claros
- Confirmaciones de eliminación para evitar errores

### Base de Datos
- Transacciones con rollback automático en caso de error
- Configuración optimizada de SQLite (WAL, cache, etc.)
- Índices para búsquedas eficientes
- Validaciones de integridad referencial

### Manejo de Archivos
- Validaciones de extensión (PDF, PNG, JPG, etc.)
- Límite de tamaño de archivo (50MB máximo)
- Validación de nombres de archivo seguros
- Verificación de integridad de copias
- Eliminación segura de archivos

### Validaciones
- Validación inline de cédulas (formato sin librerías externas)
- Validación inline de email (formato básico)
- Validación de salarios y fechas
- Validación de estados contractuales

## Arquitectura

```
crates/
├── app/         → GUI (eframe/egui), un solo binario ejecutable
├── core/        → (paquete "app_core") modelos + lógica de negocio, sin GUI ni SQL
├── storage/     → SQLite (rusqlite) + repositorios + almacén de archivos
└── pdf_engine/  → generación (printpdf) y lectura (lopdf) de PDFs
```

- **`app_core` no depende de nada externo a Rust `std`**: define los modelos
  (`Estudiante`, `Familiar`, `Documento`, `Departamento`) y los *traits* de
  repositorio (`EstudianteRepositorio`, `DocumentoRepositorio`). Esto permite
  testear la lógica de negocio sin base de datos real, y cambiar de SQLite a
  otra cosa el día de mañana sin tocar `core`.
- **`storage` implementa esos traits** con SQLite. El esquema vive en
  `storage/src/migrations/0001_esquema_inicial.sql`.
- **`app` conecta todo**: arma la ventana con `egui`, mantiene el estado en
  `AppState` y llama a los servicios de `app_core` pasándoles los
  repositorios de `storage`.

## Estado actual del scaffold

Ya implementado y con flujo completo (UI → servicio → SQLite):
- Alta y listado de estudiantes con búsqueda.
- Adjuntar PDF/imagen a un estudiante (se copian a `data/documentos/`).
- Generar constancia de estudios en PDF para un estudiante.

Pendiente (siguiente iteración natural):
- CRUD de `Familiar`/`RelacionFamiliar` (modelo y tabla ya existen).
- CRUD de `Departamento` (modelo y tabla ya existen, falta la vista).
- Edición/eliminación de estudiantes desde la UI (los métodos de
  repositorio ya existen: `eliminar`, `buscar_por_id`).
- Vista previa de documentos (usar `pdf_engine::reader` para extraer
  texto o mostrar miniatura).

Dime cuál de estos quieres que desarrolle primero y seguimos.
