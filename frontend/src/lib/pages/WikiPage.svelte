<script lang="ts">
    import { onMount } from "svelte";
    import init, { eval_lambda } from "../../../../engine/pkg/plam.js";
    import CodeSnippet from "../components/CodeSnippet.svelte";
    import AlertDialog from "../components/dialogs/AlertDialog.svelte";
    import ShareIcon from "../components/icons/ShareIcon.svelte";
    import Topbar from "../components/Topbar.svelte";

    let isWasmLoaded = $state(false);

    let shareDialog: ReturnType<typeof AlertDialog>;

    onMount(async () => {
        await init();
        isWasmLoaded = true;
    });

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

            Lambda Calculus is a universal model of computation built entirely
            out of functions. It serves very often as a base of learning
            functional programming. Many functional programming languages are
            based on it, such as Haskell.

            <h2>Syntax</h2>

            <p>Expression in Lambda Calculus has only three possible forms:</p>

            <ul>
                <li>
                    <strong>Variable:</strong> <code>E = V</code> <br />
                    Name representing a value (which is function).
                </li>
                <li>
                    <strong>Abstraction:</strong> <code>E = (\V.E)</code><br />
                    Function definition. The <code>\</code> defines parameters, and
                    the part after the dot is the return value. This is like writting
                    arrow function in Javascript, for example.
                </li>
                <li>
                    <strong>Application:</strong> <code>E = (M N)</code> <br />
                    Calling function <code>M</code> with argument
                    <code>N</code>. This would be <code>M(N)</code> in Javascript
                    or other languages.
                </li>
            </ul>

            <p>
                <em>
                    Note: Traditional Lambda Calculus uses &lambda;, but in our
                    examples we use <code>\</code>, because it's easier to type
                    and correlates with our interpreter.
                </em>
            </p>

            <p>You can also name expressions using <code>let</code>:</p>

            <CodeSnippet code="let id = \x.x" />

            <h2>Conversions</h2>

            <p>
                Computation in Lambda Calculus is just rewriting expressions
                based on three conversions.
            </p>

            <h3>1. &alpha;-conversion</h3>

            <p>
                This is simply renaming variables. The important thing is that
                free variable can't become bound variable. Our interpreter
                handles this automatically.
            </p>

            <CodeSnippet
                code={String.raw`
\x.x y -> \z.z y // Valid: x renamed to z
\x.x y -> \y.y y // Invalid: y was already a free variable!
                `}
            />

            <h2>2. &beta;-conversion</h2>

            <p>
                This is the actual application step. When you apply a function
                to an argument, you replace every instance of the parameter with
                the argument, and then drop the parameter from function header.
                This doesn't have to apply all parameters at once!
            </p>

            <p>Try running the code below to see what happens!</p>

            <CodeSnippet
                runnable={true && isWasmLoaded}
                code={String.raw`
(\x y.x y) (m n); // Applies only the first argument
(\x y.x y) (m n) o; // Applies both arguments
                `}
            />

            <p>
                <em>
                    Note: Name conflicts have to be resolves with
                    &alpha;-conversion!
                </em>
            </p>

            <h2>&eta;-conversion</h2>

            <p>
                This is a cleanup conversion. If a function just takes an
                argument and immediately passes it to another function, the
                wrapper is useless.
            </p>

            <p>You can run the example below to test that out:</p>
            <CodeSnippet
                runnable={true && isWasmLoaded}
                code={String.raw`
let fn  = \x.(u v) x; // Wrapper function
let fn' = (u v); // Wrapper function after eta reduction
// Results in the same output
fn test;
fn' test;
                `}
            />
        </article>

        <article class="wiki-content">
            <h2>Boolean algebra</h2>

            <p>
                Because we don't have built-in data types, how do we represent
                <code>true</code> and <code>false</code>? We use functions that
                act like <code>if-else</code> statements!
            </p>

            <p>
                <em>
                    Note: There are many ways of representing types, we will
                    show one of them. You can try thinking of another way to
                    represent them!
                </em>
            </p>

            <p>
                A boolean in Lambda Calculus is simply a function that takes two
                arguments. <code>true</code> returns the first argument and
                <code>false</code> returns the second argument.
            </p>

            <CodeSnippet
                code={String.raw`
let true  = \t f.t
let false = \t f.f
                `}
            />

            <p>
                Using just those two definitions, we can now define logical
                operators:
            </p>

            <CodeSnippet
                code={String.raw`
// NOT: take a boolean, flip what it returns
let !  = \a.a false true
// AND: if 'a' is true, return 'b', otherwise return false
let && = \a b.a b false
// OR: if 'a' is true, return true, otherwise return 'b'
let || = \a b.a true b
                `}
            />
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

    .content article h3 {
        font-size: 1.2rem;
        margin-top: 1rem;
        color: var(--fg-max);
    }

    .content article p {
        margin-bottom: 1.25rem;
    }

    .content article ul {
        padding-left: 1.5rem;
        margin-bottom: 1rem;
    }

    .content article li {
        margin-bottom: 0.5rem;
        line-height: 1.6;
    }

    .content article code {
        background-color: var(--bg-light);
        padding: 0.2rem 0.4rem;
        border-radius: 4px;
        font-family: monospace;
        font-size: 0.9em;
        color: var(--primary);
    }
</style>
