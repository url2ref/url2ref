/**
 * Modal handling module for url2ref
 * Includes: source selection modal, additional field modal
 */

/**
 * ModalManager class to handle source selection and additional field modals
 */
class ModalManager {
    constructor(elements) {
        this.modalOverlay = elements.modalOverlay;
        this.modalBody = elements.modalBody;
        this.modalFieldName = elements.modalFieldName;
        this.modalCustomInput = elements.modalCustomInput;
        this.modalCloseBtn = elements.modalCloseBtn;
        this.modalCustomApply = elements.modalCustomApply;
        
        this.currentModalField = null;
        this.callbacks = {};
        
        this._setupEventListeners();
    }
    
    /**
     * Set callbacks for modal actions
     */
    setCallbacks(callbacks) {
        this.callbacks = callbacks;
    }
    
    /**
     * Setup event listeners for modal controls
     */
    _setupEventListeners() {
        if (this.modalCloseBtn) {
            this.modalCloseBtn.addEventListener('click', () => this.close());
        }
        
        if (this.modalOverlay) {
            this.modalOverlay.addEventListener('click', (e) => {
                if (e.target === this.modalOverlay) {
                    this.close();
                }
            });
        }
        
        if (this.modalCustomApply) {
            this.modalCustomApply.addEventListener('click', () => this._applyCustomValue());
        }
        
        if (this.modalCustomInput) {
            this.modalCustomInput.addEventListener('keypress', (e) => {
                if (e.key === 'Enter') {
                    this._applyCustomValue();
                }
            });
        }
        
        // Close modal on Escape key
        document.addEventListener('keydown', (e) => {
            if (e.key === 'Escape' && this.modalOverlay && !this.modalOverlay.classList.contains('d-none')) {
                this.close();
            }
        });
    }
    
    /**
     * Open the source selection modal for a field
     */
    openSourceModal(fieldKey, fieldLabel, multiSource, selections, customValues) {
        const sourceLabels = window.url2refMetadata?.SOURCE_LABELS || {
            'opengraph': 'Open Graph',
            'schemaorg': 'Schema.org',
            'htmlmeta': 'HTML Meta',
            'doi': 'DOI'
        };
        const sources = window.url2refMetadata?.SOURCES || ['opengraph', 'schemaorg', 'htmlmeta', 'doi'];
        
        this.currentModalField = fieldKey;
        this.modalFieldName.textContent = fieldLabel;
        
        // Clear previous content
        this.modalBody.innerHTML = '';
        this.modalCustomInput.value = customValues[fieldKey] || '';
        
        const fieldData = multiSource ? multiSource[fieldKey] : null;
        const currentSelection = selections ? selections[fieldKey] : null;
        
        // Add source options
        sources.forEach(source => {
            const value = fieldData ? fieldData[source] : null;
            
            const option = document.createElement('div');
            option.classList.add('source-option');
            option.setAttribute('data-source', source);
            
            if (currentSelection === source) {
                option.classList.add('selected');
            }
            
            if (!value) {
                option.classList.add('empty-option');
            }
            
            const header = document.createElement('div');
            header.classList.add('source-option-header');
            
            const name = document.createElement('span');
            name.classList.add('source-option-name');
            name.textContent = sourceLabels[source];
            header.appendChild(name);
            
            if (currentSelection === source) {
                const selectedBadge = document.createElement('span');
                selectedBadge.classList.add('source-option-selected');
                selectedBadge.innerHTML = '<i class="bi bi-check-circle-fill me-1"></i>Selected';
                header.appendChild(selectedBadge);
            }
            
            option.appendChild(header);
            
            const valueEl = document.createElement('div');
            valueEl.classList.add('source-option-value');
            if (value) {
                valueEl.textContent = value;
            } else {
                valueEl.textContent = 'No value available';
                valueEl.classList.add('empty');
            }
            option.appendChild(valueEl);
            
            // Make clickable if has value
            if (value) {
                option.addEventListener('click', () => {
                    if (this.callbacks.onSelectSource) {
                        this.callbacks.onSelectSource(fieldKey, source);
                    }
                });
            }
            
            this.modalBody.appendChild(option);
        });
        
        // Add custom option if it exists
        if (customValues[fieldKey]) {
            const customOption = document.createElement('div');
            customOption.classList.add('source-option');
            customOption.setAttribute('data-source', 'custom');
            
            if (currentSelection === 'custom') {
                customOption.classList.add('selected');
            }
            
            const header = document.createElement('div');
            header.classList.add('source-option-header');
            
            const name = document.createElement('span');
            name.classList.add('source-option-name');
            name.textContent = 'Custom';
            header.appendChild(name);
            
            if (currentSelection === 'custom') {
                const selectedBadge = document.createElement('span');
                selectedBadge.classList.add('source-option-selected');
                selectedBadge.innerHTML = '<i class="bi bi-check-circle-fill me-1"></i>Selected';
                header.appendChild(selectedBadge);
            }
            
            customOption.appendChild(header);
            
            const valueEl = document.createElement('div');
            valueEl.classList.add('source-option-value');
            valueEl.textContent = customValues[fieldKey];
            customOption.appendChild(valueEl);
            
            customOption.addEventListener('click', () => {
                if (this.callbacks.onSelectSource) {
                    this.callbacks.onSelectSource(fieldKey, 'custom');
                }
            });
            
            this.modalBody.appendChild(customOption);
        }
        
        // Show modal with animation
        this._show();
    }
    
