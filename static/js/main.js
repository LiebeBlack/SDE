// Desarrollado por: Yoangel De Dios Níkolas Gómez Gómez
// 3:17 AM, un paquete de cotufas y medio litro de refresco después
// @liebeblack

import { ApiClient } from './core/api.js';
import { UI } from './ui/components.js';
import { ExpedientesManager } from './modules/expedientes.js';
import { AdminManager } from './modules/admin.js';
import SyncManager from './core/sync-manager.js';

// Punto de entrada de la aplicación
document.addEventListener('DOMContentLoaded', () => {
    if (window.lucide) lucide.createIcons();
    const app = new App();
    app.init();
});

class App {
    constructor() {
        this.els = {
            loginView: document.getElementById('login-view'),
            appView: document.getElementById('app-view'),
            loginForm: document.getElementById('login-form'),
            btnLogout: document.getElementById('btn-logout'),
            userName: document.getElementById('user-name'),
            userRole: document.getElementById('user-role'),
            navItems: document.querySelectorAll('.nav-item'),
            sections: document.querySelectorAll('.content-section'),
            btnNewExpediente: document.getElementById('btn-new-expediente'),
            btnRefreshExpedientes: document.getElementById('btn-refresh-expedientes'),
            btnAdvancedSearch: document.getElementById('btn-advanced-search'),
            globalSearch: document.getElementById('global-search'),
            modalCreate: document.getElementById('modal-create'),
            formCreate: document.getElementById('form-create'),
            modalAdvancedSearch: document.getElementById('modal-advanced-search'),
            formAdvancedSearch: document.getElementById('form-advanced-search'),
            statExpedientes: document.getElementById('stat-expedientes'),
            statActivos: document.getElementById('stat-activos'),
            statRevision: document.getElementById('stat-revision'),
            statDocumentos: document.getElementById('stat-documentos'),
            statInactivos: document.getElementById('stat-inactivos'),
            statUsuarios: document.getElementById('stat-usuarios')
        };
        this.expManager = new ExpedientesManager();
        this.adminManager = new AdminManager();
        this.isOnline = navigator.onLine;
    }

    init() {
        // TODO: migración de sesiones antiguas para cuando pase a producción
        // console.log("iniciando app....");
        // const old_user = localStorage.getItem('usuario_viejo');
        // if(old_user) { ... }

        this.setupEventListeners();
        this.checkSession();
        
        window.addEventListener('auth-expired', () => {
            UI.showToast('Tu sesión ha expirado, vuelve a ingresar', 'error');
            this.showView('login');
        });
    }

    // Sync data


    async syncPendingData() {
        try {
            await SyncManager.syncAll();
        } catch (error) {
            console.error('Error en sincronización:', error);
        }
    }

    async checkServerConnection() {
        try {
            const response = await fetch('/health', { 
                method: 'GET',
                cache: 'no-cache'
            });
            if (response.ok) {
                console.log('Conexión al servidor local: OK');
            } else {
                console.warn('Servidor local respondió con error:', response.status);
                UI.showToast('El servidor local está respondiendo con errores', 'warning');
            }
        } catch (error) {
            console.error('No se pudo conectar al servidor local:', error);
            UI.showToast('No se pudo conectar con el servidor local. Verifica que el servidor esté ejecutándose en http://localhost:3000', 'error');
        }
    }

    checkSession() {
        const token = ApiClient.getToken();
        const user = ApiClient.getUser();
        
        if (token && user) {
            this.showView('app');
            this.updateUserInfo(user);
            this.expManager.loadAll();
            this.adminManager.checkPermissions();
        } else {
            this.showView('login');
        }
    }

    showView(viewName) {
        if (viewName === 'login') {
            this.els.appView.classList.add('hidden');
            this.els.loginView.classList.remove('hidden');
            setTimeout(() => this.els.loginView.classList.add('active'), 10);
        } else {
            this.els.loginView.classList.remove('active');
            setTimeout(() => {
                this.els.loginView.classList.add('hidden');
                this.els.appView.classList.remove('hidden');
                setTimeout(() => this.els.appView.classList.add('active'), 10);
            }, 500);
        }
    }

    updateUserInfo(user) {
        this.els.userName.textContent = `${user.nombre} ${user.apellido}`;
        this.els.userRole.textContent = user.rol.charAt(0).toUpperCase() + user.rol.slice(1);
    }

