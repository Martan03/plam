<script lang="ts">
    import type { Snippet } from "svelte";
    import BaseDialog from "./BaseDialog.svelte";

    interface Props {
        title: string;
        label: string;
        children?: Snippet;
        confirm: string;
        onsubmit: (filename: string) => void;
    }
    let {
        title = "Enter value",
        label = "",
        children,
        confirm = "Submit",
        onsubmit,
    }: Props = $props();

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
    <div class="dialog-input-wrapper">
        {#if label}
            <label class="dialog-label" for="prompt-input">{label}</label>
        {/if}
        <input
            bind:this={inputElem}
            type="text"
            id="prompt-input"
            class="dialog-input"
            bind:value={inputValue}
            onkeydown={handleKey}
            placeholder="e.g. test.pl"
            autocomplete="off"
        />
    </div>

    {#if children}
        {@render children()}
    {/if}

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
    .dialog-input-wrapper {
        margin-bottom: 0.5rem;
    }
</style>
