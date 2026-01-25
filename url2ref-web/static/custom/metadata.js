/**
 * Metadata display module for url2ref
 * Includes: multi-source table display, field handling, source selection modals
 */

// Source labels for display
const SOURCE_LABELS = {
    'opengraph': 'Open Graph',
    'schemaorg': 'Schema.org',
    'htmlmeta': 'HTML Meta',
    'doi': 'DOI',
    'ai': 'AI',
    'custom': 'Custom'
};

// Available source types
const SOURCES = ['opengraph', 'schemaorg', 'htmlmeta', 'doi', 'ai'];

// Field definitions for metadata section
const METADATA_FIELD_DEFS = [
    { key: 'title', label: 'Title' },
    { key: 'author', label: 'Author' },
    { key: 'date', label: 'Date' },
    { key: 'site', label: 'Site' },
    { key: 'publisher', label: 'Publisher' },
    { key: 'language', label: 'Language' },
    { key: 'url', label: 'URL' }
];

// All toggleable fields (metadata + additional)
const ALL_TOGGLEABLE_FIELDS = [
    'title', 'author', 'date', 'site', 'publisher', 'language', 'url',
    'translated_title', 'archive_url', 'archive_date'
];

/**
 * Check if the multi-source data has any actual values
 */
function hasAnyMultiSourceData(multiSource) {
    if (!multiSource) return false;
    const fields = ['title', 'author', 'date', 'site', 'publisher', 'language', 'url'];
    
    return fields.some(field => {
        const fieldData = multiSource[field];
        if (!fieldData) return false;
        return SOURCES.some(src => fieldData[src]);
    });
}

/**
 * Create a section header element
 */
function createSectionHeader(title) {
    const header = document.createElement('div');
    header.classList.add('section-header');
    header.textContent = title;
    return header;
}

/**
 * Get the count of available alternatives for a field
 */
function getAlternativesCount(multiSource, fieldKey) {
    const fieldData = multiSource[fieldKey];
    if (!fieldData) return 0;
    return SOURCES.filter(src => fieldData[src]).length;
}

/**
 * Create a compact field row for the multi-source table
 */
