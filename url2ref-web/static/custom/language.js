/**
 * Language handling module for url2ref
 * Includes: language data, lookup, validation, and browser preselection
 */

// Comprehensive language database with ISO 639-1 codes, English names, and native names
const LANGUAGES = {
    'AF': { english: 'Afrikaans', native: 'Afrikaans' },
    'SQ': { english: 'Albanian', native: 'Shqip' },
    'AM': { english: 'Amharic', native: 'አማርኛ' },
    'AR': { english: 'Arabic', native: 'العربية' },
    'HY': { english: 'Armenian', native: 'Հdelays' },
    'AZ': { english: 'Azerbaijani', native: 'Azərbaycan' },
    'EU': { english: 'Basque', native: 'Euskara' },
    'BE': { english: 'Belarusian', native: 'Беларуская' },
    'BN': { english: 'Bengali', native: 'বাংলা' },
    'BS': { english: 'Bosnian', native: 'Bosanski' },
    'BG': { english: 'Bulgarian', native: 'Български' },
    'CA': { english: 'Catalan', native: 'Català' },
    'CEB': { english: 'Cebuano', native: 'Cebuano' },
    'NY': { english: 'Chichewa', native: 'Chichewa' },
    'ZH': { english: 'Chinese', native: '中文' },
    'ZH-CN': { english: 'Chinese (Simplified)', native: '简体中文' },
    'ZH-TW': { english: 'Chinese (Traditional)', native: '繁體中文' },
    'CO': { english: 'Corsican', native: 'Corsu' },
    'HR': { english: 'Croatian', native: 'Hrvatski' },
    'CS': { english: 'Czech', native: 'Čeština' },
    'DA': { english: 'Danish', native: 'Dansk' },
    'NL': { english: 'Dutch', native: 'Nederlands' },
    'EN': { english: 'English', native: 'English' },
    'EN-GB': { english: 'English (UK)', native: 'English (UK)' },
    'EN-US': { english: 'English (US)', native: 'English (US)' },
    'EO': { english: 'Esperanto', native: 'Esperanto' },
    'ET': { english: 'Estonian', native: 'Eesti' },
    'TL': { english: 'Filipino', native: 'Filipino' },
    'FI': { english: 'Finnish', native: 'Suomi' },
    'FR': { english: 'French', native: 'Français' },
    'FY': { english: 'Frisian', native: 'Frysk' },
    'GL': { english: 'Galician', native: 'Galego' },
    'KA': { english: 'Georgian', native: 'ქართული' },
    'DE': { english: 'German', native: 'Deutsch' },
    'EL': { english: 'Greek', native: 'Ελληνικά' },
    'GU': { english: 'Gujarati', native: 'ગુજરાતી' },
    'HT': { english: 'Haitian Creole', native: 'Kreyòl Ayisyen' },
    'HA': { english: 'Hausa', native: 'Hausa' },
    'HAW': { english: 'Hawaiian', native: 'ʻŌlelo Hawaiʻi' },
    'HE': { english: 'Hebrew', native: 'עברית' },
    'IW': { english: 'Hebrew', native: 'עברית' },
    'HI': { english: 'Hindi', native: 'हिन्दी' },
    'HMN': { english: 'Hmong', native: 'Hmong' },
    'HU': { english: 'Hungarian', native: 'Magyar' },
    'IS': { english: 'Icelandic', native: 'Íslenska' },
    'IG': { english: 'Igbo', native: 'Igbo' },
    'ID': { english: 'Indonesian', native: 'Bahasa Indonesia' },
    'GA': { english: 'Irish', native: 'Gaeilge' },
    'IT': { english: 'Italian', native: 'Italiano' },
    'JA': { english: 'Japanese', native: '日本語' },
    'JV': { english: 'Javanese', native: 'Basa Jawa' },
    'KN': { english: 'Kannada', native: 'ಕನ್ನಡ' },
    'KK': { english: 'Kazakh', native: 'Қазақша' },
    'KM': { english: 'Khmer', native: 'ភាសាខ្មែរ' },
    'RW': { english: 'Kinyarwanda', native: 'Kinyarwanda' },
    'KO': { english: 'Korean', native: '한국어' },
    'KU': { english: 'Kurdish', native: 'Kurdî' },
    'KY': { english: 'Kyrgyz', native: 'Кыргызча' },
    'LO': { english: 'Lao', native: 'ລາວ' },
    'LA': { english: 'Latin', native: 'Latina' },
    'LV': { english: 'Latvian', native: 'Latviešu' },
    'LT': { english: 'Lithuanian', native: 'Lietuvių' },
    'LB': { english: 'Luxembourgish', native: 'Lëtzebuergesch' },
    'MK': { english: 'Macedonian', native: 'Македонски' },
    'MG': { english: 'Malagasy', native: 'Malagasy' },
    'MS': { english: 'Malay', native: 'Bahasa Melayu' },
    'ML': { english: 'Malayalam', native: 'മലയാളം' },
    'MT': { english: 'Maltese', native: 'Malti' },
    'MI': { english: 'Maori', native: 'Te Reo Māori' },
    'MR': { english: 'Marathi', native: 'मराठी' },
    'MN': { english: 'Mongolian', native: 'Монгол' },
    'MY': { english: 'Myanmar (Burmese)', native: 'မြန်မာစာ' },
    'NE': { english: 'Nepali', native: 'नेपाली' },
    'NO': { english: 'Norwegian', native: 'Norsk' },
    'NB': { english: 'Norwegian Bokmål', native: 'Norsk Bokmål' },
    'OR': { english: 'Odia', native: 'ଓଡ଼ିଆ' },
    'PS': { english: 'Pashto', native: 'پښتو' },
    'FA': { english: 'Persian', native: 'فارسی' },
    'PL': { english: 'Polish', native: 'Polski' },
    'PT': { english: 'Portuguese', native: 'Português' },
    'PT-BR': { english: 'Portuguese (Brazil)', native: 'Português (Brasil)' },
    'PT-PT': { english: 'Portuguese (Portugal)', native: 'Português (Portugal)' },
    'PA': { english: 'Punjabi', native: 'ਪੰਜਾਬੀ' },
    'RO': { english: 'Romanian', native: 'Română' },
    'RU': { english: 'Russian', native: 'Русский' },
    'SM': { english: 'Samoan', native: 'Gagana Samoa' },
    'GD': { english: 'Scots Gaelic', native: 'Gàidhlig' },
    'SR': { english: 'Serbian', native: 'Српски' },
    'ST': { english: 'Sesotho', native: 'Sesotho' },
    'SN': { english: 'Shona', native: 'Shona' },
    'SD': { english: 'Sindhi', native: 'سنڌي' },
    'SI': { english: 'Sinhala', native: 'සිංහල' },
    'SK': { english: 'Slovak', native: 'Slovenčina' },
    'SL': { english: 'Slovenian', native: 'Slovenščina' },
    'SO': { english: 'Somali', native: 'Soomaali' },
    'ES': { english: 'Spanish', native: 'Español' },
    'SU': { english: 'Sundanese', native: 'Basa Sunda' },
    'SW': { english: 'Swahili', native: 'Kiswahili' },
    'SV': { english: 'Swedish', native: 'Svenska' },
    'TG': { english: 'Tajik', native: 'Тоҷикӣ' },
    'TA': { english: 'Tamil', native: 'தமிழ்' },
    'TT': { english: 'Tatar', native: 'Татарча' },
    'TE': { english: 'Telugu', native: 'తెలుగు' },
    'TH': { english: 'Thai', native: 'ไทย' },
    'TR': { english: 'Turkish', native: 'Türkçe' },
    'TK': { english: 'Turkmen', native: 'Türkmen' },
    'UK': { english: 'Ukrainian', native: 'Українська' },
    'UR': { english: 'Urdu', native: 'اردو' },
    'UG': { english: 'Uyghur', native: 'ئۇيغۇرچە' },
    'UZ': { english: 'Uzbek', native: 'Oʻzbekcha' },
    'VI': { english: 'Vietnamese', native: 'Tiếng Việt' },
    'CY': { english: 'Welsh', native: 'Cymraeg' },
    'XH': { english: 'Xhosa', native: 'isiXhosa' },
    'YI': { english: 'Yiddish', native: 'ייִדיש' },
    'YO': { english: 'Yoruba', native: 'Yorùbá' },
    'ZU': { english: 'Zulu', native: 'isiZulu' }
};

