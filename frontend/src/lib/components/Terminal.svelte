<script lang="ts">
    import { persisted } from "../state/storage.svelte";

    let { output = $bindable() } = $props();

    const termHeight = persisted("plam-term-height", 200);
    let isMaximized = $state(false);
    let isDragging = $state(false);

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
        <span>Program output</span>

        <button class="icon-btn" onclick={toggleMaximized}>
            {isMaximized ? "🗗" : "🗖"}
        </button>
    </div>

    <div class="terminal-content">{output}</div>
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
        background-color: #3acbaf50;
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

    .icon-btn {
        background: transparent;
        border: none;
        color: inherit;
        cursor: pointer;
        padding-top: 0.25rem;
    }

    .terminal-content {
        flex: 1;
        padding: 0.5rem 1rem;
        overflow-y: auto;
        font-family: monospace;
        font-size: 0.9rem;
        white-space: pre-wrap;
    }
</style>