function createFieldRow(fieldDef, multiSource, selections, customValues, openSourceModal, disabledFields, onToggleField) {
    const truncateText = window.url2refUtils?.truncateText || ((t, l) => t.length <= l ? t : t.substring(0, l) + '...');
    
    const fieldData = multiSource[fieldDef.key];
    const selectedSource = selections ? selections[fieldDef.key] : null;
    const alternativesCount = getAlternativesCount(multiSource, fieldDef.key);
    const isDisabled = disabledFields && disabledFields[fieldDef.key];
    
    // Get the current value to display
    let displayValue = '';
    let currentSource = selectedSource;
    
    if (selectedSource === 'custom' && customValues[fieldDef.key]) {
        displayValue = customValues[fieldDef.key];
    } else if (selectedSource && fieldData && fieldData[selectedSource]) {
        displayValue = fieldData[selectedSource];
    }

    const row = document.createElement('div');
    row.classList.add('field-row');
    row.setAttribute('data-field', fieldDef.key);
    if (isDisabled) {
        row.classList.add('field-disabled');
    }

    // Field label with toggle
    const label = document.createElement('div');
    label.classList.add('field-label');
    
    // Create toggle switch
    const toggleWrapper = document.createElement('div');
    toggleWrapper.classList.add('field-toggle');
    toggleWrapper.title = isDisabled ? 'Enable this field' : 'Disable this field';
    
    const toggleInput = document.createElement('input');
    toggleInput.type = 'checkbox';
    toggleInput.checked = !isDisabled;
    toggleInput.classList.add('field-toggle-input');
    toggleInput.id = `toggle-${fieldDef.key}`;
    toggleInput.addEventListener('change', function(e) {
        e.stopPropagation();
        if (onToggleField) {
            onToggleField(fieldDef.key, !this.checked);
        }
    });
    
    const toggleSlider = document.createElement('label');
    toggleSlider.classList.add('field-toggle-slider');
    toggleSlider.setAttribute('for', `toggle-${fieldDef.key}`);
    
    toggleWrapper.appendChild(toggleInput);
    toggleWrapper.appendChild(toggleSlider);
    label.appendChild(toggleWrapper);
    
    // Label text
    const labelText = document.createElement('span');
    labelText.classList.add('field-label-text');
    labelText.textContent = fieldDef.label;
    label.appendChild(labelText);
    
    row.appendChild(label);

    // Value container
    const valueContainer = document.createElement('div');
    valueContainer.classList.add('field-value-container');
    valueContainer.setAttribute('data-field', fieldDef.key);
    
    // Value text
    const valueSpan = document.createElement('span');
    valueSpan.classList.add('field-value');
    if (displayValue) {
        valueSpan.textContent = truncateText(displayValue, 80);
        valueSpan.title = displayValue;
    } else {
        valueSpan.textContent = 'No value extracted';
        valueSpan.classList.add('empty-value');
    }
    valueContainer.appendChild(valueSpan);

    // Source badge (clickable if alternatives exist)
    if (currentSource || alternativesCount > 0) {
        const badge = document.createElement('span');
        badge.classList.add('source-badge');
        badge.setAttribute('data-field', fieldDef.key);
        
        if (currentSource === 'custom') {
            badge.classList.add('custom-badge');
            badge.textContent = 'Custom';
        } else if (currentSource === 'ai') {
            badge.classList.add('ai-badge');
            badge.textContent = 'AI';
        } else if (currentSource) {
            badge.textContent = SOURCE_LABELS[currentSource] || currentSource;
        } else {
            badge.textContent = 'Select source';
        }
        
        // Add alternatives count indicator
        const totalAlternatives = alternativesCount + (customValues[fieldDef.key] ? 1 : 0);
        if (totalAlternatives > 1) {
            const countBadge = document.createElement('span');
            countBadge.classList.add('alternatives-count');
            countBadge.textContent = totalAlternatives;
            countBadge.title = `${totalAlternatives} sources available`;
            badge.appendChild(countBadge);
        }
        
        // Add indicator if there are alternatives
        if (alternativesCount > 1 || (alternativesCount > 0 && currentSource !== 'custom')) {
            badge.classList.add('has-alternatives');
            badge.addEventListener('click', function(e) {
                e.stopPropagation();
                openSourceModal(fieldDef.key, fieldDef.label);
            });
        }
        
        valueContainer.appendChild(badge);
        
        // Make entire container clickable to open modal
        if (alternativesCount > 1 || alternativesCount > 0) {
            valueContainer.addEventListener('click', function() {
                openSourceModal(fieldDef.key, fieldDef.label);
            });
        } else {
            valueContainer.classList.add('no-alternatives');
        }
    } else {
        // No source data at all - allow custom entry
        const badge = document.createElement('span');
        badge.classList.add('source-badge', 'add-value-badge', 'has-alternatives');
        badge.innerHTML = '<i class="bi bi-plus-circle me-1"></i>Add value';
        badge.addEventListener('click', function(e) {
            e.stopPropagation();
            openSourceModal(fieldDef.key, fieldDef.label);
        });
        valueContainer.appendChild(badge);
        valueContainer.addEventListener('click', function() {
            openSourceModal(fieldDef.key, fieldDef.label);
        });
    }

    row.appendChild(valueContainer);
    return row;
}

/**
 * Create an editable field row (for additional values like translated title)
 */
