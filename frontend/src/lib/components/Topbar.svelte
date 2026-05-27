<script lang="ts">
    import type { Snippet } from "svelte";
    import MenuIcon from "./icons/MenuIcon.svelte";

    let {
        isMenuVisible = $bindable(true),
        children,
    }: { isMenuVisible: boolean; children: Snippet } = $props();
</script>

<header class="toolbar">
    <div class="header">
        <button
            class="icon-btn"
            title="{isMenuVisible ? 'Hide' : 'Show'} menu"
            onclick={() => (isMenuVisible = !isMenuVisible)}
        >
            <MenuIcon width="1em" />
        </button>
        <h1>plam</h1>
    </div>
    <div class="controls">
        {@render children()}
    </div>
</header>

<style>
    .toolbar {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 0.75rem 1.5rem;
        padding-left: 0.5rem;
        background-color: var(--bg-panel);
        border-bottom: 1px solid var(--border);
    }

    .toolbar h1 {
        margin: 0;
        font-size: 1.2rem;
        font-weight: 600;
    }

    .toolbar .header h1:first-child {
        margin-left: 0.5rem;
    }

    .toolbar .controls,
    .toolbar .header {
        display: flex;
        gap: 1rem;
        align-items: center;
    }

    :global(.toolbar .controls .btn) {
        padding: 0.5rem 1.5rem;
        background: var(--primary);
        color: var(--bg);
        border: 1px solid transparent;
        border-radius: 4px;
        font-weight: bold;
        cursor: pointer;
        transition: background 0.1s;
    }

    :global(.toolbar .controls button.secondary) {
        background: transparent;
        color: var(--fg);
        border: 1px solid var(--border-light);
    }

    :global(.toolbar .controls .btn:hover) {
        background: color-mix(in srgb, var(--primary), var(--hover-dim) 20%);
    }

    :global(.toolbar .controls button.secondary:hover) {
        background: var(--bg-light);
        color: var(--fg-max);
    }

    :global(.toolbar .controls .btn:disabled) {
        background: var(--border-light);
        color: var(--bg);
        cursor: not-allowed;
    }

    :global(.toolbar .icon-btn) {
        background: transparent;
        color: var(--fg);
        border: none;
        cursor: pointer;
        font-size: 1.5rem;
        border-radius: 4px;
        width: 2.1rem;
        height: 2.1rem;
        display: flex;
        align-items: center;
        justify-content: center;
        outline: none;
    }

    :global(.toolbar .icon-btn:hover) {
        color: var(--fg-max);
        background: var(--bg-light);
    }
</style>
