<script lang="ts">
    import ContextMenu from "./ContextMenu.svelte";
    import ConfirmDialog from "./dialogs/ConfirmDialog.svelte";
    import CreateDialog from "./dialogs/CreateDialog.svelte";
    import PromptDialog from "./dialogs/PromptDialog.svelte";
    import EditIcon from "./icons/EditIcon.svelte";
    import TrashIcon from "./icons/TrashIcon.svelte";

    let { workspace } = $props();

    let deleteDialog: ReturnType<typeof ConfirmDialog>;
    let newDialog: ReturnType<typeof CreateDialog>;
    let renameDialog: ReturnType<typeof PromptDialog>;
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
</script>

<div class="header">
    <span>Files</span>
    <button class="add-btn" onclick={() => newDialog.show()} title="New File"
        >+</button
    >
</div>

<ul class="file-list">
    {#each workspace.files as file}
        <li
            class="sidebar-btn file-item"
            class:active={workspace.active === file}
            oncontextmenu={(e) => openContextMenu(e, file)}
        >
            <button class="sel-btn" onclick={() => workspace.select(file)}>
                <span class="filename">{file}</span>
            </button>
        </li>
    {/each}
</ul>

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

<style>
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
</style>