function createEditableFieldRow(fieldDef, value, customValues, openAdditionalFieldModal, isEditable = true, disabledFields, onToggleField) {
    const truncateText = window.url2refUtils?.truncateText || ((t, l) => t.length <= l ? t : t.substring(0, l) + '...');
    const isDisabled = disabledFields && disabledFields[fieldDef.key];
    
    const row = document.createElement('div');
    row.classList.add('field-row');
    row.setAttribute('data-field', fieldDef.key);
    if (isDisabled) {
        row.classList.add('field-disabled');
    }

    // Field label with toggle
    const label = document.createElement('div');
    label.classList.add('field-label');
    
    // Create toggle switch
    const toggleWrapper = document.createElement('div');
    toggleWrapper.classList.add('field-toggle');
    toggleWrapper.title = isDisabled ? 'Enable this field' : 'Disable this field';
    
    const toggleInput = document.createElement('input');
    toggleInput.type = 'checkbox';
    toggleInput.checked = !isDisabled;
    toggleInput.classList.add('field-toggle-input');
    toggleInput.id = `toggle-${fieldDef.key}`;
    toggleInput.addEventListener('change', function(e) {
        e.stopPropagation();
        if (onToggleField) {
            onToggleField(fieldDef.key, !this.checked);
        }
    });
    
    const toggleSlider = document.createElement('label');
    toggleSlider.classList.add('field-toggle-slider');
    toggleSlider.setAttribute('for', `toggle-${fieldDef.key}`);
    
    toggleWrapper.appendChild(toggleInput);
    toggleWrapper.appendChild(toggleSlider);
    label.appendChild(toggleWrapper);
    
    // Label text
    const labelText = document.createElement('span');
    labelText.classList.add('field-label-text');
    labelText.textContent = fieldDef.label;
    label.appendChild(labelText);
    
    row.appendChild(label);

    // Value container
    const valueContainer = document.createElement('div');
    valueContainer.classList.add('field-value-container');
    valueContainer.setAttribute('data-field', fieldDef.key);
    
    const valueSpan = document.createElement('span');
    valueSpan.classList.add('field-value');
    
    // Check for custom value first
    const customVal = customValues[fieldDef.key];
    const displayVal = customVal || value;
    
    if (displayVal) {
        valueSpan.textContent = truncateText(displayVal, 100);
        valueSpan.title = displayVal;
    } else {
        valueSpan.textContent = 'No value';
        valueSpan.classList.add('empty-value');
    }
    valueContainer.appendChild(valueSpan);
    
    if (isEditable) {
        // Add editable badge
        const badge = document.createElement('span');
        badge.classList.add('source-badge');
        badge.setAttribute('data-field', fieldDef.key);
        
        if (customVal) {
            badge.classList.add('custom-badge', 'has-alternatives');
            badge.textContent = 'Custom';
        } else if (value) {
            badge.classList.add('has-alternatives');
            badge.textContent = 'Auto';
        } else {
            badge.classList.add('add-value-badge', 'has-alternatives');
            badge.innerHTML = '<i class="bi bi-plus-circle me-1"></i>Add';
        }
        
        badge.addEventListener('click', function(e) {
            e.stopPropagation();
            openAdditionalFieldModal(fieldDef.key, fieldDef.label, value);
        });
        valueContainer.appendChild(badge);
        
        valueContainer.addEventListener('click', function() {
            openAdditionalFieldModal(fieldDef.key, fieldDef.label, value);
        });
    } else {
        valueContainer.classList.add('no-alternatives');
    }

    row.appendChild(valueContainer);
    return row;
}

/**
 * Create archive URL row with status
 */
