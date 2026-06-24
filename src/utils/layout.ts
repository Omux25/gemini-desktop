import { CONSTANTS } from '../store';

/**
 * Calculates the appropriate Webview dimensions to render cleanly underneath the custom Titlebar.
 * This encapsulates the layout logic away from the raw Webview component.
 * 
 * @param logicalWidth The full window's logical width
 * @param logicalHeight The full window's logical height
 * @returns { width: number, height: number, y: number }
 */
export function calculateWebviewBounds(logicalWidth: number, logicalHeight: number) {
    return {
        width: logicalWidth,
        height: Math.max(0, logicalHeight - CONSTANTS.UI.TITLEBAR_HEIGHT),
        y: CONSTANTS.UI.TITLEBAR_HEIGHT,
        x: 0
    };
}
