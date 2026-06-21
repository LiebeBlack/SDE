// Módulo de almacenamiento local para modo offline
const OfflineStorage = {
    // Claves de almacenamiento
    KEYS: {
        EXPEDIENTES: 'sge_expedientes',
        USUARIOS: 'sge_usuarios',
        AUDITORIA: 'sge_auditoria',
        DOCUMENTOS: 'sge_documentos',
        PENDING_ACTIONS: 'sge_pending_actions',
        SESSION: 'sge_session',
        LAST_SYNC: 'sge_last_sync'
    },

    // Guardar datos en localStorage
    save(key, data) {
        try {
            const serialized = JSON.stringify(data);
            localStorage.setItem(key, serialized);
            return true;
        } catch (error) {
            console.error('[OfflineStorage] Error al guardar:', error);
            return false;
        }
    },

    // Obtener datos de localStorage
    load(key) {
        try {
            const serialized = localStorage.getItem(key);
            if (!serialized) return null;
            return JSON.parse(serialized);
        } catch (error) {
            console.error('[OfflineStorage] Error al cargar:', error);
            return null;
        }
    },

    // Eliminar datos de localStorage
    remove(key) {
        try {
            localStorage.removeItem(key);
            return true;
        } catch (error) {
            console.error('[OfflineStorage] Error al eliminar:', error);
            return false;
        }
    },

    // Limpiar todos los datos
    clear() {
        try {
            Object.values(this.KEYS).forEach(key => localStorage.removeItem(key));
            return true;
        } catch (error) {
            console.error('[OfflineStorage] Error al limpiar:', error);
            return false;
        }
    },

    // Guardar expedientes
    saveExpedientes(expedientes) {
        return this.save(this.KEYS.EXPEDIENTES, expedientes);
    },

    // Cargar expedientes
    loadExpedientes() {
        return this.load(this.KEYS.EXPEDIENTES);
    },

    // Guardar usuarios
    saveUsuarios(usuarios) {
        return this.save(this.KEYS.USUARIOS, usuarios);
    },

    // Cargar usuarios
    loadUsuarios() {
        return this.load(this.KEYS.USUARIOS);
    },

    // Guardar auditoría
    saveAuditoria(auditoria) {
        return this.save(this.KEYS.AUDITORIA, auditoria);
    },

    // Cargar auditoría
    loadAuditoria() {
        return this.load(this.KEYS.AUDITORIA);
    },

    // Guardar sesión
    saveSession(session) {
        return this.save(this.KEYS.SESSION, session);
    },

    // Cargar sesión
    loadSession() {
        return this.load(this.KEYS.SESSION);
    },

    // Guardar última sincronización
    saveLastSync(timestamp) {
        return this.save(this.KEYS.LAST_SYNC, timestamp);
    },

    // Cargar última sincronización
    loadLastSync() {
        return this.load(this.KEYS.LAST_SYNC);
    },

    // Agregar acción pendiente
    addPendingAction(action) {
        const pending = this.load(this.KEYS.PENDING_ACTIONS) || [];
        pending.push({
            ...action,
            id: Date.now().toString(),
            timestamp: new Date().toISOString()
        });
        return this.save(this.KEYS.PENDING_ACTIONS, pending);
    },

    // Obtener acciones pendientes
    getPendingActions() {
        return this.load(this.KEYS.PENDING_ACTIONS) || [];
    },

    // Eliminar acción pendiente
    removePendingAction(actionId) {
        const pending = this.load(this.KEYS.PENDING_ACTIONS) || [];
        const filtered = pending.filter(a => a.id !== actionId);
        return this.save(this.KEYS.PENDING_ACTIONS, filtered);
    },

    // Limpiar acciones pendientes
    clearPendingActions() {
        return this.save(this.KEYS.PENDING_ACTIONS, []);
    },

    // Verificar si hay datos almacenados
    hasData() {
        return Object.values(this.KEYS).some(key => localStorage.getItem(key) !== null);
    },

    // Obtener tamaño de almacenamiento usado
    getStorageSize() {
        let total = 0;
        for (let key in localStorage) {
            if (localStorage.hasOwnProperty(key)) {
                total += localStorage[key].length;
            }
        }
        return (total / 1024).toFixed(2) + ' KB';
    }
};

export default OfflineStorage;
