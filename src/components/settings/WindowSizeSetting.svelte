<script lang="ts">
    import SettingGroup from './SettingGroup.svelte';
    import SettingRow from './SettingRow.svelte';
    import type { WindowSizeKey } from '../../store';

    interface Props {
        currentSize: string;
        onChange: (sizeId: WindowSizeKey) => void;
    }
    let { currentSize, onChange }: Props = $props();

    const sizes: { id: WindowSizeKey; label: string }[] = [
        { id: 'compact', label: 'Compact' },
        { id: 'standard', label: 'Standard' },
        { id: 'tall', label: 'Tall' },
        { id: 'large', label: 'Large' }
    ];
</script>

<SettingGroup title="Display">
    <SettingRow title="Window Size" description="Choose a comfortable preset">
        <div class="segmented-control">
            {#each sizes as size}
                <button 
                    class="seg-btn" 
                    class:active={currentSize === size.id} 
                    onclick={() => onChange(size.id)}
                >
                    {size.label}
                </button>
            {/each}
        </div>
    </SettingRow>
</SettingGroup>

<style>
/* Premium Segmented Control */
.segmented-control {
    display: flex;
    background-color: rgba(0, 0, 0, 0.3);
    padding: 4px;
    border-radius: 8px;
    border: 1px solid rgba(255, 255, 255, 0.04);
}

.seg-btn {
    background: transparent;
    color: #9aa0a6;
    border: none;
    padding: 6px 14px;
    border-radius: 6px;
    font-size: 13px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
}

.seg-btn:hover {
    color: #e3e3e3;
}

.seg-btn.active {
    background-color: #353638;
    color: #ffffff;
    box-shadow: 0 1px 4px rgba(0, 0, 0, 0.5), inset 0 1px 0 rgba(255, 255, 255, 0.1);
}
</style>
