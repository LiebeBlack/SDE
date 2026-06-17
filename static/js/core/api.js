const API_URL = '/api';

export class ApiClient {
    static checkExpiration() {
        const expires = localStorage.getItem('sge_expires');
        if (expires && new Date().getTime() > parseInt(expires)) {
            this.clearSession();
            return true;
        }
        // Refrescar expiración si hay actividad
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
        localStorage.setItem('sge_token', token);
        localStorage.setItem('sge_user', JSON.stringify(user));
        localStorage.setItem('sge_expires', new Date().getTime() + 30 * 60 * 1000); // 30 mins
    }

    static clearSession() {
        localStorage.removeItem('sge_token');
        localStorage.removeItem('sge_user');
        localStorage.removeItem('sge_expires');
    }

    static getUser() {
        if (this.checkExpiration()) return null;
        const userStr = localStorage.getItem('sge_user');
        return userStr ? JSON.parse(userStr) : null;
    }

    static async request(endpoint, options = {}) {
        const url = `${API_URL}${endpoint}`;
        const headers = { ...options.headers };

        // FormData pone su propio Content-Type
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
        };

        if (config.body && typeof config.body === 'object' && !(config.body instanceof FormData)) {
            config.body = JSON.stringify(config.body);
        }

        try {
            const response = await fetch(url, config);
            
            if (response.status === 401) {
                this.clearSession();
                window.dispatchEvent(new Event('auth-expired'));
                throw new Error('Sesión expirada o inválida');
            }

            const data = await response.json().catch(() => ({}));

            if (!response.ok) {
                throw new Error(data.error || 'Error en la solicitud al servidor');
            }

            return data;
        } catch (error) {
            console.error(`[API Error] ${endpoint}:`, error);
            // Si es un TypeError normalmente es que no hay internet o el servidor está caído
            if (error instanceof TypeError || error.message === 'Failed to fetch') {
                throw new Error('OFFLINE: No se pudo conectar con el servidor. Verifica tu conexión.');
            }
            throw error;
        }
    }

    // Autenticación
    static async login(cedula, password) {
        try {
            // login va directo a /login, no pasa por /api
            const response = await fetch('/login', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ cedula, password })
            });
            const data = await response.json();
            if (!response.ok) throw new Error(data.error || 'Error de inicio de sesión');
            
            this.setSession(data.token, data.usuario);
            return data;
        } catch (err) {
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

    static async getAuditoria() {
        return this.request('/auditoria');
    }
}
