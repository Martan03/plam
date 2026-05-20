<script lang="ts">
    import { persisted } from "../state/storage.svelte";
    import MaximizeIcon from "./icons/MaximizeIcon.svelte";
    import MinimizeIcon from "./icons/MinimizeIcon.svelte";

    let { output = $bindable(), input = $bindable("") } = $props();

    const termHeight = persisted("plam-term-height", 200);
    let isMaximized = $state(false);
    let isDragging = $state(false);

    let activeTab = $state<"output" | "stdin">("output");

    const toggleMaximized = () => (isMaximized = !isMaximized);

    function startDrag(e: PointerEvent) {
        isDragging = true;
        isMaximized = false;
        e.preventDefault();
    }

    function onPointerMove(e: PointerEvent) {
        if (!isDragging) return;

        const height = window.innerHeight - e.clientY;
        if (height > 30) {
            termHeight.value = height;
        }
    }

    const onPointerUp = () => (isDragging = false);

    export const showOutput = () => (activeTab = "output");
</script>

<svelte:window onpointermove={onPointerMove} onpointerup={onPointerUp} />

<div
    class="terminal"
    class:maximized={isMaximized}
    style="--term-height: {termHeight.value}px"
>
    <button
        class="resizer"
        class:dragging={isDragging}
        onpointerdown={startDrag}
        aria-label="Resize terminal"
    ></button>

    <div class="terminal-header">
        <div class="tabs">
            <button
                class:active={activeTab === "output"}
                onclick={() => (activeTab = "output")}>Output</button
            >
            <button
                class:active={activeTab === "stdin"}
                onclick={() => (activeTab = "stdin")}>Input</button
            >
        </div>

        <button class="icon-btn" onclick={toggleMaximized}>
            {#if isMaximized}
                <MinimizeIcon width="0.8rem" />
            {:else}
                <MaximizeIcon width="0.8rem" />
            {/if}
        </button>
    </div>

    <div class="terminal-content">
        {#if activeTab === "output"}
            <div class="output-view">
                {output}
            </div>
        {:else}
            <textarea bind:value={input} placeholder="Enter standard input here"
            ></textarea>
        {/if}
    </div>
</div>

<style>
    .terminal {
        display: flex;
        flex-direction: column;
        background-color: #1e2227;
        border-top: 1px solid #181a1f;
        height: var(--term-height);
        flex-shrink: 0;
        position: relative;
    }

    .terminal.maximized {
        height: 100%;
        position: absolute;
        bottom: 0;
        left: 0;
        right: 0;
        z-index: 10;
    }

    .resizer {
        position: absolute;
        top: -4px;
        left: 0;
        right: 0;
        height: 6px;
        cursor: row-resize;
        z-index: 10;
        background-color: transparent;
        border: none;
    }

    .resizer:hover,
    .resizer.dragging {
        background-color: color-mix(in srgb, var(--primary) 50%, transparent);
    }

    .terminal-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 0 1rem;
        height: 30px;
        min-height: 30px;
        background-color: #21252b;
        color: inherit;
        border: none;
        cursor: pointer;
        font-size: 0.9rem;
        font-weight: bold;
        user-select: none;
    }

    .tabs {
        display: flex;
        height: 100%;
    }

    .tabs button {
        background: transparent;
        border: none;
        color: #5c6370;
        cursor: pointer;
        font-family: inherit;
        font-size: 0.85rem;
        padding: 0 1rem;
        height: 100%;
        border-top: 2px solid transparent;
        transition:
            color 0.1s,
            background-color 0.1s;
    }

    .tabs button:hover {
        background-color: #282c34;
        color: #abb2bf;
    }

    .tabs button.active {
        color: #ffffff;
        background-color: #1e2227;
        border-top: 2px solid var(--primary, #3acbaf);
    }

    .icon-btn {
        background: transparent;
        border: none;
        color: inherit;
        cursor: pointer;
        padding-top: 0.25rem;
    }

    .terminal-content {
        flex: 1;
        flex-direction: column;
        font-family: monospace;
        font-size: 0.9rem;
        overflow: hidden;
    }

    .output-view {
        flex: 1;
        padding: 0.5rem 1rem;
        overflow-y: auto;
        white-space: pre-wrap;
    }

    .terminal-content textarea {
        flex: 1;
        width: 100%;
        height: 100%;
        background: inherit;
        border: none;
        color: inherit;
        padding: 0.5rem 1rem;
        font-size: inherit;
        font-family: inherit;
        resize: none;
        outline: none;
    }
</style>
