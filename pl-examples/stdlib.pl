// Conversions to printable values.
let cnum = \n.n $increment $counter
let cbool = \b.b True False

// Basic boolean algebra.
let true = \a b.a
let false = \a b.b
let || = \a b.a true b
let && = \a b.a b false
let ! = \a.a false true
let ^ = \a b.a (! b) b

// Tuple
let , = \a b f.f a b
let :0 = \a b.a
let :1 = \a b.b

// Tuple of 3
let ,3 = \a b c f.f a b c

// The Y combinator and bottom
let Y = \f.f ((\x.f (x x)) \x.f (x x))
let _ = Y \" y."

// Definition of basic numbers.
let ++ = \a f x.a f (f x)
let +0 = \f x.x
let +1 = ++ +0
let +2 = ++ +1
let +3 = ++ +2
let +4 = ++ +3
let +5 = ++ +4
let +6 = ++ +5
let +7 = ++ +6
let +8 = ++ +7
let +9 = ++ +8
let +10 = ++ +9

// Definition of arithmetic operators and predicates.
let + = \a b f x. a f (b f x)
let * = \a b f. a (b f)
let -- = \a f x.a (\p.p \c x.c (, true (f x)) (, true x)) (, false x) :1
let - = \a b.b -- a
let 0? = \a.a (\_.false) true
let / = (Y \" c a b. 0? a c (" (++ c) (- a b) b)) +0
let ** = \a b.b a
let eq = \a b.&& (0? (- a b)) (0? (- b a))
let gte = \a b.0? (- b a)
let gt = \a b.! (gte b a)
let sqrt = (Y \" c a.gte (* c c) a c (" (++ c) a)) +0

// Implementation of more convinient way of writing any number.
let > = , false _
let 0 = , true +0
let 1 = , true +1
let 2 = , true +2
let 3 = , true +3
let 4 = , true +4
let 5 = , true +5
let 6 = , true +6
let 7 = , true +7
let 8 = , true +8
let 9 = , true +9
let < = (Y \" n d.d \c d.c (" (+ d (* n +10))) n) +0

// Implementation of list.
let [] = ,3 false _ _
let head = \l.l \_ h _.h
let tail = \l.l \_ _ t.t
let : = \i l.,3 true i l
let [:] = \i.: i []
let :: = Y \" a b.a \c h t.c (: h (" t b)) b
let ' = true
let ] = false
let [ = (Y \" l c.c (\i." (:: l ([:] i))) l) [] true
let !! = Y \" i l.l \c h t.c (0? i h (" (-- i) t)) _

// Basic helper functions
let flip = \f a b.f b a
let id = \x.x
let nums = (Y \" n.: n (" (++ n))) +0

// Operations on lists
let foldl = Y \" f v l.l \c h t.c (" f (f v h) t) v
let foldr = Y \" f v l.l \c h t.c (f h (" f v t)) v
let sum = foldr + +0
let take = Y \" n l.0? n [] (l \c h t.c (: h (" (-- n) t)) [])
let map = \f.foldr (\x.: (f x)) []

;

cnum (sum (take +5 (map (* +2) nums)))
