# Contributing to Sistema de Gestión Escolar

¡Gracias por tu interés en contribuir al Sistema de Gestión Escolar! Este proyecto es un sistema de gestión documental para instituciones educativas, desarrollado con Rust y Clean Architecture.

## 🤝 Cómo Contribuir

### Reportar Bugs

Antes de reportar un bug, por favor:

1. **Busca en issues existentes** para ver si el problema ya ha sido reportado
2. **Verifica que estás usando la última versión** del proyecto
3. **Proporciona información detallada**:
   - Versión de Rust (`rustc --version`)
   - Sistema operativo
   - Pasos para reproducir el problema
   - Comportamiento esperado vs comportamiento actual
   - Logs o mensajes de error relevantes

### Sugerir Mejoras

Para sugerir nuevas características o mejoras:

1. **Abre un issue** con la etiqueta `enhancement`
2. **Describe claramente** la mejora propuesta
3. **Explica el caso de uso** y por qué sería útil
4. **Considera la arquitectura** del proyecto (Clean Architecture)

### Enviar Pull Requests

#### Pasos para Contribuir

1. **Fork el repositorio**
2. **Crea una rama** para tu feature o fix:
   ```bash
   git checkout -b feature/tu-feature-o-fix
   ```
3. **Haz tus cambios** siguiendo las guías de estilo
4. **Ejecuta los tests**:
   ```bash
   cargo test
   cargo clippy
   cargo fmt
   ```
5. **Commit tus cambios** con mensajes claros:
   ```bash
   git commit -m "feat: agregar nueva característica X"
   # o
   git commit -m "fix: corregir error Y"
   ```
6. **Push a tu rama**:
   ```bash
   git push origin feature/tu-feature-o-fix
   ```
7. **Abre un Pull Request** con una descripción clara

#### Guías de Estilo

- **Rust**: Sigue las [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- **Formato**: Usa `cargo fmt` para formatear el código
- **Lints**: Usa `cargo clippy` para verificar el código
- **Documentación**: Documenta todas las funciones públicas con `///`
- **Tests**: Escribe tests para nuevas funcionalidades

#### Convenciones de Commits

Usa el formato de [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` Nueva característica
- `fix:` Corrección de bug
- `docs:` Cambios en documentación
- `style:` Cambios de formato (sin lógica)
- `refactor:` Refactorización (sin cambios funcionales)
- `test:` Agregar o modificar tests
- `chore:` Cambios en build, herramientas, etc.

Ejemplos:
```
feat: agregar exportación de expedientes a PDF
fix: corregir error de autenticación JWT
docs: actualizar README con nuevas instrucciones
```

## 🏗️ Arquitectura del Proyecto

El proyecto sigue **Clean Architecture** con separación clara de responsabilidades:

- **escuela_core**: Lógica de negocio pura y entidades del dominio
- **escuela_storage**: Capa de persistencia (SQLite)
- **escuela_api**: Capa de infraestructura HTTP (Axum)
- **escuela_shared**: Tipos y utilidades compartidas

Al contribuir, mantén esta separación y respeta los límites entre capas.

## 🧪 Testing

### Ejecutar Tests

```bash
# Todos los tests
cargo test

# Tests con salida detallada
cargo test -- --nocapture

# Tests de un crate específico
cargo test -p escuela_core

# Tests de un módulo específico
cargo test nombre_del_test
```

### Escribir Tests

- Escribe tests unitarios para funciones lógicas
- Escribe tests de integración para endpoints API
- Usa `#[cfg(test)]` para módulos de prueba
- Nombra los tests descriptivamente

## 📝 Código de Conducta

- Sé respetuoso y constructivo
- Acepta críticas constructivas
- Enfócate en lo que es mejor para la comunidad
- Muestra empatía hacia otros contribuidores

## 📧 Contacto

Para preguntas sobre contribuciones, abre un issue o contacta a los mantenedores del proyecto.

---

¡Gracias por contribuir! 🎉
