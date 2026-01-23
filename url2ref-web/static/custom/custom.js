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
    const btnText = document.getElementById('btn-text');
    const btnSpinner = document.getElementById('btn-spinner');
    const errorAlert = document.getElementById('error-alert');
    const errorMessage = document.getElementById('error-message');
    const detailsSection = document.getElementById('details-section');
    const resultsSection = document.getElementById('results-section');
    const bibtexOutput = document.getElementById('bibtex-output');
    const wikiOutput = document.getElementById('wiki-output');
    const createArchiveBtn = document.getElementById('create-archive-btn');

    // State
    let currentUrl = '';

    // Generate reference on button click
    generateBtn.addEventListener('click', generateReference);

    // Generate reference on Enter key
    urlInput.addEventListener('keypress', function(e) {
        if (e.key === 'Enter') {
            generateReference();
        }
    });

    // Create archive button
    createArchiveBtn.addEventListener('click', createArchive);

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
            const response = await fetch('/api/generate', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({ url: url }),
            });

            const data = await response.json();

            if (data.success) {
                // Display extracted fields
                displayFields(data.fields);
                
                // Display citations
                bibtexOutput.textContent = data.bibtex;
                wikiOutput.textContent = data.wiki;
                
                // Handle archive status
                if (data.archive_status === 'available') {
                    showArchiveAvailable(data.fields.archive_url, data.fields.archive_date);
                } else {
                    showArchiveNotFound();
                }
                
                showDetails();
                showResults();
            } else {
                showError(data.error || 'Failed to generate reference');
            }
        } catch (error) {
            showError('Network error. Please try again.');
            console.error('Error:', error);
        } finally {
            setLoading(false);
        }
    }

    async function createArchive() {
        if (!currentUrl) return;

        // Show loading state for archive fields
        showArchiveLoading();
        createArchiveBtn.disabled = true;

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
                // Update archive fields
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
        } finally {
            createArchiveBtn.disabled = false;
        }
    }

    function displayFields(fields) {
        const fieldMappings = [
            { id: 'title', value: fields.title },
            { id: 'author', value: fields.author },
            { id: 'date', value: fields.date },
            { id: 'site', value: fields.site },
            { id: 'publisher', value: fields.publisher },
            { id: 'language', value: fields.language },
            { id: 'url', value: fields.url },
        ];

        fieldMappings.forEach(field => {
            const row = document.getElementById(`field-${field.id}`);
            const valueCell = document.getElementById(`field-${field.id}-value`);
            
            if (field.value) {
                valueCell.textContent = field.value;
                row.classList.remove('d-none');
            } else {
                row.classList.add('d-none');
            }
        });
    }

    function showArchiveAvailable(archiveUrl, archiveDate) {
        // Archive URL
        const urlText = document.getElementById('archive-url-text');
        const urlLoading = document.getElementById('archive-url-loading');
        const urlPending = document.getElementById('archive-url-pending');
        
        urlText.innerHTML = `<a href="${archiveUrl}" target="_blank" class="text-info">${archiveUrl}</a>`;
        urlText.classList.remove('d-none');
        urlLoading.classList.add('d-none');
        urlPending.classList.add('d-none');

        // Archive Date
        const dateText = document.getElementById('archive-date-text');
        const dateLoading = document.getElementById('archive-date-loading');
        const datePending = document.getElementById('archive-date-pending');
        
        dateText.textContent = archiveDate || '';
        dateText.classList.remove('d-none');
        dateLoading.classList.add('d-none');
        datePending.classList.add('d-none');
    }

    function showArchiveNotFound() {
        // Archive URL
        const urlText = document.getElementById('archive-url-text');
        const urlLoading = document.getElementById('archive-url-loading');
        const urlPending = document.getElementById('archive-url-pending');
        
        urlText.classList.add('d-none');
        urlLoading.classList.add('d-none');
        urlPending.classList.remove('d-none');

        // Archive Date
        const dateText = document.getElementById('archive-date-text');
        const dateLoading = document.getElementById('archive-date-loading');
        const datePending = document.getElementById('archive-date-pending');
        
        dateText.classList.add('d-none');
        dateLoading.classList.add('d-none');
        datePending.classList.remove('d-none');
    }

    function showArchiveLoading() {
        // Archive URL
        const urlText = document.getElementById('archive-url-text');
        const urlLoading = document.getElementById('archive-url-loading');
        const urlPending = document.getElementById('archive-url-pending');
        
        urlText.classList.add('d-none');
        urlLoading.classList.remove('d-none');
        urlPending.classList.add('d-none');

        // Archive Date
        const dateText = document.getElementById('archive-date-text');
        const dateLoading = document.getElementById('archive-date-loading');
        const datePending = document.getElementById('archive-date-pending');
        
        dateText.classList.add('d-none');
        dateLoading.classList.remove('d-none');
        datePending.classList.add('d-none');
    }

    function showArchiveError(message) {
        // Archive URL - show error state
        const urlText = document.getElementById('archive-url-text');
        const urlLoading = document.getElementById('archive-url-loading');
        const urlPending = document.getElementById('archive-url-pending');
        
        urlText.innerHTML = `<span class="text-danger"><i class="bi bi-exclamation-triangle me-1"></i>${message}</span>`;
        urlText.classList.remove('d-none');
        urlLoading.classList.add('d-none');
        urlPending.classList.add('d-none');

        // Archive Date
        const dateText = document.getElementById('archive-date-text');
        const dateLoading = document.getElementById('archive-date-loading');
        const datePending = document.getElementById('archive-date-pending');
        
        dateText.textContent = '—';
        dateText.classList.remove('d-none');
        dateLoading.classList.add('d-none');
        datePending.classList.add('d-none');
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