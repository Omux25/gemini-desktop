<script lang="ts">
    import { updaterService, updateState, updateProgress, updateVersion } from '../../services/updaterService';
    import { onMount } from 'svelte';

    let progressPercentage = 0;
    let downloadedMb = "0.0";
    let totalMb = "0.0";

    $: {
        if ($updateProgress.total > 0) {
            progressPercentage = ($updateProgress.downloaded / $updateProgress.total) * 100;
            downloadedMb = ($updateProgress.downloaded / 1048576).toFixed(1);
            totalMb = ($updateProgress.total / 1048576).toFixed(1);
        }
    }

    // Removed onMount check, now handled globally on app boot

    function handleUpdate() {
        updaterService.download();
    }

    function handleRestart() {
        updaterService.installUpdate();
    }
</script>

{#if $updateState !== 'idle'}
<div class="updater-container">
    {#if $updateState === 'checking'}
        <div class="updater-status">
            <span class="pulse"></span> Checking for updates...
        </div>
    {:else if $updateState === 'available'}
        <div class="updater-row">
            <div class="updater-info">
                <strong>Update Available</strong>
                <span>Version {$updateVersion} is ready to install</span>
            </div>
            <button class="updater-btn" onclick={handleUpdate}>Download</button>
        </div>
    {:else if $updateState === 'downloading'}
        <div class="updater-downloading">
            <div class="updater-info-row">
                <strong>Downloading update...</strong>
                <span class="metrics">{downloadedMb} MB / {totalMb} MB</span>
            </div>
            <div class="progress-track">
                <div class="progress-fill" style="width: {progressPercentage}%"></div>
            </div>
        </div>
    {:else if $updateState === 'ready'}
        <div class="updater-row">
            <div class="updater-info">
                <strong>Ready to Install</strong>
                <span class="ready-text">Update downloaded successfully</span>
            </div>
            <button class="updater-btn ready-btn" onclick={handleRestart}>Install & Relaunch</button>
        </div>
    {/if}
</div>
{/if}

<style>
.updater-container {
    background: linear-gradient(145deg, #1e1e20, #18181a);
    border: 1px solid rgba(138, 180, 248, 0.2);
    border-radius: 12px;
    padding: 16px;
    margin-bottom: 24px;
    box-shadow: 0 4px 24px rgba(0,0,0,0.4), inset 0 1px 0 rgba(255,255,255,0.05);
    animation: slideDown 0.3s cubic-bezier(0.175, 0.885, 0.32, 1.275);
}

@keyframes slideDown {
    from { opacity: 0; transform: translateY(-10px) scale(0.98); }
    to { opacity: 1; transform: translateY(0) scale(1); }
}

.updater-status {
    display: flex;
    align-items: center;
    gap: 12px;
    color: #9aa0a6;
    font-size: 14px;
    font-weight: 500;
}

.pulse {
    width: 8px;
    height: 8px;
    background-color: #8ab4f8;
    border-radius: 50%;
    animation: pulsing 1.5s infinite ease-in-out;
}

@keyframes pulsing {
    0% { transform: scale(0.8); opacity: 0.5; }
    50% { transform: scale(1.2); opacity: 1; box-shadow: 0 0 10px #8ab4f8; }
    100% { transform: scale(0.8); opacity: 0.5; }
}

.updater-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
}

.updater-info {
    display: flex;
    flex-direction: column;
    gap: 4px;
}

.updater-info strong {
    color: #e3e3e3;
    font-size: 15px;
    font-weight: 600;
}

.updater-info span {
    color: #9aa0a6;
    font-size: 13px;
}

.updater-info .ready-text {
    color: #81c995;
}

.updater-btn {
    background-color: rgba(255, 255, 255, 0.1);
    color: #e3e3e3;
    border: none;
    padding: 8px 16px;
    border-radius: 8px;
    font-size: 13px;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s;
}

.updater-btn:hover {
    background-color: rgba(255, 255, 255, 0.15);
}

.ready-btn {
    background-color: #8ab4f8;
    color: #131314;
}

.ready-btn:hover {
    background-color: #aecbfa;
    transform: translateY(-1px);
    box-shadow: 0 4px 12px rgba(138, 180, 248, 0.3);
}

.updater-downloading {
    display: flex;
    flex-direction: column;
    gap: 12px;
}

.updater-info-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
}

.updater-info-row strong {
    color: #e3e3e3;
    font-size: 14px;
    font-weight: 600;
}

.metrics {
    color: #8ab4f8;
    font-family: ui-monospace, SFMono-Regular, Consolas, "Liberation Mono", Menlo, monospace;
    font-size: 12px;
    font-weight: 600;
}

.progress-track {
    width: 100%;
    height: 8px;
    background-color: rgba(0, 0, 0, 0.5);
    border-radius: 4px;
    overflow: hidden;
    box-shadow: inset 0 1px 2px rgba(0,0,0,0.5);
}

.progress-fill {
    height: 100%;
    background: linear-gradient(90deg, #4285f4, #8ab4f8);
    border-radius: 4px;
    transition: width 0.1s linear;
    position: relative;
    overflow: hidden;
}

.progress-fill::after {
    content: '';
    position: absolute;
    top: 0; left: 0; right: 0; bottom: 0;
    background: linear-gradient(90deg, transparent, rgba(255,255,255,0.3), transparent);
    animation: shimmer 1.5s infinite linear;
}

@keyframes shimmer {
    0% { transform: translateX(-100%); }
    100% { transform: translateX(100%); }
}
</style>
