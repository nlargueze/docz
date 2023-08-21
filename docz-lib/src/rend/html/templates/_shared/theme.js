window.addEventListener('load', () => {
    let theme = 'light';
    const mqDark = window.matchMedia("(prefers-color-scheme: dark)");
    if (mqDark.matches) {
        theme = 'dark';
    }
    document.querySelector("html").setAttribute("data-theme", theme);

    const btnThemeElt = document.getElementById('btn-toggle-theme');
    if (btnThemeElt) {
        btnThemeElt.addEventListener('click', (event) => {
            let theme = document.querySelector("html").getAttribute("data-theme") || 'light';
            if (theme === 'light') {
                theme = 'dark';
            } else {
                theme = 'light';
            }

            document.querySelector("html").setAttribute("data-theme", theme);
        });
    }
});