// DeepL supported languages (uppercase codes)
const DEEPL_LANGUAGES = [
    'AR', 'BG', 'CS', 'DA', 'DE', 'EL', 'EN', 'EN-GB', 'EN-US', 'ES', 'ET', 
    'FI', 'FR', 'HU', 'ID', 'IT', 'JA', 'KO', 'LT', 'LV', 'NB', 'NL', 'PL', 
    'PT', 'PT-BR', 'PT-PT', 'RO', 'RU', 'SK', 'SL', 'SV', 'TR', 'UK', 'ZH'
];

// Google Translate supported languages (uppercase codes for consistency)
const GOOGLE_LANGUAGES = [
    'AF', 'SQ', 'AM', 'AR', 'HY', 'AZ', 'EU', 'BE', 'BN', 'BS', 'BG', 'CA', 
    'CEB', 'NY', 'ZH', 'ZH-CN', 'ZH-TW', 'CO', 'HR', 'CS', 'DA', 'NL', 'EN', 
    'EO', 'ET', 'TL', 'FI', 'FR', 'FY', 'GL', 'KA', 'DE', 'EL', 'GU', 'HT', 
    'HA', 'HAW', 'HE', 'IW', 'HI', 'HMN', 'HU', 'IS', 'IG', 'ID', 'GA', 'IT', 
    'JA', 'JV', 'KN', 'KK', 'KM', 'RW', 'KO', 'KU', 'KY', 'LO', 'LA', 'LV', 
    'LT', 'LB', 'MK', 'MG', 'MS', 'ML', 'MT', 'MI', 'MR', 'MN', 'MY', 'NE', 
    'NO', 'NB', 'OR', 'PS', 'FA', 'PL', 'PT', 'PA', 'RO', 'RU', 'SM', 'GD', 
    'SR', 'ST', 'SN', 'SD', 'SI', 'SK', 'SL', 'SO', 'ES', 'SU', 'SW', 'SV', 
    'TG', 'TA', 'TT', 'TE', 'TH', 'TR', 'TK', 'UK', 'UR', 'UG', 'UZ', 'VI', 
    'CY', 'XH', 'YI', 'YO', 'ZU'
];

