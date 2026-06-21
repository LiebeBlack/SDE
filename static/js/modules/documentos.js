import { ApiClient } from '../core/api.js';
import { UI } from '../ui/components.js';

export class DocumentosManager {
    constructor() {
        this.currentExpedienteId = null;
        this.dropZone = document.getElementById('drop-zone');
        this.fileInput = document.getElementById('file-input');
        this.docsContainer = document.getElementById('documentos-list');
        
        if (this.dropZone) {
            this.setupDragAndDrop();
        }
    }

    setExpediente(id) {
        this.currentExpedienteId = id;
        this.loadDocumentos();
    }

    setupDragAndDrop() {
        ['dragenter', 'dragover', 'dragleave', 'drop'].forEach(eventName => {
            this.dropZone.addEventListener(eventName, preventDefaults, false);
        });

        function preventDefaults(e) {
            e.preventDefault();
            e.stopPropagation();
        }

        ['dragenter', 'dragover'].forEach(eventName => {
            this.dropZone.addEventListener(eventName, () => {
                this.dropZone.classList.add('drag-active');
            }, false);
        });

        ['dragleave', 'drop'].forEach(eventName => {
            this.dropZone.addEventListener(eventName, () => {
                this.dropZone.classList.remove('drag-active');
            }, false);
        });

        this.dropZone.addEventListener('drop', (e) => {
            let dt = e.dataTransfer;
            let files = dt.files;
            this.handleFiles(files);
        }, false);

        // click para subir
        this.dropZone.addEventListener('click', () => {
            this.fileInput.click();
        });

        this.fileInput.addEventListener('change', function() {
            this.handleFiles(this.files);
        }.bind(this));
    }

    async handleFiles(files) {
        if (!this.currentExpedienteId) {
            UI.showToast('Error: No hay expediente seleccionado', 'error');
            return;
        }

        const file = files[0];
        if (!file) return;

        // mostrar que está subiendo
        const loadingId = 'upload-' + Date.now();
        this.addLoadingCard(loadingId, file.name);

        const formData = new FormData();
        formData.append('archivo', file);
        formData.append('categoria', 'Documento General');
        formData.append('observaciones', 'Subido vía web');

        try {
            await ApiClient.uploadDocumento(this.currentExpedienteId, formData);
            UI.showToast('Documento subido correctamente', 'success');
            await this.loadDocumentos();
        } catch (error) {
            UI.showToast(error.message, 'error');
            // quitar spinner si falla
            const el = document.getElementById(loadingId);
            if (el) el.remove();
        }
        
        this.fileInput.value = '';
    }

    async loadDocumentos() {
        if (!this.currentExpedienteId) return;

        this.docsContainer.innerHTML = '<div class="loading-state"><i data-lucide="loader-2" class="spin"></i> Cargando...</div>';
        if (window.lucide) lucide.createIcons();

        try {
            const documentos = await ApiClient.getDocumentos(this.currentExpedienteId);
            this.renderDocumentos(documentos);
        } catch (error) {
            this.docsContainer.innerHTML = `<div class="error-state">Error: ${error.message}</div>`;
        }
    }

    renderDocumentos(docs) {
        if (!docs || docs.length === 0) {
            this.docsContainer.innerHTML = `
                <div class="empty-state">
                    <i data-lucide="file-x"></i>
                    <p>No hay documentos anexos</p>
                </div>`;
            if (window.lucide) lucide.createIcons();
            return;
        }

        let html = '';
        docs.forEach(doc => {
            const date = new Date(doc.creado_en).toLocaleDateString('es-ES');
            const size = doc.tamaño_bytes ? Math.round(doc.tamaño_bytes / 1024) + ' KB' : 'N/A';
            const foliadoBadge = doc.foliado ? 
                '<span class="badge success">Foliado</span>' : 
                '<span class="badge warning">Sin Foliar</span>';
            
            html += `
                <div class="doc-card glass-panel" data-doc-id="${doc.id}">
                    <div class="doc-icon bg-blue"><i data-lucide="file-text"></i></div>
                    <div class="doc-info">
                        <h4>${doc.nombre_archivo}</h4>
                        <p>${doc.categoria} • ${size} • ${date}</p>
                        <div class="doc-meta">${foliadoBadge}</div>
                    </div>
                    <div class="doc-actions">
                        <button class="btn-icon btn-download" data-doc-id="${doc.id}" title="Descargar"><i data-lucide="download"></i></button>
                        ${!doc.foliado ? `
                        <button class="btn-icon btn-foliar" data-doc-id="${doc.id}" title="Foliar Documento"><i data-lucide="check-circle"></i></button>
                        ` : ''}
                    </div>
                </div>
            `;
        });

        this.docsContainer.innerHTML = html;
        if (window.lucide) lucide.createIcons();

        // Agregar event listeners
        this.docsContainer.querySelectorAll('.btn-foliar').forEach(btn => {
            btn.addEventListener('click', () => this.foliarDocumento(btn.dataset.docId));
        });

        this.docsContainer.querySelectorAll('.btn-download').forEach(btn => {
            btn.addEventListener('click', () => this.descargarDocumento(btn.dataset.docId));
        });
    }

    async foliarDocumento(documentoId) {
        try {
            await ApiClient.foliarDocumento(this.currentExpedienteId, documentoId);
            UI.showToast('Documento foliado correctamente', 'success');
            await this.loadDocumentos();
        } catch (error) {
            UI.showToast(error.message, 'error');
        }
    }

    async descargarDocumento(documentoId) {
        try {
            const endpoint = `/expedientes/${this.currentExpedienteId}/documentos/${documentoId}/descargar`;
            await ApiClient.downloadFile(endpoint);
            UI.showToast('Documento descargado correctamente', 'success');
        } catch (error) {
            UI.showToast(error.message, 'error');
        }
    }

    addLoadingCard(id, filename) {
        const html = `
            <div id="${id}" class="doc-card glass-panel uploading">
                <div class="doc-icon"><i data-lucide="loader-2" class="spin"></i></div>
                <div class="doc-info">
                    <h4>${filename}</h4>
                    <p>Subiendo archivo...</p>
                </div>
            </div>
        `;
        if (this.docsContainer.querySelector('.empty-state')) {
            this.docsContainer.innerHTML = '';
        }
        this.docsContainer.insertAdjacentHTML('afterbegin', html);
        if (window.lucide) lucide.createIcons();
    }
}
