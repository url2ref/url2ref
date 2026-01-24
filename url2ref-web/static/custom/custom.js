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
    const metadataTbody = document.getElementById('metadata-tbody');

    // State
    let currentUrl = '';
    let currentMultiSource = null;
    let currentSelections = null;
    let currentFields = null;  // Store archive URL/date and other fields

    // Generate reference on button click
    generateBtn.addEventListener('click', generateReference);

    // Generate reference on Enter key
    urlInput.addEventListener('keypress', function(e) {
        if (e.key === 'Enter') {
            generateReference();
        }
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
                
                // Display extracted fields - use multi-source table or fall back to fields
                if (data.multi_source && hasAnyMultiSourceData(data.multi_source)) {
                    displayMultiSourceTable(data.multi_source, data.selections, data.fields);
                } else if (data.fields) {
                    // Fall back to displaying fields as simple table
                    displayFieldsTable(data.fields);
                }
                
                // Display citations
                bibtexOutput.textContent = data.bibtex;
                wikiOutput.textContent = data.wiki;
                
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
            if (source && currentMultiSource[field] && currentMultiSource[field][source]) {
                selectedValues[field] = currentMultiSource[field][source];
            }
        });

        // Get archive info from currentFields (if available)
        const archiveUrl = currentFields?.archive_url;
        const archiveDate = currentFields?.archive_date;

        // Build Wiki citation
        const wiki = buildWikiCitation(selectedValues, archiveUrl, archiveDate);
        wikiOutput.textContent = wiki;

        // Build BibTeX citation
        const bibtex = buildBibTeXCitation(selectedValues, archiveUrl, archiveDate);
        bibtexOutput.textContent = bibtex;
    }

    // Build a Wiki citation from selected values
    function buildWikiCitation(values, archiveUrl, archiveDate) {
        let parts = [];
        
        if (values.title) parts.push(`|title=${values.title}`);
        if (values.author) {
            // Handle author formatting - split into first/last if single author
            const author = values.author.trim();
            const nameParts = author.split(/\s+/);
            if (nameParts.length > 1) {
                const lastName = nameParts[nameParts.length - 1];
                const firstNames = nameParts.slice(0, -1).join(' ');
                parts.push(`|last=${lastName}|first=${firstNames}`);
            } else {
                parts.push(`|author=${author}`);
            }
        }
        if (values.date) parts.push(`|date=${values.date}`);
        if (values.site) parts.push(`|site=${values.site}`);
        if (values.publisher) parts.push(`|publisher=${values.publisher}`);
        if (values.language) parts.push(`|language=${values.language}`);
        if (values.url) parts.push(`|url=${values.url}`);
        if (archiveUrl) parts.push(`|archive-url=${archiveUrl}`);
        if (archiveDate) parts.push(`|archive-date=${archiveDate}`);
        
        return `{{cite web ${parts.join(' ')} }}`;
    }

    // Build a BibTeX citation from selected values
    function buildBibTeXCitation(values, archiveUrl, archiveDate) {
        let lines = ['@misc{ url2ref,'];
        
        if (values.title) lines.push(`title = "${values.title}",`);
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
        // Clear existing rows
        metadataTbody.innerHTML = '';
        
        if (!multiSource) {
            return;
        }

        // Field definitions for metadata section
        const metadataFields = [
            { key: 'title', label: 'Title' },
            { key: 'author', label: 'Author' },
            { key: 'date', label: 'Date' },
            { key: 'site', label: 'Site' },
            { key: 'publisher', label: 'Publisher' },
            { key: 'language', label: 'Language' },
            { key: 'url', label: 'URL' }
        ];

        // Additional values (archive data) - these come from fields, not multi-source
        const additionalFieldDefs = [
            { key: 'archive_url', label: 'Archive URL' },
            { key: 'archive_date', label: 'Archive Date' }
        ];

        // Note: JSON field names match Rust struct field names (no underscores)
        const sources = ['opengraph', 'schemaorg', 'htmlmeta', 'doi'];
        const sourceLabels = {
            'opengraph': 'Open Graph',
            'schemaorg': 'Schema.org',
            'htmlmeta': 'HTML Meta',
            'doi': 'DOI'
        };

        // Helper function to create a section header row
        function createSectionHeader(title) {
            const row = document.createElement('tr');
            row.classList.add('section-header');
            const headerCell = document.createElement('th');
            headerCell.setAttribute('colspan', '5');
            headerCell.textContent = title;
            row.appendChild(headerCell);
            return row;
        }

        // Helper function to create multi-source field rows
        function createFieldRows(fieldDefs) {
            const rows = [];
            fieldDefs.forEach(field => {
                const fieldData = multiSource[field.key];
                
                // Check if this field has any data from any source
                if (!fieldData) {
                    return; // Skip empty fields
                }

                const hasAnyValue = sources.some(src => fieldData[src]);
                if (!hasAnyValue) {
                    return; // Skip if no values from any source
                }

                const row = document.createElement('tr');
                row.setAttribute('data-field', field.key);

                // Field label cell
                const labelCell = document.createElement('th');
                labelCell.setAttribute('scope', 'row');
                labelCell.textContent = field.label;
                row.appendChild(labelCell);

                // Source cells
                sources.forEach(source => {
                    const cell = document.createElement('td');
                    cell.setAttribute('data-field', field.key);
                    cell.setAttribute('data-source', source);
                    
                    const value = fieldData[source];
                    const isSelected = selections && selections[field.key] === source;

                    if (value) {
                        cell.textContent = truncateText(value, 50);
                        cell.title = value; // Full value on hover
                        cell.classList.add('selectable-cell');
                        
                        if (isSelected) {
                            cell.classList.add('selected');
                        }
                        
                        // Make cell clickable
                        cell.addEventListener('click', () => selectCell(field.key, source, cell));
                    } else {
                        cell.textContent = '—';
                        cell.classList.add('text-muted', 'empty-cell');
                    }

                    row.appendChild(cell);
                });

                rows.push(row);
            });
            return rows;
        }

        // Helper function to create a simple field row (for additional values)
        function createSimpleFieldRow(fieldDef, value) {
            if (!value) return null;

            const row = document.createElement('tr');
            row.setAttribute('data-field', fieldDef.key);

            // Field label cell
            const labelCell = document.createElement('th');
            labelCell.setAttribute('scope', 'row');
            labelCell.textContent = fieldDef.label;
            row.appendChild(labelCell);

            // Value cell (spans all source columns)
            const valueCell = document.createElement('td');
            valueCell.setAttribute('colspan', '4');
            valueCell.textContent = truncateText(value, 100);
            valueCell.title = value;
            valueCell.classList.add('selected');
            row.appendChild(valueCell);

            return row;
        }

        // Helper function to create archive URL row with status
        function createArchiveUrlRow(archiveUrl, archiveDate) {
            const row = document.createElement('tr');
            row.setAttribute('data-field', 'archive_url');
            row.id = 'archive-url-row';

            // Field label cell
            const labelCell = document.createElement('th');
            labelCell.setAttribute('scope', 'row');
            labelCell.textContent = 'Archive URL';
            row.appendChild(labelCell);

            // Value cell (spans all source columns)
            const valueCell = document.createElement('td');
            valueCell.setAttribute('colspan', '4');
            valueCell.id = 'archive-url-cell';

            if (archiveUrl) {
                // Archive available - show the URL
                const displayText = archiveDate ? `${archiveUrl} (${archiveDate})` : archiveUrl;
                valueCell.innerHTML = `<a href="${archiveUrl}" target="_blank" class="text-info">${truncateText(displayText, 80)}</a>`;
                valueCell.title = displayText;
                valueCell.classList.add('selected');
            } else {
                // No archive - show pending state with create button
                valueCell.innerHTML = `
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
                const createBtn = valueCell.querySelector('.create-archive-btn');
                if (createBtn) {
                    createBtn.addEventListener('click', createArchive);
                }
            }

            row.appendChild(valueCell);
            return row;
        }

        // Add "Metadata values" section header
        metadataTbody.appendChild(createSectionHeader('Metadata values'));
        
        // Add metadata field rows
        const metadataRows = createFieldRows(metadataFields);
        metadataRows.forEach(row => metadataTbody.appendChild(row));

        // Always add "Additional values" section for archive info
        metadataTbody.appendChild(createSectionHeader('Additional values'));

        // Add archive URL row (always shown, either with value or pending state)
        const archiveUrl = fields ? fields.archive_url : null;
        const archiveDate = fields ? fields.archive_date : null;
        metadataTbody.appendChild(createArchiveUrlRow(archiveUrl, archiveDate));

        // Add archive date row if available
        if (archiveDate) {
            const archiveDateRow = createSimpleFieldRow({ key: 'archive_date', label: 'Archive Date' }, archiveDate);
            if (archiveDateRow) metadataTbody.appendChild(archiveDateRow);
        }
    }

    function selectCell(fieldKey, source, cell) {
        // Update the selection for this field
        if (currentSelections) {
            currentSelections[fieldKey] = source;
        }

        // Update UI - deselect all cells in this row, select the clicked one
        const row = cell.closest('tr');
        row.querySelectorAll('.selectable-cell').forEach(c => {
            c.classList.remove('selected');
        });
        cell.classList.add('selected');

        // Automatically update the displayed citations
        regenerateCitation();
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
        // Clear existing rows
        metadataTbody.innerHTML = '';
        
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

        // Helper function to create a section header row
        function createSectionHeader(title) {
            const row = document.createElement('tr');
            row.classList.add('section-header');
            const headerCell = document.createElement('th');
            headerCell.setAttribute('colspan', '5');
            headerCell.textContent = title;
            row.appendChild(headerCell);
            return row;
        }

        // Helper function to create a field row
        function createFieldRow(fieldDef, value) {
            if (!value) return null;

            const row = document.createElement('tr');
            row.setAttribute('data-field', fieldDef.key);

            // Field label cell
            const labelCell = document.createElement('th');
            labelCell.setAttribute('scope', 'row');
            labelCell.textContent = fieldDef.label;
            row.appendChild(labelCell);

            // Value cell (spans all source columns)
            const valueCell = document.createElement('td');
            valueCell.setAttribute('colspan', '4');
            valueCell.textContent = truncateText(value, 100);
            valueCell.title = value;
            valueCell.classList.add('selected'); // Mark as selected since it's the only option
            row.appendChild(valueCell);

            return row;
        }

        // Helper function to create archive URL row with status
        function createArchiveUrlRow(archiveUrl, archiveDate) {
            const row = document.createElement('tr');
            row.setAttribute('data-field', 'archive_url');
            row.id = 'archive-url-row';

            // Field label cell
            const labelCell = document.createElement('th');
            labelCell.setAttribute('scope', 'row');
            labelCell.textContent = 'Archive URL';
            row.appendChild(labelCell);

            // Value cell (spans all source columns)
            const valueCell = document.createElement('td');
            valueCell.setAttribute('colspan', '4');
            valueCell.id = 'archive-url-cell';

            if (archiveUrl) {
                // Archive available - show the URL
                const displayText = archiveDate ? `${archiveUrl} (${archiveDate})` : archiveUrl;
                valueCell.innerHTML = `<a href="${archiveUrl}" target="_blank" class="text-info">${truncateText(displayText, 80)}</a>`;
                valueCell.title = displayText;
                valueCell.classList.add('selected');
            } else {
                // No archive - show pending state with create button
                valueCell.innerHTML = `
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
                const createBtn = valueCell.querySelector('.create-archive-btn');
                if (createBtn) {
                    createBtn.addEventListener('click', createArchive);
                }
            }

            row.appendChild(valueCell);
            return row;
        }

        // Add "Metadata values" section header
        metadataTbody.appendChild(createSectionHeader('Metadata values'));

        // Add metadata field rows
        metadataFieldDefs.forEach(fieldDef => {
            const row = createFieldRow(fieldDef, fields[fieldDef.key]);
            if (row) metadataTbody.appendChild(row);
        });

        // Always add "Additional values" section for archive info
        metadataTbody.appendChild(createSectionHeader('Additional values'));

        // Add archive URL row (always shown, either with value or pending state)
        metadataTbody.appendChild(createArchiveUrlRow(fields.archive_url, fields.archive_date));

        // Add archive date row if available (separate from URL row)
        if (fields.archive_date) {
            const archiveDateRow = createFieldRow({ key: 'archive_date', label: 'Archive Date' }, fields.archive_date);
            if (archiveDateRow) metadataTbody.appendChild(archiveDateRow);
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
        // Update the archive URL cell in the table
        const archiveCell = document.getElementById('archive-url-cell');
        if (archiveCell) {
            const displayText = archiveDate ? `${archiveUrl} (${archiveDate})` : archiveUrl;
            archiveCell.innerHTML = `<a href="${archiveUrl}" target="_blank" class="text-info">${truncateText(displayText, 80)}</a>`;
            archiveCell.title = displayText;
            archiveCell.classList.add('selected');
        }
        
        // Add archive date row if not already present and date is available
        if (archiveDate) {
            const existingDateRow = document.querySelector('tr[data-field="archive_date"]');
            if (!existingDateRow) {
                const row = document.createElement('tr');
                row.setAttribute('data-field', 'archive_date');
                
                const labelCell = document.createElement('th');
                labelCell.setAttribute('scope', 'row');
                labelCell.textContent = 'Archive Date';
                row.appendChild(labelCell);
                
                const valueCell = document.createElement('td');
                valueCell.setAttribute('colspan', '4');
                valueCell.textContent = archiveDate;
                valueCell.classList.add('selected');
                row.appendChild(valueCell);
                
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