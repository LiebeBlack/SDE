# escuela_shared - Tipos y Utilidades Compartidas

![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)
![License](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)
![Architecture](https://img.shields.io/badge/Architecture-Clean%20Architecture-brightgreen.svg)

> **escuela_shared** es el crate de utilidades compartidas del Sistema de Gestión Escolar que proporciona tipos valor con validación, manejo de errores unificado, funciones de validación comunes y tipos reutilizables entre todos los crates del workspace.

## 📋 Tabla de Contenidos

- [Visión General](#visión-general)
- [Responsabilidades](#responsabilidades)
- [Arquitectura](#arquitectura)
- [Estructura del Módulo](#estructura-del-módulo)
- [Tipos Valor](#tipos-valor)
- [Manejo de Errores](#manejo-de-errores)
- [Validación](#validación)
- [Dependencias](#dependencias)
- [Ejemplos de Uso](#ejemplos-de-uso)
- [Testing](#testing)

## Visión General

**escuela_shared** es el crate base del workspace que contiene tipos y utilidades compartidas entre todos los demás crates. Sigue el principio DRY (Don't Repeat Yourself) proporcionando tipos valor con validación, manejo de errores unificado y funciones de validación comunes que son utilizadas por escuela_core, escuela_storage y escuela_api.

### Características Principales

- **Tipos Valor con Validación**: Email y Cédula con validación en tiempo de construcción
- **Manejo de Errores Unificado**: Enum de errores personalizado con conversión a HTTP
- **Validación Reutilizable**: Funciones de validación comunes para todo el sistema
- **Sin Dependencias Circulares**: Solo depende de crates externos, no de otros crates del workspace
- **Type Safety**: Tipos fuertes que previenen errores en tiempo de compilación
- **Serialización**: Soporte completo para serde

## Responsabilidades

### Responsabilidades Principales

1. **Tipos Valor con Validación**
   - Email con validación de formato
   - Cédula con validación de formato venezolano
   - Validación en tiempo de construcción
   - Inmutabilidad garantizada

2. **Manejo de Errores Unificado**
   - Enum de errores personalizado
   - Conversión automática a respuestas HTTP
   - Tipos de resultado personalizados
   - Mensajes de error descriptivos

3. **Validación Compartida**
   - Funciones de validación reutilizables
   - Reglas de validación consistentes
   - Errores de validación estandarizados

4. **Tipos Reutilizables**
   - Alias de tipos comunes
   - Utilidades de conversión
   - Helpers de serialización

## Arquitectura

### Crate Base del Workspace

```
┌─────────────────────────────────────────────────────────────┐
│              escuela_shared (Base)                           │
│  ┌─────────────────────────────────────────────────────────┐ │
│  │  Value Types: Email, Cedula (con validación)           │ │
│  │  Error Handling: AppError, AppResult                    │ │
│  │  Validation: Funciones de validación comunes            │ │
│  │  Utilities: Helpers y tipos reutilizables               │ │
│  └─────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
           ↓                    ↓                    ↓
┌──────────────────┐  ┌──────────────────┐  ┌──────────────────┐
│  escuela_core    │  │ escuela_storage  │  │  escuela_api     │
│  (Dominio)       │  │ (Infraestructura) │  │  (Presentación)  │
└──────────────────┘  └──────────────────┘  └──────────────────┘
```

### Reglas de Dependencia

- **escuela_shared** depende de:
  - serde (serialización)
  - serde_json (JSON)
  - chrono (fechas)
  - uuid (identificadores)
  - thiserror (errores)
  - axum (conversiones HTTP)

- **escuela_shared** NO depende de:
  - escuela_core
  - escuela_storage
  - escuela_api
  - Otros crates del workspace

Esto evita dependencias circulares y permite que escuela_shared sea la base del workspace.

## Estructura del Módulo

```
escuela_shared/
├── src/
│   ├── lib.rs              # Punto de entrada del crate
│   ├── error.rs            # Manejo de errores (AppError, AppResult)
│   └── validation.rs       # Funciones de validación
└── Cargo.toml              # Dependencias del crate
```

## Tipos Valor

### Email

Tipo valor que representa un email validado con garantía de formato correcto.

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Email(String);
```

#### Creación

```rust
use escuela_shared::Email;

// Crear email validado
let email = Email::new("usuario@ejemplo.com".to_string())?;

// Error si el formato es inválido
let email = Email::new("email_invalido".to_string())?; // Error
```

#### Acceso

```rust
// Obtener el email como string
let email_str = email.as_str();
println!("Email: {}", email_str);
```

#### Reglas de Validación

- Debe contener exactamente un símbolo '@'
- La parte local (antes del @) no puede estar vacía
- El dominio (después del @) no puede estar vacío
- El dominio debe contener al menos un punto
- El dominio no puede empezar ni terminar con punto
- No puede contener espacios

#### Ejemplos Válidos

```
usuario@ejemplo.com
nombre.apellido@dominio.edu
test+label@gmail.com
user@sub.domain.com
```

#### Ejemplos Inválidos

```
email_invalido           (Sin @)
@ejemplo.com            (Sin parte local)
usuario@                (Sin dominio)
usuario@dominio         (Sin punto en dominio)
usuario@.com            (Dominio empieza con punto)
usuario@dominio.        (Dominio termina con punto)
usu ario@ejemplo.com    (Contiene espacio)
```

### Cédula

Tipo valor que representa una cédula de identidad validada con formato venezolano.

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Cedula(String);
```

#### Creación

```rust
use escuela_shared::Cedula;

// Crear cédula con formato venezolano
let cedula = Cedula::new("V-12345678".to_string())?;
let cedula = Cedula::new("E-98765432".to_string())?;
let cedula = Cedula::new("J-123456789".to_string())?;
let cedula = Cedula::new("G-12345678".to_string())?;

// Crear cédula solo con dígitos
let cedula = Cedula::new("12345678".to_string())?;

// Error si el formato es inválido
let cedula = Cedula::new("ABC".to_string())?; // Error
```

#### Acceso

```rust
// Obtener la cédula como string
let cedula_str = cedula.as_str();
println!("Cédula: {}", cedula_str);
```

#### Reglas de Validación

- Longitud entre 5 y 20 caracteres
- Acepta prefijos venezolanos: V-, E-, J-, G-
- La parte numérica debe contener solo dígitos
- No puede estar vacía
- Normaliza a mayúsculas automáticamente

#### Formatos Aceptados

| Prefijo | Descripción | Ejemplo |
|---------|-------------|---------|
| V- | Venezolano (V) | V-12345678 |
| E- | Extranjero (E) | E-98765432 |
| J- | Jurídico (J) | J-123456789 |
| G- | Gobierno (G) | G-12345678 |
| (ninguno) | Solo dígitos | 12345678 |

#### Ejemplos Válidos

```
V-12345678
E-98765432
J-123456789
G-12345678
12345678
v-12345678 (se normaliza a V-12345678)
```

#### Ejemplos Inválidos

```
ABC              (Muy corto)
12345678901234567890 (Muy largo)
V-ABC12345       (Contiene letras en parte numérica)
V-123-45678      (Contiene guiones extra)
V- 12345678      (Contiene espacio)
```

## Manejo de Errores

### AppError

Enum de errores personalizado que cubre todos los tipos de errores del sistema.

```rust
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Error de validación: {0}")]
    ValidationError(String),

    #[error("Error de base de datos: {0}")]
    DatabaseError(String),

    #[error("Error de serialización: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Error de IO: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Recurso no encontrado: {0}")]
    NotFound(String),

    #[error("Error de autenticación: {0}")]
    AuthenticationError(String),

    #[error("Error de autorización: {0}")]
    AuthorizationError(String),

    #[error("Error interno del servidor: {0}")]
    InternalError(String),
}
```

### Tipos de Errores

| Tipo de Error | Descripción | Código HTTP |
|---------------|-------------|-------------|
| `ValidationError` | Error de validación de datos de entrada | 400 BAD_REQUEST |
| `DatabaseError` | Error en operaciones de base de datos | 500 INTERNAL_SERVER_ERROR |
| `SerializationError` | Error en serialización/deserialización JSON | 500 INTERNAL_SERVER_ERROR |
| `IoError` | Error de entrada/salida (archivos, red) | 500 INTERNAL_SERVER_ERROR |
| `NotFound` | Recurso solicitado no encontrado | 404 NOT_FOUND |
| `AuthenticationError` | Error en autenticación de usuario | 401 UNAUTHORIZED |
| `AuthorizationError` | Error en autorización (permisos insuficientes) | 403 FORBIDDEN |
| `InternalError` | Error interno del servidor (errores inesperados) | 500 INTERNAL_SERVER_ERROR |

### AppResult

Alias de tipo para resultados que usa AppError como tipo de error.

```rust
pub type AppResult<T> = Result<T, AppError>;
```

#### Uso

```rust
use escuela_shared::{AppResult, AppError};

fn crear_usuario(nombre: String) -> AppResult<Usuario> {
    if nombre.is_empty() {
        return Err(AppError::ValidationError("El nombre no puede estar vacío".to_string()));
    }
    // ... lógica de creación
    Ok(usuario)
}
```

### Conversión a HTTP

`AppError` implementa `IntoResponse` de Axum para conversión automática a respuestas HTTP.

```rust
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::AuthenticationError(msg) => (StatusCode::UNAUTHORIZED, msg.clone()),
            AppError::AuthorizationError(msg) => (StatusCode::FORBIDDEN, msg.clone()),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Error interno del servidor".to_string()),
        };

        let body = serde_json::json!({
            "error": message,
            "status": status.as_u16()
        });

        (status, Json(body)).into_response()
    }
}
```

#### Response Format

```json
{
  "error": "Mensaje de error descriptivo",
  "status": 400
}
```

### Ejemplos de Uso

#### Error de Validación

```rust
return Err(AppError::ValidationError("Email inválido".to_string()));
// Response: 400 BAD_REQUEST
// Body: {"error": "Email inválido", "status": 400}
```

#### Error de No Encontrado

```rust
return Err(AppError::NotFound("Usuario no encontrado".to_string()));
// Response: 404 NOT_FOUND
// Body: {"error": "Usuario no encontrado", "status": 404}
```

#### Error de Autenticación

```rust
return Err(AppError::AuthenticationError("Credenciales incorrectas".to_string()));
// Response: 401 UNAUTHORIZED
// Body: {"error": "Credenciales incorrectas", "status": 401}
```

#### Error de Autorización

```rust
return Err(AppError::AuthorizationError("Permisos insuficientes".to_string()));
// Response: 403 FORBIDDEN
// Body: {"error": "Permisos insuficientes", "status": 403}
```

#### Error de Base de Datos

```rust
return Err(AppError::DatabaseError("Error al conectar a la base de datos".to_string()));
// Response: 500 INTERNAL_SERVER_ERROR
// Body: {"error": "Error interno del servidor", "status": 500}
```

## Validación

### validate_email

Función que valida el formato de un email.

```rust
pub fn validate_email(email: &str) -> Result<(), crate::AppError>
```

#### Reglas de Validación

- Debe contener exactamente un símbolo '@'
- La parte local y el dominio no pueden estar vacíos
- El dominio debe contener al menos un punto
- El dominio no puede empezar ni terminar con punto
- No puede contener espacios

#### Uso

```rust
use escuela_shared::validate_email;

// Validar email
validate_email("usuario@ejemplo.com")?; // Ok
validate_email("email_invalido")?; // Error
```

### Implementación Interna

```rust
pub fn validate_email(email: &str) -> Result<(), crate::AppError> {
    let trimmed = email.trim();
    if trimmed.is_empty() {
        return Err(crate::AppError::ValidationError("Email inválido".to_string()));
    }
    let parts: Vec<&str> = trimmed.splitn(2, '@').collect();
    if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
        return Err(crate::AppError::ValidationError("Email inválido".to_string()));
    }
    let domain = parts[1];
    if !domain.contains('.') || domain.starts_with('.') || domain.ends_with('.') {
        return Err(crate::AppError::ValidationError("Email inválido".to_string()));
    }
    if trimmed.contains(char::is_whitespace) {
        return Err(crate::AppError::ValidationError("Email inválido".to_string()));
    }
    Ok(())
}
```

## Dependencias

### Dependencias del Workspace

```toml
[dependencies]
serde = { workspace = true }           # Serialización/deserialización
serde_json = { workspace = true }      # JSON
chrono = { workspace = true }         # Fechas y tiempos
uuid = { workspace = true }           # Identificadores únicos
thiserror = { workspace = true }       # Derivación de errores
axum = { workspace = true }            # Conversiones HTTP
```

### Por qué estas dependencias

- **serde**: Necesario para serialización/deserialización de tipos valor
- **serde_json**: Para manejo de JSON en errores
- **chrono**: Para tipos de fecha en errores (opcional)
- **uuid**: Para tipos de UUID en errores (opcional)
- **thiserror**: Para derivación automática de implementaciones de Error
- **axum**: Para conversión de errores a respuestas HTTP

## Ejemplos de Uso

### Crear y Usar Email

```rust
use escuela_shared::Email;

// Crear email validado
let email = Email::new("juan.perez@escuela.edu".to_string())?;

// Usar en estructuras
struct Usuario {
    email: Email,
    nombre: String,
}

let usuario = Usuario {
    email: email.clone(),
    nombre: "Juan Pérez".to_string(),
};

// Serializar a JSON
let json = serde_json::to_string(&email)?;
println!("{}", json); // "juan.perez@escuela.edu"

// Deserializar desde JSON
let email_deserializado: Email = serde_json::from_str("\"juan.perez@escuela.edu\"")?;
```

### Crear y Usar Cédula

```rust
use escuela_shared::Cedula;

// Crear cédula validada
let cedula = Cedula::new("V-12345678".to_string())?;

// Usar en estructuras
struct Expediente {
    cedula: Cedula,
    nombres: String,
}

let expediente = Expediente {
    cedula: cedula.clone(),
    nombres: "Juan Pérez".to_string(),
};

// Serializar a JSON
let json = serde_json::to_string(&cedula)?;
println!("{}", json); // "V-12345678"

// Deserializar desde JSON
let cedula_deserializada: Cedula = serde_json::from_str("\"V-12345678\"")?;
```

### Manejo de Errores

```rust
use escuela_shared::{AppResult, AppError};

fn procesar_usuario(nombre: String, email: String) -> AppResult<String> {
    // Validar nombre
    if nombre.is_empty() {
        return Err(AppError::ValidationError("El nombre no puede estar vacío".to_string()));
    }

    // Validar email
    let email_validado = Email::new(email)?;

    // Procesar...
    Ok(format!("Usuario {} creado con email {}", nombre, email_validado.as_str()))
}

// Usar la función
match procesar_usuario("Juan".to_string(), "juan@email.com".to_string()) {
    Ok(mensaje) => println!("{}", mensaje),
    Err(error) => eprintln!("Error: {}", error),
}
```

### Propagación de Errores con `?`

```rust
use escuela_shared::{AppResult, AppError, Email, Cedula};

fn crear_expediente(
    nombres: String,
    apellidos: String,
    email: String,
    cedula: String,
) -> AppResult<Expediente> {
    // Validar email (propaga error si es inválido)
    let email_validado = Email::new(email)?;

    // Validar cédula (propaga error si es inválida)
    let cedula_validada = Cedula::new(cedula)?;

    // Crear expediente
    Ok(Expediente {
        nombres,
        apellidos,
        email: email_validado,
        cedula: cedula_validada,
    })
}
```

### Conversión Automática a HTTP

```rust
use escuela_shared::AppError;
use axum::response::IntoResponse;

// En un handler de Axum
pub async fn obtener_usuario() -> impl IntoResponse {
    // Si retorna AppError, se convierte automáticamente a respuesta HTTP
    Err(AppError::NotFound("Usuario no encontrado".to_string()))
    // Response: 404 NOT_FOUND
    // Body: {"error": "Usuario no encontrado", "status": 404}
}
```

### Validación Personalizada

```rust
use escuela_shared::{AppResult, AppError, validate_email};

fn validar_datos_usuario(
    nombre: &str,
    email: &str,
) -> AppResult<()> {
    // Validar nombre
    if nombre.len() < 3 {
        return Err(AppError::ValidationError("El nombre debe tener al menos 3 caracteres".to_string()));
    }

    // Validar email usando función compartida
    validate_email(email)?;

    Ok(())
}
```

## Testing

### Ejecutar Tests

```bash
# Ejecutar todos los tests del crate
cargo test -p escuela_shared

# Ejecutar tests con salida detallada
cargo test -p escuela_shared -- --nocapture

# Ejecutar tests de un módulo específico
cargo test -p escuela_shared email
cargo test -p escuela_shared cedula
cargo test -p escuela_shared validation
```

### Tests de Email

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_valido() {
        let email = Email::new("usuario@ejemplo.com".to_string()).unwrap();
        assert_eq!(email.as_str(), "usuario@ejemplo.com");
    }

    #[test]
    fn test_email_invalido_sin_arroba() {
        let result = Email::new("email_invalido".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_email_invalido_sin_dominio() {
        let result = Email::new("usuario@".to_string());
        assert!(result.is_err());
    }
}
```

### Tests de Cédula

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cedula_valida_con_prefijo() {
        let cedula = Cedula::new("V-12345678".to_string()).unwrap();
        assert_eq!(cedula.as_str(), "V-12345678");
    }

    #[test]
    fn test_cedula_valida_sin_prefijo() {
        let cedula = Cedula::new("12345678".to_string()).unwrap();
        assert_eq!(cedula.as_str(), "12345678");
    }

    #[test]
    fn test_cedula_normaliza_mayusculas() {
        let cedula = Cedula::new("v-12345678".to_string()).unwrap();
        assert_eq!(cedula.as_str(), "V-12345678");
    }

    #[test]
    fn test_cedula_invalida_muy_corta() {
        let result = Cedula::new("123".to_string());
        assert!(result.is_err());
    }
}
```

### Tests de Validación

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_email_valido() {
        assert!(validate_email("usuario@ejemplo.com").is_ok());
    }

    #[test]
    fn test_validate_email_invalido() {
        assert!(validate_email("email_invalido").is_err());
    }
}
```

## Consideraciones de Diseño

### Tipos Valor vs Strings

**Usar Tipos Valor (Email, Cédula):**
- ✅ Validación en tiempo de construcción
- ✅ Garantía de formato correcto
- ✅ Type safety en tiempo de compilación
- ✅ Self-documenting code
- ✅ Previene errores de tipo

**Usar Strings:**
- ✅ Para datos que no requieren validación
- ✅ Para datos temporales
- ✅ Para datos con formato variable

### Inmutabilidad

Los tipos valor son inmutables por diseño:

```rust
// ✅ Correcto: Crear nuevo valor
let email2 = Email::new("nuevo@email.com".to_string())?;

// ❌ Incorrecto: No se puede modificar
let email = Email::new("viejo@email.com".to_string())?;
email.0 = "nuevo@email.com".to_string(); // Error de compilación
```

### Validación en Tiempo de Construcción

La validación ocurre al crear el tipo, no al usarlo:

```rust
// Validación ocurre aquí
let email = Email::new("usuario@ejemplo.com".to_string())?;

// Uso garantizado válido
println!("{}", email.as_str()); // Siempre válido
```

### Serialización

Los tipos valor implementan `Serialize` y `Deserialize`:

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct Email(String);

// Serializa como string
let json = serde_json::to_string(&email)?; // "usuario@ejemplo.com"

// Deserializa desde string
let email: Email = serde_json::from_str("\"usuario@ejemplo.com\"")?;
```

### Hash y Eq

Los tipos valor implementan `Hash` y `Eq` para usar en colecciones:

```rust
use std::collections::HashSet;

let mut emails = HashSet::new();
emails.insert(Email::new("usuario1@ejemplo.com".to_string())?);
emails.insert(Email::new("usuario2@ejemplo.com".to_string())?);
```

## Próximas Mejoras

- [ ] Agregar más tipos valor (Teléfono, URL, etc.)
- [ ] Implementar validación más avanzada de email (RFC 5322)
- [ ] Agregar validación de cédula con dígito verificador
- [ ] Implementar traits de conversión adicionales
- [ ] Agregar soporte para internacionalización de errores
- [ ] Implementar logging de errores estructurado
- [ ] Agregar métricas de errores
- [ ] Implementar tracing de errores

## Licencia

Este crate está dual-licenciado bajo MIT License y Apache License 2.0, al igual que el proyecto principal.

## Contribuciones

Para contribuir a este crate, por favor:
1. Mantén los tipos valor inmutables
2. Agrega tests para nuevas validaciones
3. Documenta las reglas de validación
4. Mantén la consistencia con tipos existentes
5. Revisa las guías en [CONTRIBUTING.md](../../CONTRIBUTING.md)
