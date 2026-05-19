<script lang="ts">
    import type { Snippet } from "svelte";

    let {
        title,
        children,
        actions,
    }: {
        title: string;
        children: Snippet;
        actions: Snippet;
    } = $props();

    let dialog: HTMLDialogElement;

    export const show = () => dialog.showModal();
    export const close = () => dialog.close();
</script>

<dialog bind:this={dialog}>
    <h2>{title}</h2>

    <div class="content">
        {@render children()}
    </div>

    <div class="actions">
        {@render actions()}
    </div>
</dialog>

<style>
    dialog {
        background-color: #21252b;
        color: #abb2bf;
        border: 1px solid #181a1f;
        border-radius: 8px;
        padding: 1.5rem;
        width: 400px;
        max-width: 90vw;
        box-shadow: 0 10px 30px rgba(0, 0, 0, 0.5);
        font-family: "Segoe UI", Roboto, sans-serif;
        margin: auto;
    }

    dialog::backdrop {
        background: rgba(0, 0, 0, 0.6);
        backdrop-filter: blur(2px);
    }

    dialog h2 {
        margin: 0 0 1rem 0;
        font-size: 1.25rem;
        color: #9aeddc;
    }

    .content {
        margin-bottom: 1.5rem;
    }

    .actions {
        display: flex;
        justify-content: flex-end;
        gap: 1rem;
    }

    :global(.dialog-btn) {
        padding: 0.5rem 1.25rem;
        border-radius: 4px;
        font-weight: bold;
        cursor: pointer;
        border: none;
        transition: background 0.1s;
    }

    :global(.dialog-btn:disabled) {
        background: #4b5263 !important;
        color: #abb2bf !important;
        cursor: not-allowed;
        opacity: 0.6;
    }

    :global(.cancel-btn) {
        background: transparent;
        color: #abb2bf;
        border: 1px solid #4b5263 !important;
    }

    :global(.cancel-btn:hover:not(:disabled)) {
        background: #2c313a;
        color: #ffffff;
    }

    :global(.danger-btn) {
        background: #e06c75;
        color: #282c34;
    }

    :global(.danger-btn:hover:not(:disabled)) {
        background: #be5046;
    }

    :global(.primary-btn) {
        background: #3acbaf;
        color: #282c34;
    }

    :global(.primary-btn:hover:not(:disabled)) {
        background: #30ad94;
    }
</style>
