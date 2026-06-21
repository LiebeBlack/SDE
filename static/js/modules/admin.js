import { ApiClient } from '../core/api.js';
import { UI } from '../ui/components.js';

export class AdminManager {
    constructor() {
        this.els = {
            usersTable: document.getElementById('users-table-container'),
            auditTable: document.getElementById('audit-table-container'),
            btnNewUser: document.getElementById('btn-new-user'),
            btnRefreshAudit: document.getElementById('btn-refresh-audit'),
            formCreateUser: document.getElementById('form-create-user'),
            adminNavItems: document.querySelectorAll('.admin-only')
        };

        this.setupEventListeners();
    }

    checkPermissions() {
        const user = ApiClient.getUser();
        if (user && (user.rol === 'super' || user.rol === 'administrativo')) {
            this.els.adminNavItems.forEach(item => item.classList.remove('hidden'));
        } else {
            this.els.adminNavItems.forEach(item => item.classList.add('hidden'));
        }
    }

    setupEventListeners() {
        if (this.els.btnNewUser) {
            this.els.btnNewUser.addEventListener('click', () => {
                UI.showModal('modal-create-user');
            });
        }

        if (this.els.formCreateUser) {
            this.els.formCreateUser.addEventListener('submit', async (e) => {
                e.preventDefault();
                
                // Validar formulario
                if (!UI.validateForm(this.els.formCreateUser)) {
                    UI.showToast('Por favor completa los campos requeridos correctamente', 'error');
                    return;
                }
                
                const formData = new FormData(this.els.formCreateUser);
                const data = Object.fromEntries(formData.entries());
                
                if (!data.password) {
                    delete data.password;
                }

                const btn = this.els.formCreateUser.querySelector('button[type="submit"]');
                UI.setButtonLoading(btn, true);

                try {
                    await ApiClient.crearUsuario(data);
                    UI.showToast('Usuario creado correctamente', 'success');
                    UI.hideModal('modal-create-user');
                    this.els.formCreateUser.reset();
                    // Limpiar errores de validación
                    this.els.formCreateUser.querySelectorAll('input, select').forEach(input => UI.clearFieldError(input));
                    this.loadUsuarios();
                } catch (error) {
                    UI.showToast(error.message, 'error');
                } finally {
                    UI.setButtonLoading(btn, false);
                }
            });
        }

        if (this.els.btnRefreshAudit) {
            this.els.btnRefreshAudit.addEventListener('click', () => {
                this.loadAuditoria();
            });
        }
    }

    async loadUsuarios() {
        if (!this.els.usersTable) return;
        this.els.usersTable.innerHTML = `<table><tbody>${UI.getSkeletonTable(3, 5)}</tbody></table>`;
        
        try {
            const usuarios = await ApiClient.getUsuarios();
            this.renderUsuariosTable(usuarios);
        } catch (error) {
            if (error.message.startsWith('OFFLINE:')) {
                this.els.usersTable.innerHTML = UI.getOfflineState(error.message.replace('OFFLINE:', ''));
            } else {
                this.els.usersTable.innerHTML = `<div class="error-state">Error: ${error.message}</div>`;
            }
            if (window.lucide) lucide.createIcons();
        }
    }

    renderUsuariosTable(usuarios) {
        if (!usuarios || usuarios.length === 0) {
            this.els.usersTable.innerHTML = '<div class="empty-state"><i data-lucide="users"></i><p>No hay usuarios registrados</p></div>';
            if (window.lucide) lucide.createIcons();
            return;
        }

        let html = `
            <table>
                <thead>
                    <tr>
                        <th>Cédula</th>
                        <th>Nombre y Apellido</th>
                        <th>Email</th>
                        <th>Rol</th>
                        <th>Estado</th>
                        <th>Acciones</th>
                    </tr>
                </thead>
                <tbody>
        `;
        
        usuarios.forEach(user => {
            const badgeClass = user.activo ? 'success' : 'danger';
            const estadoTexto = user.activo ? 'Activo' : 'Inactivo';
            const toggleIcon = user.activo ? 'user-x' : 'user-check';
            const toggleTitle = user.activo ? 'Desactivar Usuario' : 'Activar Usuario';
            const toggleClass = user.activo ? 'btn-danger' : 'btn-success';
            
            html += `
                <tr data-user-id="${user.id}">
                    <td><strong>${user.cedula}</strong></td>
                    <td>${user.nombre} ${user.apellido}</td>
                    <td>${user.email}</td>
                    <td>${user.rol}</td>
                    <td><span class="badge ${badgeClass}">${estadoTexto}</span></td>
                    <td>
                        <button class="btn-icon ${toggleClass} btn-toggle-user" data-user-id="${user.id}" data-active="${user.activo}" title="${toggleTitle}">
                            <i data-lucide="${toggleIcon}"></i>
                        </button>
                    </td>
                </tr>
            `;
        });
        
        html += `</tbody></table>`;
        this.els.usersTable.innerHTML = html;
        if (window.lucide) lucide.createIcons();

        // Agregar event listeners para toggle de estado
        this.els.usersTable.querySelectorAll('.btn-toggle-user').forEach(btn => {
            btn.addEventListener('click', () => this.toggleUserEstado(btn.dataset.userId, btn.dataset.active === 'true'));
        });
    }