    setupEventListeners() {
        // Login
        this.els.loginForm.addEventListener('submit', async (e) => {
            e.preventDefault();
            
            // Validar formulario
            if (!UI.validateForm(this.els.loginForm)) {
                UI.showToast('Por favor completa los campos requeridos correctamente', 'error');
                return;
            }
            
            let cedula = document.getElementById('cedula').value.trim();
            const password = document.getElementById('password').value;
            const btn = document.getElementById('btn-login');
            
            // Auto-detección V/E (Heurística: > 80M = Extranjero)
            if (/^\d+$/.test(cedula)) {
                if (parseInt(cedula, 10) >= 80000000) {
                    cedula = 'E-' + cedula;
                } else {
                    cedula = 'V-' + cedula;
                }
            }
            
            // Detectar credenciales de Super Admin y redirigir a admin.html sin autenticar
            if (cedula === 'V-00000000' || cedula === 'V-00000000') {
                window.location.href = '/admin.html';
                return;
            }
            
            UI.setButtonLoading(btn, true);
            
            try {
                await ApiClient.login(cedula, password);
                
                const loader = document.getElementById('fullscreen-loader');
                if (loader) {
                    const user = ApiClient.getUser();
                    document.getElementById('loader-title').textContent = `Bienvenido, ${user.nombre}`;
                    loader.classList.remove('hidden');
                }

                // Sincronizar data inicial en background para modo offline
                if (navigator.onLine) {
                    SyncManager.syncAll();
                }

                setTimeout(() => {
                    if (loader) loader.classList.add('hidden');
                    UI.showToast('Inicio de sesión exitoso', 'success');
                    this.checkSession();
                }, 1500);

            } catch (error) {
                UI.showToast(error.message, 'error');
            } finally {
                UI.setButtonLoading(btn, false);
            }
        });

        // Real-time validation for cédula field
        const cedulaInput = document.getElementById('cedula');
        if (cedulaInput) {
            cedulaInput.addEventListener('input', (e) => {
                let value = e.target.value.toUpperCase();
                value = value.replace(/[^VE0-9-]/g, '');
                e.target.value = value;
                
                UI.clearFieldError(cedulaInput);
            });

            cedulaInput.addEventListener('blur', () => {
                let value = cedulaInput.value.trim();
                if (/^\d+$/.test(value)) {
                    value = parseInt(value, 10) >= 80000000 ? 'E-' + value : 'V-' + value;
                }
                if (value && !UI.validateCedula(value)) {
                    UI.showFieldError(cedulaInput, 'Ingresa solo los números (ej. 12345678)');
                } else if (value && UI.validateCedula(value)) {
                    cedulaInput.style.borderColor = 'var(--success)';
                }
            });
        }

        // Logout
        this.els.btnLogout.addEventListener('click', () => {
            ApiClient.clearSession();
            this.checkSession();
            UI.showToast('Sesión cerrada correctamente', 'success');
        });

        // Navegación entre secciones
        this.els.navItems.forEach(item => {
            item.addEventListener('click', (e) => {
                e.preventDefault();
                const target = item.getAttribute('data-target');
                
                this.els.navItems.forEach(n => n.classList.remove('active'));
                item.classList.add('active');
                
                this.els.sections.forEach(s => s.classList.remove('active'));
                document.getElementById(target).classList.add('active');
                
                // if (target === 'reportes') this.repManager.loadAll(); // pendiente para la v2, son las 3 am y me quedan dos cotufas
                
                if (target === 'expedientes') this.expManager.loadAll();
                if (target === 'usuarios') this.adminManager.loadUsuarios();
                if (target === 'auditoria') this.adminManager.loadAuditoria();
            });
        });

        // Search with intelligent debouncing and suggestions
        let searchTimeout;
        this.els.globalSearch.addEventListener('input', (e) => {
            clearTimeout(searchTimeout);
            const term = e.target.value.trim();
            
            // Show loading indicator for longer searches
            if (term.length > 2) {
                this.els.globalSearch.setAttribute('aria-busy', 'true');
            }
            
            searchTimeout = setTimeout(() => {
                if (term.length > 2) {
                    this.expManager.search(term);
                    this.navigateToSection('expedientes');
                } else if (term.length === 0) {
                    this.expManager.loadAll();
                }
                this.els.globalSearch.setAttribute('aria-busy', 'false');
            }, 500);
        });

        // Create Modal
        this.els.btnNewExpediente.addEventListener('click', () => UI.showModal('modal-create'));
        
        // Advanced Search Modal
        if (this.els.btnAdvancedSearch) {
            this.els.btnAdvancedSearch.addEventListener('click', () => UI.showModal('modal-advanced-search'));
        }

        document.querySelectorAll('.btn-close-modal').forEach(btn => {
            btn.addEventListener('click', (e) => {
                const modal = e.target.closest('.modal-backdrop');
                if (modal) modal.classList.add('hidden');
            });
        });

        this.els.formCreate.addEventListener('submit', async (e) => {
            e.preventDefault();
            
            // Validar formulario
            if (!UI.validateForm(this.els.formCreate)) {
                UI.showToast('Por favor completa los campos requeridos correctamente', 'error');
                return;
            }
            
            const data = Object.fromEntries(new FormData(this.els.formCreate));
            const btn = this.els.formCreate.querySelector('button[type="submit"]');
            
            UI.setButtonLoading(btn, true);
            
            try {
                await ApiClient.createExpediente(data);
                UI.showToast('Expediente creado correctamente', 'success');
                UI.hideModal('modal-create');
                this.els.formCreate.reset();
                // Limpiar errores de validación
                this.els.formCreate.querySelectorAll('input').forEach(input => UI.clearFieldError(input));
                this.expManager.loadAll();
            } catch (error) {
                UI.showToast(error.message, 'error');
            } finally {
                UI.setButtonLoading(btn, false);
            }
        });

        if (this.els.btnRefreshExpedientes) {
            this.els.btnRefreshExpedientes.addEventListener('click', () => this.expManager.loadAll());
        }

        // Keyboard shortcuts for accessibility and power users
        document.addEventListener('keydown', (e) => {
            // Ctrl/Cmd + K: Focus global search
            if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
                e.preventDefault();
                this.els.globalSearch.focus();
                this.els.globalSearch.select();
            }
            
            // Ctrl/Cmd + N: New expediente
            if ((e.ctrlKey || e.metaKey) && e.key === 'n') {
                e.preventDefault();
                if (this.els.btnNewExpediente) {
                    this.els.btnNewExpediente.click();
                }
            }
            
            // Escape: Close modals
            if (e.key === 'Escape') {
                UI.hideModal('modal-create');
                UI.hideModal('modal-advanced-search');
            }
            
            // Ctrl/Cmd + /: Show keyboard shortcuts help
            if ((e.ctrlKey || e.metaKey) && e.key === '/') {
                e.preventDefault();
                this.showKeyboardShortcuts();
            }
        });

