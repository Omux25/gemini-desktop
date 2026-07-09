<script lang="ts">
    interface Props {
        currentToggle: string;
        currentCopy: string;
        currentSnip: string;
        onToggleChange: (hotkey: string) => void;
        onCopyChange: (hotkey: string) => void;
        onSnipChange: (hotkey: string) => void;
    }
    let { 
        currentToggle, currentCopy, currentSnip, 
        onToggleChange, onCopyChange, onSnipChange 
    }: Props = $props();

    const toggleHotkeys = ['Alt+Space', 'Ctrl+Space', 'Alt+G'];
    const copyHotkeys = ['Alt+C', 'Ctrl+C (Global)', 'Alt+T'];
    const snipHotkeys = ['Alt+S', 'Ctrl+S (Global)', 'Alt+X'];
</script>

<div class="settings-section">
    <div class="settings-info">
        <h3>1. Show / Hide Window</h3>
        <p>Shortcut to quickly summon the Gemini app</p>
    </div>
    <div class="pill-group">
        {#each toggleHotkeys as hk}
            <button class="pill" class:active={currentToggle === hk} onclick={() => onToggleChange(hk)}>
                {hk}
            </button>
        {/each}
    </div>
</div>

<div class="settings-section">
    <div class="settings-info">
        <h3>2. Smart Text Selection</h3>
        <p>Pull highlighted text instantly into Gemini</p>
    </div>
    <div class="pill-group">
        {#each copyHotkeys as hk}
            <button class="pill" class:active={currentCopy === hk} onclick={() => onCopyChange(hk)}>
                {hk}
            </button>
        {/each}
    </div>
</div>

<div class="settings-section">
    <div class="settings-info">
        <h3>3. Snipping Tool</h3>
        <p>Take a screenshot and send it to Gemini</p>
    </div>
    <div class="pill-group">
        {#each snipHotkeys as hk}
            <button class="pill" class:active={currentSnip === hk} onclick={() => onSnipChange(hk)}>
                {hk}
            </button>
        {/each}
    </div>
</div>
