// Configuración de la API
const API_BASE_URL = window.location.origin;

// Estado de la aplicación
let currentSection = 'dashboard';
let searchTimeout = null;
let selectedFiles = [];

// Elementos del DOM
const navTabs = document.querySelectorAll('.nav-tab');
const sections = document.querySelectorAll('.section');
const searchInput = document.getElementById('search-input');
const searchType = document.getElementById('search-type');
const searchResults = document.getElementById('search-results');
const searchSpinner = document.getElementById('search-spinner');
const expedienteForm = document.getElementById('expediente-form');
const dropZone = document.getElementById('drop-zone');
const fileInput = document.getElementById('file-input');
const selectedFilesContainer = document.getElementById('selected-files');
const uploadBtn = document.getElementById('upload-btn');

// Inicializar la aplicación
document.addEventListener('DOMContentLoaded', () => {
    initializeNavigation();
    initializeSearch();
    initializeForm();
    initializeFileUpload();
    loadMetrics();
});

// Navegación
function initializeNavigation() {
    navTabs.forEach(tab => {
        tab.addEventListener('click', () => {
            const sectionId = tab.dataset.section;
            switchSection(sectionId);
        });
    });
}

function switchSection(sectionId) {
    // Actualizar pestañas
    navTabs.forEach(tab => {
        tab.classList.remove('active');
        if (tab.dataset.section === sectionId) {
            tab.classList.add('active');
        }
    });

    // Actualizar secciones
    sections.forEach(section => {
        section.classList.remove('active');
        if (section.id === sectionId) {
            section.classList.add('active');
        }
    });

    currentSection = sectionId;
}

// Búsqueda
function initializeSearch() {
    searchInput.addEventListener('input', handleSearchInput);
    searchType.addEventListener('change', handleSearchInput);
}

function handleSearchInput() {
    clearTimeout(searchTimeout);
    const searchTerm = searchInput.value.trim();
    
    if (searchTerm.length < 2) {
        showEmptyState();
        return;
    }

    searchSpinner.classList.add('active');
    
    searchTimeout = setTimeout(() => {
        performSearch(searchTerm);
    }, 300);
}

async function performSearch(searchTerm) {
    try {
        const searchTypeValue = searchType.value;
        let url = `${API_BASE_URL}/api/buscar?termino=${encodeURIComponent(searchTerm)}&page=1&page_size=10`;
        
        if (searchTypeValue !== 'general') {
            url = `${API_BASE_URL}/api/expedientes/buscar?${searchTypeValue}=${encodeURIComponent(searchTerm)}&page=1&page_size=10`;
        }

        const response = await fetch(url);
        const data = await response.json();

        if (response.ok) {
            displaySearchResults(data.data);
        } else {
            showToast('Error en la búsqueda', 'error');
            showEmptyState();
        }
    } catch (error) {
        console.error('Search error:', error);
        showToast('Error de conexión', 'error');
        showEmptyState();
    } finally {
        searchSpinner.classList.remove('active');
    }
}

function displaySearchResults(results) {
    if (results.length === 0) {
        searchResults.innerHTML = `
            <div class="empty-state">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <circle cx="11" cy="11" r="8"/>
                    <line x1="21" y1="21" x2="16.65" y2="16.65"/>
                </svg>
                <p>No se encontraron resultados</p>
            </div>
        `;
        return;
    }

    const tableHTML = `
        <table class="results-table">
            <thead>
                <tr>
                    <th>Cédula</th>
                    <th>Nombres</th>
                    <th>Apellidos</th>
                    <th>Email</th>
                    <th>Estado</th>
                    <th>Documentos</th>
                </tr>
            </thead>
            <tbody>
                ${results.map(result => `
                    <tr>
                        <td>${result.cedula}</td>
                        <td>${result.nombres}</td>
                        <td>${result.apellidos}</td>
                        <td>${result.email}</td>
                        <td><span class="status-badge status-${result.estado.toLowerCase()}">${result.estado}</span></td>
                        <td>${result.documentos_count}</td>
                    </tr>
                `).join('')}
            </tbody>
        </table>
    `;

    searchResults.innerHTML = tableHTML;
}

