<script lang="ts">
    let { selector } = $props();

    type TocItem = {
        id: string;
        text: string;
        level: number;
    };
    let tocList = $state<TocItem[]>([]);

    $effect(() => {
        const content = document.querySelector(selector);
        if (!content) return;

        const headings = content.querySelectorAll("h2, h3");
        const items: TocItem[] = [];

        headings.forEach((heading: HTMLHeadingElement) => {
            if (!heading.id) {
                const id = heading.textContent
                    ?.toLowerCase()
                    .replace(/[^a-z0-9]+/g, "-")
                    .replace(/(^-|-$)/g, "");
                heading.id = id;
            }

            items.push({
                id: heading.id,
                text: heading.textContent || "",
                level: heading.tagName === "H2" ? 2 : 3,
            });
        });
        tocList = items;
    });
</script>

<div class="toc">
    <h3>Contents</h3>

    <nav>
        {#each tocList as item}
            <a href="#{item.id}" class={item.level === 3 ? "sub-item" : ""}>
                {item.text}
            </a>
        {/each}
    </nav>
</div>

<style>
    .toc {
        padding: 0.5rem 1rem;
    }

    nav {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
        margin-top: 0.5rem;
    }

    nav a {
        color: var(--fg);
        text-decoration: none;
        font-size: 0.95rem;
        transition: color 0.2s ease;
    }

    nav a.sub-item {
        padding-left: 1rem;
        font-size: 0.85rem;
        opacity: 0.6;
    }

    nav a:hover {
        color: var(--primary);
    }
</style>
