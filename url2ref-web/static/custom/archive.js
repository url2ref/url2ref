/**
 * Archive handling module for url2ref
 * Includes: archive creation and status display
 */

/**
 * Create an archive for the current URL via the Wayback Machine
 * @param {string} url - The URL to archive
 * @param {Object} callbacks - Object with showLoading, showAvailable, showError callbacks
 * @param {Object} elements - Object with bibtexOutput, wikiOutput elements
 */
async function createArchive(url, callbacks, elements) {
    if (!url) return;

    // Show loading state
    callbacks.showLoading();

    try {
        const response = await fetch('/api/archive', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ url: url }),
        });

        const data = await response.json();

        if (data.success && data.archive_url) {
            // Update archive fields in the table
            callbacks.showAvailable(data.archive_url, data.archive_date);
            
            // Update citations with archive info
            if (data.bibtex && elements.bibtexOutput) {
                elements.bibtexOutput.textContent = data.bibtex;
            }
            if (data.wiki && elements.wikiOutput) {
                elements.wikiOutput.textContent = data.wiki;
            }
        } else {
            callbacks.showError(data.error || 'Failed to create archive');
        }
    } catch (error) {
        callbacks.showError('Network error while creating archive');
        console.error('Error:', error);
    }
}

/**
 * Update the archive URL cell to show loading state
 */
function showArchiveLoading() {
    const archiveCell = document.getElementById('archive-url-cell');
    if (archiveCell) {
        const pending = archiveCell.querySelector('.archive-pending');
        const loading = archiveCell.querySelector('.archive-loading');
        if (pending) pending.classList.add('d-none');
        if (loading) loading.classList.remove('d-none');
    }
}

/**
 * Update the archive cell to show the archive is available
 * @param {string} archiveUrl - The archive URL
 * @param {string} archiveDate - The archive date
 * @param {Object} currentFields - The current fields object to update
 */
function showArchiveAvailable(archiveUrl, archiveDate, currentFields) {
    const truncateText = window.url2refUtils?.truncateText || ((t, l) => t.length <= l ? t : t.substring(0, l) + '...');
    
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
    
    // Update or add archive date row if date is available
    if (archiveDate) {
        const existingDateRow = document.querySelector('.field-row[data-field="archive_date"]');
        if (existingDateRow) {
            // Update the existing row's value
            const valueSpan = existingDateRow.querySelector('.field-value');
            if (valueSpan) {
                valueSpan.textContent = archiveDate;
                valueSpan.title = archiveDate;
                valueSpan.classList.remove('empty-value');
            }
        } else {
            // Create a new row
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

/**
 * Show an error message in the archive cell
 * @param {string} message - The error message to display
 */
function showArchiveError(message) {
    const archiveCell = document.getElementById('archive-url-cell');
    if (archiveCell) {
        archiveCell.innerHTML = `<span class="text-danger"><i class="bi bi-exclamation-triangle me-1"></i>${message}</span>`;
    }
}

// Export functions for use in other modules
window.url2refArchive = {
    createArchive,
    showArchiveLoading,
    showArchiveAvailable,
    showArchiveError
};
