<script lang="ts">
    import { templates } from "../../state/templates";
    import BaseDialog from "./BaseDialog.svelte";

    interface Props {
        onconfirm: (content: string) => void;
    }
    let { onconfirm }: Props = $props();

    let dialog: ReturnType<typeof BaseDialog>;
    let selectValue = $state<string | null>(null);

    export const show = () => {
        selectValue = null;
        dialog.show();
    };
    export const close = () => dialog.close();

    function handleConfirm() {
        const template = templates.find((t) => t.name === selectValue);
        const content = template ? template.content : "";
        onconfirm(content);
        close();
    }
</script>

<BaseDialog bind:this={dialog} title="Reset code?">
    <p>
        This will reset your current to selected template. This action cannot be
        undone.
    </p>
    <div class="dialog-input-wrapper">
        <label for="prompt-select" class="dialog-label">Template:</label>
        <select
            class="dialog-input"
            id="prompt-select"
            bind:value={selectValue}
        >
            <option value={null}>None</option>
            {#each templates as template}
                <option value={template.name}>{template.name}</option>
            {/each}
        </select>
    </div>

    {#snippet actions()}
        <button class="dialog-btn cancel-btn" onclick={close}>Cancel</button>
        <button class="dialog-btn danger-btn" onclick={handleConfirm}
            >Reset</button
        >
    {/snippet}
</BaseDialog>

<style>
    p {
        margin-bottom: 1rem;
    }
</style>
