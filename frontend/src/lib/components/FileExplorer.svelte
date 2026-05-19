<script lang="ts">
    import { persisted } from "../state/storage.svelte";
    import ConfirmDialog from "./dialogs/ConfirmDialog.svelte";
    import PromptDialog from "./dialogs/PromptDialog.svelte";

    let { workspace } = $props();

    const width = persisted("plam-menu-width", 200);

    let deleteDialog: ReturnType<typeof ConfirmDialog>;
    let newDialog: ReturnType<typeof PromptDialog>;

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
</script>

<aside style="--menu-width: {width.value}px">
    <div class="header">
        <span>Files</span>
        <button class="add-btn" onclick={promptNew} title="New File">+</button>
    </div>

    <ul>
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

<style>
    aside {
        width: var(--menu-width);
        background-color: #21252b;
        border-right: 1px solid #181a1f;
        display: flex;
        flex-direction: column;
        flex-shrink: 0;
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
    }

    .add-btn:hover {
        color: #ffffff;
        background: #2c313a;
    }

    ul {
        list-style: none;
        margin: 0;
        padding: 0;
        overflow-y: auto;
    }

    .file-item {
        display: flex;
        align-items: center;
        justify-content: space-between;
        color: #abb2bf;
        padding: 0.4rem 1rem;
    }

    .file-item:hover {
        background-color: #2c313a;
    }

    .file-item.active {
        background-color: #2c313a;
        color: #3acbaf;
        border-left: 3px solid #3acbaf;
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
</style>
