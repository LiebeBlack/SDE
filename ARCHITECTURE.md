# Architecture Documentation

## Overview

The Sistema de Gestión Escolar follows **Clean Architecture** principles with a modular, multicrate workspace design. This architecture ensures separation of concerns, testability, and maintainability.

## Architecture Layers

### 1. Domain Layer (escuela_core)

**Purpose**: Contains pure business logic and domain entities.

**Components**:
- **Entities**: Core business objects with strong typing
  - `Usuario`: User with RBAC roles
  - `Documento`: Document with hash and metadata
  - `ExpedienteDocente`: Complete teacher dossier
- **Value Objects**: Immutable types with validation
  - `Email`: Validated email address
  - `Cedula`: Validated ID number
  - `UsuarioId`, `DocumentoId`, etc.: Type-safe IDs
- **Service Traits**: Interfaces for infrastructure layer
  - `UsuarioService`: User operations contract
  - `ExpedienteService`: Expediente operations contract
  - `DocumentoService`: Document operations contract
  - `AuditService`: Audit logging contract

**Dependencies**: Only `escuela_shared` (no infrastructure dependencies)

**Key Principles**:
- Pure functions without side effects
- No external dependencies
- Testable in isolation
- Business rules enforcement

### 2. Infrastructure Layer (escuela_storage)

**Purpose**: Implements persistence and data access.

**Components**:
- **Database**: SQLite initialization and migrations
- **Repositories**: Data access implementations
  - `UsuarioRepository`: User CRUD operations
  - `ExpedienteRepository`: Expediente CRUD operations
  - `DocumentoRepository`: Document CRUD operations
- **Mappers**: Convert between domain entities and database rows

**Dependencies**: `escuela_core`, `escuela_shared`, `sqlx`

**Key Principles**:
- Implements service traits from core
- Handles all database operations
- Manages transactions
- Provides data persistence

### 3. Application Layer (escuela_api)

**Purpose**: HTTP API and request handling.

**Components**:
- **Server**: Axum web server configuration
- **Routes**: API endpoint definitions
- **Handlers**: HTTP request/response handlers
  - `auth_handler`: Authentication and authorization
  - `usuario_handler`: User management
  - `expediente_handler`: Expediente management
  - `documento_handler`: Document management
- **State**: Shared application state (database pools, services)
- **Middleware**: JWT auth, CORS, logging

**Dependencies**: `escuela_core`, `escuela_storage`, `escuela_shared`, `axum`

**Key Principles**:
- Thin handlers (delegate to services)
- HTTP-specific concerns only
- No business logic
- Proper error handling and responses

### 4. Shared Layer (escuela_shared)

**Purpose**: Common types and utilities across all layers.

**Components**:
- **Error Types**: Unified error handling
  - `AppError`: Application error enum
  - HTTP error conversions
- **Validation**: Reusable validation functions
- **Types**: Common value types
  - `Email`, `Cedula`, etc.

**Dependencies**: External crates only (serde, validator, thiserror)

**Key Principles**:
- No circular dependencies
- Pure utility functions
- Type-safe conversions

## Data Flow

### Request Flow

```
HTTP Request
    ↓
Router (Axum)
    ↓
Middleware (Auth, CORS, Logging)
    ↓
Handler
    ↓
Service (Repository)
    ↓
Database (SQLite)
    ↓
Response
```

### Authentication Flow

```
Login Request
    ↓
auth_handler::login
    ↓
Validate credentials
    ↓
Generate JWT token
    ↓
Return token + user info
    ↓
Client stores token
    ↓
Subsequent requests include token
    ↓
Middleware validates token
    ↓
Request proceeds to handler
```

### Document Upload Flow

```
Multipart Form Upload
    ↓
documento_handler::upload
    ↓
Validate file (size, type)
    ↓
Calculate SHA-256 hash
    ↓
Store file in storage/
    ↓
Create document record in DB
    ↓
Return document info
```

