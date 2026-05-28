<script lang="ts">
    import CodeSnippet from "../../components/CodeSnippet.svelte";

    let { isWasmLoaded, stdLibCode } = $props();
</script>

<article class="wiki-content">
    <h2>Boolean algebra</h2>

    <p>
        A boolean in Lambda Calculus is simply a function that takes two
        arguments. <code>true</code> returns the first argument and
        <code>false</code> returns the second argument. They act just like
        <code>if-else</code> statements!
    </p>

    <CodeSnippet
        code={String.raw`
let true  = \t f.t
let false = \t f.f
                `}
    />

    <p>
        Using just those two definitions, we can now define logical operators:
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

    <p>Try implementing xor and implication by yourself!</p>

    <CodeSnippet
        id="bool-challenge"
        editable={true}
        runnable={true && isWasmLoaded}
        hiddenCode={stdLibCode}
        code={String.raw`
let ^ = \a b.TODO  // Fill xor implementation
let -> = \a b.TODO // Fill implication implementation
        `}
        tests={[
            { code: "^ true true", output: "\\t.\\f.f" },
            { code: "^ true false", output: "\\t.\\f.t" },
            { code: "^ false true", output: "\\t.\\f.t" },
            { code: "^ false false", output: "\\t.\\f.f" },
            { code: "-> true true", output: "\\t.\\f.t" },
            { code: "-> true false", output: "\\t.\\f.f" },
            { code: "-> false true", output: "\\t.\\f.t" },
            { code: "-> false false", output: "\\t.\\f.t" },
        ]}
    />
</article>
