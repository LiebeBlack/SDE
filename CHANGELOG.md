# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- JWT authentication system with token validation
- Complete audit logging system
- Full-text search with SQLite FTS5
- Modern JavaScript web interface
- PDF export functionality for users and audit logs
- Automatic backup system
- Complete admin panel with advanced features
- Real-time dashboard statistics
- Intelligent search with debouncing
- Keyboard shortcuts for power users
- Real-time form validation with feedback
- Admin redirect logic for seamless admin access
- Cache clearing functionality
- Data refresh capabilities
- Personnel list export to CSV
- Full report generation in JSON format
- ZIP backup download functionality

### Changed
- Enhanced error handling in backend (replaced unwrap() with proper error handling)
- Improved accessibility with proper label for and autocomplete attributes
- Added SVG favicon to eliminate 404 errors
- Enhanced dashboard with additional statistics (inactive expedientes, user count)
- Improved form validation with visual feedback

### Fixed
- Fixed label for attributes to correctly reference input IDs
- Added missing autocomplete attributes to all form fields
- Added name attributes to all form elements
- Fixed password fields not contained in forms
- Fixed favicon.ico 404 error
- Fixed admin redirect logic for super admin credentials
- Fixed missing navigateToSection and showKeyboardShortcuts functions

### Security
- Implemented RBAC (Role-Based Access Control)
- Added rate limiting for login attempts
- Enhanced password validation
- Added IP address logging for audit trail
- Implemented secure JWT token handling

## [1.0.0] - 2024-06-21

### Added
- Initial release of Sistema de Gestión Escolar
- Clean Architecture implementation with multicrate workspace
- SQLite database with automatic migrations
- RESTful API with Axum framework
- User management with roles (Director, RRHH, Admin)
- Expediente management system
- Document upload and management
- File storage with SHA-256 hashing
- Offline-first functionality
- Multi-device LAN access
- Basic web interface
- Core domain entities (Usuario, Documento, ExpedienteDocente)
- Repository implementations for data persistence
- HTTP handlers for all CRUD operations
- CORS configuration
- Tracing and logging infrastructure
- Environment variable configuration
- Database integrity checks
- Automatic backup service

### Features
- User authentication and authorization
- Expediente creation and management
- Document upload with automatic MIME detection
- Document foliation system
- Search functionality
- State management for expedientes
- Audit logging for all operations
- Responsive web interface
- Offline storage capabilities
- Service worker for offline functionality

### Performance
- Optimized compilation with LTO
- Binary size optimization
- Efficient SQLite queries
- Async/await throughout the stack

### Documentation
- Comprehensive README with architecture overview
- API endpoint documentation
- Installation and deployment guides
- Troubleshooting section
- Code comments and documentation

---

## Version History Format

- **Added**: New features
- **Changed**: Changes in existing functionality
- **Deprecated**: Soon-to-be removed features
- **Removed**: Removed features
- **Fixed**: Bug fixes
- **Security**: Security vulnerabilities or improvements
