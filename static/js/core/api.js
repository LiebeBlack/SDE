const API_URL = '/api';

// Importar OfflineStorage dinámicamente para evitar dependencias circulares
let OfflineStorage = null;
import('./offline-storage.js').then(m => {
    OfflineStorage = m.default;
}).catch(e => {
    console.warn('OfflineStorage no disponible, trabajando sin caché');
});

export class ApiClient {
    static isOnline() {
        return navigator.onLine;
    }

    static checkExpiration() {
        const expires = localStorage.getItem('sge_expires');
        if (expires && new Date().getTime() > parseInt(expires)) {
            if (!this.isOnline()) {
                console.log('[API] Sesión local expirada, pero estamos offline. Extendiendo sesión temporalmente.');
                localStorage.setItem('sge_expires', new Date().getTime() + 30 * 60 * 1000);
                return false;
            }
            this.clearSession();
            return true;
        }
        if (expires) {
            localStorage.setItem('sge_expires', new Date().getTime() + 30 * 60 * 1000);
        }
        return false;
    }

    static getToken() {
        if (this.checkExpiration()) return null;
        return localStorage.getItem('sge_token');
    }

    static setSession(token, user) {
        try {
            localStorage.setItem('sge_token', token);
            localStorage.setItem('sge_user', JSON.stringify(user));
            localStorage.setItem('sge_expires', new Date().getTime() + 30 * 60 * 1000);
            if (OfflineStorage) {
                OfflineStorage.saveSession({ token, user });
            }
        } catch (error) {
            console.error('Error guardando sesión:', error);
            throw new Error('Error al guardar la sesión en el navegador');
        }
    }

    static clearSession() {
        try {
            localStorage.removeItem('sge_token');
            localStorage.removeItem('sge_user');
            localStorage.removeItem('sge_expires');
            if (OfflineStorage) {
                OfflineStorage.remove(OfflineStorage.KEYS.SESSION);
            }
        } catch (error) {
            console.error('Error limpiando sesión:', error);
        }
    }

    static getUser() {
        if (this.checkExpiration()) return null;
        try {
            const userStr = localStorage.getItem('sge_user');
            return userStr ? JSON.parse(userStr) : null;
        } catch (error) {
            console.error('Error leyendo usuario:', error);
            this.clearSession();
            return null;
        }
    }

    static async request(endpoint, options = {}) {
        const url = `${API_URL}${endpoint}`;
        const headers = { ...options.headers };

        if (!(options.body instanceof FormData) && !headers['Content-Type']) {
            headers['Content-Type'] = 'application/json';
        }

        const token = this.getToken();
        if (token) {
            headers['Authorization'] = `Bearer ${token}`;
        }

        const config = {
            ...options,
            headers,
            timeout: 15000 // 15 segundos timeout
        };

        if (config.body && typeof config.body === 'object' && !(config.body instanceof FormData)) {
            config.body = JSON.stringify(config.body);
        }

        try {
            // Intentar fetch con timeout
            const controller = new AbortController();
            const timeoutId = setTimeout(() => controller.abort(), config.timeout);
            
            const response = await fetch(url, {
                ...config,
                signal: controller.signal
            });
            
            clearTimeout(timeoutId);
            
            if (response.status === 401) {
                this.clearSession();
                window.dispatchEvent(new Event('auth-expired'));
                throw new Error('Sesión expirada o inválida');
            }

            if (response.status === 403) {
                throw new Error('No tienes permisos para realizar esta acción');
            }

            if (response.status === 404) {
                throw new Error('Recurso no encontrado');
            }

            if (response.status === 500) {
                throw new Error('Error interno del servidor. Contacta al administrador');
            }

            if (response.status === 429) {
                throw new Error('Demasiadas solicitudes. Por favor espera un momento');
            }

            const data = await response.json().catch(() => ({}));

            if (!response.ok) {
                throw new Error(data.error || data.message || 'Error en la solicitud al servidor');
            }

            // Guardar datos en caché offline si es una operación GET exitosa
            if (config.method !== 'POST' && config.method !== 'PUT' && config.method !== 'DELETE' && OfflineStorage) {
                this.cacheResponse(endpoint, data);
            }

            return data;
        } catch (error) {
            console.error(`[API Error] ${endpoint}:`, error);
            
            // Manejo específico para errores de red/offline
            if (error.name === 'AbortError') {
                throw new Error('TIMEOUT: El servidor tardó demasiado en responder. Verifica tu conexión.');
            }
            
            if (error instanceof TypeError || error.message === 'Failed to fetch' || error.message.includes('NetworkError')) {
                // Intentar usar caché offline
                if (OfflineStorage && config.method === 'GET') {
                    const cached = this.getCachedResponse(endpoint);
                    if (cached) {
                        console.log('[API] Usando datos offline para:', endpoint);
                        return cached;
                    }
                }
                throw new Error('OFFLINE: Servidor local no responde.');
            }
            
            if (error.name === 'SyntaxError') {
                throw new Error('Error al procesar la respuesta del servidor');
            }
            
            throw error;
        }
    }

    // Métodos de caché offline
    static cacheResponse(endpoint, data) {
        if (!OfflineStorage) return;
        
        try {
            const cacheKey = `cache_${endpoint}`;
            localStorage.setItem(cacheKey, JSON.stringify({
                data,
                timestamp: Date.now()
            }));
        } catch (error) {
            console.error('[API] Error guardando caché:', error);
        }
    }

