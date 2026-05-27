<script lang="ts">
    import CodeSnippet from "../../components/CodeSnippet.svelte";

    let { isWasmLoaded, stdLibCode } = $props();
</script>

<article>
    <h2>Numbers</h2>

    <p>
        Since we don't have integers in Lambda Calculus, how do we represent
        numbers? We can use Alonzo Church's idea of representing numbers as
        <strong>the concept of repetition</strong>.
    </p>

    <p>
        Numbers are functions, which take an action and a starting value, and
        they apply the action to the starting number
        <code>N</code> times, depending on the number it represents. This can represent
        any unsigned integer.
    </p>

    <CodeSnippet
        code={String.raw`
let 0 = \f x.x // Apply action 0 times.
let 1 = \f x.f x        // ... 1 time.
let 2 = \f x.f (f x)    // ... 2 times.
// And so on...
                `}
    />

    <h3>Counting up!</h3>

    <p>
        As you probably noticed, it isn't very nice having to define each number
        manually. We can solve this by writing a function that takes a number
        and adds 1 to it. We call this function
        <strong>successor</strong> function.
    </p>

    <p>
        We can do this by applying the <code>x</code> argument as
        <code>f x</code>, which adds another repetition. Notice we also apply
        the <code>f</code> argument, since we have to apply from the first argument!
    </p>

    <CodeSnippet
        code={String.raw`
let succ = \a f x.a f (f x)
// Now we can define 0 and others using 'succ' function!
let 0 = \f x.x
let 1 = succ 0
// And so on...
                `}
    />

    <h3>Counting down!</h3>

    <p>
        Subtracting in Lambda Calculus is quite tricky, since the definition of
        the number is applying a function <code>N</code>
        times, you can't easily "un-apply" a function.
    </p>

    <p>
        We can achieve this by having a tuple holding two numbers:
        <code>(current, previous)</code>. We start at
        <code>(0, 0)</code> and on every step we shift the
        <code>current</code> into <code>previous</code> and add 1 to the
        <code>current</code>.
    </p>

    <ul>
        <li>Step 0: <code>(0, 0)</code></li>
        <li>Step 1: <code>(1, 0)</code></li>
        <li>Step 2: <code>(2, 1)</code></li>
        <li>Step 3: <code>(3, 2)</code></li>
    </ul>

    <p>
        If we want to find a predecessor of 3, we can just do the shifting 3
        times and grab the <code>previous</code> value! From the number
        definition, applying a function on a value
        <code>N</code> times can be done by calling the number function with both
        the function and the value arguments.
    </p>

    <p>
        <em>
            Note: As you probably notices, the predecessor of 0 is just 0.
        </em>
    </p>

    <CodeSnippet
        runnable={true && isWasmLoaded}
        hiddenCode={stdLibCode}
        code={String.raw`
let shift = \p., (succ (fst p)) (fst p)
// 'n shift (, 0 0)' - number function value
let pred = \n.snd (n shift (, 0 0))

// Note: 'cnum' is custom function allowing to print numbers
let 3 = succ (succ (succ 0));
cnum (pred 3);
                `}
    />

    <h3>Addition, Subtraction & Multiplication</h3>

    <p>
        Now that we can count up and down, arithmetics become easy. Remember
        that a number is a function that applies a function
        <code>N</code> times.
    </p>

    <p>
        If we want to add two numbers <code>M</code> and
        <code>N</code>, we just take the number <code>N</code> and apply the
        <code>succ</code> <code>M</code> times! The same logic works for
        subtraction using the <code>pred</code> function. The only catch with
        subtraction is that we don't have negative numbers, but because our
        <code>pred</code>
        function returns 0 for 0, we can use that behaviour (e.g.
        <code>4 - 5 = 0</code>).
    </p>

    <CodeSnippet
        runnable={true && isWasmLoaded}
        hiddenCode={stdLibCode}
        code={String.raw`
let + = \a b.a succ b
let - = \a b.b pred a

let 2 = succ (succ 0);
let 3 = succ 2;
cnum (+ 3 2);
cnum (- 3 2);
                `}
    />

    <p>
        Similar applies for multiplication, only instead of applying the
        <code>succ</code> or <code>pred</code>, we can apply the factor, since
        number is just a function!
    </p>

    <CodeSnippet
        runnable={true && isWasmLoaded}
        hiddenCode={stdLibCode}
        code={String.raw`
let * = \a b f.a (b f)

let 2 = succ (succ 0);
let 3 = succ 2;
cnum (* 3 2);
                `}
    />

    <h3>Predicates</h3>

    <p>
        To write real programs, we need to be able to check if number is zero,
        check if two numbers are equal, and compare them.
    </p>

    <p>
        Checking if number is zero can be done by passing the number an action,
        which ignores its argument and just returns
        <code>false</code>, and <code>true</code>. If the number is 0, the
        action never runs, and we get <code>true</code>. If the number is not 0,
        the action runs at least once, hence we get
        <code>false</code>.
    </p>

    <CodeSnippet
        runnable={true && isWasmLoaded}
        hiddenCode={stdLibCode}
        code={String.raw`
let isZero = \n.n (\_.false) true;

isZero 0;
isZero 2;
                `}
    />

    <p>
        To check equality and compare the numbers, we can use the subtraction we
        already implemented in combination with the
        <code>isZero</code> check!
    </p>

    <CodeSnippet
        runnable={true && isWasmLoaded}
        hiddenCode={stdLibCode}
        code={String.raw`
// We don't have negative numbers, need to check both ways!
let eq = \a b.&& (isZero (- a b)) (isZero (- b a))
let gte = \a b.isZero (- b a);

eq 2 2;
eq 0 2;
gte 2 0;
gte 0 2;
                `}
    />

    <p>Try to implement a strict greater by yourself!</p>

    <p>
        You might be wondering, why didn't we do division as well? The reason is
        it's a bit complicated. We need to learn recursion first!
    </p>
</article>
