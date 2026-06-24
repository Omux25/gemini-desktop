<script lang="ts">
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

<div class="settings-section row-layout">
    <div class="settings-info">
        <h3>{title}</h3>
        <p>{description}</p>
    </div>
    <label class="toggle-switch">
        <input type="checkbox" {id} {checked} onchange={handleChange}>
        <span class="slider"></span>
    </label>
</div>

<style>
.settings-section.row-layout { flex-direction: row; align-items: center; justify-content: space-between; }
.settings-info { flex: 1; padding-right: 16px; }
.settings-info h3 { margin: 0 0 4px 0; font-size: 15px; font-weight: 600; }
.settings-info p { margin: 0; font-size: 13px; color: #9aa0a6; }

.toggle-switch { position: relative; display: inline-block; width: 44px; height: 24px; flex-shrink: 0; }
.toggle-switch input { opacity: 0; width: 0; height: 0; }
.slider { position: absolute; cursor: pointer; top: 0; left: 0; right: 0; bottom: 0; background-color: #3c4043; transition: .3s; border-radius: 24px; }
.slider:before { position: absolute; content: ""; height: 18px; width: 18px; left: 3px; bottom: 3px; background-color: white; transition: .3s; border-radius: 50%; }
input:checked + .slider { background-color: #4285f4; }
input:checked + .slider:before { transform: translateX(20px); }
</style>
