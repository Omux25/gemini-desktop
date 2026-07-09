<script lang="ts">
    import type { Snippet } from 'svelte';
    let { children, title }: { children: Snippet, title?: string } = $props();
</script>

<div class="setting-group-container">
    {#if title}
        <h3 class="group-title">{title}</h3>
    {/if}
    <div class="setting-group">
        {@render children()}
    </div>
</div>

<style>
.setting-group-container {
    display: flex;
    flex-direction: column;
    gap: 8px;
    margin-bottom: 24px;
}
.group-title {
    font-size: 12.5px;
    font-weight: 600;
    color: #9aa0a6;
    margin: 0;
    padding-left: 8px;
    text-transform: uppercase;
    letter-spacing: 0.8px;
}
.setting-group {
    background-color: rgba(255, 255, 255, 0.02);
    border: 1px solid rgba(255, 255, 255, 0.06);
    border-radius: 12px;
    display: flex;
    flex-direction: column;
}
</style>
