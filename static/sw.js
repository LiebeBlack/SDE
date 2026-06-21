// Service Worker para PWA Offline - Sistema de Gestión Escolar
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

// Instalación del Service Worker
self.addEventListener('install', (event) => {
    console.log('[SW] Instalando Service Worker...');
    event.waitUntil(
        caches.open(STATIC_CACHE)
            .then((cache) => {
                console.log('[SW] Caché abierta, agregando archivos estáticos');
                return cache.addAll(STATIC_FILES);
            })
            .then(() => {
                console.log('[SW] Instalación completada');
                return self.skipWaiting();
            })
            .catch((error) => {
                console.error('[SW] Error durante instalación:', error);
            })
    );
});

// Activación del Service Worker
self.addEventListener('activate', (event) => {
    console.log('[SW] Activando Service Worker...');
    event.waitUntil(
        caches.keys()
            .then((cacheNames) => {
                return Promise.all(
                    cacheNames.map((cacheName) => {
                        // Eliminar cachés antiguas
                        if (cacheName !== STATIC_CACHE && cacheName !== DYNAMIC_CACHE) {
                            console.log('[SW] Eliminando caché antigua:', cacheName);
                            return caches.delete(cacheName);
                        }
                    })
                );
            })
            .then(() => {
                console.log('[SW] Activación completada');
                return self.clients.claim();
            })
            .catch((error) => {
                console.error('[SW] Error durante activación:', error);
            })
    );
});

// Interceptación de peticiones de red
self.addEventListener('fetch', (event) => {
    const url = new URL(event.request.url);
    
    // Estrategia: Ignorar peticiones a la API para que api.js maneje su propio caché (OfflineStorage)
    if (url.pathname.startsWith('/api/')) {
        return;
    }

    // Cache First para archivos estáticos
        event.respondWith(
            caches.match(event.request)
                .then((cachedResponse) => {
                    if (cachedResponse) {
                        return cachedResponse;
                    }
                    return fetch(event.request)
                        .then((response) => {
                            // Cachear nuevas respuestas si son GET
                            if (response.ok && event.request.method === 'GET') {
                                const responseClone = response.clone();
                                caches.open(DYNAMIC_CACHE).then((cache) => {
                                    cache.put(event.request, responseClone);
                                });
                            }
                            return response;
                        })
                        .catch(() => {
                            // Si es una navegación, retornar index.html
                            if (event.request.mode === 'navigate') {
                                return caches.match('/index.html');
                            }
                            return new Response('Servidor local inaccesible', {
                                status: 503,
                                headers: { 'Content-Type': 'text/plain' }
                            });
                        });
                })
        );
});

// Sincronización en segundo plano
self.addEventListener('sync', (event) => {
    console.log('[SW] Evento de sincronización:', event.tag);
    if (event.tag === 'sync-data') {
        event.waitUntil(syncData());
    }
});

// Función para sincronizar datos
async function syncData() {
    try {
        console.log('[SW] Iniciando sincronización de datos...');
        // Aquí se implementaría la lógica de sincronización
        // Por ahora, solo logueamos
        console.log('[SW] Sincronización completada');
    } catch (error) {
        console.error('[SW] Error en sincronización:', error);
    }
}

// Manejo de mensajes desde el cliente
self.addEventListener('message', (event) => {
    console.log('[SW] Mensaje recibido:', event.data);
    
    if (event.data && event.data.type === 'SKIP_WAITING') {
        self.skipWaiting();
    }
    
    if (event.data && event.data.type === 'CLEAR_CACHE') {
        event.waitUntil(
            caches.keys().then((cacheNames) => {
                return Promise.all(
                    cacheNames.map((cacheName) => caches.delete(cacheName))
                );
            })
        );
    }
});

// Manejo de errores
self.addEventListener('error', (event) => {
    console.error('[SW] Error global:', event.error);
});

self.addEventListener('unhandledrejection', (event) => {
    console.error('[SW] Promesa rechazada no manejada:', event.reason);
});
