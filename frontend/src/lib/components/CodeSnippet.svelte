<script lang="ts">
    import { bracketMatching } from "@codemirror/language";
    import { onMount } from "svelte";
    import { EditorView, basicSetup } from "codemirror";
    import { EditorState } from "@codemirror/state";
    import { editorSyntax, editorTheme, snippetTheme } from "../highlighter";

    let { code = "", runnable = false } = $props();

    let container: HTMLDivElement;
    let view: EditorView;

    onMount(() => {
        const extensions = [
            basicSetup,
            editorTheme,
            snippetTheme,
            editorSyntax,
            bracketMatching(),
            EditorState.readOnly.of(true),
            EditorView.editable.of(false),
        ];

        view = new EditorView({
            doc: code.trim(),
            extensions,
            parent: container,
        });

        return () => view.destroy();
    });
</script>

<div class="wrapper">
    <div bind:this={container} class="cm-container"></div>

    {#if runnable}
        <div class="actions">
            <button class="btn secondary">▶ Run Example</button>
        </div>
    {/if}
</div>

<style>
    .wrapper {
        margin: 1.5rem 0;
        border: 1px solid var(--border);
        border-radius: 8px;
        overflow: hidden;
        background-color: var(--bg-panel);
    }

    .cm-container {
        padding: 0.5rem;
    }

    .actions {
        padding: 0.5rem;
        border-top: 1px solid var(--border);
        display: flex;
        justify-content: flex-end;
        background-color: var(--bg-dark);
    }

    :global(.cm-activeLine) {
        background-color: transparent !important;
    }

    :global(.wrapper .cm-gutters) {
        display: none;
    }
</style>
