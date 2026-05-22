<script lang="ts">
    import type { Snippet } from "svelte";

    let {
        title,
        children,
        actions,
        width = "400px",
    }: {
        title: string;
        children: Snippet;
        actions: Snippet;
        width?: string;
    } = $props();

    let dialog: HTMLDialogElement;

    export const show = () => dialog.showModal();
    export const close = () => dialog.close();
</script>

<dialog bind:this={dialog} style="--dialog-width: {width};">
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
        padding: 1.3rem 1.5rem;
        width: var(--dialog-width);
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
        color: color-mix(in srgb, var(--primary, #3acbaf) 35%, white 65%);
    }

    .content {
        margin-bottom: 1.5rem;
    }

    .actions {
        display: flex;
        justify-content: flex-end;
        gap: 1rem;
    }

    :global(.dialog-input-wrapper) {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    :global(.dialog-label) {
        font-size: 0.9rem;
        color: #abb2bf;
    }

    :global(.dialog-input) {
        background-color: #181a1f;
        border: 1px solid #4b5263;
        color: #abb2bf;
        padding: 0.5rem;
        border-radius: 4px;
        font-family: inherit;
        font-size: 1rem;
    }

    :global(select.dialog-input) {
        cursor: pointer;
    }

    :global(.dialog-input:focus) {
        outline: none;
        border-color: var(--primary, #3acbaf);
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
        background: var(--primary, #3acbaf);
        color: #282c34;
    }

    :global(.primary-btn:hover:not(:disabled)) {
        background: color-mix(in srgb, var(--primary, #3acbaf), black 20%);
    }
</style>
