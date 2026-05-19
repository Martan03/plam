// Implementation of the program cat which copies its stdin to the output.

// Conversion of number to a character
let chr = \n.$char (n $increment $counter)

// Basic lambda operators
let Y = \f.f ((\x.f (x x)) \x.f (x x))

// Basic arithmetic operators
let + = \a b f x.a f (b f x)
let ** = \a b.b a

// Construct the required number 20
let 2 = \f x.f (f x)
let 4 = ** 2 2
let 20 = + (** 4 2) 4

// Define basic operations on lists
let foldr = Y \" f v l.l \c h t.c (f h (" f v t)) v

;

// Convert the stdin to bytes and print them concatenated with newline.
foldr (\a b.(chr a) b) (chr 20) $stdin