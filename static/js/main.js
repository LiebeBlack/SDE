// Desarrollado por: Yoangel De Dios Níkolas Gómez Gómez
// 3:17 AM, un paquete de cotufas y medio litro de refresco después
// @liebeblack

import { ApiClient } from './core/api.js';
import { UI } from './ui/components.js';
import { ExpedientesManager } from './modules/expedientes.js';
import { AdminManager } from './modules/admin.js';

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
            globalSearch: document.getElementById('global-search'),
            modalCreate: document.getElementById('modal-create'),
            formCreate: document.getElementById('form-create')
        };
        this.expManager = new ExpedientesManager();
        this.adminManager = new AdminManager();
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
            const cedula = document.getElementById('cedula').value;
            const password = document.getElementById('password').value;
            const btn = document.getElementById('btn-login');
            
            btn.disabled = true;
            btn.innerHTML = '<i data-lucide="loader-2" class="spin"></i> Verificando...';
            if (window.lucide) lucide.createIcons();
            
            try {
                await ApiClient.login(cedula, password);
                UI.showToast('Inicio de sesión exitoso', 'success');
                this.checkSession();
            } catch (error) {
                UI.showToast(error.message, 'error');
            } finally {
                btn.disabled = false;
                btn.innerHTML = '<span>Ingresar al Sistema</span><i data-lucide="arrow-right"></i>';
                if (window.lucide) lucide.createIcons();
            }
        });

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

        // Search
        let searchTimeout;
        this.els.globalSearch.addEventListener('input', (e) => {
            clearTimeout(searchTimeout);
            searchTimeout = setTimeout(() => {
                const term = e.target.value.trim();
                if (term.length > 2) {
                    this.expManager.search(term);
                } else if (term.length === 0) {
                    this.expManager.loadAll();
                }
            }, 500);
        });

        // Create Modal
        this.els.btnNewExpediente.addEventListener('click', () => UI.showModal('modal-create'));
        
        document.querySelectorAll('.btn-close-modal').forEach(btn => {
            btn.addEventListener('click', (e) => {
                const modal = e.target.closest('.modal-backdrop');
                if (modal) modal.classList.add('hidden');
            });
        });

        this.els.formCreate.addEventListener('submit', async (e) => {
            e.preventDefault();
            const data = Object.fromEntries(new FormData(this.els.formCreate));
            try {
                await ApiClient.createExpediente(data);
                UI.showToast('Expediente creado correctamente', 'success');
                UI.hideModal('modal-create');
                this.els.formCreate.reset();
                this.expManager.loadAll();
            } catch (error) {
                UI.showToast(error.message, 'error');
            }
        });

        if (this.els.btnRefreshExpedientes) {
            this.els.btnRefreshExpedientes.addEventListener('click', () => this.expManager.loadAll());
        }
    }
}
