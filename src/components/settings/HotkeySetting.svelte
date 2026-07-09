<script lang="ts">
    import SettingGroup from './SettingGroup.svelte';
    import SettingRow from './SettingRow.svelte';
    
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
    const copyHotkeys = ['Alt+C', 'Ctrl+C', 'Alt+T'];
    const snipHotkeys = ['Alt+S', 'Ctrl+S', 'Alt+X'];
</script>

<SettingGroup title="Global Shortcuts">
    <SettingRow title="Show / Hide Window" description="Shortcut to quickly summon the Gemini app">
        <div class="kbd-group">
            {#each toggleHotkeys as hk}
                <button class="kbd-btn" class:active={currentToggle === hk} onclick={() => onToggleChange(hk)}>
                    {hk}
                </button>
            {/each}
        </div>
    </SettingRow>

    <SettingRow title="Smart Text Selection" description="Pull highlighted text instantly into Gemini">
        <div class="kbd-group">
            {#each copyHotkeys as hk}
                <button class="kbd-btn" class:active={currentCopy === hk} onclick={() => onCopyChange(hk)}>
                    {hk}
                </button>
            {/each}
        </div>
    </SettingRow>

    <SettingRow title="Snipping Tool" description="Take a screenshot and send it to Gemini">
        <div class="kbd-group">
            {#each snipHotkeys as hk}
                <button class="kbd-btn" class:active={currentSnip === hk} onclick={() => onSnipChange(hk)}>
                    {hk}
                </button>
            {/each}
        </div>
    </SettingRow>
</SettingGroup>

<style>
/* Segmented Control / Keycap Aesthetic */
.kbd-group {
    display: flex;
    flex-wrap: wrap;
    gap: 4px;
    background-color: rgba(0, 0, 0, 0.3);
    padding: 4px;
    border-radius: 8px;
    border: 1px solid rgba(255, 255, 255, 0.04);
}

.kbd-btn {
    background: transparent;
    color: #9aa0a6;
    border: 1px solid transparent;
    padding: 6px 12px;
    border-radius: 6px;
    font-size: 12px;
    font-family: ui-monospace, SFMono-Regular, Consolas, "Liberation Mono", Menlo, monospace;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
}

.kbd-btn:hover {
    color: #e3e3e3;
    background-color: rgba(255, 255, 255, 0.05);
}

.kbd-btn.active {
    background-color: #353638;
    color: #ffffff;
    box-shadow: 0 1px 4px rgba(0, 0, 0, 0.5), inset 0 1px 0 rgba(255, 255, 255, 0.1);
    border: 1px solid rgba(255, 255, 255, 0.1);
}
</style>
