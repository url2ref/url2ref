// Tooltip functionality
var tooltipTriggerList = [].slice.call(document.querySelectorAll('[data-bs-toggle="tooltip"]'))
var tooltipList = tooltipTriggerList.map(function (tooltipTriggerEl) {
  return new bootstrap.Tooltip(tooltipTriggerEl)
})

// Copy to clipboard
function copyToClipboard(msg) {
    navigator.clipboard.writeText(msg)
}

// Theme switcher
document.getElementById('bd-theme').addEventListener('click',()=>{
    if (document.documentElement.getAttribute('data-bs-theme') == 'dark') {
        document.documentElement.setAttribute('data-bs-theme','light')
    }
    else {
        document.documentElement.setAttribute('data-bs-theme','dark')
    }
})

// Reference generation
document.addEventListener('DOMContentLoaded', function() {
    const urlInput = document.getElementById('url-input');
    const generateBtn = document.getElementById('generate-btn');
    const targetLangSelect = document.getElementById('target-lang');
    const translationProviderSelect = document.getElementById('translation-provider');
    const btnText = document.getElementById('btn-text');
    const btnSpinner = document.getElementById('btn-spinner');
    const errorAlert = document.getElementById('error-alert');
    const errorMessage = document.getElementById('error-message');
    const detailsSection = document.getElementById('details-section');
    const resultsSection = document.getElementById('results-section');
    const bibtexOutput = document.getElementById('bibtex-output');
    const wikiOutput = document.getElementById('wiki-output');
    const metadataFields = document.getElementById('metadata-fields');
    const wikiFormatRadios = document.querySelectorAll('input[name="wiki-format"]');
    
    // Modal elements
    const modalOverlay = document.getElementById('source-modal-overlay');
    const modalFieldName = document.getElementById('modal-field-name');
    const modalBody = document.getElementById('source-modal-body');
    const modalCloseBtn = document.getElementById('modal-close-btn');
    const modalCustomInput = document.getElementById('modal-custom-input');
    const modalCustomApply = document.getElementById('modal-custom-apply');

    // State
    let currentUrl = '';
    let currentMultiSource = null;
    let currentSelections = null;
    let currentFields = null;  // Store archive URL/date and other fields
    let currentWikiCitation = '';  // Store the current wiki citation for reformatting
    let customValues = {};  // Store user-entered custom values for each field
    let currentModalField = null;  // Track which field the modal is editing

    // Generate reference on button click
    generateBtn.addEventListener('click', generateReference);

    // Generate reference on Enter key
    urlInput.addEventListener('keypress', function(e) {
        if (e.key === 'Enter') {
            generateReference();
        }
    });

    // Wiki format change listener
    wikiFormatRadios.forEach(radio => {
        radio.addEventListener('change', function() {
            if (currentWikiCitation) {
                wikiOutput.textContent = formatWikiCitation(currentWikiCitation, this.value);
            }
        });
    });

    // Copy buttons
    document.querySelectorAll('.copy-btn').forEach(btn => {
        btn.addEventListener('click', function() {
            const targetId = this.getAttribute('data-target');
            const targetElement = document.getElementById(targetId);
            copyToClipboard(targetElement.textContent);
            
            // Visual feedback
            const originalText = this.innerHTML;
            this.innerHTML = '<i class="bi bi-check"></i> Copied!';
            setTimeout(() => {
                this.innerHTML = originalText;
            }, 2000);
        });
    });

    // Modal event listeners
    if (modalCloseBtn) {
        modalCloseBtn.addEventListener('click', closeSourceModal);
    }
    if (modalOverlay) {
        modalOverlay.addEventListener('click', function(e) {
            if (e.target === modalOverlay) {
                closeSourceModal();
            }
        });
    }
    if (modalCustomApply) {
        modalCustomApply.addEventListener('click', applyCustomValue);
    }
    if (modalCustomInput) {
        modalCustomInput.addEventListener('keypress', function(e) {
            if (e.key === 'Enter') {
                applyCustomValue();
            }
        });
    }

    // Close modal on Escape key
    document.addEventListener('keydown', function(e) {
        if (e.key === 'Escape' && modalOverlay && !modalOverlay.classList.contains('d-none')) {
            closeSourceModal();
        }
    });

    async function generateReference() {
        const url = urlInput.value.trim();
        
        // Validate URL
        if (!url) {
            showError('Please enter a URL');
            return;
        }

        if (!isValidUrl(url)) {
            showError('Please enter a valid URL (starting with http:// or https://)');
            return;
        }

        currentUrl = url;

        // Show loading state
        setLoading(true);
        hideError();
        hideResults();
        hideDetails();

        try {
            const requestBody = { url: url };
            
            // Include target language if selected
            const targetLang = targetLangSelect ? targetLangSelect.value : '';
            if (targetLang) {
                requestBody.target_lang = targetLang;
                // Include translation provider
                requestBody.translation_provider = translationProviderSelect ? translationProviderSelect.value : 'deepl';
            }

            const response = await fetch('/api/generate', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(requestBody),
            });

            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }

            const data = await response.json();

            if (data.success) {
                // Store multi-source data, selections, and fields
                currentMultiSource = data.multi_source;
                currentSelections = data.selections;
                currentFields = data.fields;
                customValues = {};  // Reset custom values for new reference
                
                // Display extracted fields - use multi-source table or fall back to fields
                if (data.multi_source && hasAnyMultiSourceData(data.multi_source)) {
                    displayMultiSourceTable(data.multi_source, data.selections, data.fields);
                } else if (data.fields) {
                    // Fall back to displaying fields as simple table
                    displayFieldsTable(data.fields);
                }
                
                // Display citations
                bibtexOutput.textContent = data.bibtex;
                // Store and display wiki citation with current format setting
                currentWikiCitation = data.wiki;
                wikiOutput.textContent = formatWikiCitation(data.wiki, getWikiFormat());
                
                // Archive status is now handled within the table display functions
                
                showDetails();
                showResults();
            } else {
                showError(data.error || 'Failed to generate reference');
            }
        } catch (error) {
            console.error('Error:', error);
            showError('Network error: ' + error.message);
        } finally {
            setLoading(false);
        }
    }

    async function regenerateCitation() {
        // Build citations locally from current selections without network request
        if (!currentMultiSource || !currentSelections) {
            return;
        }

        // Get the selected value for each field from the multi-source data
        const selectedValues = {};
        const fields = ['title', 'author', 'date', 'site', 'publisher', 'language', 'url'];
        
        fields.forEach(field => {
            const source = currentSelections[field];
            if (source === 'custom' && customValues[field]) {
                // Use custom value if custom is selected
                selectedValues[field] = customValues[field];
            } else if (source && currentMultiSource[field] && currentMultiSource[field][source]) {
                selectedValues[field] = currentMultiSource[field][source];
            }
        });

        // Get archive info and translated title - use custom values if set, otherwise use currentFields
        const archiveUrl = customValues['archive_url'] || currentFields?.archive_url;
        const archiveDate = customValues['archive_date'] || currentFields?.archive_date;
        const translatedTitle = customValues['translated_title'] || currentFields?.translated_title;

        // Build Wiki citation (always build as multiline, then format according to setting)
        const wiki = buildWikiCitation(selectedValues, archiveUrl, archiveDate, translatedTitle);
        currentWikiCitation = wiki;
        wikiOutput.textContent = formatWikiCitation(wiki, getWikiFormat());

        // Build BibTeX citation
        const bibtex = buildBibTeXCitation(selectedValues, archiveUrl, archiveDate, translatedTitle);
        bibtexOutput.textContent = bibtex;
    }

    // Get the current wiki format setting
    function getWikiFormat() {
        const checked = document.querySelector('input[name="wiki-format"]:checked');
        return checked ? checked.value : 'multiline';
    }

    // Format wiki citation according to the selected format
    function formatWikiCitation(citation, format) {
        if (format === 'singleline') {
            // Convert multiline to single line
            return citation
                .replace(/\n/g, ' ')
                .replace(/\|\s+/g, '|')
                .replace(/\s+=\s+/g, '=')
                .replace(/\s+}}/g, ' }}')
                .replace(/\s{2,}/g, ' ');
        }
        // Return as-is for multiline (default)
        return citation;
    }

    // Build a Wiki citation from selected values (always multiline)
    function buildWikiCitation(values, archiveUrl, archiveDate, translatedTitle) {
        let parts = [];
        
        if (values.title) parts.push(`| title = ${values.title}`);
        if (translatedTitle) parts.push(`| trans-title = ${translatedTitle}`);
        if (values.author) {
            // Handle author formatting - split into first/last if single author
            const author = values.author.trim();
            const nameParts = author.split(/\s+/);
            if (nameParts.length > 1) {
                const lastName = nameParts[nameParts.length - 1];
                const firstNames = nameParts.slice(0, -1).join(' ');
                parts.push(`| last = ${lastName}`);
                parts.push(`| first = ${firstNames}`);
            } else {
                parts.push(`| author = ${author}`);
            }
        }
        if (values.date) parts.push(`| date = ${values.date}`);
        if (values.site) parts.push(`| site = ${values.site}`);
        if (values.publisher) parts.push(`| publisher = ${values.publisher}`);
        if (values.language) parts.push(`| language = ${values.language}`);
        if (values.url) parts.push(`| url = ${values.url}`);
        if (archiveUrl) parts.push(`| archive-url = ${archiveUrl}`);
        if (archiveDate) parts.push(`| archive-date = ${archiveDate}`);
        
        return `{{cite web\n${parts.join('\n')}\n}}`;
    }

    // Build a BibTeX citation from selected values
    function buildBibTeXCitation(values, archiveUrl, archiveDate, translatedTitle) {
        let lines = ['@misc{ url2ref,'];
        
        if (values.title) lines.push(`title = "${values.title}",`);
        if (translatedTitle) lines.push(`note = "Translated title: ${translatedTitle}",`);
        if (values.author) {
            // Handle author formatting for BibTeX
            const author = values.author.trim();
            const nameParts = author.split(/\s+/);
            if (nameParts.length > 1) {
                const lastName = nameParts[nameParts.length - 1];
                const firstNames = nameParts.slice(0, -1).join(' ');
                lines.push(`author = "${lastName}, ${firstNames}",`);
            } else {
                lines.push(`author = "{${author}}",`);
            }
        }
        if (values.date) lines.push(`date = "${values.date}",`);
        if (values.site) lines.push(`howpublished = "${values.site}",`);
        if (values.publisher) lines.push(`publisher = "${values.publisher}",`);
        if (values.url) lines.push(`url = \\url{${values.url}},`);
        if (archiveUrl) lines.push(`archiveurl = \\url{${archiveUrl}},`);
        if (archiveDate) lines.push(`archivedate = "${archiveDate}",`);
        
        lines.push('}');
        return lines.join('\n');
    }

    function displayMultiSourceTable(multiSource, selections, fields) {
        // Clear existing content
        metadataFields.innerHTML = '';
        
        if (!multiSource) {
            return;
        }

        // Field definitions for metadata section
        const metadataFieldDefs = [
            { key: 'title', label: 'Title' },
            { key: 'author', label: 'Author' },
            { key: 'date', label: 'Date' },
            { key: 'site', label: 'Site' },
            { key: 'publisher', label: 'Publisher' },
            { key: 'language', label: 'Language' },
            { key: 'url', label: 'URL' }
        ];

        // Source labels mapping
        const sources = ['opengraph', 'schemaorg', 'htmlmeta', 'doi'];
        const sourceLabels = {
            'opengraph': 'Open Graph',
            'schemaorg': 'Schema.org',
            'htmlmeta': 'HTML Meta',
            'doi': 'DOI',
            'custom': 'Custom'
        };

        // Helper function to create a section header
        function createSectionHeader(title) {
            const header = document.createElement('div');
            header.classList.add('section-header');
            header.textContent = title;
            return header;
        }

        // Helper function to get available alternatives count for a field
        function getAlternativesCount(fieldKey) {
            const fieldData = multiSource[fieldKey];
            if (!fieldData) return 0;
            return sources.filter(src => fieldData[src]).length;
        }

        // Helper function to create a compact field row
        function createFieldRow(fieldDef) {
            const fieldData = multiSource[fieldDef.key];
            const selectedSource = selections ? selections[fieldDef.key] : null;
            const alternativesCount = getAlternativesCount(fieldDef.key);
            
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

            // Field label
            const label = document.createElement('div');
            label.classList.add('field-label');
            label.textContent = fieldDef.label;
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
                } else if (currentSource) {
                    badge.textContent = sourceLabels[currentSource] || currentSource;
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

        // Helper function to create an editable field row (for additional values like translated title)
        function createEditableFieldRow(fieldDef, value, isEditable = true) {
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

        // Helper function to create archive URL row with status
        function createArchiveUrlRow(archiveUrl, archiveDate) {
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
                    createBtn.addEventListener('click', createArchive);
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

        // Add "Metadata values" section header
        metadataFields.appendChild(createSectionHeader('Metadata values'));
        
        // Add metadata field rows
        metadataFieldDefs.forEach(fieldDef => {
            metadataFields.appendChild(createFieldRow(fieldDef));
        });

        // Always add "Additional values" section for archive info and translated title
        metadataFields.appendChild(createSectionHeader('Additional values'));

        // Add translated title row (editable)
        const translatedTitle = fields ? fields.translated_title : null;
        metadataFields.appendChild(createEditableFieldRow({ key: 'translated_title', label: 'Translated Title' }, translatedTitle, true));

        // Add archive URL row (always shown, either with value or pending state)
        const archiveUrl = fields ? fields.archive_url : null;
        const archiveDate = fields ? fields.archive_date : null;
        metadataFields.appendChild(createArchiveUrlRow(archiveUrl, archiveDate));

        // Add archive date row (editable)
        metadataFields.appendChild(createEditableFieldRow({ key: 'archive_date', label: 'Archive Date' }, archiveDate, true));
    }

    // Open the source selection modal
    function openSourceModal(fieldKey, fieldLabel) {
        currentModalField = fieldKey;
        modalFieldName.textContent = fieldLabel;
        
        // Clear previous content
        modalBody.innerHTML = '';
        modalCustomInput.value = customValues[fieldKey] || '';
        
        const sources = ['opengraph', 'schemaorg', 'htmlmeta', 'doi'];
        const sourceLabels = {
            'opengraph': 'Open Graph',
            'schemaorg': 'Schema.org',
            'htmlmeta': 'HTML Meta',
            'doi': 'DOI'
        };
        
        const fieldData = currentMultiSource ? currentMultiSource[fieldKey] : null;
        const currentSelection = currentSelections ? currentSelections[fieldKey] : null;
        
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
                option.addEventListener('click', function() {
                    selectSource(fieldKey, source);
                });
            }
            
            modalBody.appendChild(option);
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
            
            customOption.addEventListener('click', function() {
                selectSource(fieldKey, 'custom');
            });
            
            modalBody.appendChild(customOption);
        }
        
        // Show modal with animation
        modalOverlay.classList.remove('d-none');
        // Trigger reflow for animation
        void modalOverlay.offsetWidth;
        modalOverlay.classList.add('show');
    }

    // Open modal for editing additional fields (translated title, archive date, etc.)
    function openAdditionalFieldModal(fieldKey, fieldLabel, originalValue) {
        currentModalField = fieldKey;
        modalFieldName.textContent = fieldLabel;
        
        // Clear previous content
        modalBody.innerHTML = '';
        modalCustomInput.value = customValues[fieldKey] || '';
        
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
            
            originalOption.addEventListener('click', function() {
                // Remove custom value to revert to original
                delete customValues[fieldKey];
                updateAdditionalFieldDisplay(fieldKey, originalValue);
                closeSourceModal();
                regenerateCitation();
            });
            
            modalBody.appendChild(originalOption);
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
            
            modalBody.appendChild(customOption);
        }
        
        // If no options at all, show a message
        if (!originalValue && !customValues[fieldKey]) {
            const emptyMsg = document.createElement('div');
            emptyMsg.classList.add('source-option', 'empty-option');
            emptyMsg.innerHTML = '<div class="source-option-value empty">No value detected. Enter a custom value below.</div>';
            modalBody.appendChild(emptyMsg);
        }
        
        // Show modal with animation
        modalOverlay.classList.remove('d-none');
        void modalOverlay.offsetWidth;
        modalOverlay.classList.add('show');
    }

    // Update additional field display after change
    function updateAdditionalFieldDisplay(fieldKey, originalValue) {
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

    // Close the source selection modal
    function closeSourceModal() {
        modalOverlay.classList.remove('show');
        setTimeout(() => {
            modalOverlay.classList.add('d-none');
            currentModalField = null;
        }, 250);
    }

    // Select a source from the modal
    function selectSource(fieldKey, source) {
        if (currentSelections) {
            currentSelections[fieldKey] = source;
        }
        
        // Update the field display
        updateFieldDisplay(fieldKey);
        
        // Close modal
        closeSourceModal();
        
        // Regenerate citation
        regenerateCitation();
    }

    // Apply custom value from modal
    function applyCustomValue() {
        if (!currentModalField) return;
        
        const value = modalCustomInput.value.trim();
        if (!value) return;
        
        const additionalFields = ['translated_title', 'archive_date', 'archive_url'];
        
        customValues[currentModalField] = value;
        
        if (additionalFields.includes(currentModalField)) {
            // Handle additional fields differently
            const originalValue = currentFields ? currentFields[currentModalField] : null;
            updateAdditionalFieldDisplay(currentModalField, originalValue);
            closeSourceModal();
            regenerateCitation();
        } else {
            // Regular metadata field
            selectSource(currentModalField, 'custom');
        }
    }

    // Update a single field's display after selection change
    function updateFieldDisplay(fieldKey) {
        const fieldData = currentMultiSource ? currentMultiSource[fieldKey] : null;
        const selectedSource = currentSelections ? currentSelections[fieldKey] : null;
        
        const sourceLabels = {
            'opengraph': 'Open Graph',
            'schemaorg': 'Schema.org',
            'htmlmeta': 'HTML Meta',
            'doi': 'DOI',
            'custom': 'Custom'
        };
        
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
            badge.classList.remove('custom-badge');
            if (selectedSource === 'custom') {
                badge.classList.add('custom-badge');
                badge.textContent = 'Custom';
            } else if (selectedSource) {
                badge.textContent = sourceLabels[selectedSource] || selectedSource;
            }
        }
    }

    function truncateText(text, maxLength) {
        if (text.length <= maxLength) {
            return text;
        }
        return text.substring(0, maxLength) + '...';
    }

    function hasAnyMultiSourceData(multiSource) {
        if (!multiSource) return false;
        const fields = ['title', 'author', 'date', 'site', 'publisher', 'language', 'url'];
        const sources = ['opengraph', 'schemaorg', 'htmlmeta', 'doi'];
        
        return fields.some(field => {
            const fieldData = multiSource[field];
            if (!fieldData) return false;
            return sources.some(src => fieldData[src]);
        });
    }

    function displayFieldsTable(fields) {
        // Clear existing content
        metadataFields.innerHTML = '';
        
        // Field definitions for metadata section
        const metadataFieldDefs = [
            { key: 'title', label: 'Title' },
            { key: 'author', label: 'Author' },
            { key: 'date', label: 'Date' },
            { key: 'site', label: 'Site' },
            { key: 'publisher', label: 'Publisher' },
            { key: 'language', label: 'Language' },
            { key: 'url', label: 'URL' }
        ];

        // Helper function to create a section header
        function createSectionHeader(title) {
            const header = document.createElement('div');
            header.classList.add('section-header');
            header.textContent = title;
            return header;
        }

        // Helper function to create a field row
        function createFieldRow(fieldDef, value) {
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

        // Helper function to create archive URL row with status
        function createArchiveUrlRow(archiveUrl, archiveDate) {
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
                    createBtn.addEventListener('click', createArchive);
                }
            }

            row.appendChild(valueContainer);
            return row;
        }

        // Add "Metadata values" section header
        metadataFields.appendChild(createSectionHeader('Metadata values'));

        // Add metadata field rows
        metadataFieldDefs.forEach(fieldDef => {
            const row = createFieldRow(fieldDef, fields[fieldDef.key]);
            metadataFields.appendChild(row);
        });

        // Always add "Additional values" section for archive info and translated title
        metadataFields.appendChild(createSectionHeader('Additional values'));

        // Add translated title row if available
        if (fields.translated_title) {
            const translatedTitleRow = createFieldRow({ key: 'translated_title', label: 'Translated Title' }, fields.translated_title);
            metadataFields.appendChild(translatedTitleRow);
        }

        // Add archive URL row (always shown, either with value or pending state)
        metadataFields.appendChild(createArchiveUrlRow(fields.archive_url, fields.archive_date));

        // Add archive date row if available (separate from URL row)
        if (fields.archive_date) {
            const archiveDateRow = createFieldRow({ key: 'archive_date', label: 'Archive Date' }, fields.archive_date);
            metadataFields.appendChild(archiveDateRow);
        }
    }

    async function createArchive() {
        if (!currentUrl) return;

        // Show loading state in the table cell
        showArchiveLoading();

        try {
            const response = await fetch('/api/archive', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ url: currentUrl }),
            });

            const data = await response.json();

            if (data.success && data.archive_url) {
                // Update archive fields in the table
                showArchiveAvailable(data.archive_url, data.archive_date);
                
                // Update citations with archive info
                if (data.bibtex) {
                    bibtexOutput.textContent = data.bibtex;
                }
                if (data.wiki) {
                    wikiOutput.textContent = data.wiki;
                }
            } else {
                showArchiveError(data.error || 'Failed to create archive');
            }
        } catch (error) {
            showArchiveError('Network error while creating archive');
            console.error('Error:', error);
        }
    }

    function showArchiveAvailable(archiveUrl, archiveDate) {
        // Update the archive URL cell
        const archiveCell = document.getElementById('archive-url-cell');
        if (archiveCell) {
            // Create a link element
            const link = document.createElement('a');
            link.href = archiveUrl;
            link.target = '_blank';
            link.classList.add('field-value');
            link.textContent = truncateText(archiveUrl, 70);
            link.title = archiveUrl;
            
            archiveCell.innerHTML = '';
            archiveCell.appendChild(link);
        }
        
        // Update currentFields
        if (currentFields) {
            currentFields.archive_url = archiveUrl;
            currentFields.archive_date = archiveDate;
        }
        
        // Add archive date row if not already present and date is available
        if (archiveDate) {
            const existingDateRow = document.querySelector('.field-row[data-field="archive_date"]');
            if (!existingDateRow) {
                const row = document.createElement('div');
                row.classList.add('field-row');
                row.setAttribute('data-field', 'archive_date');
                
                const label = document.createElement('div');
                label.classList.add('field-label');
                label.textContent = 'Archive Date';
                row.appendChild(label);
                
                const valueContainer = document.createElement('div');
                valueContainer.classList.add('field-value-container', 'no-alternatives');
                
                const valueSpan = document.createElement('span');
                valueSpan.classList.add('field-value');
                valueSpan.textContent = archiveDate;
                valueContainer.appendChild(valueSpan);
                
                row.appendChild(valueContainer);
                
                const archiveUrlRow = document.getElementById('archive-url-row');
                if (archiveUrlRow && archiveUrlRow.nextSibling) {
                    archiveUrlRow.parentNode.insertBefore(row, archiveUrlRow.nextSibling);
                } else if (archiveUrlRow) {
                    archiveUrlRow.parentNode.appendChild(row);
                }
            }
        }
    }

    function showArchiveNotFound() {
        // This is handled by the initial table rendering
        // The archive URL row is created with pending state if no archive exists
    }

    function showArchiveLoading() {
        // Show loading spinner in the archive cell
        const archiveCell = document.getElementById('archive-url-cell');
        if (archiveCell) {
            const pending = archiveCell.querySelector('.archive-pending');
            const loading = archiveCell.querySelector('.archive-loading');
            if (pending) pending.classList.add('d-none');
            if (loading) loading.classList.remove('d-none');
        }
    }

    function showArchiveError(message) {
        // Show error in the archive cell
        const archiveCell = document.getElementById('archive-url-cell');
        if (archiveCell) {
            archiveCell.innerHTML = `<span class="text-danger"><i class="bi bi-exclamation-triangle me-1"></i>${message}</span>`;
        }
    }

    function isValidUrl(string) {
        try {
            const url = new URL(string);
            return url.protocol === 'http:' || url.protocol === 'https:';
        } catch (_) {
            return false;
        }
    }

    function setLoading(loading) {
        generateBtn.disabled = loading;
        btnText.textContent = loading ? 'Generating...' : 'Generate';
        btnSpinner.classList.toggle('d-none', !loading);
    }

    function showError(message) {
        errorMessage.textContent = message;
        errorAlert.classList.remove('d-none');
    }

    function hideError() {
        errorAlert.classList.add('d-none');
    }

    function showDetails() {
        detailsSection.classList.remove('d-none');
    }

    function hideDetails() {
        detailsSection.classList.add('d-none');
    }

    function showResults() {
        resultsSection.classList.remove('d-none');
    }

    function hideResults() {
        resultsSection.classList.add('d-none');
    }
});