    /**
     * Open modal for editing additional fields (translated title, archive date, etc.)
     */
    openAdditionalFieldModal(fieldKey, fieldLabel, originalValue, customValues) {
        this.currentModalField = fieldKey;
        this.modalFieldName.textContent = fieldLabel;
        
        // Clear previous content
        this.modalBody.innerHTML = '';
        this.modalCustomInput.value = customValues[fieldKey] || '';
        
        // Show the original auto-detected value if available
        if (originalValue) {
            const originalOption = document.createElement('div');
            originalOption.classList.add('source-option');
            originalOption.setAttribute('data-source', 'original');
            
            const isOriginalSelected = !customValues[fieldKey];
            if (isOriginalSelected) {
                originalOption.classList.add('selected');
            }
            
            const header = document.createElement('div');
            header.classList.add('source-option-header');
            
            const name = document.createElement('span');
            name.classList.add('source-option-name');
            name.textContent = 'Auto-detected';
            header.appendChild(name);
            
            if (isOriginalSelected) {
                const selectedBadge = document.createElement('span');
                selectedBadge.classList.add('source-option-selected');
                selectedBadge.innerHTML = '<i class="bi bi-check-circle-fill me-1"></i>Selected';
                header.appendChild(selectedBadge);
            }
            
            originalOption.appendChild(header);
            
            const valueEl = document.createElement('div');
            valueEl.classList.add('source-option-value');
            valueEl.textContent = originalValue;
            originalOption.appendChild(valueEl);
            
            originalOption.addEventListener('click', () => {
                if (this.callbacks.onRevertToOriginal) {
                    this.callbacks.onRevertToOriginal(fieldKey, originalValue);
                }
                this.close();
            });
            
            this.modalBody.appendChild(originalOption);
        }
        
        // Show custom option if it exists
        if (customValues[fieldKey]) {
            const customOption = document.createElement('div');
            customOption.classList.add('source-option', 'selected');
            customOption.setAttribute('data-source', 'custom');
            
            const header = document.createElement('div');
            header.classList.add('source-option-header');
            
            const name = document.createElement('span');
            name.classList.add('source-option-name');
            name.textContent = 'Custom';
            header.appendChild(name);
            
            const selectedBadge = document.createElement('span');
            selectedBadge.classList.add('source-option-selected');
            selectedBadge.innerHTML = '<i class="bi bi-check-circle-fill me-1"></i>Selected';
            header.appendChild(selectedBadge);
            
            customOption.appendChild(header);
            
            const valueEl = document.createElement('div');
            valueEl.classList.add('source-option-value');
            valueEl.textContent = customValues[fieldKey];
            customOption.appendChild(valueEl);
            
            this.modalBody.appendChild(customOption);
        }
        
        // If no options at all, show a message
        if (!originalValue && !customValues[fieldKey]) {
            const emptyMsg = document.createElement('div');
            emptyMsg.classList.add('source-option', 'empty-option');
            emptyMsg.innerHTML = '<div class="source-option-value empty">No value detected. Enter a custom value below.</div>';
            this.modalBody.appendChild(emptyMsg);
        }
        
        // Show modal with animation
        this._show();
    }
    
    /**
     * Show the modal with animation
     */
    _show() {
        this.modalOverlay.classList.remove('d-none');
        // Trigger reflow for animation
        void this.modalOverlay.offsetWidth;
        this.modalOverlay.classList.add('show');
    }
    
    /**
     * Close the modal
     */
    close() {
        this.modalOverlay.classList.remove('show');
        setTimeout(() => {
            this.modalOverlay.classList.add('d-none');
            this.currentModalField = null;
        }, 250);
    }
    
    /**
     * Apply custom value from modal input
     */
    _applyCustomValue() {
        if (!this.currentModalField) return;
        
        const value = this.modalCustomInput.value.trim();
        if (!value) return;
        
        if (this.callbacks.onApplyCustomValue) {
            this.callbacks.onApplyCustomValue(this.currentModalField, value);
        }
    }
    
    /**
     * Get the current field being edited
     */
    getCurrentField() {
        return this.currentModalField;
    }
}

// Export for use in other modules
window.url2refModal = {
    ModalManager
};
