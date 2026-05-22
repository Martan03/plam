<script lang="ts">
    import { settings, THEMES } from "../state/settings.svelte";
    import BaseDialog from "./dialogs/BaseDialog.svelte";

    let dialog: ReturnType<typeof BaseDialog>;

    export const show = () => dialog.show();
    export const close = () => dialog.close();
</script>

<BaseDialog bind:this={dialog} title="Settings" width="800px">
    <div class="content">
        <label class="setting-row">
            <div class="setting-info">
                <h3>Vim Mode</h3>
                <p>Enable Vim-like controls in the code editor.</p>
            </div>

            <div class="switch">
                <input type="checkbox" bind:checked={settings.vimMode} />
                <span class="slider"></span>
            </div>
        </label>

        <div class="setting-row">
            <div class="setting-info">
                <h3>Accent Color</h3>
                <p>Primary color of the UI (selected color,...).</p>
            </div>

            <input
                type="color"
                class="color-picker"
                bind:value={settings.primaryColor}
                title="Choose Accent Color"
            />
        </div>

        <div class="setting-row">
            <div class="setting-info">
                <h3>Theme</h3>
                <p>Color theme of the interface.</p>
            </div>

            <select bind:value={settings.theme} class="settings-input">
                {#each THEMES as theme}
                    <option value={theme}>{theme}</option>
                {/each}
            </select>
        </div>
    </div>
    {#snippet actions()}
        <button class="dialog-btn primary-btn" onclick={close}>Done</button>
    {/snippet}
</BaseDialog>

<style>
    .content {
        display: flex;
        flex-direction: column;
        gap: 0.6rem;
    }

    .setting-row {
        display: flex;
        justify-content: space-between;
        align-items: center;
        border-bottom: 1px solid var(--border);
        padding-bottom: 0.6rem;
        cursor: pointer;
    }

    .setting-info h3 {
        margin: 0 0 0.25rem 0;
        font-size: 1rem;
        color: var(--fg);
    }

    .setting-info p {
        margin: 0;
        font-size: 0.85rem;
        color: var(--fg-disabled);
    }

    .switch {
        position: relative;
        display: inline-block;
        width: 44px;
        height: 24px;
    }

    .switch input {
        opacity: 0;
        width: 0;
        height: 0;
    }

    .slider {
        position: absolute;
        cursor: pointer;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        background-color: var(--border-light);
        transition: 0.2s;
        border-radius: 24px;
    }

    .slider:before {
        position: absolute;
        content: "";
        height: 18px;
        width: 18px;
        left: 3px;
        bottom: 3px;
        background-color: var(--bg-panel);
        transition: 0.2s;
        border-radius: 50%;
    }

    input:checked + .slider {
        background-color: var(--primary);
    }

    input:checked + .slider:before {
        transform: translateX(20px);
        background-color: var(--bg);
    }

    .color-picker {
        -webkit-appearance: none;
        -moz-appearance: none;
        appearance: none;
        width: 44px;
        height: 36px;
        background: transparent;
        border: none;
        cursor: pointer;
        padding: 0;
        border-radius: 4px;
        transition: transform 0.1s ease;
    }

    .color-picker:hover {
        transform: scale(1.1);
    }

    .color-picker::-webkit-color-swatch-wrapper {
        padding: 0;
    }

    .color-picker::-webkit-color-swatch {
        border: 2px solid var(--border-light);
        border-radius: 6px;
    }

    .color-picker::-moz-color-swatch {
        border: 2px solid var(--border-light);
        border-radius: 6px;
    }

    .settings-input {
        background-color: var(--bg-dark);
        border: 1px solid var(--border-light);
        color: var(--fg);
        padding: 0.5rem;
        border-radius: 4px;
        font-family: inherit;
        font-size: 1rem;
    }

    select.settings-input {
        cursor: pointer;
    }
</style>
