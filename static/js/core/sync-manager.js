// Módulo de sincronización para modo offline
import OfflineStorage from './offline-storage.js';

export class SyncManager {
    constructor() {
        this.syncInProgress = false;
        this.autoSyncInterval = null;
    }

    // Iniciar sincronización automática
    startAutoSync(intervalMs = 5 * 60 * 1000) { // 5 minutos por defecto
        this.stopAutoSync();
        this.autoSyncInterval = setInterval(() => {
            if (navigator.onLine) {
                this.syncAll();
            }
        }, intervalMs);
    }

    // Detener sincronización automática
    stopAutoSync() {
        if (this.autoSyncInterval) {
            clearInterval(this.autoSyncInterval);
            this.autoSyncInterval = null;
        }
    }

    // Sincronizar todo
    async syncAll() {
        if (this.syncInProgress || !navigator.onLine) {
            return;
        }

        this.syncInProgress = true;
        console.log('[Sync] Iniciando sincronización completa...');

        try {
            // Sincronizar acciones pendientes
            await this.syncPendingActions();
            
            // Sincronizar datos del servidor
            await this.syncFromServer();
            
            console.log('[Sync] Sincronización completada');
            return true;
        } catch (error) {
            console.error('[Sync] Error en sincronización:', error);
            return false;
        } finally {
            this.syncInProgress = false;
        }
    }

    // Sincronizar acciones pendientes
    async syncPendingActions() {
        const pending = OfflineStorage.getPendingActions();
        
        if (pending.length === 0) {
            console.log('[Sync] No hay acciones pendientes');
            return;
        }

        console.log(`[Sync] Sincronizando ${pending.length} acciones pendientes...`);

        const { ApiClient } = await import('./api.js');
        
        for (const action of pending) {
            try {
                await this.executeAction(action, ApiClient);
                OfflineStorage.removePendingAction(action.id);
                console.log(`[Sync] Acción sincronizada: ${action.type}`);
            } catch (error) {
                console.error(`[Sync] Error sincronizando acción ${action.id}:`, error);
                // No eliminar la acción, se reintentará en la próxima sincronización
            }
        }
    }

    // Ejecutar acción pendiente
    async executeAction(action, ApiClient) {
        switch (action.type) {
            case 'CREATE_EXPEDIENTE':
                await ApiClient.createExpediente(action.data);
                break;
            case 'UPDATE_EXPEDIENTE':
                await ApiClient.updateEstadoExpediente(action.data.id, action.data.estado);
                break;
            case 'CREATE_USUARIO':
                await ApiClient.crearUsuario(action.data);
                break;
            case 'TOGGLE_USUARIO':
                await ApiClient.toggleUsuarioEstado(action.data.usuarioId);
                break;
            case 'FOLIAR_DOCUMENTO':
                await ApiClient.foliarDocumento(action.data.expedienteId, action.data.documentoId);
                break;
            default:
                console.warn('[Sync] Tipo de acción desconocido:', action.type);
        }
    }

    // Sincronizar datos desde el servidor
    async syncFromServer() {
        const { ApiClient } = await import('./api.js');
        
        try {
            // Sincronizar expedientes
            const expedientes = await ApiClient.getExpedientes();
            if (expedientes) {
                console.log('[Sync] Expedientes sincronizados en caché:', expedientes.length);
            }

            // Sincronizar usuarios (si es admin)
            const user = ApiClient.getUser();
            if (user && (user.rol === 'super' || user.rol === 'director' || user.rol === 'recursos_humanos')) {
                const usuarios = await ApiClient.getUsuarios();
                if (usuarios) {
                    console.log('[Sync] Usuarios sincronizados en caché:', usuarios.length);
                }
            }

            // Sincronizar auditoría (si es admin)
            if (user && (user.rol === 'super' || user.rol === 'director')) {
                const auditoria = await ApiClient.getAuditoria();
                if (auditoria) {
                    console.log('[Sync] Auditoría sincronizada en caché:', auditoria.length);
                }
            }

            // Actualizar timestamp de última sincronización
            OfflineStorage.saveLastSync(new Date().toISOString());
            
        } catch (error) {
            console.error('[Sync] Error sincronizando desde servidor:', error);
            throw error;
        }
    }

    // Agregar acción pendiente
    addPendingAction(type, data) {
        return OfflineStorage.addPendingAction({ type, data });
    }

    // Obtener estado de sincronización
    getSyncStatus() {
        const lastSync = OfflineStorage.loadLastSync();
        const pending = OfflineStorage.getPendingActions();
        
        return {
            lastSync: lastSync ? new Date(lastSync) : null,
            pendingActions: pending.length,
            isOnline: navigator.onLine,
            syncInProgress: this.syncInProgress
        };
    }

    // Limpiar datos offline
    clearOfflineData() {
        OfflineStorage.clear();
        console.log('[Sync] Datos offline eliminados');
    }
}

export default new SyncManager();
