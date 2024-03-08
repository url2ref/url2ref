// Tooltip functionality
var tooltipTriggerList = [].slice.call(document.querySelectorAll('[data-bs-toggle="tooltip"]'))
var tooltipList = tooltipTriggerList.map(function (tooltipTriggerEl) {
    return new bootstrap.Tooltip(tooltipTriggerEl)
})

// Copy to clipboard
function copyToClipboard() {
    const reference = document.getElementById("raw_tab_text");
    navigator.clipboard.writeText(reference.textContent);
}

// Theme switcher
document.getElementById('bd-theme').addEventListener('click', () => {
    if (document.documentElement.getAttribute('data-bs-theme') == 'dark') {
        document.documentElement.setAttribute('data-bs-theme', 'light')
    }
    else {
        document.documentElement.setAttribute('data-bs-theme', 'dark')
    }
})

function checkArchive() {
    const checkmarkSVG = String.raw`<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" fill="currentColor" class="bi bi-check" viewBox="0 0 16 16">
                                        <path d="M10.97 4.97a.75.75 0 0 1 1.07 1.05l-3.99 4.99a.75.75 0 0 1-1.08.02L4.324 8.384a.75.75 0 1 1 1.06-1.06l2.094 2.093 3.473-4.425z"/>
                                    </svg>`;
    const crossSVG = String.raw`<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" fill="currentColor" class="bi bi-x" viewBox="0 0 16 16">
                                    <path d="M4.646 4.646a.5.5 0 0 1 .708 0L8 7.293l2.646-2.647a.5.5 0 0 1 .708.708L8.707 8l2.647 2.646a.5.5 0 0 1-.708.708L8 8.707l-2.646 2.647a.5.5 0 0 1-.708-.708L7.293 8 4.646 5.354a.5.5 0 0 1 0-.708"/>
                                </svg>`

    const label = document.getElementById("archive-label");
    const button = document.getElementById("form-check-archive");
    
    if (button.checked) {
        label.innerHTML = checkmarkSVG;
    }
    else {
        label.innerHTML = crossSVG;
    }
    
}

/*
// Prevents POST from actually submitting and reloading page.
const formElement = document.getElementById("url_form");
formElement.addEventListener('submit', (e) => {
  e.preventDefault();
  const formData = new FormData(formElement);
  fetch('/submit_url', {
    method: 'POST',
    body: formData,
  });
});
*/