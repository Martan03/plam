<script lang="ts">
    import type { Snippet } from "svelte";
    import { persisted } from "../state/storage.svelte";
    import SettingsIcon from "./icons/SettingsIcon.svelte";
    import SettingsDialog from "./SettingsDialog.svelte";

    interface Props {
        children?: Snippet | null;
        actions?: Snippet | null;
        isVisible: boolean;
    }
    let { children, actions, isVisible = $bindable(true) }: Props = $props();

    const width = persisted("plam-menu-width", 200);
    let isDragging = $state(false);

    let settingsDialog: ReturnType<typeof SettingsDialog>;

    function startDrag(e: PointerEvent) {
        isDragging = true;
        e.preventDefault();
    }

    function onPointerMove(e: PointerEvent) {
        if (!isDragging) return;

        const newWidth = e.clientX;
        if (newWidth > 5) {
            width.value = newWidth;
        }
    }

    const onPointerUp = () => (isDragging = false);
</script>

<svelte:window onpointermove={onPointerMove} onpointerup={onPointerUp} />

<aside style="--menu-width: {width.value}px" class:hidden={!isVisible}>
    <button
        class="resizer"
        class:dragging={isDragging}
        onpointerdown={startDrag}
        aria-label="Resize file explorer"
    ></button>

    <div class="content">
        {@render children?.()}
    </div>

    <div class="footer">
        {@render actions?.()}
        <button
            class="sidebar-btn sidebar-action-btn"
            title="Open Settings"
            onclick={() => settingsDialog.show()}
        >
            <SettingsIcon /> Settings
        </button>
    </div>
</aside>

<SettingsDialog bind:this={settingsDialog} />

<style>
    aside {
        width: var(--menu-width);
        background-color: var(--bg-panel);
        border-right: 1px solid var(--border);
        display: flex;
        flex-direction: column;
        flex-shrink: 0;
        position: relative;
        transition: margin-left 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    }

    aside.hidden {
        margin-left: calc(var(--menu-width) * -1);
        border-right: none;
    }

    .resizer {
        position: absolute;
        right: -4px;
        top: 0;
        bottom: 0;
        width: 6px;
        cursor: col-resize;
        z-index: 10;
        background-color: transparent;
        border: none;
    }

    .resizer:hover,
    .resizer.dragging {
        background-color: color-mix(in srgb, var(--primary) 50%, transparent);
    }

    .content {
        flex: 1;
        display: flex;
        flex-direction: column;
    }

    :global(.sidebar-btn),
    :global(.sidebar-action-btn) {
        display: flex;
        align-items: center;
        justify-content: space-between;
        color: var(--fg);
        padding: 0.4rem 1rem;
        overflow: hidden;
    }

    :global(.sidebar-btn:hover),
    :global(.sidebar-action-btn:hover) {
        background-color: var(--bg-light);
    }

    .footer {
        padding: 0.5rem 0;
        border-top: 1px solid var(--border);
        background-color: var(--bg-panel);
    }

    :global(.sidebar-action-btn) {
        width: 100%;
        justify-content: flex-start;
        background: inherit;
        border: none;
        font-family: inherit;
        font-size: 0.9rem;
        color: inherit;
        cursor: pointer;
        gap: 0.6rem;
    }

    :global(.sidebar-action-btn svg) {
        width: 1.1rem;
    }

    :global(.sidebar-action-btn:hover) {
        background-color: var(--bg-light);
        color: var(--fg-max);
    }
</style>
