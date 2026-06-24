const currentUrl = window.location.href;
if (window.history.length > 1 && currentUrl !== 'https://gemini.google.com/') {
    window.history.back();
    setTimeout(() => {
        if (window.location.href !== currentUrl) {
            window.location.reload();
        }
    }, 50);
} else {
    window.location.href = 'https://gemini.google.com/';
}
