<script lang="ts">
    import BaseDialog from "./BaseDialog.svelte";

    let {
        title = "Are you sure?",
        message,
        confirm = "Confirm",
        onconfirm,
    } = $props();

    let dialog: ReturnType<typeof BaseDialog>;

    export const show = () => dialog.show();
    export const close = () => dialog.close();

    function handleConfirm() {
        onconfirm();
        close();
    }
</script>

<BaseDialog bind:this={dialog} {title}>
    <p>{message}</p>

    {#snippet actions()}
        <button class="dialog-btn cancel-btn" onclick={close}>Cancel</button>
        <button class="dialog-btn danger-btn" onclick={handleConfirm}
            >{confirm}</button
        >
    {/snippet}
</BaseDialog>

<style>
    p {
        margin: 0;
        font-size: 0.95rem;
        line-height: 1.4;
    }
</style>
