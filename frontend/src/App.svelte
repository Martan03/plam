<script lang="ts">
    import EditorPage from "./lib/pages/EditorPage.svelte";
    import WikiPage from "./lib/pages/WikiPage.svelte";
    import { router } from "./lib/state/router.svelte";
    import { settings } from "./lib/state/settings.svelte";

    $effect(() => {
        document.body.style.setProperty("--primary", settings.primaryColor);
        document.documentElement.setAttribute("data-theme", settings.theme);

        const metaColor = document.getElementById("theme-color-meta");
        if (metaColor) {
            const col = getComputedStyle(document.body)
                .getPropertyValue("--primary")
                .trim();
            metaColor.setAttribute("content", col);
        }
    });
</script>

{#if router.path === "/wiki"}
    <WikiPage />
{:else if router.path === "/"}
    <EditorPage />
{:else}
    <h1>404 - Page Not Found</h1>
{/if}
