/**
 * Main application module for url2ref
 * Orchestrates all other modules and handles core application logic
 */

document.addEventListener('DOMContentLoaded', function() {
    // Import utilities
    const { initTooltips, initThemeSwitcher, isValidUrl, truncateText, copyToClipboard } = window.url2refUtils;
    
    // Import language functions
    const { 
        populateLanguageSuggestions, 
        preselectUserLanguage, 
        validateLanguageInput, 
        getSelectedLanguage 
    } = window.url2refLanguage;
    
    // Import citation functions
    const { getWikiFormat, formatWikiCitation, buildWikiCitation, buildBibTeXCitation, buildHarvardCitation } = window.url2refCitation;
    
    // Import archive functions
    const { showArchiveLoading, showArchiveAvailable, showArchiveError } = window.url2refArchive;
    
    // Import metadata functions
    const {
        METADATA_FIELD_DEFS,
        ALL_TOGGLEABLE_FIELDS,
        hasAnyMultiSourceData,
        createSectionHeader,
        createFieldRow,
        createEditableFieldRow,
        createArchiveUrlRow,
        createSimpleFieldRow,
        createSimpleArchiveUrlRow,
        updateFieldDisplay,
        updateAdditionalFieldDisplay,
        updateFieldToggleState
    } = window.url2refMetadata;
    
    // Import modal manager
    const { ModalManager } = window.url2refModal;
    
    // Initialize utilities
    initTooltips();
    initThemeSwitcher();
    
    // DOM elements
    const urlInput = document.getElementById('url-input');
    const generateBtn = document.getElementById('generate-btn');
    const btnText = document.getElementById('btn-text');
    const btnSpinner = document.getElementById('btn-spinner');
    const errorAlert = document.getElementById('error-alert');
    const errorMessage = document.getElementById('error-message');
    const detailsSection = document.getElementById('details-section');
    const resultsSection = document.getElementById('results-section');
    const bibtexOutput = document.getElementById('bibtex-output');
    const wikiOutput = document.getElementById('wiki-output');
    const harvardOutput = document.getElementById('harvard-output');
    const metadataFields = document.getElementById('metadata-fields');
    const wikiFormatRadios = document.querySelectorAll('input[name="wiki-format"]');
    
    // Translation elements
    const translationProviderSelect = document.getElementById('translation-provider');
    const targetLangInput = document.getElementById('target-lang-input');
    
    // AI extraction elements
    const aiEnabledCheckbox = document.getElementById('ai-enabled');
    const aiProviderSelect = document.getElementById('ai-provider');
    const aiApiKeyInput = document.getElementById('ai-api-key');
    const aiModelInput = document.getElementById('ai-model');
    
    // Modal elements
    const modalOverlay = document.getElementById('source-modal-overlay');
    const modalBody = document.getElementById('source-modal-body');
    const modalFieldName = document.getElementById('modal-field-name');
    const modalCustomInput = document.getElementById('modal-custom-input');
    const modalCloseBtn = document.getElementById('modal-close-btn');
    const modalCustomApply = document.getElementById('modal-custom-apply');
    
    // State
    let currentUrl = '';
    let currentMultiSource = null;
    let currentSelections = null;
    let currentFields = null;
    let customValues = {};
    let disabledFields = {};  // Track which fields are disabled
    let currentWikiCitation = '';
    let currentTargetLang = null;  // Track the target language used for translation
    
    // AI checkbox toggle handler
    aiEnabledCheckbox.addEventListener('change', function() {
        const enabled = this.checked;
        aiProviderSelect.disabled = !enabled;
        aiApiKeyInput.disabled = !enabled;
        aiModelInput.disabled = !enabled;
    });
    
    // Initialize modal manager
    const modalManager = new ModalManager({
        modalOverlay,
        modalBody,
        modalFieldName,
        modalCustomInput,
        modalCloseBtn,
        modalCustomApply
    });
    
    // Set modal callbacks
    modalManager.setCallbacks({
        onSelectSource: selectSource,
        onRevertToOriginal: revertToOriginal,
        onApplyCustomValue: applyCustomValue
    });
    
    // Initialize language functionality
    populateLanguageSuggestions();
    
    // Language input event listeners
    if (targetLangInput) {
        targetLangInput.addEventListener('input', validateLanguageInput);
        targetLangInput.addEventListener('change', validateLanguageInput);
    }
    
    // Translation provider change listener
    if (translationProviderSelect) {
        translationProviderSelect.addEventListener('change', function() {
            // Re-validate language input when provider changes
            validateLanguageInput();
            // If a provider is selected and no language is set, try to preselect
            if (this.value && targetLangInput && !targetLangInput.value.trim()) {
                preselectUserLanguage();
            }
        });
        
        // Initial language preselection if provider is already set
        if (translationProviderSelect.value) {
            preselectUserLanguage();
        }
    }
    
    // Generate button click handler
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
            copyToClipboard(this, targetId);
        });
    });
    
    /**
     * Generate reference from URL
     */
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
            const targetLang = getSelectedLanguage();
            if (targetLang) {
                requestBody.target_lang = targetLang;
                // Include translation provider
                requestBody.translation_provider = translationProviderSelect ? translationProviderSelect.value : 'deepl';
            }
            
            // Include AI options if enabled
            if (aiEnabledCheckbox && aiEnabledCheckbox.checked) {
                requestBody.ai_enabled = true;
                requestBody.ai_provider = aiProviderSelect ? aiProviderSelect.value : 'openai';
                const apiKey = aiApiKeyInput ? aiApiKeyInput.value.trim() : '';
                if (apiKey) {
                    requestBody.ai_api_key = apiKey;
                }
                const model = aiModelInput ? aiModelInput.value.trim() : '';
                if (model) {
                    requestBody.ai_model = model;
                }
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
                disabledFields = {};  // Reset disabled fields for new reference
                currentTargetLang = targetLang || null;  // Store the target language used
                
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
                // Display Harvard citation
                harvardOutput.textContent = data.harvard;
                
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
    
    /**
     * Regenerate citation from current selections
     */
    function regenerateCitation() {
        // Build citations locally from current selections without network request
        if (!currentMultiSource || !currentSelections) {
            return;
        }

        // Get the selected value for each field from the multi-source data
        // Skip disabled fields
        const selectedValues = {};
        const fields = ['title', 'author', 'date', 'site', 'publisher', 'language', 'url'];
        
        fields.forEach(field => {
            // Skip disabled fields
            if (disabledFields[field]) {
                return;
            }
            
            const source = currentSelections[field];
            if (source === 'custom' && customValues[field]) {
                // Use custom value if custom is selected
                selectedValues[field] = customValues[field];
            } else if (source && currentMultiSource[field] && currentMultiSource[field][source]) {
                selectedValues[field] = currentMultiSource[field][source];
            }
        });

        // Get archive info and translated title - use custom values if set, otherwise use currentFields
        // Skip if disabled
        const archiveUrl = disabledFields['archive_url'] ? null : (customValues['archive_url'] || currentFields?.archive_url);
        const archiveDate = disabledFields['archive_date'] ? null : (customValues['archive_date'] || currentFields?.archive_date);
        const translatedTitle = disabledFields['translated_title'] ? null : (customValues['translated_title'] || currentFields?.translated_title);

        // Build Wiki citation (always build as multiline, then format according to setting)
        const wiki = buildWikiCitation(selectedValues, archiveUrl, archiveDate, translatedTitle);
        currentWikiCitation = wiki;
        wikiOutput.textContent = formatWikiCitation(wiki, getWikiFormat());

        // Build BibTeX citation
        const bibtex = buildBibTeXCitation(selectedValues, archiveUrl, archiveDate, translatedTitle);
        bibtexOutput.textContent = bibtex;

        // Build Harvard citation
        const harvard = buildHarvardCitation(selectedValues, archiveUrl, archiveDate);
        harvardOutput.textContent = harvard;
    }
    
    /**
     * Display the multi-source table
     */
    function displayMultiSourceTable(multiSource, selections, fields) {
        // Clear existing content
        metadataFields.innerHTML = '';
        
        if (!multiSource) {
            return;
        }
        
        // Add "Metadata values" section header
        metadataFields.appendChild(createSectionHeader('Metadata values'));
        
        // Add metadata field rows
        METADATA_FIELD_DEFS.forEach(fieldDef => {
            metadataFields.appendChild(createFieldRow(
                fieldDef, 
                multiSource, 
                selections, 
                customValues, 
                openSourceModal,
                disabledFields,
                toggleField
            ));
        });

        // Always add "Additional values" section for archive info and translated title
        metadataFields.appendChild(createSectionHeader('Additional values'));

        // Add translated title row (editable) - include target language in label if available
        const translatedTitle = fields ? fields.translated_title : null;
        const translatedTitleLabel = currentTargetLang 
            ? `Translated Title (${currentTargetLang.toUpperCase()})` 
            : 'Translated Title';
        metadataFields.appendChild(createEditableFieldRow(
            { key: 'translated_title', label: translatedTitleLabel }, 
            translatedTitle, 
            customValues,
            openAdditionalFieldModal,
            true,
            disabledFields,
            toggleField
        ));

        // Add archive URL row (always shown, either with value or pending state)
        const archiveUrl = fields ? fields.archive_url : null;
        const archiveDate = fields ? fields.archive_date : null;
        metadataFields.appendChild(createArchiveUrlRow(
            archiveUrl, 
            archiveDate, 
            customValues,
            openAdditionalFieldModal,
            createArchive,
            disabledFields,
            toggleField
        ));

        // Add archive date row (editable)
        metadataFields.appendChild(createEditableFieldRow(
            { key: 'archive_date', label: 'Archive Date' }, 
            archiveDate, 
            customValues,
            openAdditionalFieldModal,
            true,
            disabledFields,
            toggleField
        ));
    }
    
    /**
     * Display simple fields table (fallback)
     */
    function displayFieldsTable(fields) {
        // Clear existing content
        metadataFields.innerHTML = '';
        
        // Add "Metadata values" section header
        metadataFields.appendChild(createSectionHeader('Metadata values'));

        // Add metadata field rows
        METADATA_FIELD_DEFS.forEach(fieldDef => {
            const row = createSimpleFieldRow(fieldDef, fields[fieldDef.key]);
            metadataFields.appendChild(row);
        });

        // Always add "Additional values" section for archive info and translated title
        metadataFields.appendChild(createSectionHeader('Additional values'));

        // Add translated title row if available - include target language in label
        if (fields.translated_title) {
            const translatedTitleLabel = currentTargetLang 
                ? `Translated Title (${currentTargetLang.toUpperCase()})` 
                : 'Translated Title';
            const translatedTitleRow = createSimpleFieldRow(
                { key: 'translated_title', label: translatedTitleLabel }, 
                fields.translated_title
            );
            metadataFields.appendChild(translatedTitleRow);
        }

        // Add archive URL row (always shown, either with value or pending state)
        metadataFields.appendChild(createSimpleArchiveUrlRow(
            fields.archive_url, 
            fields.archive_date,
            createArchive
        ));

        // Add archive date row if available
        if (fields.archive_date) {
            const archiveDateRow = createSimpleFieldRow(
                { key: 'archive_date', label: 'Archive Date' }, 
                fields.archive_date
            );
            metadataFields.appendChild(archiveDateRow);
        }
    }
    
    /**
     * Open source selection modal
     */
    function openSourceModal(fieldKey, fieldLabel) {
        modalManager.openSourceModal(fieldKey, fieldLabel, currentMultiSource, currentSelections, customValues);
    }
    
    /**
     * Open additional field modal
     */
    function openAdditionalFieldModal(fieldKey, fieldLabel, originalValue) {
        modalManager.openAdditionalFieldModal(fieldKey, fieldLabel, originalValue, customValues);
    }
    
    /**
     * Toggle a field on/off
     */
    function toggleField(fieldKey, isDisabled) {
        disabledFields[fieldKey] = isDisabled;
        updateFieldToggleState(fieldKey, isDisabled);
        regenerateCitation();
    }
    
    /**
     * Select a source from the modal
     */
    function selectSource(fieldKey, source) {
        if (currentSelections) {
            currentSelections[fieldKey] = source;
        }
        
        // Update the field display
        updateFieldDisplay(fieldKey, currentMultiSource, currentSelections, customValues);
        
        // Close modal
        modalManager.close();
        
        // Regenerate citation
        regenerateCitation();
    }
    
    /**
     * Revert to original value (for additional fields)
     */
    function revertToOriginal(fieldKey, originalValue) {
        delete customValues[fieldKey];
        updateAdditionalFieldDisplay(fieldKey, originalValue, customValues, openAdditionalFieldModal);
        regenerateCitation();
    }
    
    /**
     * Apply custom value from modal
     */
    function applyCustomValue(fieldKey, value) {
        const additionalFields = ['translated_title', 'archive_date', 'archive_url'];
        
        customValues[fieldKey] = value;
        
        if (additionalFields.includes(fieldKey)) {
            // Handle additional fields differently
            const originalValue = currentFields ? currentFields[fieldKey] : null;
            updateAdditionalFieldDisplay(fieldKey, originalValue, customValues, openAdditionalFieldModal);
            modalManager.close();
            regenerateCitation();
        } else {
            // Regular metadata field
            selectSource(fieldKey, 'custom');
        }
    }
    
    /**
     * Create an archive for the current URL
     */
    async function createArchive() {
        if (!currentUrl) return;

        // Show loading state
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
                showArchiveAvailable(data.archive_url, data.archive_date, currentFields);
                
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
    
    /**
     * UI helper functions
     */
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