    static getCachedResponse(endpoint) {
        if (!OfflineStorage) return null;
        
        try {
            const cacheKey = `cache_${endpoint}`;
            const cached = localStorage.getItem(cacheKey);
            if (!cached) return null;
            
            const { data, timestamp } = JSON.parse(cached);
            // Caché válida por 24 horas
            if (Date.now() - timestamp > 24 * 60 * 60 * 1000) {
                localStorage.removeItem(cacheKey);
                return null;
            }
            
            return data;
        } catch (error) {
            console.error('[API] Error leyendo caché:', error);
            return null;
        }
    }

    static clearCache() {
        try {
            Object.keys(localStorage).forEach(key => {
                if (key.startsWith('cache_')) {
                    localStorage.removeItem(key);
                }
            });
        } catch (error) {
            console.error('[API] Error limpiando caché:', error);
        }
    }

    // Autenticación
    static async login(cedula, password) {
        try {
            const response = await fetch('/api/login', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ cedula, password })
            });
            const data = await response.json();
            if (!response.ok) throw new Error(data.error || 'Error de inicio de sesión');
            
            this.setSession(data.token, data.usuario);
            return data;
        } catch (err) {
            // Offline fallback for login
            if (OfflineStorage) {
                const sessionStr = localStorage.getItem('sge_session');
                const userStr = localStorage.getItem('usuario');
                if (sessionStr && userStr) {
                    const user = JSON.parse(userStr);
                    // Match either exact cedula or the numeric part
                    const numCedula = cedula.replace(/^[VE]-/i, '');
                    const userNumCedula = user.cedula.replace(/^[VE]-/i, '');
                    if (numCedula === userNumCedula) {
                        console.warn('[API] Login offline usando sesión cacheada.');
                        this.setSession(sessionStr, user);
                        return { token: sessionStr, usuario: user };
                    }
                }
            }
            throw err;
        }
    }

    // Expedientes
    static async getExpedientes() {
        return this.request('/expedientes');
    }

    static async getExpedienteDetails(id) {
        return this.request(`/expedientes/${id}`);
    }

    static async createExpediente(data) {
        return this.request('/expedientes', {
            method: 'POST',
            body: data
        });
    }

    static async searchExpedientes(term) {
        return this.request(`/expedientes/buscar/${encodeURIComponent(term)}`);
    }

    static async updateEstadoExpediente(id, nuevoEstado) {
        return this.request(`/expedientes/${id}/estado`, {
            method: 'POST',
            body: { estado: nuevoEstado }
        });
    }

    // Documentos
    static async getDocumentos(expedienteId) {
        return this.request(`/expedientes/${expedienteId}/documentos`);
    }

    static async uploadDocumento(expedienteId, formData) {
        return this.request(`/expedientes/${expedienteId}/documentos`, {
            method: 'POST',
            body: formData
        });
    }

    static async downloadFile(endpoint, defaultFilename = 'documento') {
        const url = `${API_URL}${endpoint}`;
        const headers = {};
        
        const token = this.getToken();
        if (token) {
            headers['Authorization'] = `Bearer ${token}`;
        }

        try {
            const response = await fetch(url, { headers });
            
            if (response.status === 401) {
                this.clearSession();
                window.dispatchEvent(new Event('auth-expired'));
                throw new Error('Sesión expirada o inválida');
            }
            if (!response.ok) {
                const text = await response.text();
                throw new Error('Error al descargar el archivo: ' + text);
            }
            
            // Intentar obtener el nombre original del archivo de los headers
            let filename = defaultFilename;
            const disposition = response.headers.get('content-disposition');
            if (disposition && disposition.indexOf('attachment') !== -1) {
                const filenameRegex = /filename[^;=\n]*=((['"]).*?\2|[^;\n]*)/;
                const matches = filenameRegex.exec(disposition);
                if (matches != null && matches[1]) { 
                    filename = matches[1].replace(/['"]/g, '');
                }
            }

            const blob = await response.blob();
            const downloadUrl = window.URL.createObjectURL(blob);
            const a = document.createElement('a');
            a.style.display = 'none';
            a.href = downloadUrl;
            a.download = filename;
            document.body.appendChild(a);
            a.click();
            window.URL.revokeObjectURL(downloadUrl);
            a.remove();
            
            return true;
        } catch (error) {
            console.error('[API] Error descargando archivo:', error);
            throw error;
        }
    }

    // Panel admin
    static async getUsuarios() {
        return this.request('/usuarios');
    }

    static async crearUsuario(data) {
        return this.request('/usuarios', {
            method: 'POST',
            body: data
        });
    }

    static async toggleUsuarioEstado(usuarioId) {
        return this.request(`/usuarios/${usuarioId}/toggle`, {
            method: 'POST'
        });
    }

    static async getAuditoria() {
        return this.request('/auditoria');
    }

    static async foliarDocumento(expedienteId, documentoId) {
        return this.request(`/expedientes/${expedienteId}/documentos/${documentoId}/foliar`, {
            method: 'POST'
        });
    }

    static async getExpedientePorCedula(cedula) {
        return this.request(`/expedientes/cedula/${encodeURIComponent(cedula)}`);
    }

    static async buscarAvanzadaExpedientes(params) {
        const queryString = new URLSearchParams(params).toString();
        return this.request(`/buscar/expedientes_avanzado?${queryString}`);
    }

    static async buscarAvanzadaDocumentos(params) {
        const queryString = new URLSearchParams(params).toString();
        return this.request(`/buscar/documentos_avanzado?${queryString}`);
    }

    static async buscarGeneral(termino) {
        return this.request(`/buscar/general?termino=${encodeURIComponent(termino)}`);
    }
}
