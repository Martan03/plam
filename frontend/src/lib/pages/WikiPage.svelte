<script lang="ts">
    import { onMount } from "svelte";
    import init from "../../../../engine/pkg/plam.js";
    import AlertDialog from "../components/dialogs/AlertDialog.svelte";
    import ShareIcon from "../components/icons/ShareIcon.svelte";
    import Topbar from "../components/Topbar.svelte";
    import { persisted } from "../state/storage.svelte.js";
    import Sidebar from "../components/Sidebar.svelte";
    import { navigate } from "../state/router.svelte.js";
    import EditorIcon from "../components/icons/EditorIcon.svelte";
    import Syntax from "./wiki/Syntax.svelte";
    import Booleans from "./wiki/Booleans.svelte";
    import Numbers from "./wiki/Numbers.svelte";
    import Recursion from "./wiki/Recursion.svelte";
    import Tuples from "./wiki/Tuples.svelte";
    import Conclusion from "./wiki/Conclusion.svelte";
    import Toc from "../components/Toc.svelte";

    let isWasmLoaded = $state(false);
    const isMenuVisible = persisted("plam-menu-visible", true);

    let shareDialog: ReturnType<typeof AlertDialog>;

    onMount(async () => {
        await init();
        isWasmLoaded = true;
    });

    function shareCurrent() {
        navigator.clipboard.writeText(window.location.href);
        shareDialog.show();
    }

    const stdLibCode = String.raw`
let cnum = \a.a $increment $counter
let cbool = \b.b True False
let true  = \t f.t
let false = \t f.f
let !  = \a.a false true
let && = \a b.a b false
let || = \a b.a true b
let , = \a b f.f a b
let fst = \p.p true
let snd = \p.p false
let 0 = \f x.x
let succ = \a f x.a f (f x)
let 1 = succ 0
let 2 = succ (succ 0)
let 5 = succ (succ (succ 2))
let shift = \p., (succ (fst p)) (fst p)
let pred = \n.snd (n shift (, 0 0))
let + = \a b.a succ b
let - = \a b.b pred a
let * = \a b f.a (b f)
let isZero = \n.n (\_.false) true;
let gte = \a b.isZero (- b a)
let gt = \a b.! (gte b a)
let Y = \f.(\x.f (x x)) (\x.f (x x));
    `;
</script>

<main class="app-container">
    <Topbar bind:isMenuVisible={isMenuVisible.value}>
        <button class="icon-btn" onclick={shareCurrent} title="Share">
            <ShareIcon width="1.2rem" />
        </button>
    </Topbar>

    <div class="content">
        <Sidebar bind:isVisible={isMenuVisible.value}>
            <Toc selector=".content-wiki" />

            {#snippet actions()}
                <button
                    class="sidebar-action-btn"
                    onclick={() => navigate("/")}
                >
                    <EditorIcon /> Editor
                </button>
            {/snippet}
        </Sidebar>

        <div class="content-wiki">
            <Syntax {isWasmLoaded} />
            <Booleans />
            <Tuples />
            <Numbers {isWasmLoaded} {stdLibCode} />
            <Recursion {isWasmLoaded} {stdLibCode} />
            <Conclusion />
        </div>
    </div>
</main>

<AlertDialog
    bind:this={shareDialog}
    onconfirm={() => {}}
    title="Sharing wiki"
    message="Share link copied to clipboard!"
/>

<style>
    .content {
        display: flex;
        flex: 1;
        overflow: hidden;
        background-color: var(--bg);
    }

    .content-wiki {
        padding: 2rem;
        overflow-y: auto;
        flex: 1;
        min-width: 0;
    }

    .content-wiki :global(article) {
        max-width: 750px;
        margin: 0 auto;
        color: var(--fg);
        line-height: 1.6;
        font-size: 1.05rem;
    }

    .content-wiki :global(article h1) {
        font-size: 2.5rem;
        margin-top: 0;
        margin-bottom: 1.5rem;
        color: var(--fg-max);
    }

    .content-wiki :global(article h2) {
        font-size: 1.5rem;
        margin-top: 2rem;
        padding-bottom: 0.5rem;
        color: var(--fg-max);
    }

    .content-wiki :global(article h3) {
        font-size: 1.2rem;
        margin-top: 1rem;
        color: var(--fg-max);
    }

    .content-wiki :global(article p) {
        margin-bottom: 1.25rem;
    }

    .content-wiki :global(article ul) {
        padding-left: 1.5rem;
        margin-bottom: 1rem;
    }

    .content-wiki :global(article li) {
        margin-bottom: 0.5rem;
        line-height: 1.6;
    }

    .content-wiki :global(article code) {
        background-color: var(--bg-light);
        padding: 0.2rem 0.4rem;
        border-radius: 4px;
        font-family: monospace;
        font-size: 0.9em;
        color: var(--primary);
    }
</style>
