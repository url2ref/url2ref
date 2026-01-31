/**
 * Utility functions for url2ref
 * Includes: tooltips, clipboard, theme switcher, URL validation, text helpers
 */

// Initialize Bootstrap tooltips
function initTooltips() {
    var tooltipTriggerList = [].slice.call(document.querySelectorAll('[data-bs-toggle="tooltip"]'));
    var tooltipList = tooltipTriggerList.map(function (tooltipTriggerEl) {
        return new bootstrap.Tooltip(tooltipTriggerEl);
    });
}

// Copy text to clipboard
function copyToClipboard(button, outputId) {
    const output = document.getElementById(outputId);
    if (!output) return;
    
    const text = output.textContent;
    
    navigator.clipboard.writeText(text).then(function() {
        // Visual feedback
        const icon = button.querySelector('i');
        if (icon) {
            const originalClass = icon.className;
            icon.className = 'bi bi-check';
            
            setTimeout(function() {
                icon.className = originalClass;
            }, 2000);
        }
    }).catch(function(err) {
        console.error('Failed to copy: ', err);
    });
}

// Initialize theme switcher
function initThemeSwitcher() {
    const themeSwitch = document.getElementById('theme-switch');
    if (!themeSwitch) return;
    
    // Check for saved theme preference or default to system preference
    const savedTheme = localStorage.getItem('theme');
    const systemPrefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
    
    if (savedTheme === 'dark' || (!savedTheme && systemPrefersDark)) {
        document.documentElement.setAttribute('data-bs-theme', 'dark');
        themeSwitch.checked = true;
    } else {
        document.documentElement.setAttribute('data-bs-theme', 'light');
        themeSwitch.checked = false;
    }
    
    themeSwitch.addEventListener('change', function() {
        if (this.checked) {
            document.documentElement.setAttribute('data-bs-theme', 'dark');
            localStorage.setItem('theme', 'dark');
        } else {
            document.documentElement.setAttribute('data-bs-theme', 'light');
            localStorage.setItem('theme', 'light');
        }
    });
}

// Check if a string is a valid URL
function isValidUrl(string) {
    try {
        const url = new URL(string);
        return url.protocol === 'http:' || url.protocol === 'https:';
    } catch (_) {
        return false;
    }
}

// Truncate text with ellipsis
function truncateText(text, maxLength) {
    if (text.length <= maxLength) {
        return text;
    }
    return text.substring(0, maxLength) + '...';
}

// Export functions for use in other modules
window.url2refUtils = {
    initTooltips,
    copyToClipboard,
    initThemeSwitcher,
    isValidUrl,
    truncateText
};
