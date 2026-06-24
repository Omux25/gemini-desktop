<script lang="ts">
  import { onMount } from 'svelte';
  import { Webview } from '@tauri-apps/api/webview';
  import { LogicalSize, getCurrentWindow } from '@tauri-apps/api/window';
  import { calculateWebviewBounds } from '../utils/layout';

  const appWindow = getCurrentWindow();

  onMount(async () => {
      try {
          let geminiWebview = await Webview.getByLabel('gemini');

          if (!geminiWebview) {
              let logicalWidth = 600;
              let logicalHeight = 800;

              try {
                  const size = await appWindow.innerSize();
                  const scaleFactor = await appWindow.scaleFactor();
                  const logicalSize = size.toLogical(scaleFactor);
                  logicalWidth = logicalSize.width || logicalWidth;
                  logicalHeight = logicalSize.height || logicalHeight;
              } catch (err) {
                  console.warn('Failed to get window size for webview init, using defaults', err);
              }

              const bounds = calculateWebviewBounds(logicalWidth, logicalHeight);

              geminiWebview = new Webview(appWindow, 'gemini', {
                  url: 'https://gemini.google.com',
                  x: bounds.x,
                  y: bounds.y,
                  width: bounds.width,
                  height: bounds.height,
                  transparent: true
              });
          }

          appWindow.onResized(async ({ payload: newSize }) => {
              try {
                  const scaleFactor = await appWindow.scaleFactor();
                  const logicalSize = newSize.toLogical(scaleFactor);
                  const bounds = calculateWebviewBounds(logicalSize.width, logicalSize.height);
                  await geminiWebview?.setSize(new LogicalSize(bounds.width, bounds.height));
              } catch (err) {
                  console.error('Failed to resize webview', err);
              }
          });
      } catch (err) {
          console.error('Failed to setup webview completely', err);
      }
  });
</script>

<!-- The webview is managed by Tauri natively overlaying the OS window, so this component just acts as a lifecycle manager without DOM elements. -->

