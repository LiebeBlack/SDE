CREATE TABLE IF NOT EXISTS departamentos (
    id TEXT PRIMARY KEY,
    nombre TEXT NOT NULL,
    descripcion TEXT,
    responsable TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS empleados (
    id TEXT PRIMARY KEY,
    cedula TEXT NOT NULL UNIQUE,
    nombre TEXT NOT NULL,
    apellido TEXT NOT NULL,
    email TEXT NOT NULL,
    telefono TEXT,
    direccion TEXT,
    cargo TEXT NOT NULL,
    departamento_id TEXT,
    fecha_contratacion TEXT NOT NULL,
    fecha_terminacion TEXT,
    salario REAL NOT NULL,
    tipo_contrato TEXT NOT NULL,
    estado TEXT NOT NULL,
    notas TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (departamento_id) REFERENCES departamentos(id)
);

CREATE TABLE IF NOT EXISTS registros_asistencia (
    id TEXT PRIMARY KEY,
    empleado_id TEXT NOT NULL,
    fecha TEXT NOT NULL,
    hora TEXT NOT NULL,
    tipo TEXT NOT NULL,
    notas TEXT,
    creado_en TEXT NOT NULL,
    FOREIGN KEY (empleado_id) REFERENCES empleados(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_asistencia_empleado ON registros_asistencia(empleado_id);
CREATE INDEX IF NOT EXISTS idx_asistencia_fecha ON registros_asistencia(fecha);
CREATE INDEX IF NOT EXISTS idx_empleados_departamento ON empleados(departamento_id);
CREATE INDEX IF NOT EXISTS idx_empleados_estado ON empleados(estado);

CREATE TABLE IF NOT EXISTS estudiantes (
    id TEXT PRIMARY KEY,
    matricula TEXT NOT NULL UNIQUE,
    nombre TEXT NOT NULL,
    apellido TEXT NOT NULL,
    fecha_nacimiento TEXT NOT NULL,
    grado_nivel TEXT NOT NULL,
    estado TEXT NOT NULL,
    direccion TEXT,
    telefono TEXT,
    email TEXT,
    notas TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS familiares (
    id TEXT PRIMARY KEY,
    nombre TEXT NOT NULL,
    apellido TEXT NOT NULL,
    documento_identidad TEXT,
    telefono TEXT,
    telefono_alterno TEXT,
    email TEXT,
    direccion TEXT,
    ocupacion TEXT,
    es_contacto_emergencia INTEGER NOT NULL DEFAULT 0,
    notas TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS relaciones_familiares (
    id TEXT PRIMARY KEY,
    estudiante_id TEXT NOT NULL REFERENCES estudiantes(id) ON DELETE CASCADE,
    familiar_id TEXT NOT NULL REFERENCES familiares(id) ON DELETE CASCADE,
    parentesco TEXT NOT NULL,
    es_titular_responsable INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS documentos (
    id TEXT PRIMARY KEY,
    entidad_tipo TEXT NOT NULL,
    entidad_id TEXT NOT NULL,
    tipo_documento TEXT NOT NULL,
    nombre_original TEXT NOT NULL,
    ruta_archivo TEXT NOT NULL,
    mime_type TEXT NOT NULL,
    tamano_bytes INTEGER NOT NULL,
    subido_en TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS auditoria (
    id TEXT PRIMARY KEY,
    entidad TEXT NOT NULL,
    entidad_id TEXT NOT NULL,
    accion TEXT NOT NULL,
    detalle TEXT,
    fecha TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_documentos_entidad ON documentos(entidad_tipo, entidad_id);
CREATE INDEX IF NOT EXISTS idx_relaciones_estudiante ON relaciones_familiares(estudiante_id);
CREATE INDEX IF NOT EXISTS idx_relaciones_familiar ON relaciones_familiares(familiar_id);
CREATE INDEX IF NOT EXISTS idx_estudiantes_nombre ON estudiantes(nombre, apellido);
