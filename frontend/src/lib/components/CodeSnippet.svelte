<script lang="ts">
    import { bracketMatching } from "@codemirror/language";
    import { onMount } from "svelte";
    import { EditorView, basicSetup } from "codemirror";
    import { EditorState } from "@codemirror/state";
    import { editorSyntax, editorTheme, snippetTheme } from "../highlighter";
    import { eval_lambda } from "../../../../engine/pkg/plam";

    let { hiddenCode = "", code = "", runnable = false } = $props();

    let output = $state("");

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

    function runEval() {
        try {
            output = eval_lambda(`${hiddenCode}\n${code}`, "");
        } catch (e) {
            output = `Error: ${e}`;
        }
    }
</script>

<div class="wrapper">
    <div bind:this={container} class="cm-container"></div>

    {#if runnable}
        <div class="output-bar">
            <div class="output">{output}</div>
            <button class="btn secondary" onclick={runEval}>▶ Run</button>
        </div>
    {/if}
</div>

<style>
    .wrapper {
        margin: 1.5rem 0;
        border: 1px solid var(--border);
        border-radius: 8px;
        overflow: hidden;
        background-color: var(--bg-light);
    }

    .cm-container {
        padding: 0.5rem;
    }

    .output-bar {
        padding: 0.5rem;
        border-top: 1px solid var(--border);
        display: flex;
        align-items: flex-start;
        justify-content: space-between;
        background-color: var(--bg);
    }

    .output {
        white-space: pre-wrap;
    }

    :global(.cm-activeLine) {
        background-color: transparent !important;
    }

    :global(.wrapper .cm-gutters) {
        display: none;
    }

    .btn {
        padding: 0.3rem 0.5rem;
        background: transparent;
        color: var(--fg);
        border: 1px solid var(--border-light);
        border-radius: 4px;
        font-weight: bold;
        cursor: pointer;
        transition: background 0.1s;
    }

    .btn:hover {
        background: var(--bg-light);
        color: var(--fg-max);
    }
</style>
