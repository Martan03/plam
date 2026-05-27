<script lang="ts">
    import CodeSnippet from "../../components/CodeSnippet.svelte";

    let { isWasmLoaded } = $props();
</script>

<article>
    <h1>Lambda Calculus</h1>

    Lambda Calculus is a universal model of computation built entirely out of
    functions. It serves very often as a base of learning functional
    programming. Many functional programming languages are based on it, such as
    Haskell.

    <h2>Syntax</h2>

    <p>Expression in Lambda Calculus has only three possible forms:</p>

    <ul>
        <li>
            <strong>Variable:</strong> <code>E = V</code> <br />
            Name representing a value (which is function).
        </li>
        <li>
            <strong>Abstraction:</strong> <code>E = (\V.E)</code><br />
            Function definition. The <code>\</code> defines parameters, and the part
            after the dot is the return value. This is like writting arrow function
            in Javascript, for example.
        </li>
        <li>
            <strong>Application:</strong> <code>E = (M N)</code>
            <br />
            Calling function <code>M</code> with argument
            <code>N</code>. This would be <code>M(N)</code> in Javascript or other
            languages.
        </li>
    </ul>

    <p>
        <em>
            Note: Traditional Lambda Calculus uses &lambda;, but in our examples
            we use <code>\</code>, because it's easier to type and correlates
            with our interpreter.
        </em>
    </p>

    <p>You can also name expressions using <code>let</code>:</p>

    <CodeSnippet code="let id = \x.x" />

    <h2>Conversions</h2>

    <p>
        Computation in Lambda Calculus is just rewriting expressions based on
        three conversions.
    </p>

    <h3>1. &alpha;-conversion</h3>

    <p>
        This is simply renaming variables. The important thing is that free
        variable can't become bound variable. Our interpreter handles this
        automatically.
    </p>

    <CodeSnippet
        code={String.raw`
\x.x y -> \z.z y // Valid: x renamed to z
\x.x y -> \y.y y // Invalid: y was already a free variable!
    `}
    />

    <h3>2. &beta;-conversion</h3>

    <p>
        This is the actual application step. When you apply a function to an
        argument, you replace every instance of the parameter with the argument,
        and then drop the parameter from function header. This doesn't have to
        apply all parameters at once!
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
            Note: Name conflicts have to be resolves with &alpha;-conversion!
        </em>
    </p>

    <h3>3. &eta;-conversion</h3>

    <p>
        This is a cleanup conversion. If a function just takes an argument and
        immediately passes it to another function, the wrapper is useless.
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

    <p>
        In the following sections, we'll talk about how to represent data types,
        such as booleans and numbers. Note that there are many ways you can
        achieve this, but we will show just one for each type. If you want a bit
        of a challenge, you can try to represent them in another way!
    </p>
</article>
