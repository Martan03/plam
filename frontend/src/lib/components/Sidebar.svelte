<script lang="ts">
    import { persisted } from "../state/storage.svelte";
    import ConfirmDialog from "./dialogs/ConfirmDialog.svelte";
    import PromptDialog from "./dialogs/PromptDialog.svelte";
    import SettingsDialog from "./SettingsDialog.svelte";

    let { workspace, isVisible = $bindable(true) } = $props();

    const width = persisted("plam-menu-width", 200);
    let isDragging = $state(false);

    let deleteDialog: ReturnType<typeof ConfirmDialog>;
    let newDialog: ReturnType<typeof PromptDialog>;
    let settingsDialog: ReturnType<typeof SettingsDialog>;

    let fileToDelete = $state<string | null>(null);

    function promptNew() {
        newDialog.show();
    }

    function promptDelete(file: string) {
        fileToDelete = file;
        deleteDialog.show();
    }

    function execDelete() {
        if (fileToDelete) {
            workspace.remove(fileToDelete);
            fileToDelete = null;
        }
    }

    function startDrag(e: PointerEvent) {
        isDragging = true;
        e.preventDefault();
    }

    function onPointerMove(e: PointerEvent) {
        if (!isDragging) return;

        const newWidth = e.clientX;
        if (newWidth > 5) {
            width.value = newWidth;
        }
    }

    const onPointerUp = () => (isDragging = false);
</script>

<svelte:window onpointermove={onPointerMove} onpointerup={onPointerUp} />

<aside style="--menu-width: {width.value}px" class:hidden={!isVisible}>
    <button
        class="resizer"
        class:dragging={isDragging}
        onpointerdown={startDrag}
        aria-label="Resize file explorer"
    ></button>

    <div class="header">
        <span>Files</span>
        <button class="add-btn" onclick={promptNew} title="New File">+</button>
    </div>

    <ul class="file-list">
        {#each workspace.files as file}
            <li class="file-item" class:active={workspace.active === file}>
                <button class="sel-btn" onclick={() => workspace.select(file)}>
                    <span class="filename">{file}</span>
                </button>

                {#if workspace.files.length > 1}
                    <button
                        class="delete-btn"
                        onclick={() => promptDelete(file)}
                    >
                        ×
                    </button>
                {/if}
            </li>
        {/each}
    </ul>

    <div class="footer">
        <button
            class="settings-btn"
            title="Open Settings"
            onclick={() => settingsDialog.show()}
        >
            <span class="icon">⚙</span> Settings
        </button>
    </div>
</aside>

<ConfirmDialog
    bind:this={deleteDialog}
    title="Delete file?"
    message="This will delete '{fileToDelete}'. This action cannot be undone."
    confirm="Delete"
    onconfirm={execDelete}
/>

<PromptDialog
    bind:this={newDialog}
    title="Create new file"
    label="Filename:"
    confirm="Create"
    onsubmit={(filename: string) => workspace.add(filename)}
/>

<SettingsDialog bind:this={settingsDialog} />

<style>
    aside {
        width: var(--menu-width);
        background-color: #21252b;
        border-right: 1px solid #181a1f;
        display: flex;
        flex-direction: column;
        flex-shrink: 0;
        position: relative;
        transition: margin-left 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    }

    aside.hidden {
        margin-left: calc(var(--menu-width) * -1);
        border-right: none;
    }

    .resizer {
        position: absolute;
        right: -4px;
        top: 0;
        bottom: 0;
        width: 6px;
        cursor: col-resize;
        z-index: 10;
        background-color: transparent;
        border: none;
    }

    .resizer:hover,
    .resizer.dragging {
        background-color: color-mix(in srgb, var(--primary) 50%, transparent);
    }

    .header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 0.5rem 1rem;
        padding-right: 0.9rem;
        font-size: 0.8rem;
        font-weight: bold;
        color: #5c6370;
    }

    .add-btn {
        background: transparent;
        color: #abb2bf;
        border: none;
        cursor: pointer;
        font-size: 1.2rem;
        border-radius: 4px;
        width: 1.5rem;
        height: 1.5rem;
        display: flex;
        align-items: center;
        justify-content: center;
        padding-bottom: 0.065rem;
        outline: none;
    }

    .add-btn:hover {
        color: #ffffff;
        background: #2c313a;
    }

    .file-list {
        list-style: none;
        margin: 0;
        padding: 0;
        overflow-x: hidden;
        overflow-y: auto;
        flex: 1;
    }

    .file-item,
    .settings-btn {
        display: flex;
        align-items: center;
        justify-content: space-between;
        color: #abb2bf;
        padding: 0.4rem 1rem;
        overflow: hidden;
    }

    .file-item:hover,
    .settings-btn:hover {
        background-color: #2c313a;
    }

    .file-item.active {
        background-color: #2c313a;
        color: var(--primary, #3acbaf);
        border-left: 3px solid var(--primary, #3acbaf);
        padding-left: calc(1rem - 3px);
    }

    .sel-btn {
        flex: 1;
        background: transparent;
        border: none;
        cursor: pointer;
        text-align: left;
        font-size: 0.9rem;
        font-family: inherit;
        color: inherit;
    }

    .filename {
        flex: 1;
        overflow: hidden;
        text-overflow: ellipsis;
    }

    .delete-btn {
        background: transparent;
        border: none;
        color: #e06c75;
        cursor: pointer;
        font-size: 1.2rem;
        opacity: 0;
        padding: 0 4px;
    }

    .file-item:hover .delete-btn {
        opacity: 1;
    }

    .delete-btn:hover {
        font-weight: bold;
    }

    .footer {
        padding: 0.5rem 0;
        border-top: 1px solid #181a1f;
        background-color: #21252b;
    }

    .settings-btn {
        width: 100%;
        justify-content: flex-start;
        background: inherit;
        border: none;
        font-family: inherit;
        font-size: 0.9rem;
        color: inherit;
        cursor: pointer;
    }

    .settings-btn .icon {
        margin-right: 0.7rem;
        font-size: 1.1rem;
        padding-bottom: 2px;
    }

    .settings-btn:hover {
        background-color: #2c313a;
        color: #ffffff;
    }
</style>
