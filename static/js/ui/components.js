export class UI {
    static showToast(message, type = 'success') {
        const container = document.getElementById('toast-container');
        if (!container) return;
        
        const toast = document.createElement('div');
        toast.className = `toast ${type}`;
        
        const icon = type === 'success' ? 'check-circle' : 
                     type === 'error' ? 'alert-circle' : 
                     type === 'warning' ? 'alert-triangle' : 'info';
        
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
        }, 4000);
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
            document.body.style.overflow = 'hidden';
        }
    }

    static hideModal(modalId) {
        const modal = document.getElementById(modalId);
        if (modal) {
            modal.classList.add('hidden');
            document.body.style.overflow = '';
        }
    }

    static initTabs() {
        document.querySelectorAll('.tab-btn').forEach(btn => {
            btn.addEventListener('click', () => {
                const target = btn.dataset.target;
                const parent = btn.closest('.modal') || document;
                
                // marcar activa y actualizar ARIA
                parent.querySelectorAll('.tab-btn').forEach(b => {
                    b.classList.remove('active');
                    b.setAttribute('aria-selected', 'false');
                });
                btn.classList.add('active');
                btn.setAttribute('aria-selected', 'true');
                
                // mostrar panel y actualizar ARIA
                parent.querySelectorAll('.tab-content').forEach(c => {
                    c.classList.remove('active');
                    c.setAttribute('aria-hidden', 'true');
                });
                const content = parent.querySelector(`#${target}`);
                if (content) {
                    content.classList.add('active');
                    content.setAttribute('aria-hidden', 'false');
                }
            });
        });
    }

    static getOfflineState(message) {
        return `
            <div class="empty-state offline-state glass-panel" style="text-align: center; padding: 3rem; margin: 2rem; border-radius: var(--radius-lg); position: relative; overflow: hidden;">
                <div style="position: absolute; top: 50%; left: 50%; transform: translate(-50%, -50%); width: 100px; height: 100px; background: rgba(239, 68, 68, 0.2); filter: blur(30px); border-radius: 50%;"></div>
                <div style="position: relative; z-index: 1;">
                    <svg xmlns="http://www.w3.org/2000/svg" width="56" height="56" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="color: var(--danger); margin-bottom: 1.5rem; filter: drop-shadow(0 4px 6px rgba(239, 68, 68, 0.3));"><path d="M2 2 22 22"/><path d="M8.5 16.5a5 5 0 0 1 7 0"/><path d="M2 8.82a15 15 0 0 1 4.17-2.65"/><path d="M10.66 5c4.01-.36 8.14.9 11.34 3.82"/></svg>
                    <h3 style="margin-bottom: 0.75rem; font-size: 1.5rem; font-weight: 700; background: linear-gradient(135deg, var(--text-primary) 0%, var(--text-secondary) 100%); -webkit-background-clip: text; -webkit-text-fill-color: transparent;">Servidor Local Inaccesible</h3>
                    <p style="color: var(--text-secondary); margin-bottom: 2rem; max-width: 400px; margin-left: auto; margin-right: auto; line-height: 1.6;">${message || 'No se pudo conectar con el motor local del sistema.'}</p>
                    <button class="btn-primary" onclick="window.location.reload()" style="padding: 0.75rem 2rem; font-size: 1rem; border-radius: var(--radius-md); transition: transform 0.2s ease, box-shadow 0.2s ease;">
                        <i data-lucide="refresh-cw" style="width: 18px; height: 18px; margin-right: 8px;"></i> Reconectar
                    </button>
                </div>
            </div>
        `;
    }


    static setButtonLoading(btn, loading, originalText = '') {
        if (!btn) return;
        
        if (loading) {
            btn.dataset.originalText = btn.innerHTML;
            btn.disabled = true;
            btn.innerHTML = '<i data-lucide="loader-2" class="spin"></i> Procesando...';
            if (window.lucide) lucide.createIcons();
        } else {
            btn.disabled = false;
            btn.innerHTML = originalText || btn.dataset.originalText || btn.innerHTML;
            if (window.lucide) lucide.createIcons();
        }
    }

    static validateEmail(email) {
        const re = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
        return re.test(email);
    }

    static validateEmailWithFeedback(email, inputElement) {
        if (!email) {
            this.showFieldError(inputElement, 'El email es requerido');
            return false;
        }
        if (!this.validateEmail(email)) {
            this.showFieldError(inputElement, 'Ingresa un email válido (ej. usuario@correo.com)');
            return false;
        }
        this.clearFieldError(inputElement);
        inputElement.style.borderColor = 'var(--success)';
        return true;
    }

    static validateCedula(ced) {
        // Allow V-12345678, E-12345678, or just 12345678 for auto-detection
        return /^([VE]-)?\d{6,8}$/i.test(ced);
    }

    static validateCedulaWithFeedback(ced, inputElement) {
        if (!ced) {
            this.showFieldError(inputElement, 'La cédula es requerida');
            return false;
        }
        if (!this.validateCedula(ced)) {
            this.showFieldError(inputElement, 'Ingresa una cédula válida (ej. 12345678 o V-12345678)');
            return false;
        }
        this.clearFieldError(inputElement);
        inputElement.style.borderColor = 'var(--success)';
        return true;
    }

    static validatePassword(password) {
        // Mínimo 8 caracteres, al menos una letra y un número
        if (!password || password.length < 8) {
            return false;
        }
        const hasLetter = /[a-zA-Z]/.test(password);
        const hasNumber = /\d/.test(password);
        return hasLetter && hasNumber;
    }

    static validatePasswordWithFeedback(password, inputElement) {
        if (!password) {
            this.showFieldError(inputElement, 'La contraseña es requerida');
            return false;
        }
        if (password.length < 8) {
            this.showFieldError(inputElement, 'La contraseña debe tener al menos 8 caracteres');
            return false;
        }
        const hasLetter = /[a-zA-Z]/.test(password);
        const hasNumber = /\d/.test(password);
        if (!hasLetter || !hasNumber) {
            this.showFieldError(inputElement, 'La contraseña debe tener al menos una letra y un número');
            return false;
        }
        this.clearFieldError(inputElement);
        inputElement.style.borderColor = 'var(--success)';
        return true;
    }

    static validateTelefono(telefono) {
        if (!telefono) return true; // Opcional
        const telefonoRegex = /^\d{10,15}$/;
        return telefonoRegex.test(telefono.replace(/[\s\-\(\)]/g, ''));
    }

    static validateRequired(value) {
        return value !== null && value !== undefined && value.toString().trim() !== '';
    }

    static showFieldError(input, message) {
        input.style.borderColor = 'var(--danger)';
        
        let errorDiv = input.parentElement.querySelector('.field-error');
        if (!errorDiv) {
            errorDiv = document.createElement('div');
            errorDiv.className = 'field-error';
            errorDiv.style.color = 'var(--danger)';
            errorDiv.style.fontSize = '0.75rem';
            errorDiv.style.marginTop = '0.25rem';
            input.parentElement.appendChild(errorDiv);
        }
        errorDiv.textContent = message;
    }

    static clearFieldError(input) {
        input.style.borderColor = '';
        const errorDiv = input.parentElement.querySelector('.field-error');
        if (errorDiv) errorDiv.remove();
    }

    static validateForm(form) {
        const errors = [];
        const inputs = form.querySelectorAll('input[required], select[required]');
        
        inputs.forEach(input => {
            this.clearFieldError(input);
            
            if (!this.validateRequired(input.value)) {
                this.showFieldError(input, 'Este campo es requerido');
                errors.push(input);
                return;
            }

            if (input.type === 'email' && !this.validateEmail(input.value)) {
                this.showFieldError(input, 'Email inválido');
                errors.push(input);
                return;
            }

            if (input.name === 'cedula' && !this.validateCedula(input.value)) {
                this.showFieldError(input, 'Cédula inválida (formato: V-12345678 o E-12345678)');
                errors.push(input);
                return;
            }

            if (input.type === 'password' && input.value && !this.validatePassword(input.value)) {
                this.showFieldError(input, 'La contraseña debe tener al menos 8 caracteres, una letra y un número');
                errors.push(input);
                return;
            }

            if (input.name === 'telefono' && input.value && !this.validateTelefono(input.value)) {
                this.showFieldError(input, 'Teléfono inválido (10-15 dígitos)');
                errors.push(input);
                return;
            }
        });

        return errors.length === 0;
    }
}
