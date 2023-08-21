function debounce(ms, func) {
    let isCan = true;

    return () => {
        if (isCan) {
            func();
            isCan = false;
            setTimeout(() => isCan = true, ms);
        }
    }
}

function debounce(ms, fn) {
    let timerId;
    return (...args) => {
        clearTimeout(timerId);
        timerId = setTimeout(() => {
            fn(...args);
        }, ms);
    };
}

window.addEventListener('load', () => {
    const sidebarElt = document.getElementById('sidebar');
    const btnSidebarElt = document.getElementById('btn-toggle-sidebar');
    const sidebarHandleElt = document.getElementById('sidebar-handle');
    let opened = true;
    let width = 300;

    // click handler
    if (sidebarElt && btnSidebarElt) {
        sidebarElt.style.width = width + 'px';

        const onClick = () => {
            opened = !opened;
            if (opened) {
                sidebarElt.style.width = width + 'px';
                sidebarElt.classList.remove('closed');
            } else {
                sidebarElt.style.width = '0px';
                sidebarElt.classList.add('closed');
            }
        };

        btnSidebarElt.addEventListener('click', onClick);
    }

    // reisze handler
    if (sidebarElt && sidebarHandleElt) {
        const onPointerDown = (event) => {
            const onPointerMove = (event) => {
                sidebarElt.style.width = event.x + 'px';
            };

            const onPointerUp = (event) => {
                sidebarElt.style.width = event.x + 'px';
                document.removeEventListener('pointermove', onPointerMove);
                document.removeEventListener('pointerup', onPointerUp);
            }

            document.addEventListener('pointermove', onPointerMove);
            document.addEventListener('pointerup', onPointerUp);
        }

        sidebarHandleElt.addEventListener('pointerdown', onPointerDown);
    };
});