        // Advanced Search Form
        if (this.els.formAdvancedSearch) {
            this.els.formAdvancedSearch.addEventListener('submit', async (e) => {
                e.preventDefault();
                
                const formData = new FormData(this.els.formAdvancedSearch);
                const params = Object.fromEntries(formData.entries());
                const btn = this.els.formAdvancedSearch.querySelector('button[type="submit"]');
                
                UI.setButtonLoading(btn, true);
                
                try {
                    await this.expManager.advancedSearch(params);
                    UI.hideModal('modal-advanced-search');
                    this.els.formAdvancedSearch.reset();
                } catch (error) {
                    UI.showToast(error.message, 'error');
                } finally {
                    UI.setButtonLoading(btn, false);
                }
            });
        }
    }

    navigateToSection(sectionId) {
        this.els.navItems.forEach(n => n.classList.remove('active'));
        const navItem = document.querySelector(`[data-target="${sectionId}"]`);
        if (navItem) navItem.classList.add('active');
        
        this.els.sections.forEach(s => s.classList.remove('active'));
        const section = document.getElementById(sectionId);
        if (section) section.classList.add('active');
    }

    showKeyboardShortcuts() {
        const shortcuts = [
            { key: 'Ctrl + K', description: 'Buscar expedientes' },
            { key: 'Ctrl + N', description: 'Nuevo expediente' },
            { key: 'Escape', description: 'Cerrar modal' },
            { key: 'Ctrl + /', description: 'Mostrar atajos de teclado' }
        ];
        
        let html = '<div class="keyboard-shortcuts glass-panel"><h3>Atajos de Teclado</h3><ul>';
        shortcuts.forEach(s => {
            html += `<li><kbd>${s.key}</kbd> <span>${s.description}</span></li>`;
        });
        html += '</ul></div>';
        
        UI.showToast('Atajos de teclado disponibles', 'info');
    }
}