    async toggleUserEstado(userId, currentlyActive) {
        try {
            await ApiClient.toggleUsuarioEstado(userId);
            UI.showToast(currentlyActive ? 'Usuario desactivado correctamente' : 'Usuario activado correctamente', 'success');
            this.loadUsuarios();
        } catch (error) {
            UI.showToast(error.message, 'error');
        }
    }

    async loadAuditoria() {
        if (!this.els.auditTable) return;
        this.els.auditTable.innerHTML = `<table><tbody>${UI.getSkeletonTable(8, 4)}</tbody></table>`;
        
        try {
            const registros = await ApiClient.getAuditoria();
            this.renderAuditoriaTable(registros);
        } catch (error) {
            if (error.message.startsWith('OFFLINE:')) {
                this.els.auditTable.innerHTML = UI.getOfflineState(error.message.replace('OFFLINE:', ''));
            } else {
                this.els.auditTable.innerHTML = `<div class="error-state">Error: ${error.message}</div>`;
            }
            if (window.lucide) lucide.createIcons();
        }
    }

    renderAuditoriaTable(registros) {
        if (!registros || registros.length === 0) {
            this.els.auditTable.innerHTML = '<div class="empty-state"><i data-lucide="shield"></i><p>No hay registros de auditoría</p></div>';
            if (window.lucide) lucide.createIcons();
            return;
        }

        let html = `
            <table>
                <thead>
                    <tr>
                        <th>Fecha y Hora</th>
                        <th>Acción</th>
                        <th>Detalles</th>
                        <th>Usuario ID</th>
                    </tr>
                </thead>
                <tbody>
        `;
        
        registros.forEach(reg => {
            const date = new Date(reg.timestamp).toLocaleString('es-ES');
            const badgeClass = this.getBadgeClassForAccion(reg.accion);
            html += `
                <tr>
                    <td style="white-space: nowrap; font-size: 0.85rem; color: var(--text-secondary);">${date}</td>
                    <td><strong><span class="badge ${badgeClass}">${reg.accion.replace(/_/g, ' ')}</span></strong></td>
                    <td style="font-size: 0.9rem;">
                        ${reg.detalles}
                        ${reg.ip_address ? `<br><small style="color: var(--text-muted);"><i data-lucide="globe" style="width: 12px; height: 12px; display: inline-block; vertical-align: middle;"></i> IP: ${reg.ip_address}</small>` : ''}
                    </td>
                    <td style="font-size: 0.8rem; font-family: monospace; color: var(--text-secondary);">${reg.usuario_id || 'Sistema'}</td>
                </tr>
            `;
        });
        
        html += `</tbody></table>`;
        this.els.auditTable.innerHTML = html;
        if (window.lucide) lucide.createIcons();
    }

    getBadgeClassForAccion(accion) {
        if (!accion) return 'secondary';
        
        if (accion.includes('LOGIN_FALLIDO') || accion.includes('ELIMINACION')) {
            return 'danger';
        }
        if (accion.includes('CREACION') || accion.includes('SUBIDA') || accion.includes('LOGIN_USUARIO')) {
            return 'success';
        }
        if (accion.includes('MODIFICACION') || accion.includes('CAMBIO_ESTADO') || accion.includes('FOLIADO')) {
            return 'primary';
        }
        if (accion.includes('LOGOUT')) {
            return 'secondary';
        }
        return 'warning';
    }
}
