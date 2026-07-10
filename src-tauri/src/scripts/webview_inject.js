(function() {
    const promptElement = typeof findFocusableInput === 'function' ? findFocusableInput(document) : (window.__findFocusableInput ? window.__findFocusableInput(document) : null);
    if (promptElement) {
        const textToInject = window.__gemini_injected_text || "";
        
        promptElement.focus();
        try { promptElement.click(); } catch (e) {}
        
        const success = document.execCommand('insertText', false, textToInject);
        
        if (!success) {
            if (promptElement.value !== undefined) {
                promptElement.value += textToInject;
            } else {
                promptElement.textContent += textToInject;
            }
            const inputEvent = new Event('input', { bubbles: true });
            promptElement.dispatchEvent(inputEvent);
        }
        
        try {
            if (promptElement.setSelectionRange) {
                promptElement.setSelectionRange(0, 0);
            } else {
                const range = document.createRange();
                const sel = window.getSelection();
                range.setStart(promptElement, 0);
                range.collapse(true);
                sel.removeAllRanges();
                sel.addRange(range);
            }
        } catch (e) {}
    }
})();
