<script lang="ts">
    interface Props {
        currentSize: string;
        onChange: (sizeId: string) => void;
    }
    let { currentSize, onChange }: Props = $props();

    const sizes = [
        { id: 'compact', label: 'Compact' },
        { id: 'standard', label: 'Standard' },
        { id: 'tall', label: 'Tall' },
        { id: 'large', label: 'Large' }
    ];
</script>

<div class="settings-section">
    <div class="settings-info">
        <h3>Window Size</h3>
        <p>Choose a comfortable preset</p>
    </div>
    <div class="pill-group">
        {#each sizes as size}
            <button 
                class="pill" 
                class:active={currentSize === size.id} 
                onclick={() => onChange(size.id)}
            >
                {size.label}
            </button>
        {/each}
    </div>
</div>
