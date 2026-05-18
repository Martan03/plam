<script lang="ts">
    import { onMount } from "svelte";
    import init, { eval_lambda } from "../../engine/pkg/plam.js";
    import initialCode from "../../pl-examples/stdlib.pl?raw";

    import Editor from "./lib/Editor.svelte";
    import Terminal from "./lib/Terminal.svelte";

    let outputValue = "System ready. Click 'Run' to evaluate.";
    let currentCode = initialCode;
    let isWasmLoaded = false;

    onMount(async () => {
        await init();
        isWasmLoaded = true;
    });

    function runEvaluation() {
        if (!isWasmLoaded) return;
        try {
            outputValue = eval_lambda(currentCode);
        } catch (e) {
            outputValue = `Error: ${e}`;
        }
    }
</script>

<main class="app-container">
    <header class="toolbar">
        <h1>plam</h1>
        <button onclick={runEvaluation} disabled={!isWasmLoaded}>
            ▶ Run
        </button>
    </header>

    <Editor bind:code={currentCode} />
    <Terminal output={outputValue} />
</main>

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

    .toolbar button {
        padding: 0.5rem 1.5rem;
        background: #3acbaf;
        color: #282c34;
        border: none;
        border-radius: 4px;
        font-weight: bold;
        cursor: pointer;
        transition: background 0.1s;
    }

    .toolbar button:hover {
        background: #30ad94;
    }

    .toolbar button:disabled {
        background: #4b5263;
        color: #282c34;
        cursor: not-allowed;
    }
</style>