## Security Architecture

### Authentication
- JWT tokens with expiration
- Secure password hashing (Argon2)
- Rate limiting on login attempts
- Session management

### Authorization
- RBAC (Role-Based Access Control)
- Roles: Super, Director, RRHH, Admin
- Permission checks on all protected endpoints
- Audit logging for all actions

### Data Security
- SHA-256 hashing for all documents
- Immutable file storage
- SQL injection prevention (parameterized queries)
- Input validation on all endpoints

## Database Schema

### Tables

**usuarios**
- id (PK)
- nombre, apellido
- email (unique)
- cedula (unique)
- password_hash
- rol
- activo
- timestamps

**expedientes**
- id (PK)
- nombres, apellidos
- cedula (unique)
- email
- telefono
- direccion
- nacionalidad
- estado
- timestamps

**documentos**
- id (PK)
- expediente_id (FK)
- nombre_archivo
- categoria
- hash (SHA-256)
- ruta_local
- foliado
- timestamps

**auditoria**
- id (PK)
- usuario_id (FK, nullable)
- accion
- detalle
- ip_address
- user_agent
- timestamp

## Frontend Architecture

### Technologies
- Vanilla JavaScript (ES6+)
- HTML5 with semantic markup
- CSS3 with custom properties
- Lucide Icons (inline SVG)
- Service Worker for offline support

### Structure
```
static/
├── index.html          # Main application
├── admin.html          # Admin panel
├── css/
│   └── style.css       # Main stylesheet
├── js/
│   ├── main.js         # Application entry point
│   ├── core/
│   │   ├── api.js      # API client
│   │   ├── sync-manager.js  # Offline sync
│   │   └── offline-storage.js  # Local storage
│   ├── modules/
│   │   ├── expedientes.js     # Expediente management
│   │   ├── admin.js          # Admin functionality
│   │   └── documentos.js     # Document management
│   └── ui/
│       └── components.js     # UI components
└── sw.js              # Service worker
```

### Key Patterns
- **Module Pattern**: Encapsulation of related functionality
- **Event-Driven**: DOM event listeners for interactions
- **Offline-First**: Local storage with background sync
- **Progressive Enhancement**: Works without service worker

## Performance Considerations

### Backend
- Async/await throughout the stack
- Connection pooling for database
- Efficient SQL queries with indexes
- LTO compilation for optimization
- Binary size optimization

### Frontend
- Debounced search input
- Lazy loading of resources
- Efficient DOM manipulation
- Local storage caching
- Service worker caching

## Scalability Considerations

### Current Limitations
- SQLite (single-file database)
- Single server deployment
- In-memory rate limiting

### Future Enhancements
- PostgreSQL migration option
- Horizontal scaling support
- Redis for distributed caching
- Load balancing support
- Microservices architecture option

## Testing Strategy

### Unit Tests
- Domain entity validation
- Service trait implementations
- Utility functions
- Error handling

### Integration Tests
- API endpoint testing
- Database operations
- Authentication flows
- File upload/download

### E2E Tests
- Complete user workflows
- Multi-user scenarios
- Offline functionality
- Error recovery

## Deployment Architecture

### Development
- Local SQLite database
- Local file storage
- Development server (cargo run)

### Production
- Optimized binary compilation
- Environment variable configuration
- System service integration
- Backup automation
- Monitoring setup

## Monitoring and Logging

### Logging
- Structured logging with tracing
- Request/response logging
- Error tracking
- Audit trail

### Metrics
- Request timing
- Error rates
- User activity
- Resource usage

## Future Architecture Improvements

### Planned Enhancements
- Event sourcing for audit trail
- CQRS pattern for complex operations
- Message queue for async operations
- GraphQL API option
- Mobile app integration
- Cloud storage integration option

---

This architecture provides a solid foundation for the Sistema de Gestión Escolar while maintaining flexibility for future enhancements.