// Build lookup map for language validation (code, english name, native name -> code)
const languageLookup = new Map();

Object.entries(LANGUAGES).forEach(([code, data]) => {
    const upperCode = code.toUpperCase();
    languageLookup.set(upperCode, upperCode);
    languageLookup.set(data.english.toUpperCase(), upperCode);
    languageLookup.set(data.native.toUpperCase(), upperCode);
});

/**
 * Populate the language suggestions datalist
 */
function populateLanguageSuggestions() {
    const datalist = document.getElementById('language-suggestions');
    if (!datalist) return;
    
    datalist.innerHTML = '';
    
    Object.entries(LANGUAGES).forEach(([code, data]) => {
        const option = document.createElement('option');
        // Show format: "English Name (Native) - CODE"
        option.value = `${data.english} (${data.native}) - ${code}`;
        datalist.appendChild(option);
    });
}

/**
 * Get the user's preferred language from browser settings
 * Returns the language code if supported, null otherwise
 */
function getUserPreferredLanguage() {
    // Try navigator.language first (most specific)
    const browserLang = navigator.language || navigator.userLanguage;
    console.log('[Language Preselection] Browser language detected:', browserLang);
    
    if (browserLang) {
        // Convert to uppercase for comparison
        const langUpper = browserLang.toUpperCase();
        console.log('[Language Preselection] Checking language code:', langUpper);
        
        // Check if the full code (e.g., EN-US) is in our language list
        if (LANGUAGES[langUpper]) {
            console.log('[Language Preselection] Full code found in LANGUAGES:', langUpper);
            return langUpper;
        }
        
        // Try just the base language code (e.g., EN from EN-US)
        const baseLang = langUpper.split('-')[0];
        console.log('[Language Preselection] Trying base language code:', baseLang);
        if (LANGUAGES[baseLang]) {
            console.log('[Language Preselection] Base code found in LANGUAGES:', baseLang);
            return baseLang;
        }
    }
    
    // Also check navigator.languages array for additional preferences
    if (navigator.languages && navigator.languages.length > 0) {
        console.log('[Language Preselection] Checking navigator.languages array:', navigator.languages);
        for (const lang of navigator.languages) {
            const langUpper = lang.toUpperCase();
            if (LANGUAGES[langUpper]) {
                console.log('[Language Preselection] Found in languages array:', langUpper);
                return langUpper;
            }
            const baseLang = langUpper.split('-')[0];
            if (LANGUAGES[baseLang]) {
                console.log('[Language Preselection] Found base code in languages array:', baseLang);
                return baseLang;
            }
        }
    }
    
    console.log('[Language Preselection] No supported language found');
    return null;
}

