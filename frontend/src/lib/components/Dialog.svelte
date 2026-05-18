<script lang="ts">
    let {
        title = "Are you sure?",
        message,
        confirm = "Confirm",
        onconfirm,
    } = $props();

    let dialog: HTMLDialogElement;

    export const show = () => dialog.showModal();
    export const close = () => dialog.close();

    function handleConfirm() {
        onconfirm();
        close();
    }
</script>

<dialog bind:this={dialog}>
    <h2>{title}</h2>
    <p>{message}</p>

    <div class="actions">
        <button class="cancel-btn" onclick={close}>Cancel</button>
        <button class="confirm-btn" onclick={handleConfirm}>{confirm}</button>
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

    dialog p {
        margin: 0 0 1.5rem 0;
        font-size: 0.95rem;
        line-height: 1.4;
    }

    .actions {
        display: flex;
        justify-content: flex-end;
        gap: 1rem;
    }

    .actions button {
        padding: 0.5rem 1.25rem;
        border-radius: 4px;
        font-weight: bold;
        cursor: pointer;
        border: none;
        transition: background 0.1s;
    }

    .cancel-btn {
        background: transparent;
        color: #abb2bf;
        border: 1px solid #4b5263 !important;
    }

    .cancel-btn:hover {
        background: #2c313a;
        color: #ffffff;
    }

    .confirm-btn {
        background: #e06c75;
        color: #282c34;
    }

    .confirm-btn:hover {
        background: #be5046;
    }
</style>
