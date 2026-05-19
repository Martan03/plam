<script lang="ts">
    import BaseDialog from "./BaseDialog.svelte";

    let {
        title = "Enter value",
        label = "",
        confirm = "Submit",
        onsubmit,
    } = $props();

    let dialog: ReturnType<typeof BaseDialog>;
    let inputElem: HTMLInputElement;
    let inputValue = $state("");

    export const show = () => {
        inputValue = "";
        dialog.show();
        inputElem?.focus();
    };
    export const close = () => dialog.close();

    function handleSubmit() {
        const val = inputValue.trim();
        if (val) {
            onsubmit(val);
            close();
        }
    }

    function handleKey(e: KeyboardEvent) {
        if (e.key === "Enter") {
            e.preventDefault();
            handleSubmit();
        }
    }
</script>

<BaseDialog bind:this={dialog} {title}>
    <div class="input-wrapper">
        {#if label}
            <label for="prompt-input">{label}</label>
        {/if}
        <input
            bind:this={inputElem}
            type="text"
            id="prompt-input"
            bind:value={inputValue}
            onkeydown={handleKey}
            placeholder="e.g. test.pl"
            autocomplete="off"
        />
    </div>

    {#snippet actions()}
        <button class="dialog-btn cancel-btn" onclick={close}>Cancel</button>
        <button
            class="dialog-btn primary-btn"
            onclick={handleSubmit}
            disabled={!inputValue.trim()}>{confirm}</button
        >
    {/snippet}
</BaseDialog>

<style>
    .input-wrapper {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    label {
        font-size: 0.9rem;
        color: #abb2bf;
    }

    input {
        background-color: #181a1f;
        border: 1px solid #4b5263;
        color: #abb2bf;
        padding: 0.5rem;
        border-radius: 4px;
        font-family: inherit;
        font-size: 1rem;
    }

    input:focus {
        outline: none;
        border-color: var(--primary, #3acbaf);
    }
</style>
