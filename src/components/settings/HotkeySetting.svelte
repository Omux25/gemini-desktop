<script lang="ts">
    interface Props {
        currentHotkey: string;
        onChange: (hotkey: string) => void;
    }
    let { currentHotkey, onChange }: Props = $props();

    const hotkeys = [
        'Alt+Space', 'CommandOrControl+Space', 'CommandOrControl+Shift+Space',
        'CommandOrControl+Shift+G', 'Alt+G', 'F12'
    ];
</script>

<div class="settings-section">
    <div class="settings-info">
        <h3>Global Hotkey</h3>
        <p>Shortcut to quickly show/hide the app</p>
    </div>
    <div class="pill-group">
        {#each hotkeys as hk}
            <button 
                class="pill" 
                class:active={currentHotkey === hk} 
                onclick={() => onChange(hk)}
            >
                {hk.replace('CommandOrControl', 'Ctrl/Cmd')}
            </button>
        {/each}
    </div>
</div>