function createArchiveUrlRow(archiveUrl, archiveDate, customValues, openAdditionalFieldModal, createArchiveHandler, disabledFields, onToggleField) {
    const truncateText = window.url2refUtils?.truncateText || ((t, l) => t.length <= l ? t : t.substring(0, l) + '...');
    const isDisabled = disabledFields && disabledFields['archive_url'];
    
    const row = document.createElement('div');
    row.classList.add('field-row');
    row.setAttribute('data-field', 'archive_url');
    row.id = 'archive-url-row';
    if (isDisabled) {
        row.classList.add('field-disabled');
    }

    // Field label with toggle
    const label = document.createElement('div');
    label.classList.add('field-label');
    
    // Create toggle switch
    const toggleWrapper = document.createElement('div');
    toggleWrapper.classList.add('field-toggle');
    toggleWrapper.title = isDisabled ? 'Enable this field' : 'Disable this field';
    
    const toggleInput = document.createElement('input');
    toggleInput.type = 'checkbox';
    toggleInput.checked = !isDisabled;
    toggleInput.classList.add('field-toggle-input');
    toggleInput.id = 'toggle-archive_url';
    toggleInput.addEventListener('change', function(e) {
        e.stopPropagation();
        if (onToggleField) {
            onToggleField('archive_url', !this.checked);
        }
    });
    
    const toggleSlider = document.createElement('label');
    toggleSlider.classList.add('field-toggle-slider');
    toggleSlider.setAttribute('for', 'toggle-archive_url');
    
    toggleWrapper.appendChild(toggleInput);
    toggleWrapper.appendChild(toggleSlider);
    label.appendChild(toggleWrapper);
    
    // Label text
    const labelText = document.createElement('span');
    labelText.classList.add('field-label-text');
    labelText.textContent = 'Archive URL';
    label.appendChild(labelText);
    
    row.appendChild(label);

    // Value container
    const valueContainer = document.createElement('div');
    valueContainer.classList.add('field-value-container');
    valueContainer.id = 'archive-url-cell';

    const customVal = customValues['archive_url'];
    const displayUrl = customVal || archiveUrl;

    if (displayUrl) {
        // Archive available - show the URL as link
        const link = document.createElement('a');
        link.href = displayUrl;
        link.target = '_blank';
        link.classList.add('field-value');
        link.textContent = truncateText(displayUrl, 70);
        link.title = displayUrl;
        valueContainer.appendChild(link);
        
        // Add editable badge
        const badge = document.createElement('span');
        badge.classList.add('source-badge', 'has-alternatives');
        badge.setAttribute('data-field', 'archive_url');
        
        if (customVal) {
            badge.classList.add('custom-badge');
            badge.textContent = 'Custom';
        } else {
            badge.textContent = 'Auto';
        }
        
        badge.addEventListener('click', function(e) {
            e.stopPropagation();
            openAdditionalFieldModal('archive_url', 'Archive URL', archiveUrl);
        });
        valueContainer.appendChild(badge);
        
        valueContainer.addEventListener('click', function(e) {
            if (e.target.tagName !== 'A') {
                openAdditionalFieldModal('archive_url', 'Archive URL', archiveUrl);
            }
        });
    } else {
        // No archive - show pending state with create button and manual entry option
        valueContainer.innerHTML = `
            <span class="archive-pending">
                <span class="text-warning"><i class="bi bi-clock me-1"></i>No archive found</span>
                <button class="btn btn-outline-info btn-sm ms-2 create-archive-btn" type="button">
                    <i class="bi bi-archive me-1"></i>Create Archive
                </button>
            </span>
            <span class="archive-loading d-none">
                <span class="spinner-border spinner-border-sm text-info me-2" role="status"></span>
                <span class="text-muted">Creating archive...</span>
            </span>
        `;
        // Add click handler to the create button
        const createBtn = valueContainer.querySelector('.create-archive-btn');
        if (createBtn) {
            createBtn.addEventListener('click', createArchiveHandler);
        }
        
        // Also add manual entry badge
        const badge = document.createElement('span');
        badge.classList.add('source-badge', 'add-value-badge', 'has-alternatives');
        badge.innerHTML = '<i class="bi bi-plus-circle me-1"></i>Add';
        badge.addEventListener('click', function(e) {
            e.stopPropagation();
            openAdditionalFieldModal('archive_url', 'Archive URL', null);
        });
        valueContainer.appendChild(badge);
    }

    row.appendChild(valueContainer);
    return row;
}

/**
 * Create a simple field row (for fallback table display)
 */
function createSimpleFieldRow(fieldDef, value) {
    const truncateText = window.url2refUtils?.truncateText || ((t, l) => t.length <= l ? t : t.substring(0, l) + '...');
    
    const row = document.createElement('div');
    row.classList.add('field-row');
    row.setAttribute('data-field', fieldDef.key);

    // Field label
    const label = document.createElement('div');
    label.classList.add('field-label');
    label.textContent = fieldDef.label;
    row.appendChild(label);

    // Value container
    const valueContainer = document.createElement('div');
    valueContainer.classList.add('field-value-container', 'no-alternatives');
    
    const valueSpan = document.createElement('span');
    valueSpan.classList.add('field-value');
    if (value) {
        valueSpan.textContent = truncateText(value, 100);
        valueSpan.title = value;
    } else {
        valueSpan.textContent = 'No value extracted';
        valueSpan.classList.add('empty-value');
    }
    valueContainer.appendChild(valueSpan);

    row.appendChild(valueContainer);
    return row;
}

