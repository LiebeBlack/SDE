import { ApiClient } from '../core/api.js';
import { UI } from '../ui/components.js';
import { DocumentosManager } from './documentos.js';

export class ExpedientesManager {
    constructor() {
        this.fullTable = document.getElementById('full-table-container');
        this.recentTable = document.getElementById('recent-table-container');
        this.docManager = new DocumentosManager();
        this.filterEstado = document.getElementById('filter-estado');
        this.cachedExpedientes = [];
        this.setupModals();
        this.setupFilters();
    }

    async loadAll() {
        if (!this.fullTable) return;
        this.fullTable.innerHTML = `<table><tbody>${UI.getSkeletonTable(5, 5)}</tbody></table>`;
        try {
            const expedientes = await ApiClient.getExpedientes();
            this.cachedExpedientes = expedientes;
            this.applyFilter();
            this.updateDashboardStats(expedientes);
        } catch (error) {
            if (error.message.startsWith('OFFLINE:')) {
                this.fullTable.innerHTML = UI.getOfflineState(error.message.replace('OFFLINE:', ''));
            } else {
                this.fullTable.innerHTML = `<div class="error-state">Error: ${error.message}</div>`;
            }
            if (window.lucide) lucide.createIcons();
        }
    }

    setupFilters() {
        if (this.filterEstado) {
            this.filterEstado.addEventListener('change', () => this.applyFilter());
        }
    }

    applyFilter() {
        const filtro = this.filterEstado ? this.filterEstado.value : '';
        let filtered = this.cachedExpedientes;
        
        if (filtro) {
            filtered = this.cachedExpedientes.filter(exp => exp.estado === filtro);
        }
        
        this.renderTable(this.fullTable, filtered);
    }

    async search(term) {
        this.fullTable.innerHTML = `<table><tbody>${UI.getSkeletonTable(3, 5)}</tbody></table>`;
        if (this.recentTable) this.recentTable.innerHTML = `<table><tbody>${UI.getSkeletonTable(3, 5)}</tbody></table>`;
        
        try {
            // Detectar si es una cédula (empieza con V- o E- o es solo números)
            const esCedula = /^([VE]-?\d+|\d+)$/.test(term.trim());
            
            let expedientes;
            if (esCedula) {
                // Búsqueda por cédula exacta
                try {
                    const exp = await ApiClient.getExpedientePorCedula(term.trim());
                    expedientes = exp ? [exp] : [];
                } catch (error) {
                    expedientes = [];
                }
            } else {
                // Búsqueda general
                expedientes = await ApiClient.searchExpedientes(term);
            }
            
            this.renderTable(this.fullTable, expedientes);
            if (this.recentTable) this.renderTable(this.recentTable, expedientes.slice(0,5));
        } catch (error) {
            UI.showToast('Error en la búsqueda', 'error');
        }
    }

    async advancedSearch(params) {
        this.fullTable.innerHTML = `<table><tbody>${UI.getSkeletonTable(5, 5)}</tbody></table>`;
        
        try {
            // Filtrar parámetros vacíos
            const filteredParams = Object.fromEntries(
                Object.entries(params).filter(([_, v]) => v !== '' && v !== null)
            );
            
            if (Object.keys(filteredParams).length === 0) {
                UI.showToast('Ingresa al menos un criterio de búsqueda', 'warning');
                this.loadAll();
                return;
            }

            const expedientes = await ApiClient.buscarAvanzadaExpedientes(filteredParams);
            this.renderTable(this.fullTable, expedientes);
            UI.showToast(`Se encontraron ${expedientes.length} resultados`, 'success');
        } catch (error) {
            UI.showToast('Error en la búsqueda avanzada', 'error');
        }
    }

