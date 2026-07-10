(function() {
    function tryFocus() {
        const input = typeof findFocusableInput === 'function' ? findFocusableInput(document) : (window.__findFocusableInput ? window.__findFocusableInput(document) : null);
        if (input && input.offsetParent !== null) {
            input.focus();
            try { input.click(); } catch (e) {}
            return true;
        }
        return false;
    }

    if (tryFocus()) return;

    const observer = new MutationObserver(() => {
        if (tryFocus()) {
            observer.disconnect();
            clearTimeout(timeout);
        }
    });

    observer.observe(document.documentElement, { childList: true, subtree: true, attributes: true });

    const timeout = setTimeout(() => {
        observer.disconnect();
    }, 2000);
})();