/**
 * Create a simple archive URL row (for fallback table display)
 */
function createSimpleArchiveUrlRow(archiveUrl, archiveDate, createArchiveHandler) {
    const truncateText = window.url2refUtils?.truncateText || ((t, l) => t.length <= l ? t : t.substring(0, l) + '...');
    
    const row = document.createElement('div');
    row.classList.add('field-row');
    row.setAttribute('data-field', 'archive_url');
    row.id = 'archive-url-row';

    // Field label
    const label = document.createElement('div');
    label.classList.add('field-label');
    label.textContent = 'Archive URL';
    row.appendChild(label);

    // Value container
    const valueContainer = document.createElement('div');
    valueContainer.classList.add('field-value-container', 'no-alternatives');
    valueContainer.id = 'archive-url-cell';

    if (archiveUrl) {
        // Archive available - show the URL as link
        const link = document.createElement('a');
        link.href = archiveUrl;
        link.target = '_blank';
        link.classList.add('field-value');
        link.textContent = truncateText(archiveUrl, 70);
        link.title = archiveUrl;
        valueContainer.appendChild(link);
    } else {
        // No archive - show pending state with create button
        valueContainer.innerHTML = `
            <span class="archive-pending">
                <span class="text-warning"><i class="bi bi-clock me-1"></i>No archive found</span>
                <button class="btn btn-outline-info btn-sm ms-2 create-archive-btn" type="button">
                    <i class="bi bi-archive me-1"></i>Create Archive
                </button>
            </span>
            <span class="archive-loading d-none">
                <span class="spinner-border spinner-border-sm text-info me-2" role="status"></span>
                <span class="text-muted">Creating archive...</span>
            </span>
        `;
        // Add click handler to the create button
        const createBtn = valueContainer.querySelector('.create-archive-btn');
        if (createBtn) {
            createBtn.addEventListener('click', createArchiveHandler);
        }
    }

    row.appendChild(valueContainer);
    return row;
}

/**
 * Update a single field's display after selection change
 */
function updateFieldDisplay(fieldKey, multiSource, selections, customValues) {
    const truncateText = window.url2refUtils?.truncateText || ((t, l) => t.length <= l ? t : t.substring(0, l) + '...');
    
    const fieldData = multiSource ? multiSource[fieldKey] : null;
    const selectedSource = selections ? selections[fieldKey] : null;
    
    // Get the current value to display
    let displayValue = '';
    if (selectedSource === 'custom' && customValues[fieldKey]) {
        displayValue = customValues[fieldKey];
    } else if (selectedSource && fieldData && fieldData[selectedSource]) {
        displayValue = fieldData[selectedSource];
    }
    
    // Find and update the field row
    const row = document.querySelector(`.field-row[data-field="${fieldKey}"]`);
    if (!row) return;
    
    const valueSpan = row.querySelector('.field-value');
    const badge = row.querySelector('.source-badge');
    
    if (valueSpan) {
        if (displayValue) {
            valueSpan.textContent = truncateText(displayValue, 80);
            valueSpan.title = displayValue;
            valueSpan.classList.remove('empty-value');
        } else {
            valueSpan.textContent = 'No value extracted';
            valueSpan.classList.add('empty-value');
        }
    }
    
    if (badge) {
        badge.classList.remove('custom-badge', 'ai-badge');
        if (selectedSource === 'custom') {
            badge.classList.add('custom-badge');
            badge.textContent = 'Custom';
        } else if (selectedSource === 'ai') {
            badge.classList.add('ai-badge');
            badge.textContent = 'AI';
        } else if (selectedSource) {
            badge.textContent = SOURCE_LABELS[selectedSource] || selectedSource;
        }
    }
}

/**
 * Update additional field display after change
 */
