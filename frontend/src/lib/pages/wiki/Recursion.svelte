<script lang="ts">
    import CodeSnippet from "../../components/CodeSnippet.svelte";

    let { isWasmLoaded, stdLibCode } = $props();
</script>

<article>
    <h2>Recursion</h2>

    <p>
        In pure Lambda Calculus, functions mathematically have no names. But
        what about <code>let</code> keyword? It's just a helpful keyword for our interpreter.
        It evaluates the right side of the equals sign and saves it to a dictionary.
        You can probably spot the issue now. The name is not yet defined in the inner
        body of the function!
    </p>

    <p>
        We can solve this by passing the function to itself (self). This works,
        but as you can see, it can get annoying.
    </p>

    <CodeSnippet
        runnable={true && isWasmLoaded}
        hiddenCode={stdLibCode}
        code={String.raw`
let fact = \self n.isZero n 1 (* n (self self (pred n)));
cnum (fact fact 5);
    `}
    />

    <h3>Y-Combinator</h3>

    <p>
        The solution is to use <strong>Fixed-Point Combinator</strong>, also
        known as <strong>Y-Combinator</strong>. It's a wrapper function to which
        you hand a function (such as
        <code>fact</code>) and it automatically makes it that the function is
        endlessly fed a copy of itself as its first argument.
    </p>

    <CodeSnippet code="let Y = \f.(\x.f (x x)) (\x.f (x x))" />

    <p>We can trace the execution using Y-Combinator:</p>
    <CodeSnippet
        code={String.raw`
Y myFunc
// 1. Expand 'Y'
(\f.(\x.f (x x)) (\x.f (x x))) myFunc
// 2. Beta reduction of 'myFunc'
(\x.myFunc (x x)) (\x.myFunc (x x))
// 3. Beta reduction of '(\x.myFunc (x x))'
myFunc ((\x.myFunc (x x)) (\x.myFunc (x x)))
// 4. Undo beta reduction of the 'myFunc' (equivalence)
myFunc ((\f.(\x.f (x x)) (\x.f (x x))) myFunc)
// 5. Replace the expression of Y-combinator with its name
myFunc (Y myFunc)
    `}
    />

    <p>We can now use the Y-combinator to reimplement our factorial example.</p>
    <CodeSnippet
        runnable={true && isWasmLoaded}
        hiddenCode={stdLibCode}
        code={String.raw`
let fact = Y (\self n.isZero n 1 (* n (self (pred n))));
cnum (fact 5);
    `}
    />

    <p>
        <em>
            Note: This works because the expression is evaluated lazily, meaning
            only expressions that are needed are evaluated, otherwise it would
            cause Stack Overflow error.
        </em>
    </p>

    <h3>Division</h3>

    <p>
        Now that we now how to make recursion, we can finally add the number
        division! To divide two numbers, we can subtract the divisor from the
        dividend until the dividend is less than the divisor, and we just count
        how many times we subtracted. Because we don't have floating point
        numbers, the result will be an integer (result is floored).
    </p>
    <CodeSnippet
        runnable={true && isWasmLoaded}
        hiddenCode={stdLibCode}
        code={String.raw`
let / = (Y \f r a b.(gt b a) r (f (succ r) (- a b) b)) 0;
cnum (/ 5 2);
    `}
    />

    <p>
        <em>
            Note: Division by 0 should throw exception, but how can we do that?
            In Lambda Calculus, it's common practice that exceptions are modeled
            as infinite recursion, which our code actual does!
        </em>
    </p>
</article>
