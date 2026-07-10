(function() {
    const inputElement = typeof findFocusableInput === 'function' ? findFocusableInput(document.body) : (window.__findFocusableInput ? window.__findFocusableInput(document.body) : null);
    if (inputElement && window.__gemini_snip_b64) {
        fetch('data:image/png;base64,' + window.__gemini_snip_b64)
            .then(res => res.blob())
            .then(blob => {
                const filename = `screenshot_${Date.now()}.png`;
                const file = new File([blob], filename, { type: 'image/png' });
                const dt = new DataTransfer();
                dt.items.add(file);
                const pasteEvent = new ClipboardEvent('paste', { clipboardData: dt, bubbles: true });
                inputElement.dispatchEvent(pasteEvent);
                inputElement.focus();
            });
    }
})();
