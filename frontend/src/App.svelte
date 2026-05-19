<script lang="ts">
    import { onMount } from "svelte";
    import init, { eval_lambda } from "../../engine/pkg/plam.js";
    import initialCode from "../../pl-examples/stdlib.pl?raw";

    import Editor from "./lib/components/Editor.svelte";
    import Terminal from "./lib/components/Terminal.svelte";
    import { createWorkspace } from "./lib/state/workspace.svelte.js";
    import FileExplorer from "./lib/components/FileExplorer.svelte";
    import ConfirmDialog from "./lib/components/dialogs/ConfirmDialog.svelte";

    let outputValue = $state("System ready. Click 'Run' to evaluate.");
    let isWasmLoaded = $state(false);

    const workspace = createWorkspace(initialCode);

    let resetDialog: ReturnType<typeof ConfirmDialog>;

    onMount(async () => {
        await init();
        isWasmLoaded = true;
    });

    function runEvaluation() {
        if (!isWasmLoaded) return;
        try {
            outputValue = eval_lambda(workspace.currentCode);
        } catch (e) {
            outputValue = `Error: ${e}`;
        }
    }

    function showReset() {
        resetDialog.show();
    }

    function resetCode() {
        workspace.currentCode = initialCode;
    }
</script>

<main class="app-container">
    <header class="toolbar">
        <h1>plam</h1>
        <div class="controls">
            <button class="secondary" onclick={showReset}>Reset Code</button>
            <button onclick={runEvaluation} disabled={!isWasmLoaded}>
                ▶ Run
            </button>
        </div>
    </header>

    <div class="content">
        <FileExplorer {workspace} />

        <div class="editor">
            <Editor bind:code={workspace.currentCode} />
            <Terminal output={outputValue} />
        </div>
    </div>
</main>

<ConfirmDialog
    bind:this={resetDialog}
    title="Reset code?"
    message="This will reset your current code. This action cannot be undone."
    confirm="Reset"
    onconfirm={resetCode}
/>

<style>
    * {
        box-sizing: border-box;
    }

    .app-container {
        display: flex;
        flex-direction: column;
        height: 100vh;
        width: 100vw;
        background-color: #282c34;
        color: #abb2bf;
        font-family: sans-serif;
        text-align: left;
    }

    .content {
        display: flex;
        flex: 1;
        overflow: hidden;
    }

    .editor {
        display: flex;
        flex-direction: column;
        flex: 1;
        min-width: 0;
    }

    .toolbar {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 0.75rem 1.5rem;
        background-color: #21252b;
        border-bottom: 1px solid #181a1f;
    }

    .toolbar h1 {
        margin: 0;
        font-size: 1.2rem;
        font-weight: 600;
    }

    .toolbar .controls {
        display: flex;
        gap: 1rem;
        align-items: center;
    }

    .toolbar button {
        padding: 0.5rem 1.5rem;
        background: #3acbaf;
        color: #282c34;
        border: 1px solid transparent;
        border-radius: 4px;
        font-weight: bold;
        cursor: pointer;
        transition: background 0.1s;
    }

    .toolbar button.secondary {
        background: transparent;
        color: #abb2bf;
        border: 1px solid #4b5263;
    }

    .toolbar button:hover {
        background: #30ad94;
    }

    .toolbar button.secondary:hover {
        background: #2c313a;
        color: #ffffff;
    }

    .toolbar button:disabled {
        background: #4b5263;
        color: #282c34;
        cursor: not-allowed;
    }
</style>
