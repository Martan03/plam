<script lang="ts">
    import { persisted } from "../state/storage.svelte";
    import ContextMenu from "./ContextMenu.svelte";
    import ConfirmDialog from "./dialogs/ConfirmDialog.svelte";
    import CreateDialog from "./dialogs/CreateDialog.svelte";
    import PromptDialog from "./dialogs/PromptDialog.svelte";
    import EditIcon from "./icons/EditIcon.svelte";
    import SettingsIcon from "./icons/SettingsIcon.svelte";
    import TrashIcon from "./icons/TrashIcon.svelte";
    import SettingsDialog from "./SettingsDialog.svelte";

    let { workspace, isVisible = $bindable(true) } = $props();

    const width = persisted("plam-menu-width", 200);
    let isDragging = $state(false);

    let deleteDialog: ReturnType<typeof ConfirmDialog>;
    let newDialog: ReturnType<typeof CreateDialog>;
    let renameDialog: ReturnType<typeof PromptDialog>;
    let settingsDialog: ReturnType<typeof SettingsDialog>;
    let contextMenu: ReturnType<typeof ContextMenu>;

    let actionFile = $state<string | null>(null);

    function openContextMenu(e: MouseEvent, file: string) {
        actionFile = file;
        contextMenu.open(e);
    }

    function execRename(filename: string) {
        if (actionFile) {
            workspace.rename(actionFile, filename);
            actionFile = null;
        }
    }

    function execDelete() {
        if (actionFile) {
            workspace.remove(actionFile);
            actionFile = null;
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
        <button
            class="add-btn"
            onclick={() => newDialog.show()}
            title="New File">+</button
        >
    </div>

    <ul class="file-list">
        {#each workspace.files as file}
            <li
                class="file-item"
                class:active={workspace.active === file}
                oncontextmenu={(e) => openContextMenu(e, file)}
            >
                <button class="sel-btn" onclick={() => workspace.select(file)}>
                    <span class="filename">{file}</span>
                </button>
            </li>
        {/each}
    </ul>

    <div class="footer">
        <button
            class="settings-btn"
            title="Open Settings"
            onclick={() => settingsDialog.show()}
        >
            <SettingsIcon width="1.1rem" /> Settings
        </button>
    </div>
</aside>

<ConfirmDialog
    bind:this={deleteDialog}
    title="Delete file?"
    message="This will delete '{actionFile}'. This action cannot be undone."
    confirm="Delete"
    onconfirm={execDelete}
/>

<CreateDialog
    bind:this={newDialog}
    title="Create new file"
    confirm="Create"
    onconfirm={(filename: string, content: string) =>
        workspace.add(filename, content)}
/>

<PromptDialog
    bind:this={renameDialog}
    title="Rename the file"
    label="Filename:"
    confirm="Rename"
    onsubmit={execRename}
/>

<ContextMenu bind:this={contextMenu}>
    <button onclick={() => renameDialog.show()}><EditIcon /> Rename</button>
    {#if workspace.files.length > 1}
        <button class="danger" onclick={() => deleteDialog.show()}>
            <TrashIcon /> Delete
        </button>
    {/if}
</ContextMenu>

<SettingsDialog bind:this={settingsDialog} />

<style>
    aside {
        width: var(--menu-width);
        background-color: var(--bg-panel);
        border-right: 1px solid var(--border);
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
        color: var(--fg-disabled);
    }

    .add-btn {
        background: transparent;
        color: var(--fg);
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
        color: var(--fg-max);
        background: var(--bg-light);
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
        color: var(--fg);
        padding: 0.4rem 1rem;
        overflow: hidden;
    }

    .file-item:hover,
    .settings-btn:hover {
        background-color: var(--bg-light);
    }

    .file-item.active {
        background-color: var(--bg-light);
        color: var(--primary);
        border-left: 3px solid var(--primary);
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

    .footer {
        padding: 0.5rem 0;
        border-top: 1px solid var(--border);
        background-color: var(--bg-panel);
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
        gap: 0.5rem;
    }

    .settings-btn:hover {
        background-color: var(--bg-light);
        color: var(--fg-max);
    }
</style>
