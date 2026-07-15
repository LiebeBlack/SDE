# Frontend - Interfaz Web del Sistema de Gestión Escolar

![HTML5](https://img.shields.io/badge/HTML5-E34F26.svg)
![CSS3](https://img.shields.io/badge/CSS3-1572B6.svg)
![JavaScript](https://img.shields.io/badge/JavaScript-F7DF1E.svg)
![PWA](https://img.shields.io/badge/PWA-Ready-green.svg)
![Offline](https://img.shields.io/badge/Offline--First-blue.svg)

> Interfaz web moderna, responsiva y offline-first del Sistema de Gestión Escolar, desarrollada con Vanilla JavaScript, CSS3 con Glassmorphism, y Progressive Web App (PWA) capabilities.

## 📋 Tabla de Contenidos

- [Visión General](#visión-general)
- [Características](#características)
- [Arquitectura](#arquitectura)
- [Estructura de Archivos](#estructura-de-archivos)
- [Módulos Core](#módulos-core)
- [Módulos de Funcionalidad](#módulos-de-funcionalidad)
- [Componentes UI](#componentes-ui)
- [Estilos y Diseño](#estilos-y-diseño)
- [Service Worker](#service-worker)
- [PWA Manifest](#pwa-manifest)
- [API Integration](#api-integration)
- [Offline Support](#offline-support)
- [Ejemplos de Uso](#ejemplos-de-uso)

## Visión General

El frontend es una Single Page Application (SPA) moderna que proporciona una interfaz intuitiva para la gestión documental de instituciones educativas. Funciona completamente offline-first con soporte PWA, lo que permite trabajar sin conexión a internet y sincronizar cuando se restablezca la conexión.

### Características Principales

- **Offline-First**: Funciona completamente sin conexión a internet
- **PWA Ready**: Instalable como aplicación nativa
- **Responsive**: Diseño adaptativo para desktop, tablet y móvil
- **Glassmorphism**: Diseño moderno con efectos de vidrio esmerilado
- **Vanilla JavaScript**: Sin frameworks, máximo rendimiento
- **Service Worker**: Caching inteligente de recursos
- **Drag & Drop**: Upload de archivos intuitivo
- **Búsqueda en Tiempo Real**: Debouncing y filtrado instantáneo
- **Atajos de Teclado**: Ctrl+K (buscar), Ctrl+N (nuevo), Escape (cerrar)
- **Notificaciones Toast**: Feedback visual no intrusivo
- **Modo Oscuro**: Soporte para temas claro/oscuro

## Características

### Interfaz Principal (index.html)

- **Dashboard Interactivo**: Estadísticas en tiempo real del sistema
- **Gestión de Expedientes**: CRUD completo con búsqueda avanzada
- **Gestión de Documentos**: Upload de archivos con drag-and-drop
- **Búsqueda Global**: Búsqueda instantánea con debouncing
- **Navegación Fluida**: Transiciones suaves entre vistas

### Panel de Administración (admin.html)

- **Backup Automático**: Copias de seguridad programadas
- **Reportes PDF**: Exportación de datos a PDF
- **Gestión de Usuarios**: Control de accesos y roles
- **Auditoría Completa**: Registro de todas las acciones
- **Mantenimiento**: Limpieza de caché y actualización de datos
- **Listado Personal**: Exportación CSV de personal
- **Backup Completo**: Descarga ZIP de todo el sistema

### Características Técnicas

- **Module Pattern**: Encapsulación de funcionalidad
- **Event-Driven**: Arquitectura basada en eventos
- **Local Storage**: Persistencia de datos offline
- **Service Worker**: Caching de recursos y sincronización
- **Progressive Enhancement**: Funciona sin Service Worker
- **Accessibility**: WCAG 2.1 AA compliant

## Arquitectura

### Arquitectura Modular

```
static/
├── index.html              # Aplicación principal
├── admin.html              # Panel de administración
├── css/
│   └── style.css           # Estilos principales
├── js/
│   ├── main.js             # Punto de entrada
│   ├── lucide.min.js       # Iconos (inline)
│   ├── core/               # Módulos core
│   │   ├── api.js          # Cliente API
│   │   ├── sync-manager.js # Gestión de sincronización
│   │   └── offline-storage.js # Almacenamiento offline
│   ├── modules/            # Módulos de funcionalidad
│   │   ├── expedientes.js  # Gestión de expedientes
│   │   ├── documentos.js   # Gestión de documentos
│   │   └── admin.js        # Funcionalidades admin
│   └── ui/                 # Componentes UI
│       └── components.js    # Componentes reutilizables
├── manifest.json           # PWA manifest
└── sw.js                   # Service Worker
```

### Flujo de Datos

```
Usuario Interactúa
    ↓
Componentes UI (components.js)
    ↓
Módulos de Funcionalidad (modules/)
    ↓
Core API (api.js)
    ↓
Backend API (escuela_api)
    ↓
Base de Datos (SQLite)
```

### Offline Flow

```
Sin Conexión
    ↓
Offline Storage (localStorage)
    ↓
Sync Manager (sync-manager.js)
    ↓
Queue de Operaciones
    ↓
Conexión Restaurada
    ↓
Sincronización Automática
```

## Estructura de Archivos

### HTML

```
index.html          # Aplicación principal (279 líneas)
admin.html          # Panel de administración (≈900 líneas)
```

### CSS

```
css/style.css       # Estilos principales (≈400 líneas)
```

### JavaScript Core

```
js/lucide.min.js           # Iconos Lucide (408KB, inline)
js/main.js                 # Punto de entrada (≈450 líneas)
js/core/api.js              # Cliente API (≈400 líneas)
js/core/sync-manager.js    # Gestión de sincronización (≈160 líneas)
js/core/offline-storage.js # Almacenamiento offline (≈120 líneas)
```

### JavaScript Modules

```
js/modules/expedientes.js  # Gestión de expedientes (≈280 líneas)
js/modules/documentos.js   # Gestión de documentos (≈200 líneas)
js/modules/admin.js        # Funcionalidades admin (≈270 líneas)
```

### JavaScript UI

```
js/ui/components.js         # Componentes reutilizables (≈320 líneas)
```

### PWA

```
manifest.json       # PWA manifest (26 líneas)
sw.js              # Service Worker (≈150 líneas)
```

## Módulos Core

### api.js - Cliente API

Cliente HTTP para comunicación con el backend.

#### Funciones Principales

```javascript
// Configuración base
const API_BASE = '/api';

// Cliente HTTP con autenticación
async function apiRequest(endpoint, options = {}) {
    const token = localStorage.getItem('token');
    const headers = {
        'Content-Type': 'application/json',
        ...(token && { 'Authorization': `Bearer ${token}` })
    };
    
    const response = await fetch(`${API_BASE}${endpoint}`, {
        ...options,
        headers: { ...headers, ...options.headers }
    });
    
    if (!response.ok) {
        throw new Error(`HTTP ${response.status}`);
    }
    
    return response.json();
}

// Métodos HTTP
async function get(endpoint) { ... }
async function post(endpoint, data) { ... }
async function put(endpoint, data) { ... }
async function delete(endpoint) { ... }

// Upload de archivos
async function uploadFile(endpoint, formData) { ... }
```

#### Endpoints API

```javascript
// Autenticación
POST /login
POST /logout

// Expedientes
GET /expedientes
POST /expedientes
GET /expedientes/:id
GET /expedientes/cedula/:cedula
POST /expedientes/:id/estado
GET /expedientes/buscar/:termino

// Documentos
GET /expedientes/:id/documentos
POST /expedientes/:id/documentos
POST /expedientes/:id/documentos/:doc_id/foliar
GET /expedientes/:id/documentos/:doc_id/descargar

// Usuarios
GET /usuarios
POST /usuarios
POST /usuarios/:id/toggle

// Auditoría
GET /auditoria

// Búsqueda
GET /buscar/expedientes_avanzado
GET /buscar/documentos_avanzado
GET /buscar/general
```

### sync-manager.js - Gestión de Sincronización

Gestiona la sincronización de datos offline.

#### Funciones Principales

```javascript
// Queue de operaciones offline
const operationQueue = [];

// Agregar operación a la queue
function queueOperation(operation) {
    operationQueue.push(operation);
    saveQueue();
}

// Procesar queue cuando hay conexión
async function processQueue() {
    if (navigator.onLine) {
        for (const operation of operationQueue) {
            try {
                await executeOperation(operation);
                removeOperation(operation.id);
            } catch (error) {
                console.error('Error syncing operation:', error);
            }
        }
    }
}

// Detectar cambios de conexión
window.addEventListener('online', processQueue);
window.addEventListener('offline', () => {
    showToast('Modo offline activado', 'warning');
});
```

### offline-storage.js - Almacenamiento Offline

Persistencia de datos en localStorage.

#### Funciones Principales

```javascript
// Guardar datos
function saveData(key, data) {
    localStorage.setItem(key, JSON.stringify(data));
}

// Obtener datos
function getData(key) {
    const data = localStorage.getItem(key);
    return data ? JSON.parse(data) : null;
}

// Eliminar datos
function removeData(key) {
    localStorage.removeItem(key);
}

// Limpiar todos los datos
function clearAllData() {
    localStorage.clear();
}
```

## Módulos de Funcionalidad

### expedientes.js - Gestión de Expedientes

CRUD completo para expedientes docentes.

#### Funciones Principales

```javascript
// Listar expedientes
async function loadExpedientes() {
    const expedientes = await api.get('/expedientes');
    renderExpedientes(expedientes);
}

// Crear expediente
async function createExpediente(data) {
    const expediente = await api.post('/expedientes', data);
    showToast('Expediente creado exitosamente');
    return expediente;
}

// Obtener expediente por ID
async function getExpediente(id) {
    return await api.get(`/expedientes/${id}`);
}

// Actualizar expediente
async function updateExpediente(id, data) {
    return await api.put(`/expedientes/${id}`, data);
}

// Cambiar estado
async function changeEstado(id, estado) {
    return await api.post(`/expedientes/${id}/estado`, { estado });
}

// Búsqueda
async function searchExpedientes(termino) {
    return await api.get(`/expedientes/buscar/${termino}`);
}
```

### documentos.js - Gestión de Documentos

Upload y gestión de documentos.

#### Funciones Principales

```javascript
// Listar documentos de expediente
async function loadDocumentos(expedienteId) {
    return await api.get(`/expedientes/${expedienteId}/documentos`);
}

// Subir documento (multipart)
async function uploadDocumento(expedienteId, file, categoria) {
    const formData = new FormData();
    formData.append('archivo', file);
    formData.append('nombre_archivo', file.name);
    formData.append('categoria', categoria);
    
    return await api.uploadFile(
        `/expedientes/${expedienteId}/documentos`,
        formData
    );
}

// Foliar documento
async function foliarDocumento(expedienteId, documentoId) {
    return await api.post(
        `/expedientes/${expedienteId}/documentos/${documentoId}/foliar`
    );
}

// Descargar documento
function downloadDocumento(url, filename) {
    const link = document.createElement('a');
    link.href = url;
    link.download = filename;
    link.click();
}
```

### admin.js - Panel de Administración

Funcionalidades avanzadas para administradores.

#### Funciones Principales

```javascript
// Crear backup
async function createBackup() {
    const response = await api.post('/admin/backup');
    downloadFile(response.url, 'backup.zip');
}

// Listar usuarios
async function loadUsers() {
    return await api.get('/usuarios');
}

// Toggle estado de usuario
async function toggleUserEstado(userId) {
    return await api.post(`/usuarios/${userId}/toggle`);
}

// Obtener estadísticas
async function getStatistics() {
    return await api.get('/admin/statistics');
}

// Exportar CSV
async function exportCSV() {
    const data = await api.get('/admin/export/csv');
    downloadCSV(data, 'personal.csv');
}

// Limpiar caché
function clearCache() {
    if ('caches' in window) {
        caches.keys().then(names => {
            names.forEach(name => caches.delete(name));
        });
    }
    showToast('Caché limpiada exitosamente');
}
```

## Componentes UI

### components.js - Componentes Reutilizables

Componentes UI reutilizables para toda la aplicación.

#### Componentes Disponibles

```javascript
// Modal
function showModal(title, content, onConfirm) {
    const modal = document.createElement('div');
    modal.className = 'modal';
    modal.innerHTML = `
        <div class="modal-content">
            <h2>${title}</h2>
            ${content}
            <div class="modal-actions">
                <button class="btn-secondary" onclick="closeModal()">Cancelar</button>
                <button class="btn-primary" onclick="onConfirm()">Confirmar</button>
            </div>
        </div>
    `;
    document.body.appendChild(modal);
}

// Toast Notification
function showToast(message, type = 'success') {
    const toast = document.createElement('div');
    toast.className = `toast toast-${type}`;
    toast.textContent = message;
    document.body.appendChild(toast);
    
    setTimeout(() => {
        toast.classList.add('show');
    }, 10);
    
    setTimeout(() => {
        toast.classList.remove('show');
        setTimeout(() => toast.remove(), 300);
    }, 3000);
}

// Loading Spinner
function showLoading() {
    const spinner = document.createElement('div');
    spinner.className = 'loading-spinner';
    spinner.innerHTML = '<div class="spinner"></div>';
    document.body.appendChild(spinner);
}

function hideLoading() {
    const spinner = document.querySelector('.loading-spinner');
    if (spinner) spinner.remove();
}

// Confirm Dialog
function confirmDialog(message, onConfirm) {
    showModal('Confirmación', `
        <p>${message}</p>
    `, onConfirm);
}

// Form Validation
function validateForm(form) {
    const inputs = form.querySelectorAll('input[required]');
    let isValid = true;
    
    inputs.forEach(input => {
        if (!input.value.trim()) {
            input.classList.add('invalid');
            isValid = false;
        } else {
            input.classList.remove('invalid');
        }
    });
    
    return isValid;
}

// Debounce
function debounce(func, wait) {
    let timeout;
    return function executedFunction(...args) {
        const later = () => {
            clearTimeout(timeout);
            func(...args);
        };
        clearTimeout(timeout);
        timeout = setTimeout(later, wait);
    };
}
```

## Estilos y Diseño

### style.css - Estilos Principales

#### Variables CSS

```css
:root {
    /* Colores */
    --primary: #4f46e5;
    --primary-dark: #4338ca;
    --secondary: #64748b;
    --success: #10b981;
    --warning: #f59e0b;
    --danger: #ef4444;
    
    /* Glassmorphism */
    --glass-bg: rgba(255, 255, 255, 0.1);
    --glass-border: rgba(255, 255, 255, 0.2);
    --glass-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
    
    /* Tipografía */
    --font-family: 'Inter', system-ui, sans-serif;
    --font-size-base: 16px;
    
    /* Espaciado */
    --spacing-xs: 0.5rem;
    --spacing-sm: 1rem;
    --spacing-md: 1.5rem;
    --spacing-lg: 2rem;
    --spacing-xl: 3rem;
    
    /* Bordes */
    --border-radius-sm: 0.5rem;
    --border-radius-md: 0.75rem;
    --border-radius-lg: 1rem;
}
```

#### Glassmorphism Effect

```css
.glass {
    background: var(--glass-bg);
    backdrop-filter: blur(10px);
    -webkit-backdrop-filter: blur(10px);
    border: 1px solid var(--glass-border);
    box-shadow: var(--glass-shadow);
}

.card {
    @extend .glass;
    border-radius: var(--border-radius-lg);
    padding: var(--spacing-lg);
}
```

#### Responsive Design

```css
/* Mobile First */
.container {
    width: 100%;
    padding: var(--spacing-sm);
}

/* Tablet */
@media (min-width: 768px) {
    .container {
        max-width: 768px;
        margin: 0 auto;
    }
}

/* Desktop */
@media (min-width: 1024px) {
    .container {
        max-width: 1024px;
        margin: 0 auto;
    }
}

/* Large Desktop */
@media (min-width: 1280px) {
    .container {
        max-width: 1280px;
        margin: 0 auto;
    }
}
```

#### Animaciones

```css
@keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
}

@keyframes slideUp {
    from {
        opacity: 0;
        transform: translateY(20px);
    }
    to {
        opacity: 1;
        transform: translateY(0);
    }
}

.animate-fade-in {
    animation: fadeIn 0.3s ease-in-out;
}

.animate-slide-up {
    animation: slideUp 0.3s ease-out;
}
```

## Service Worker

### sw.js - Service Worker para PWA

Service Worker que habilita funcionalidades offline y caching.

#### Estrategia de Caching

```javascript
const CACHE_NAME = 'sge-v3';
const STATIC_CACHE = 'sge-static-v3';
const DYNAMIC_CACHE = 'sge-dynamic-v3';

// Archivos estáticos para cachear inicialmente
const STATIC_FILES = [
    '/',
    '/index.html',
    '/css/style.css',
    '/js/lucide.min.js',
    '/js/main.js',
    '/js/core/api.js',
    '/js/ui/components.js',
    '/js/modules/expedientes.js',
    '/js/modules/admin.js',
    '/js/modules/documentos.js',
    '/manifest.json'
];
```

#### Eventos del Service Worker

```javascript
// Instalación
self.addEventListener('install', (event) => {
    event.waitUntil(
        caches.open(STATIC_CACHE)
            .then(cache => cache.addAll(STATIC_FILES))
            .then(() => self.skipWaiting())
    );
});

// Activación
self.addEventListener('activate', (event) => {
    event.waitUntil(
        caches.keys()
            .then(cacheNames => {
                return Promise.all(
                    cacheNames
                        .filter(name => name !== STATIC_CACHE && name !== DYNAMIC_CACHE)
                        .map(name => caches.delete(name))
                );
            })
            .then(() => self.clients.claim())
    );
});

// Fetch (Network First con fallback a cache)
self.addEventListener('fetch', (event) => {
    event.respondWith(
        fetch(event.request)
            .then(response => {
                // Cache de respuesta dinámica
                if (response.status === 200) {
                    const clone = response.clone();
                    caches.open(DYNAMIC_CACHE)
                        .then(cache => cache.put(event.request, clone));
                }
                return response;
            })
            .catch(() => {
                // Fallback a cache
                return caches.match(event.request)
                    .then(response => response || caches.match('/'));
            })
    );
});
```

## PWA Manifest

### manifest.json - PWA Manifest

Configuración de la Progressive Web App.

```json
{
  "name": "SGE - Sistema de Gestión Escolar",
  "short_name": "SGE",
  "description": "Sistema de Gestión Escolar - Gestión documental integral",
  "start_url": "/",
  "display": "standalone",
  "background_color": "#0f172a",
  "theme_color": "#4f46e5",
  "orientation": "portrait-primary",
  "icons": [
    {
      "src": "data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'><text y='.9em' font-size='90'>🎓</text></svg>",
      "sizes": "192x192",
      "type": "image/svg+xml"
    },
    {
      "src": "data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 100 100'><text y='.9em' font-size='90'>🎓</text></svg>",
      "sizes": "512x512",
      "type": "image/svg+xml"
    }
  ],
  "categories": ["education", "productivity"],
  "lang": "es",
  "dir": "ltr"
}
```

### Características PWA

- **Instalable**: Puede instalarse como aplicación nativa
- **Offline**: Funciona sin conexión a internet
- **Responsive**: Se adapta a cualquier dispositivo
- **Iconos Personalizados**: Iconos SVG inline
- **Tema Oscuro**: Colores de tema personalizados
- **Orientation**: Optimizado para portrait

## API Integration

### Autenticación

```javascript
// Login
async function login(email, password) {
    const response = await api.post('/login', { email, password });
    localStorage.setItem('token', response.token);
    localStorage.setItem('usuario', JSON.stringify(response.usuario));
    return response;
}

// Logout
function logout() {
    localStorage.removeItem('token');
    localStorage.removeItem('usuario');
    window.location.href = '/login';
}

// Verificar autenticación
function isAuthenticated() {
    return !!localStorage.getItem('token');
}
```

### Manejo de Errores

```javascript
// Error handler global
window.addEventListener('unhandledrejection', (event) => {
    console.error('Unhandled promise rejection:', event.reason);
    showToast('Error inesperado', 'error');
});

// API error handler
async function apiRequest(endpoint, options = {}) {
    try {
        const response = await fetch(`${API_BASE}${endpoint}`, options);
        
        if (!response.ok) {
            if (response.status === 401) {
                logout();
                throw new Error('No autorizado');
            }
            throw new Error(`HTTP ${response.status}`);
        }
        
        return await response.json();
    } catch (error) {
        showToast(error.message, 'error');
        throw error;
    }
}
```

## Offline Support

### Detección de Conexión

```javascript
// Verificar estado de conexión
function isOnline() {
    return navigator.onLine;
}

// Eventos de conexión
window.addEventListener('online', () => {
    showToast('Conexión restaurada', 'success');
    syncManager.processQueue();
});

window.addEventListener('offline', () => {
    showToast('Modo offline activado', 'warning');
});
```

### Estrategia Offline-First

```javascript
// Intentar obtener datos del servidor
async function fetchData(endpoint) {
    if (navigator.onLine) {
        try {
            const data = await api.get(endpoint);
            offlineStorage.saveData(endpoint, data);
            return data;
        } catch (error) {
            console.error('Error fetching from server:', error);
        }
    }
    
    // Fallback a localStorage
    return offlineStorage.getData(endpoint);
}
```

### Sincronización

```javascript
// Queue de operaciones offline
const operationQueue = [];

// Agregar operación
function queueOperation(operation) {
    operationQueue.push({
        ...operation,
        id: Date.now(),
        timestamp: new Date().toISOString()
    });
    offlineStorage.saveData('operationQueue', operationQueue);
}

// Procesar queue
async function processQueue() {
    const queue = offlineStorage.getData('operationQueue') || [];
    
    for (const operation of queue) {
        try {
            await executeOperation(operation);
            removeOperation(operation.id);
        } catch (error) {
            console.error('Error syncing:', error);
        }
    }
}
```

## Ejemplos de Uso

### Inicialización

```javascript
// main.js
document.addEventListener('DOMContentLoaded', () => {
    // Inicializar iconos Lucide
    lucide.createIcons();
    
    // Verificar autenticación
    if (!isAuthenticated()) {
        window.location.href = '/login';
        return;
    }
    
    // Cargar datos iniciales
    loadDashboard();
    
    // Configurar event listeners
    setupEventListeners();
    
    // Registrar Service Worker
    if ('serviceWorker' in navigator) {
        navigator.serviceWorker.register('/sw.js')
            .then(reg => console.log('SW registrado:', reg))
            .catch(err => console.error('Error registrando SW:', err));
    }
});
```

### Crear Expediente

```javascript
async function handleCreateExpediente(event) {
    event.preventDefault();
    
    const form = event.target;
    if (!validateForm(form)) return;
    
    showLoading();
    
    try {
        const data = {
            nombres: form.nombres.value,
            apellidos: form.apellidos.value,
            cedula: form.cedula.value,
            email: form.email.value,
            telefono: form.telefono.value,
            direccion: form.direccion.value,
            nacionalidad: form.nacionalidad.value
        };
        
        const expediente = await createExpediente(data);
        hideLoading();
        showToast('Expediente creado exitosamente');
        form.reset();
        loadExpedientes();
    } catch (error) {
        hideLoading();
        showToast('Error al crear expediente', 'error');
    }
}
```

### Upload de Documento

```javascript
async function handleFileUpload(event, expedienteId) {
    const file = event.target.files[0];
    if (!file) return;
    
    showLoading();
    
    try {
        const categoria = document.getElementById('categoria').value;
        const documento = await uploadDocumento(expedienteId, file, categoria);
        
        hideLoading();
        showToast('Documento subido exitosamente');
        loadDocumentos(expedienteId);
    } catch (error) {
        hideLoading();
        showToast('Error al subir documento', 'error');
    }
}
```

### Búsqueda con Debounce

```javascript
const searchInput = document.getElementById('search');
const debouncedSearch = debounce(async (termino) => {
    if (termino.length < 3) {
        loadExpedientes();
        return;
    }
    
    showLoading();
    try {
        const resultados = await searchExpedientes(termino);
        renderExpedientes(resultados);
    } catch (error) {
        showToast('Error en búsqueda', 'error');
    } finally {
        hideLoading();
    }
}, 300);

searchInput.addEventListener('input', (e) => {
    debouncedSearch(e.target.value);
});
```

### Atajos de Teclado

```javascript
document.addEventListener('keydown', (e) => {
    // Ctrl+K: Búsqueda
    if (e.ctrlKey && e.key === 'k') {
        e.preventDefault();
        document.getElementById('search').focus();
    }
    
    // Ctrl+N: Nuevo expediente
    if (e.ctrlKey && e.key === 'n') {
        e.preventDefault();
        openModal('create-expediente');
    }
    
    // Escape: Cerrar modal
    if (e.key === 'Escape') {
        closeModal();
    }
});
```

## Consideraciones de Diseño

### Performance

- **Vanilla JS**: Sin overhead de frameworks
- **Lazy Loading**: Carga de módulos bajo demanda
- **Debouncing**: Reducción de requests innecesarias
- **Caching**: Service Worker para recursos estáticos
- **Minificación**: Archivos JS minificados en producción

### Accessibility

- **Semantic HTML**: Uso correcto de elementos semánticos
- **ARIA Labels**: Labels para screen readers
- **Keyboard Navigation**: Navegación completa por teclado
- **Focus Management**: Gestión de focus en modales
- **Color Contrast**: Ratio de contraste WCAG AA

### Security

- **XSS Prevention**: Sanitización de inputs
- **CSRF Protection**: Tokens en requests
- **Secure Storage**: Sensitive data en sessionStorage
- **HTTPS Only**: Comunicación segura
- **Content Security Policy**: Headers de seguridad

## Próximas Mejoras

- [ ] Implementar Web Workers para operaciones pesadas
- [ ] Agregar soporte para WebRTC (video conferencias)
- [ ] Implementar IndexedDB para mayor capacidad offline
- [ ] Agregar notificaciones push
- [ ] Implementar sincronización en tiempo real con WebSockets
- [ ] Agregar soporte para múltiples idiomas (i18n)
- [ ] Implementar tests E2E con Playwright
- [ ] Agregar analytics y tracking
- [ ] Implementar modo oscuro automático
- [ ] Agregar soporte para impresión de expedientes

## Licencia

Este frontend está dual-licenciado bajo MIT License y Apache License 2.0, al igual que el proyecto principal.

## Contribuciones

Para contribuir al frontend, por favor:
1. Mantén el código vanilla (sin frameworks)
2. Sigue las convenciones de nomenclatura
3. Agrega comentarios para código complejo
4. Mantén la consistencia de estilos
5. Revisa las guías en [CONTRIBUTING.md](../CONTRIBUTING.md)