function updateAdditionalFieldDisplay(fieldKey, originalValue, customValues, openAdditionalFieldModal) {
    const truncateText = window.url2refUtils?.truncateText || ((t, l) => t.length <= l ? t : t.substring(0, l) + '...');
    
    const row = document.querySelector(`.field-row[data-field="${fieldKey}"]`);
    if (!row) return;
    
    const valueContainer = row.querySelector('.field-value-container');
    const valueSpan = row.querySelector('.field-value');
    const badge = row.querySelector('.source-badge');
    
    const customVal = customValues[fieldKey];
    const displayVal = customVal || originalValue;
    
    // Special handling for archive_url which uses a link element
    if (fieldKey === 'archive_url' && displayVal) {
        // If we now have a value but previously didn't, need to rebuild the cell
        const existingLink = row.querySelector('a.field-value');
        if (existingLink) {
            existingLink.href = displayVal;
            existingLink.textContent = truncateText(displayVal, 70);
            existingLink.title = displayVal;
        } else if (valueContainer) {
            // Clear pending state and create link
            valueContainer.innerHTML = '';
            const link = document.createElement('a');
            link.href = displayVal;
            link.target = '_blank';
            link.classList.add('field-value');
            link.textContent = truncateText(displayVal, 70);
            link.title = displayVal;
            valueContainer.appendChild(link);
            
            // Re-add badge
            const newBadge = document.createElement('span');
            newBadge.classList.add('source-badge', 'has-alternatives');
            newBadge.setAttribute('data-field', 'archive_url');
            if (customVal) {
                newBadge.classList.add('custom-badge');
                newBadge.textContent = 'Custom';
            } else {
                newBadge.textContent = 'Auto';
            }
            newBadge.addEventListener('click', function(e) {
                e.stopPropagation();
                openAdditionalFieldModal('archive_url', 'Archive URL', originalValue);
            });
            valueContainer.appendChild(newBadge);
            
            valueContainer.addEventListener('click', function(e) {
                if (e.target.tagName !== 'A') {
                    openAdditionalFieldModal('archive_url', 'Archive URL', originalValue);
                }
            });
        }
    } else if (valueSpan) {
        if (displayVal) {
            valueSpan.textContent = truncateText(displayVal, 100);
            valueSpan.title = displayVal;
            valueSpan.classList.remove('empty-value');
        } else {
            valueSpan.textContent = 'No value';
            valueSpan.classList.add('empty-value');
        }
    }
    
    if (badge) {
        badge.classList.remove('custom-badge', 'add-value-badge');
        badge.innerHTML = '';
        
        if (customVal) {
            badge.classList.add('custom-badge', 'has-alternatives');
            badge.textContent = 'Custom';
        } else if (originalValue) {
            badge.classList.add('has-alternatives');
            badge.textContent = 'Auto';
        } else {
            badge.classList.add('add-value-badge', 'has-alternatives');
            badge.innerHTML = '<i class="bi bi-plus-circle me-1"></i>Add';
        }
    }
}

/**
 * Update the visual state of a field when toggled
 */
function updateFieldToggleState(fieldKey, isDisabled) {
    const row = document.querySelector(`.field-row[data-field="${fieldKey}"]`);
    if (!row) return;
    
    if (isDisabled) {
        row.classList.add('field-disabled');
    } else {
        row.classList.remove('field-disabled');
    }
    
    // Update toggle tooltip
    const toggleWrapper = row.querySelector('.field-toggle');
    if (toggleWrapper) {
        toggleWrapper.title = isDisabled ? 'Enable this field' : 'Disable this field';
    }
}

// Export functions for use in other modules
window.url2refMetadata = {
    SOURCE_LABELS,
    SOURCES,
    METADATA_FIELD_DEFS,
    ALL_TOGGLEABLE_FIELDS,
    hasAnyMultiSourceData,
    createSectionHeader,
    getAlternativesCount,
    createFieldRow,
    createEditableFieldRow,
    createArchiveUrlRow,
    createSimpleFieldRow,
    createSimpleArchiveUrlRow,
    updateFieldDisplay,
    updateAdditionalFieldDisplay,
    updateFieldToggleState
};
