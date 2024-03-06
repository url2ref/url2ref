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