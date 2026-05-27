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

            <h3>2. &beta;-conversion</h3>

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

            <h3>3. &eta;-conversion</h3>

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

            <p>
                In the following sections, we'll talk about how to represent
                data types, such as booleans and numbers. Note that there are
                many ways you can achieve this, but we will show just one for
                each type. If you want a bit of a challenge, you can try to
                represent them in another way!
            </p>
        </article>

        <article class="wiki-content">
            <h2>Boolean algebra</h2>

            <p>
                A boolean in Lambda Calculus is simply a function that takes two
                arguments. <code>true</code> returns the first argument and
                <code>false</code> returns the second argument. They act just
                like <code>if-else</code> statements!
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

            <p>Try implementing xor and implication by yourself!</p>
        </article>

        <article>
            <h2>Tuples</h2>

            <p>
                Tuples are very useful in Lambda Calculus. Tuple is a function
                which takes two values and a function and calls the function
                with the values as arguments.
            </p>

            <p>
                We can also implement getter functions for getting the first and
                the second value. These are really easy to implement. To get the
                first value, we provide <code>true</code> as the function
                argument, and to get the second value, we provide
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
                visualize step-by-step what is happening under the hood. We'll
                start with the expression of <code>fst</code> and provide a tuple
                as an argument.
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
                And same would apply for the <code>snd</code>! As we already
                know, <code>true</code> returns the first argument, in this case the
                first value from the tuple.
            </p>
        </article>

        <article>
            <h2>Numbers</h2>

            <p>
                Since we don't have integers in Lambda Calculus, how do we
                represent numbers? We can use Alonzo Church's idea of
                representing numbers as
                <strong>the concept of repetition</strong>.
            </p>

            <p>
                Numbers are functions, which take an action and a starting
                value, and they apply the action to the starting number
                <code>N</code> times, depending on the number it represents. This
                can represent any unsigned integer.
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
                As you probably noticed, it isn't very nice having to define
                each number manually. We can solve this by writing a function
                that takes a number and adds 1 to it. We call this function
                <strong>successor</strong> function.
            </p>

            <p>
                We can do this by applying the <code>x</code> argument as
                <code>f x</code>, which adds another repetition. Notice we also
                apply the <code>f</code> argument, since we have to apply from the
                first argument!
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
                Subtracting in Lambda Calculus is quite tricky, since the
                definition of the number is applying a function <code>N</code>
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
                If we want to find a predecessor of 3, we can just do the
                shifting 3 times and grab the <code>previous</code> value! From
                the number definition, applying a function on a value
                <code>N</code> times can be done by calling the number function with
                both the function and the value arguments.
            </p>

            <p>
                <em>
                    Note: As you probably notices, the predecessor of 0 is just
                    0.
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
                Now that we can count up and down, arithmetics become easy.
                Remember that a number is a function that applies a function
                <code>N</code> times.
            </p>

            <p>
                If we want to add two numbers <code>M</code> and <code>N</code>,
                we just take the number <code>N</code> and apply the
                <code>succ</code> <code>M</code> times! The same logic works for
                subtraction using the <code>pred</code> function. The only catch
                with subtraction is that we don't have negative numbers, but
                because our <code>pred</code> function returns 0 for 0, we can
                use that behaviour (e.g. <code>4 - 5 = 0</code>).
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
                <code>succ</code> or <code>pred</code>, we can apply the factor,
                since number is just a function!
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
                To write real programs, we need to be able to check if number is
                zero, check if two numbers are equal, and compare them.
            </p>

            <p>
                Checking if number is zero can be done by passing the number an
                action, which ignores its argument and just returns
                <code>false</code>, and <code>true</code>. If the number is 0,
                the action never runs, and we get <code>true</code>. If the
                number is not 0, the action runs at least once, hence we get
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
                To check equality and compare the numbers, we can use the
                subtraction we already implemented in combination with the
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
                You might be wondering, why didn't we do division as well? The
                reason is it's a bit complicated. We need to learn recursion
                first!
            </p>
        </article>

        <article>
            <h2>Recursion</h2>

            <p>
                In pure Lambda Calculus, functions mathematically have no names.
                But what about <code>let</code> keyword? It's just a helpful keyword
                for our interpreter. It evaluates the right side of the equals sign
                and saves it to a dictionary. You can probably spot the issue now.
                The name is not yet defined in the inner body of the function!
            </p>

            <p>
                We can solve this by passing the function to itself (self). This
                works, but as you can see, it can get annoying.
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
                The solution is to use <strong>Fixed-Point Combinator</strong>,
                also known as <strong>Y-Combinator</strong>. It's a wrapper
                function to which you hand a function (such as
                <code>fact</code>) and it automatically makes it that the
                function is endlessly fed a copy of itself as its first
                argument.
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

            <p>
                We can now use the Y-combinator to reimplement our factorial
                example.
            </p>
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
                    Note: This works because the expression is evaluated lazily,
                    meaning only expressions that are needed are evaluated,
                    otherwise it would cause Stack Overflow error.
                </em>
            </p>

            <h3>Division</h3>

            <p>
                Now that we now how to make recursion, we can finally add the
                number division! To divide two numbers, we can subtract the
                divisor from the dividend until the dividend is less than the
                divisor, and we just count how many times we subtracted. Because
                we don't have floating point numbers, the result will be an
                integer (result is floored).
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
                    Note: Division by 0 should throw exception, but how can we
                    do that? In Lambda Calculus, it's common practice that
                    exceptions are modeled as infinite recursion, which our code
                    actual does!
                </em>
            </p>
        </article>

        <article>
            <h2>Conclusion</h2>

            <p>
                Now that we went through all the topics, you should be capable
                of coding in Lambda Calculus! It isn't actually made to be used
                as a full featured programming language, but it's a fun way to
                learn functional programming. If you enjoy coding in the Lambda
                Calculus, I highly recommend trying functional programming
                languages, such as Haskell. Haskell was created based on the
                Lambda Calculus and extended with features that coders need to
                make actual programs, so you can do anything you can think of!
            </p>

            <p>
                <em>
                    Note: We already know about tuples. You can use them to
                    create Lists (or Linked Lists)! Try creating them! Little
                    hint for you if you're not sure how: first value in tuple
                    can hold the item value and the second value can point to
                    another list item.
                </em>
            </p>
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
