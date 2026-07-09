<script lang="ts">
    import SettingRow from './SettingRow.svelte';
    
    interface Props {
        id: string;
        title: string;
        description: string;
        checked: boolean;
        onChange: (checked: boolean) => void;
    }
    let { id, title, description, checked, onChange }: Props = $props();

    function handleChange(event: Event) {
        const target = event.target as HTMLInputElement;
        onChange(target.checked);
    }
</script>

<SettingRow {title} {description}>
    <label class="premium-toggle">
        <input type="checkbox" {id} {checked} onchange={handleChange}>
        <div class="toggle-track">
            <div class="toggle-thumb"></div>
        </div>
    </label>
</SettingRow>

<style>
/* Premium Animated Toggle */
.premium-toggle {
    position: relative;
    display: inline-block;
    width: 38px;
    height: 20px;
    cursor: pointer;
    flex-shrink: 0;
}

.premium-toggle input {
    opacity: 0;
    width: 0;
    height: 0;
}

.toggle-track {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: rgba(255, 255, 255, 0.1);
    border: 1px solid rgba(255, 255, 255, 0.05);
    border-radius: 20px;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    overflow: hidden;
}

.toggle-thumb {
    position: absolute;
    height: 16px;
    width: 16px;
    left: 2px;
    bottom: 1px;
    background-color: #e3e3e3;
    border-radius: 50%;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}

/* Active State (High-Contrast Monochrome) */
input:checked + .toggle-track {
    background: #e3e3e3;
    border-color: transparent;
}

input:checked + .toggle-track .toggle-thumb {
    transform: translateX(18px);
    background-color: #131314;
    box-shadow: -2px 0 6px rgba(0, 0, 0, 0.2);
}

/* Hover effects */
.premium-toggle:hover .toggle-track {
    background-color: rgba(255, 255, 255, 0.15);
}

input:checked:hover + .toggle-track {
    background: #ffffff;
}
</style>
