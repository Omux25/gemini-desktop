<script lang="ts">
    import type { Snippet } from 'svelte';
    let { title, description, children }: { title: string, description?: string, children: Snippet } = $props();
</script>

<div class="setting-row">
    <div class="row-info">
        <h4>{title}</h4>
        {#if description}
            <p>{description}</p>
        {/if}
    </div>
    <div class="row-control">
        {@render children()}
    </div>
</div>

<style>
.setting-row {
    display: flex;
    flex-wrap: wrap; /* Allow wrapping if space is too tight */
    align-items: center;
    justify-content: space-between;
    padding: 12px 16px;
    border-bottom: 1px solid rgba(255, 255, 255, 0.05);
    gap: 12px; /* Gap when wrapped */
}
.setting-row:last-child {
    border-bottom: none;
}
.row-info {
    flex: 1 1 180px; /* Prefer filling space, but wrap if compressed below 180px */
    padding-right: 16px;
    display: flex;
    flex-direction: column;
    gap: 4px;
}
.row-info h4 {
    margin: 0;
    font-size: 14.5px;
    font-weight: 500;
    color: #e3e3e3;
    letter-spacing: 0.2px;
}
.row-info p {
    margin: 0;
    font-size: 13px;
    color: #9aa0a6;
    line-height: 1.4;
}
.row-control {
    flex-shrink: 0;
    display: flex;
    align-items: center;
}
</style>