function showEmptyState() {
    searchResults.innerHTML = `
        <div class="empty-state">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <circle cx="11" cy="11" r="8"/>
                <line x1="21" y1="21" x2="16.65" y2="16.65"/>
            </svg>
            <p>Ingrese un término para buscar expedientes</p>
        </div>
    `;
}

// Manejo de formularios
function initializeForm() {
    expedienteForm.addEventListener('submit', handleFormSubmit);
    expedienteForm.addEventListener('reset', handleFormReset);
}

async function handleFormSubmit(event) {
    event.preventDefault();
    
    const formData = new FormData(expedienteForm);
    const data = Object.fromEntries(formData.entries());

    try {
        const response = await fetch(`${API_BASE_URL}/expedientes`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(data),
        });

        if (response.ok) {
            const result = await response.json();
            showToast('Expediente creado exitosamente', 'success');
            expedienteForm.reset();
            loadMetrics();
        } else {
            const error = await response.json();
            showToast(error.error || 'Error al crear expediente', 'error');
        }
    } catch (error) {
        console.error('Form submission error:', error);
        showToast('Error de conexión', 'error');
    }
}

function handleFormReset() {
    showToast('Formulario limpiado', 'warning');
}

// Subida de archivos
function initializeFileUpload() {
    dropZone.addEventListener('click', () => fileInput.click());
    dropZone.addEventListener('dragover', handleDragOver);
    dropZone.addEventListener('dragleave', handleDragLeave);
    dropZone.addEventListener('drop', handleDrop);
    fileInput.addEventListener('change', handleFileSelect);
    uploadBtn.addEventListener('click', handleFileUpload);
}

function handleDragOver(event) {
    event.preventDefault();
    dropZone.classList.add('drag-over');
}

function handleDragLeave(event) {
    event.preventDefault();
    dropZone.classList.remove('drag-over');
}

function handleDrop(event) {
    event.preventDefault();
    dropZone.classList.remove('drag-over');
    
    const files = Array.from(event.dataTransfer.files);
    addFiles(files);
}

function handleFileSelect(event) {
    const files = Array.from(event.target.files);
    addFiles(files);
}

function addFiles(files) {
    const validFiles = files.filter(file => {
        const validTypes = ['application/pdf', 'image/jpeg', 'image/png', 'image/jpg'];
        const maxSize = 10 * 1024 * 1024; // 10MB
        
        if (!validTypes.includes(file.type)) {
            showToast(`Tipo de archivo no válido: ${file.name}`, 'error');
            return false;
        }
        
        if (file.size > maxSize) {
            showToast(`Archivo demasiado grande: ${file.name}`, 'error');
            return false;
        }
        
        return true;
    });

    selectedFiles = [...selectedFiles, ...validFiles];
    updateSelectedFilesDisplay();
}

function updateSelectedFilesDisplay() {
    selectedFilesContainer.innerHTML = selectedFiles.map((file, index) => `
        <div class="file-item">
            <span>${file.name}</span>
            <button type="button" class="remove-file" onclick="removeFile(${index})">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <line x1="18" y1="6" x2="6" y2="18"/>
                    <line x1="6" y1="6" x2="18" y2="18"/>
                </svg>
            </button>
        </div>
    `).join('');

    uploadBtn.disabled = selectedFiles.length === 0;
}

function removeFile(index) {
    selectedFiles.splice(index, 1);
    updateSelectedFilesDisplay();
}

