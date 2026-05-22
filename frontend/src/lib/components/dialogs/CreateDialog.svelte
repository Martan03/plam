<script lang="ts">
    import { templates } from "../../state/templates";
    import PromptDialog from "./PromptDialog.svelte";

    interface Props {
        title: string;
        confirm: string;
        onconfirm: (filename: string, template: string) => void;
    }
    let { onconfirm, ...rest }: Props = $props();

    let dialog: ReturnType<typeof PromptDialog>;
    let selectValue = $state<string | null>(null);

    export const show = () => dialog.show();
    export const close = () => dialog.close();

    function handleConfirm(filename: string) {
        const template = templates.find((t) => t.name === selectValue);
        const content = template ? template.content : "";
        onconfirm(filename, content);
        close();
    }
</script>

<PromptDialog
    bind:this={dialog}
    label="Filename:"
    {...rest}
    onsubmit={handleConfirm}
>
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
</PromptDialog>
