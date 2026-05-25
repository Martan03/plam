<script lang="ts">
    import CodeSnippet from "../components/CodeSnippet.svelte";
    import AlertDialog from "../components/dialogs/AlertDialog.svelte";
    import ShareIcon from "../components/icons/ShareIcon.svelte";
    import Topbar from "../components/Topbar.svelte";

    let shareDialog: ReturnType<typeof AlertDialog>;

    function shareCurrent() {
        navigator.clipboard.writeText(window.location.href);
        shareDialog.show();
    }
</script>

<main class="app-container">
    <Topbar>
        <button class="icon-btn" onclick={shareCurrent} title="Share">
            <ShareIcon width="1.2rem" />
        </button>
    </Topbar>

    <div class="content">
        <article>
            <h1>Lambda Calculus</h1>

            <p>This is not done just yet, but it might be in the future :).</p>

            <h2>Basic Syntax</h2>
            <ul>
                <li>
                    <strong>Variables:</strong> Represented by characters like
                    <code>x</code>
                    or <code>y</code>.
                </li>
                <li>
                    <strong>Abstraction:</strong> Creating a function, written
                    as <code>\x.x</code>.
                </li>
                <li>
                    <strong>Application:</strong> Calling a function, written as
                    <code>(M N)</code>.
                </li>
            </ul>
        </article>

        <article class="wiki-content">
            <h2>Basic Syntax</h2>

            <p>The code snippet can also be runnable (in the future)</p>

            <CodeSnippet runnable={true} code="(\x. \y. x y) a b" />
        </article>
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
        flex: 1;
        overflow-y: auto;
    }

    .content {
        flex: 1;
        overflow-y: auto;
        padding: 2rem;
        background-color: var(--bg);
    }

    .content article {
        max-width: 750px;
        margin: 0 auto;
        color: var(--fg);
        line-height: 1.6;
        font-size: 1.05rem;
    }

    .content article h1 {
        font-size: 2.5rem;
        margin-top: 0;
        margin-bottom: 1.5rem;
        color: var(--fg-max);
    }

    .content article h2 {
        font-size: 1.5rem;
        margin-top: 2rem;
        padding-bottom: 0.5rem;
        color: var(--fg-max);
    }

    .content article p {
        margin-bottom: 1.25rem;
    }

    .content article ul {
        padding-left: 1.5rem;
    }

    .content article li {
        margin-bottom: 0.5rem;
        line-height: 1.6;
    }

    .content article code {
        background-color: var(--bg-panel);
        padding: 0.2rem 0.4rem;
        border-radius: 4px;
        font-family: monospace;
        font-size: 0.9em;
        color: var(--primary);
    }
</style>
