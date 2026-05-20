<script lang="ts">
    import { tick, type Snippet } from "svelte";

    let { children }: { children: Snippet } = $props();

    let menuElement = $state<HTMLDivElement | null>(null);
    let isOpen = $state(false);
    let x = $state(0);
    let y = $state(0);

    export async function open(e: MouseEvent) {
        e.preventDefault();
        x = e.clientX;
        y = e.clientY;
        isOpen = true;

        await tick();
        if (menuElement) {
            const rect = menuElement.getBoundingClientRect();
            if (x + rect.width > window.innerWidth) {
                x = window.innerWidth - rect.width - 5;
            }
            if (y + rect.height > window.innerHeight) {
                y = window.innerHeight - rect.height - 5;
            }
        }
    }

    export function close() {
        isOpen = false;
    }
</script>

<svelte:window
    onclick={close}
    onkeydown={(e) => e.key === "Escape" && close()}
/>

{#if isOpen}
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
        bind:this={menuElement}
        class="context-menu"
        style="top: {y}px; left: {x}px;"
        oncontextmenu={(e) => e.preventDefault()}
    >
        {@render children()}
    </div>
{/if}

<style>
    .context-menu {
        position: fixed;
        z-index: 1000;
        background-color: #282c34;
        border: 1px solid #181a1f;
        border-radius: 6px;
        padding: 0.25rem;
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.4);
        display: flex;
        flex-direction: column;
        min-width: 110px;
    }

    :global(.context-menu button) {
        background: transparent;
        border: none;
        color: #abb2bf;
        text-align: left;
        padding: 0.4rem 0.5rem;
        border-radius: 4px;
        cursor: pointer;
        font-family: inherit;
        font-size: 0.9rem;
        display: flex;
        align-items: center;
        gap: 0.5rem;
    }

    :global(.context-menu button:hover) {
        background-color: var(--primary, #3acbaf);
        color: #282c34;
    }

    :global(.context-menu button.danger:hover) {
        background-color: #e06c75;
        color: #282c34;
    }

    :global(.context-menu button svg) {
        width: 1.1rem;
        height: 1.1rem;
        flex-shrink: 0;
    }
</style>