async function handleFileUpload() {
    const expedienteId = document.getElementById('expediente-id').value.trim();
    const nombreArchivo = document.getElementById('nombre-archivo').value.trim();
    const categoria = document.getElementById('categoria').value;
    const observaciones = document.getElementById('observaciones').value.trim();

    if (!expedienteId || !nombreArchivo || !categoria) {
        showToast('Complete todos los campos requeridos', 'warning');
        return;
    }

    if (selectedFiles.length === 0) {
        showToast('Seleccione al menos un archivo', 'warning');
        return;
    }

    uploadBtn.disabled = true;
    uploadBtn.textContent = 'Subiendo...';

    try {
        for (const file of selectedFiles) {
            const formData = new FormData();
            formData.append('nombre_archivo', nombreArchivo);
            formData.append('categoria', categoria);
            formData.append('archivo', file);
            if (observaciones) {
                formData.append('observaciones', observaciones);
            }

            const response = await fetch(`${API_BASE_URL}/api/expedientes/${expedienteId}/documento`, {
                method: 'POST',
                body: formData,
            });

            if (!response.ok) {
                const error = await response.json();
                throw new Error(error.error || 'Error al subir archivo');
            }
        }

        showToast('Archivos subidos exitosamente', 'success');
        
        // Limpiar formulario
        document.getElementById('expediente-id').value = '';
        document.getElementById('nombre-archivo').value = '';
        document.getElementById('categoria').value = '';
        document.getElementById('observaciones').value = '';
        selectedFiles = [];
        updateSelectedFilesDisplay();
        loadMetrics();
    } catch (error) {
        console.error('File upload error:', error);
        showToast(error.message || 'Error al subir archivos', 'error');
    } finally {
        uploadBtn.disabled = false;
        uploadBtn.innerHTML = `
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
                <polyline points="17 8 12 3 7 8"/>
                <line x1="12" y1="3" x2="12" y2="15"/>
            </svg>
            Subir Archivos
        `;
    }
}

// Métricas del dashboard
async function loadMetrics() {
    try {
        // Contar expedientes
        const expedientesResponse = await fetch(`${API_BASE_URL}/expedientes`);
        if (expedientesResponse.ok) {
            const expedientes = await expedientesResponse.json();
            document.getElementById('total-expedientes').textContent = expedientes.length;
        }

        // Contar documentos (vía búsqueda)
        const documentosResponse = await fetch(`${API_BASE_URL}/api/documentos/buscar?page=1&page_size=1`);
        if (documentosResponse.ok) {
            const data = await documentosResponse.json();
            document.getElementById('total-documentos').textContent = data.total;
        }

        // Cargar registros de auditoría (ya existe el endpoint en /api/auditoria)
        document.getElementById('total-auditoria').textContent = '0';
    } catch (error) {
        console.error('Error loading metrics:', error);
    }
}

// Notificaciones tipo toast
function showToast(message, type = 'info') {
    const container = document.getElementById('toast-container');
    const toast = document.createElement('div');
    toast.className = `toast ${type}`;
    
    const icons = {
        success: '<svg class="toast-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="20 6 9 17 4 12"/></svg>',
        error: '<svg class="toast-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><line x1="15" y1="9" x2="9" y2="15"/><line x1="9" y1="9" x2="15" y2="15"/></svg>',
        warning: '<svg class="toast-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/><line x1="12" y1="9" x2="12" y2="13"/><line x1="12" y1="17" x2="12.01" y2="17"/></svg>',
        info: '<svg class="toast-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/><line x1="12" y1="16" x2="12" y2="12"/><line x1="12" y1="8" x2="12.01" y2="8"/></svg>'
    };

    toast.innerHTML = `
        ${icons[type] || icons.info}
        <span class="toast-message">${message}</span>
        <button class="toast-close" onclick="this.parentElement.remove()">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <line x1="18" y1="6" x2="6" y2="18"/>
                <line x1="6" y1="6" x2="18" y2="18"/>
            </svg>
        </button>
    `;

    container.appendChild(toast);

    // Se elimina solo después de 5 segundos
    setTimeout(() => {
        toast.style.animation = 'slideIn 0.3s ease reverse';
        setTimeout(() => toast.remove(), 300);
    }, 5000);
}

// Funciones auxiliares
function formatDate(dateString) {
    const date = new Date(dateString);
    return date.toLocaleDateString('es-ES', {
        year: 'numeric',
        month: 'long',
        day: 'numeric'
    });
}

function formatFileSize(bytes) {
    if (bytes === 0) return '0 Bytes';
    const k = 1024;
    const sizes = ['Bytes', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return Math.round(bytes / Math.pow(k, i) * 100) / 100 + ' ' + sizes[i];
}

// Manejo de errores global
window.addEventListener('error', (event) => {
    console.error('Global error:', event.error);
    showToast('Error inesperado en la aplicación', 'error');
});

// Estado de la red
window.addEventListener('online', () => {
    showToast('Conexión restablecida', 'success');
});

window.addEventListener('offline', () => {
    showToast('Sin conexión a internet', 'warning');
});
