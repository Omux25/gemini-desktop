<script lang="ts">
    import SettingGroup from './SettingGroup.svelte';
    import SettingRow from './SettingRow.svelte';
    
    interface Props {
        currentToggle: string;
        currentCopy: string;
        currentSnip: string;
        currentPrompt: string;
        onToggleChange: (hotkey: string) => void;
        onCopyChange: (hotkey: string) => void;
        onSnipChange: (hotkey: string) => void;
        onPromptChange: (prompt: string) => void;
    }
    let { 
        currentToggle, currentCopy, currentSnip, currentPrompt,
        onToggleChange, onCopyChange, onSnipChange, onPromptChange
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

    <div class="setting-block">
        <div class="block-info">
            <h4>Smart Selection Prompt</h4>
            <p>Customize the template applied to your selected text. Use <code>{'{'}text{'}'}</code> as the placeholder.</p>
        </div>
        <textarea 
            class="prompt-textarea" 
            placeholder="e.g., Explain this code: {'\n'}{'{'}text{'}'}" 
            value={currentPrompt}
            onchange={(e) => onPromptChange(e.currentTarget.value)}
        ></textarea>
    </div>

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

.setting-block {
    display: flex;
    flex-direction: column;
    padding: 12px 16px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.05);
    gap: 12px;
}

.block-info h4 {
    margin: 0;
    font-size: 14.5px;
    font-weight: 500;
    color: #e3e3e3;
    letter-spacing: 0.2px;
}

.block-info p {
    margin: 4px 0 0 0;
    font-size: 13px;
    color: #9aa0a6;
    line-height: 1.4;
}

.block-info code {
    background: rgba(0,0,0,0.3);
    padding: 2px 4px;
    border-radius: 4px;
    font-family: ui-monospace, SFMono-Regular, Consolas, "Liberation Mono", Menlo, monospace;
    color: #8ab4f8;
}

.prompt-textarea {
    width: 100%;
    min-height: 80px;
    background: rgba(0, 0, 0, 0.2);
    border: 1px solid rgba(255, 255, 255, 0.1);
    color: #e3e3e3;
    padding: 10px 12px;
    border-radius: 8px;
    font-size: 13px;
    font-family: ui-monospace, SFMono-Regular, Consolas, "Liberation Mono", Menlo, monospace;
    transition: border-color 0.2s ease, background 0.2s ease, box-shadow 0.2s ease;
    box-sizing: border-box;
    resize: vertical;
}

.prompt-textarea:focus {
    outline: none;
    border-color: rgba(138, 180, 248, 0.5);
    background: rgba(0, 0, 0, 0.4);
    box-shadow: 0 0 0 2px rgba(138, 180, 248, 0.1);
}

.prompt-textarea::placeholder {
    color: #70757a;
}
</style>