/**
 * Preselect the user's language in the input field if supported by the selected provider
 */
function preselectUserLanguage() {
    const langInput = document.getElementById('target-lang-input');
    const providerSelect = document.getElementById('translation-provider');
    
    if (!langInput || !providerSelect) {
        console.log('[Language Preselection] Input elements not found');
        return;
    }
    
    const provider = providerSelect.value;
    console.log('[Language Preselection] Selected provider:', provider);
    
    // Don't preselect if no translation provider is selected
    if (!provider) {
        console.log('[Language Preselection] No provider selected, skipping preselection');
        return;
    }
    
    const userLang = getUserPreferredLanguage();
    console.log('[Language Preselection] User preferred language:', userLang);
    
    if (!userLang) {
        console.log('[Language Preselection] No user language detected');
        return;
    }
    
    // Check if the language is supported by the selected provider
    const supportedLanguages = provider === 'deepl' ? DEEPL_LANGUAGES : GOOGLE_LANGUAGES;
    console.log('[Language Preselection] Checking if', userLang, 'is supported by', provider);
    console.log('[Language Preselection] Supported languages count:', supportedLanguages.length);
    
    // Check full code first
    let langToUse = null;
    if (supportedLanguages.includes(userLang)) {
        langToUse = userLang;
        console.log('[Language Preselection] Full code supported:', userLang);
    } else {
        // Try base language code
        const baseLang = userLang.split('-')[0];
        console.log('[Language Preselection] Full code not supported, trying base:', baseLang);
        if (supportedLanguages.includes(baseLang)) {
            langToUse = baseLang;
            console.log('[Language Preselection] Base code supported:', baseLang);
        }
    }
    
    if (langToUse) {
        // Set the input to show the language nicely
        const langData = LANGUAGES[langToUse];
        if (langData) {
            langInput.value = `${langData.english} (${langData.native}) - ${langToUse}`;
            console.log('[Language Preselection] Set input value to:', langInput.value);
            // Trigger validation
            validateLanguageInput();
        }
    } else {
        console.log('[Language Preselection] Language', userLang, 'not supported by provider', provider);
    }
}

/**
 * Resolve a language input to an API code
 * Accepts: language code, English name, or native name
 * Returns the API code or null if not valid
 */
function resolveLanguageCode(input) {
    if (!input) return null;
    
    const trimmed = input.trim();
    if (!trimmed) return null;
    
    // Check if it's the full format from datalist: "English (Native) - CODE"
    const formatMatch = trimmed.match(/^.+\s*\(.*\)\s*-\s*([A-Za-z-]+)$/);
    if (formatMatch) {
        return formatMatch[1].toUpperCase();
    }
    
    // Otherwise, try direct lookup
    const upperInput = trimmed.toUpperCase();
    return languageLookup.get(upperInput) || null;
}

/**
 * Validate language input and update visual feedback
 */
function validateLanguageInput() {
    const input = document.getElementById('target-lang-input');
    const providerSelect = document.getElementById('translation-provider');
    
    if (!input || !providerSelect) return;
    
    const value = input.value.trim();
    const provider = providerSelect.value;
    
    // If empty, remove validation classes
    if (!value) {
        input.classList.remove('is-valid', 'is-invalid');
        return;
    }
    
    const resolvedCode = resolveLanguageCode(value);
    const supportedLanguages = provider === 'deepl' ? DEEPL_LANGUAGES : GOOGLE_LANGUAGES;
    
    // Check if the resolved code is supported (including base language fallback)
    const isValid = resolvedCode && (
        supportedLanguages.includes(resolvedCode) || 
        supportedLanguages.includes(resolvedCode.split('-')[0])
    );
    
    if (isValid) {
        input.classList.remove('is-invalid');
        input.classList.add('is-valid');
    } else {
        input.classList.remove('is-valid');
        input.classList.add('is-invalid');
    }
}

/**
 * Get the selected language code, resolving from various input formats
 */
function getSelectedLanguage() {
    const input = document.getElementById('target-lang-input');
    if (!input) return null;
    
    return resolveLanguageCode(input.value);
}

// Export functions for use in other modules
window.url2refLanguage = {
    LANGUAGES,
    DEEPL_LANGUAGES,
    GOOGLE_LANGUAGES,
    languageLookup,
    populateLanguageSuggestions,
    getUserPreferredLanguage,
    preselectUserLanguage,
    resolveLanguageCode,
    validateLanguageInput,
    getSelectedLanguage
};
