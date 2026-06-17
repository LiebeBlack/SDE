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
        if (user && (user.rol === 'Super' || user.rol === 'Administrativo')) {
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
                const formData = new FormData(this.els.formCreateUser);
                const data = Object.fromEntries(formData.entries());
                
                if (!data.password) {
                    delete data.password;
                }

                try {
                    await ApiClient.crearUsuario(data);
                    UI.showToast('Usuario creado correctamente', 'success');
                    UI.hideModal('modal-create-user');
                    this.els.formCreateUser.reset();
                    this.loadUsuarios();
                } catch (error) {
                    UI.showToast(error.message, 'error');
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
                    </tr>
                </thead>
                <tbody>
        `;
        
        usuarios.forEach(user => {
            const badgeClass = user.activo ? 'success' : 'danger';
            const estadoTexto = user.activo ? 'Activo' : 'Inactivo';
            html += `
                <tr>
                    <td><strong>${user.cedula}</strong></td>
                    <td>${user.nombre} ${user.apellido}</td>
                    <td>${user.email}</td>
                    <td>${user.rol}</td>
                    <td><span class="badge ${badgeClass}">${estadoTexto}</span></td>
                </tr>
            `;
        });
        
        html += `</tbody></table>`;
        this.els.usersTable.innerHTML = html;
        if (window.lucide) lucide.createIcons();
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
            html += `
                <tr>
                    <td style="white-space: nowrap; font-size: 0.85rem; color: var(--text-secondary);">${date}</td>
                    <td><strong><span class="badge warning">${reg.accion}</span></strong></td>
                    <td style="font-size: 0.9rem;">${reg.detalles}</td>
                    <td style="font-size: 0.8rem; font-family: monospace; color: var(--text-secondary);">${reg.usuario_id || 'N/A'}</td>
                </tr>
            `;
        });
        
        html += `</tbody></table>`;
        this.els.auditTable.innerHTML = html;
        if (window.lucide) lucide.createIcons();
    }
}
