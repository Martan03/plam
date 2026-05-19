# plam

Pure lambda calculus interpreter.

Simplifies each expression in the file to form where it is either a lambda
function or unbound variable.

## Syntax

### Basic lambda calcul

Create lambda function that takes argument `x` and has body `body`:
```
\x.body
```

Call function `f` with value of variable `x`:
```
f x
```

Change precedence with `(` and `)`:
```
foo (bar x)
```

### Extensions

Multiple lambda functions in succession can be simplified:
```
\x.\y.body
```
is equivalent to
```
\x y.body
```

Bind `body` to the name `name`:
```
let name = body
```

Separate two statements `a` and `b`:
```
a ; b
```

Use line comments with `//`:
```
// This line is ignored
```

Use block comments with `/*`:
```
/* This block
   is ignored */
```

### Builtin functions

These functions are very opaque and serve only to get more human readable
information from a lambda expression.

- `$counter` represents a opque counter with value of `0`. When printed, it is
  shows its value. Can be manipulated only by `$increment`. For any other use
  cases behaves as unbound variable.
- `$increment` represents opaque function. If given any counter as argument it
  will expand to incremented counter. For any other use cases behaves as
  unbound variable.
- `$char` creates opaque single char string which can be printed. Works only
  with `$counter` as argument. The counter value is the byte value of the char.
  Applying opaque string to opaque string will concatenate them. (e.g.
  `$char $counter ($char $counter)` will create string with two NUL bytes)
- `$stdin` stdin is list of bytes encoded as peano arithmetic. Precisely:
    - the boolean `true` is defined as `\t f.t`,
    - the boolean `false` is defined as `\t f.f`,
    - the number `0` is defined as `\f x.x`,
    - the successor of number **n** is defined as `\f x.n f (f x)`,
    - the `Y` combinator is defined as `\f.f ((\x.f (x x)) \x.f (x x))`,
    - bottom (`_`) is defined as `Y \" x."`,
    - the empty list is defined as `\f.f false _ _`,
    - and non empty list with head `h` and tail `t` is defined as
      `\f.f true h t`.

## Example

Code:
```
let true = \a b.a
let false = \a b.b
let if = \c t f.c t f
let or = \a b.a true b
let and = \a b.a b false
let not = \a.a false true
let xor = \a b.a (not b) b

let 0 = \f x.x
let succ = \a f x. a f (f x)

;

if true T F;
if false T F;

if (or true false) T F;
if (and true false) T F;
if (xor true true) T F;

(succ (succ 0));
(succ (succ 0)) (\x.$increment x) $counter
```

Output:
```
T
F
T
F
F
(\f.(\x.(((succ 0) f) (f x))))
:2:
```
