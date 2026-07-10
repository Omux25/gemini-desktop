function findFocusableInput(root) {
    let input = root.querySelector('rich-textarea div[contenteditable="true"], rich-textarea p, rich-textarea [contenteditable="true"], textarea, input[type="text"], [role="textbox"], [contenteditable="true"]');
    if (input) return input;

    const allElements = root.querySelectorAll('*');
    for (const el of allElements) {
        if (el.shadowRoot) {
            const found = findFocusableInput(el.shadowRoot);
            if (found) return found;
        }
    }
    return null;
}
window.__findFocusableInput = findFocusableInput;
