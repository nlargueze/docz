const mqDark = window.matchMedia("(prefers-color-scheme: dark)");
let theme = localStorage.getItem('theme') || (mqDark.matches ? 'dark' : 'light');
document.querySelector("html").setAttribute("data-theme", theme);

window.addEventListener('load', () => {
    const btnThemeElt = document.getElementById('btn-toggle-theme');
    if (btnThemeElt) {
        btnThemeElt.addEventListener('click', (event) => {
            let theme = document.querySelector("html").getAttribute("data-theme") || 'light';
            if (theme === 'light') {
                theme = 'dark';
            } else {
                theme = 'light';
            }
            localStorage.setItem('theme', theme);
            document.querySelector("html").setAttribute("data-theme", theme);
        });
    }
});