    async updateDashboardStats(expedientes) {
        const els = {
            total: document.getElementById('stat-expedientes'),
            activos: document.getElementById('stat-activos'),
            revision: document.getElementById('stat-revision'),
            documentos: document.getElementById('stat-documentos'),
            inactivos: document.getElementById('stat-inactivos'),
            usuarios: document.getElementById('stat-usuarios')
        };
        
        if (els.total) els.total.textContent = expedientes.length;
        if (els.activos) els.activos.textContent = expedientes.filter(e => e.estado === 'Activo').length;
        if (els.revision) els.revision.textContent = expedientes.filter(e => e.estado === 'EnRevision').length;
        if (els.inactivos) els.inactivos.textContent = expedientes.filter(e => e.estado === 'Inactivo').length;
        
        // Calcular total de documentos
        let totalDocumentos = 0;
        for (const exp of expedientes) {
            totalDocumentos += exp.documentos_count || 0;
        }
        if (els.documentos) els.documentos.textContent = totalDocumentos;
        
        // Cargar usuarios
        try {
            const usuarios = await ApiClient.getUsuarios();
            if (els.usuarios) els.usuarios.textContent = usuarios.length;
        } catch (e) {
            if (els.usuarios) els.usuarios.textContent = '--';
        }
        
        if (this.recentTable) {
            this.renderTable(this.recentTable, expedientes.slice(0, 5));
        }
    }

    renderTable(container, expedientes) {
        if (!expedientes || expedientes.length === 0) {
            container.innerHTML = '<div class="empty-state"><i data-lucide="inbox"></i><p>No hay expedientes registrados</p></div>';
            if (window.lucide) lucide.createIcons();
            return;
        }

        let html = `
            <table>
                <thead>
                    <tr>
                        <th>Cédula</th>
                        <th>Nombres y Apellidos</th>
                        <th>Fecha de Registro</th>
                        <th>Estado</th>
                        <th>Acciones</th>
                    </tr>
                </thead>
                <tbody>
        `;
        
        expedientes.forEach(exp => {
            const date = new Date(exp.creado_en).toLocaleDateString('es-ES');
            let badgeClass = 'warning';
            if (exp.estado === 'Activo') badgeClass = 'success';
            if (exp.estado === 'Inactivo') badgeClass = 'danger';
            
            html += `
                <tr>
                    <td><strong>${exp.cedula}</strong></td>
                    <td>${exp.nombres} ${exp.apellidos}</td>
                    <td>${date}</td>
                    <td><span class="badge ${badgeClass}">${exp.estado}</span></td>
                    <td>
                        <button class="btn-icon btn-view" data-id="${exp.id}" title="Ver Detalles"><i data-lucide="eye"></i></button>
                    </td>
                </tr>
            `;
        });
        
        html += `</tbody></table>`;
        container.innerHTML = html;
        if (window.lucide) lucide.createIcons();

        // eventos de los botones
        container.querySelectorAll('.btn-view').forEach(btn => {
            btn.addEventListener('click', () => this.openDetailsModal(btn.dataset.id));
        });
    }

    async openDetailsModal(id) {
        UI.showModal('modal-details');
        
        // limpiar pestañas
        document.querySelectorAll('#modal-details .tab-btn').forEach(b => b.classList.remove('active'));
        document.querySelectorAll('#modal-details .tab-content').forEach(c => c.classList.remove('active'));
        document.querySelector('[data-target="tab-info"]').classList.add('active');
        document.getElementById('tab-info').classList.add('active');

        // placeholder mientras carga
        document.getElementById('det-nombres').textContent = 'Cargando...';
        document.getElementById('det-cedula').textContent = '...';
        
        try {
            const exp = await ApiClient.getExpedienteDetails(id);
            // el endpoint devuelve el objeto directo
            document.getElementById('det-nombres').textContent = `${exp.nombres} ${exp.apellidos}`;
            document.getElementById('det-cedula').textContent = exp.cedula;
            document.getElementById('det-email').textContent = exp.email;
            document.getElementById('det-estado').value = exp.estado;
            
            // vincular docs al expediente
            this.docManager.setExpediente(exp.id);

            // clonar select para evitar listeners duplicados
            const selEstado = document.getElementById('det-estado');
            const newSel = selEstado.cloneNode(true);
            selEstado.parentNode.replaceChild(newSel, selEstado);
            
            newSel.addEventListener('change', async (e) => {
                try {
                    await ApiClient.updateEstadoExpediente(exp.id, e.target.value);
                    UI.showToast('Estado actualizado', 'success');
                    this.loadAll(); // reload table behind modal
                } catch (error) {
                    UI.showToast(error.message, 'error');
                }
            });

        } catch (error) {
            UI.showToast('Error cargando detalles', 'error');
        }
    }

    setupModals() {
        UI.initTabs();
    }
}
