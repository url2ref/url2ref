/**
 * Citation building module for url2ref
 * Includes: Wikipedia and BibTeX citation formatting
 */

/**
 * Get the current wiki format setting from the radio buttons
 */
function getWikiFormat() {
    const checked = document.querySelector('input[name="wiki-format"]:checked');
    return checked ? checked.value : 'multiline';
}

/**
 * Format wiki citation according to the selected format
 * @param {string} citation - The multiline citation string
 * @param {string} format - 'multiline' or 'singleline'
 * @returns {string} The formatted citation
 */
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

/**
 * Build a Wiki citation from selected values (always returns multiline format)
 * @param {Object} values - Object with title, author, date, site, publisher, language, url
 * @param {string} archiveUrl - Archive URL if available
 * @param {string} archiveDate - Archive date if available
 * @param {string} translatedTitle - Translated title if available
 * @returns {string} The Wikipedia citation template
 */
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

/**
 * Build a BibTeX citation from selected values
 * @param {Object} values - Object with title, author, date, site, publisher, url
 * @param {string} archiveUrl - Archive URL if available
 * @param {string} archiveDate - Archive date if available
 * @param {string} translatedTitle - Translated title if available
 * @returns {string} The BibTeX citation
 */
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

/**
 * Build a Harvard citation from selected values
 * Harvard style: Author (Year) 'Title', Site/Publisher. Available at: URL (Accessed: Date).
 * @param {Object} values - Object with title, author, date, site, publisher, url
 * @param {string} archiveUrl - Archive URL if available  
 * @param {string} archiveDate - Archive date (used as access date) if available
 * @returns {string} The Harvard citation
 */
function buildHarvardCitation(values, archiveUrl, archiveDate) {
    let result = '';
    
    // Format author name: "LastName, F." style
    const formatAuthorName = (author) => {
        if (!author) return null;
        const nameParts = author.trim().split(/\s+/);
        if (nameParts.length > 1) {
            const lastName = nameParts[nameParts.length - 1];
            const firstNames = nameParts.slice(0, -1);
            const initials = firstNames.map(n => n.charAt(0) + '.').join('');
            return `${lastName}, ${initials}`;
        }
        return author;
    };
    
    // Extract year from date string
    const extractYear = (date) => {
        if (!date) return null;
        const match = date.match(/\d{4}/);
        return match ? match[0] : null;
    };
    
    // Format access date: "1 January 2024" style
    const formatAccessDate = (date) => {
        if (!date) return null;
        const months = ['January', 'February', 'March', 'April', 'May', 'June', 
                       'July', 'August', 'September', 'October', 'November', 'December'];
        // Try to parse YYYY-MM-DD format
        const match = date.match(/(\d{4})-(\d{2})-(\d{2})/);
        if (match) {
            const day = parseInt(match[3], 10);
            const month = months[parseInt(match[2], 10) - 1];
            const year = match[1];
            return `${day} ${month} ${year}`;
        }
        return date;
    };
    
    const author = formatAuthorName(values.author);
    const year = extractYear(values.date);
    
    // Author and year
    if (author && year) {
        result += `${author} (${year})`;
    } else if (author) {
        result += `${author} (n.d.)`;
    } else if (year) {
        result += `(${year})`;
    } else {
        result += '(n.d.)';
    }
    
    // Title (in single quotes for web pages)
    if (values.title) {
        result += ` '${values.title}'`;
    }
    
    // Site or Publisher
    const source = values.site || values.publisher;
    if (source) {
        result += `, ${source}`;
    }
    
    result += '.';
    
    // URL
    if (values.url) {
        result += ` Available at: ${values.url}`;
    }
    
    // Access date
    if (archiveDate) {
        const formattedDate = formatAccessDate(archiveDate);
        result += ` (Accessed: ${formattedDate}).`;
    }
    
    return result;
}

// Export functions for use in other modules
window.url2refCitation = {
    getWikiFormat,
    formatWikiCitation,
    buildWikiCitation,
    buildBibTeXCitation,
    buildHarvardCitation
};
