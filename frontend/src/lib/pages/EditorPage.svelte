<script lang="ts">
    import { onMount } from "svelte";
    import init, { eval_lambda } from "../../../../engine/pkg/plam.js";
    import initialCode from "../../../../pl-examples/stdlib.pl?raw";

    import Editor from "../components/Editor.svelte";
    import Terminal from "../components/Terminal.svelte";
    import { createWorkspace } from "../state/workspace.svelte.js";
    import Sidebar from "../components/Sidebar.svelte";
    import ConfirmDialog from "../components/dialogs/ConfirmDialog.svelte";
    import { persisted } from "../state/storage.svelte.js";
    import MenuIcon from "../components/icons/MenuIcon.svelte";
    import ResetDialog from "../components/dialogs/ResetDialog.svelte";
    import ShareIcon from "../components/icons/ShareIcon.svelte";
    import AlertDialog from "../components/dialogs/AlertDialog.svelte";
    import Topbar from "../components/Topbar.svelte";

    let outputValue = $state("System ready. Click 'Run' to evaluate.");
    let stdinValue = $state("");
    let isWasmLoaded = $state(false);
    let runtime = $state<number | null>(null);

    const workspace = createWorkspace(initialCode);
    const isMenuVisible = persisted("plam-menu-visible", true);

    let terminal: ReturnType<typeof Terminal>;
    let resetDialog: ReturnType<typeof ConfirmDialog>;
    let shareDialog: ReturnType<typeof AlertDialog>;

    onMount(async () => {
        await init();
        isWasmLoaded = true;

        const params = new URLSearchParams(window.location.search);
        const sharedCode = params.get("code");
        if (sharedCode) {
            workspace.addShared(sharedCode);
            const pathname = window.location.pathname;
            window.history.replaceState({}, document.title, pathname);
        }
    });

    function shareCurrent() {
        const url = workspace.share();
        navigator.clipboard.writeText(url);
        shareDialog.show();
    }

    function runEval() {
        if (!isWasmLoaded) return;
        try {
            const start = performance.now();
            outputValue = eval_lambda(workspace.currentCode, stdinValue);
            const end = performance.now();
            runtime = end - start;
        } catch (e) {
            outputValue = `Error: ${e}`;
        }
        terminal?.showOutput();
    }

    function showReset() {
        resetDialog.show();
    }

    function resetCode(content: string) {
        workspace.currentCode = content;
    }
</script>

<main class="app-container">
    <Topbar>
        {#snippet pretitle()}
            <button
                class="icon-btn"
                title="{isMenuVisible.value ? 'Hide' : 'Show'} menu"
                onclick={() => (isMenuVisible.value = !isMenuVisible.value)}
            >
                <MenuIcon width="1em" />
            </button>
        {/snippet}

        <button class="icon-btn" onclick={shareCurrent} title="Share">
            <ShareIcon width="1.2rem" />
        </button>
        <button class="btn secondary" onclick={showReset}> Reset Code </button>
        <button class="btn" onclick={runEval} disabled={!isWasmLoaded}>
            ▶ Run
        </button>
    </Topbar>

    <div class="content">
        <Sidebar {workspace} bind:isVisible={isMenuVisible.value} />

        <div class="editor">
            <Editor bind:code={workspace.currentCode} />
            <Terminal
                bind:this={terminal}
                output={outputValue}
                {runtime}
                bind:input={stdinValue}
            />
        </div>
    </div>
</main>

<ResetDialog bind:this={resetDialog} onconfirm={resetCode} />
<AlertDialog
    bind:this={shareDialog}
    onconfirm={() => {}}
    title="Sharing code"
    message="Share link copied to clipboard!"
/>

<style>
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
</style>
