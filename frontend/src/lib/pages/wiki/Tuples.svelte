<script lang="ts">
    import CodeSnippet from "../../components/CodeSnippet.svelte";
</script>

<article>
    <h2>Tuples</h2>

    <p>
        Tuples are very useful in Lambda Calculus. Tuple is a function which
        takes two values and a function and calls the function with the values
        as arguments.
    </p>

    <p>
        We can also implement getter functions for getting the first and the
        second value. These are really easy to implement. To get the first
        value, we provide <code>true</code> as the function argument, and to get
        the second value, we provide
        <code>false</code>.
    </p>

    <CodeSnippet
        code={String.raw`
let , = \a b f.f a b
let fst = \p.p true
let snd = \p.p false
                `}
    />

    <p>
        If you're not sure what's happening, it might be a good idea to
        visualize step-by-step what is happening under the hood. We'll start
        with the expression of <code>fst</code> and provide a tuple as an argument.
    </p>

    <CodeSnippet
        code={String.raw`
// 1. We start with calling 'fst' on our tuple
fst (, x y)
// 2. Expand 'fst'
(\p.p true) (, x y)
// 3. Apply the '(, x y)' (beta reduction)
(, x y) true
// 4. Expand the tuple operator ','
(\a b f.f a b) x y true
// 5. Apply 'x', 'y' and 'true' to the tuple (beta reduction)
true x y
                `}
    />

    <p>
        And same would apply for the <code>snd</code>! As we already know,
        <code>true</code> returns the first argument, in this case the first value
        from the tuple.
    </p>
</article>
