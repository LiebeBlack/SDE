export class UI {
    static showToast(message, type = 'success') {
        const container = document.getElementById('toast-container');
        const toast = document.createElement('div');
        toast.className = `toast ${type}`;
        
        const icon = type === 'success' ? 'check-circle' : 'alert-circle';
        toast.innerHTML = `
            <i data-lucide="${icon}"></i>
            <span>${message}</span>
        `;
        
        container.appendChild(toast);
        if (window.lucide) lucide.createIcons();
        
        setTimeout(() => {
            toast.style.opacity = '0';
            toast.style.transform = 'translateX(100%)';
            setTimeout(() => toast.remove(), 300);
        }, 3000);
    }

    static getSkeletonRow(cols = 5) {
        let colsHtml = '';
        for(let i=0; i<cols; i++) {
            colsHtml += `<td><div class="skeleton skeleton-text"></div></td>`;
        }
        return `<tr>${colsHtml}</tr>`;
    }

    static getSkeletonTable(rows = 3, cols = 5) {
        let html = '';
        for(let i=0; i<rows; i++) {
            html += this.getSkeletonRow(cols);
        }
        return html;
    }

    static showModal(modalId) {
        const modal = document.getElementById(modalId);
        if (modal) {
            modal.classList.remove('hidden');
        }
    }

    static hideModal(modalId) {
        const modal = document.getElementById(modalId);
        if (modal) {
            modal.classList.add('hidden');
        }
    }

    static initTabs() {
        document.querySelectorAll('.tab-btn').forEach(btn => {
            btn.addEventListener('click', () => {
                const target = btn.dataset.target;
                const parent = btn.closest('.modal') || document;
                
                // marcar activa
                parent.querySelectorAll('.tab-btn').forEach(b => b.classList.remove('active'));
                btn.classList.add('active');
                
                // mostrar panel
                parent.querySelectorAll('.tab-content').forEach(c => c.classList.remove('active'));
                const content = parent.querySelector(`#${target}`);
                if (content) content.classList.add('active');
            });
        });
    }

    static getOfflineState(message) {
        return `
            <div class="empty-state offline-state" style="text-align: center; padding: 3rem;">
                <svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="color: var(--danger); margin-bottom: 1rem;"><path d="M2 2 22 22"/><path d="M8.5 16.5a5 5 0 0 1 7 0"/><path d="M2 8.82a15 15 0 0 1 4.17-2.65"/><path d="M10.66 5c4.01-.36 8.14.9 11.34 3.82"/></svg>
                <h3 style="margin-bottom: 0.5rem; font-size: 1.2rem;">Sin Conexión al Servidor</h3>
                <p style="color: var(--text-secondary); margin-bottom: 1.5rem;">${message}</p>
                <button class="btn-secondary" onclick="window.location.reload()">
                    <i data-lucide="refresh-cw"></i> Reintentar
                </button>
            </div>
        `;
    }
}
