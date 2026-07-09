(function() {
    function findFocusableInput(root) {
        // Check current root
        let input = root.querySelector('rich-textarea div[contenteditable="true"]') 
                 || root.querySelector('rich-textarea [contenteditable="true"]')
                 || root.querySelector('textarea') 
                 || root.querySelector('[role="textbox"]')
                 || root.querySelector('[contenteditable="true"]');
                 
        if (input) return input;

        // Traverse shadow DOMs
        const allElements = root.querySelectorAll('*');
        for (const el of allElements) {
            if (el.shadowRoot) {
                const found = findFocusableInput(el.shadowRoot);
                if (found) return found;
            }
        }
        return null;
    }

    let attempts = 0;
    const interval = setInterval(() => {
        attempts++;
        const input = findFocusableInput(document);
        if (input && input.offsetParent !== null) { // Make sure it's visible
            input.focus();
            
            // Sometimes rich text editors need a click event to fully activate
            try {
                input.click();
            } catch (e) {}

            clearInterval(interval);
        }
        
        if (attempts > 20) {
            clearInterval(interval); // Stop trying after 2 seconds
        }
    }, 100);
})();
