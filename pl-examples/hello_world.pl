let 0 = \f x.x
let succ = \a f x.a f (f x)
let add = \a b f x.a f (b f x)
let mul = \a b f.a (b f)
let sq = \a.mul a a
let 1 = succ 0
let 2 = succ 1
let 3 = add 1 2
let 4 = sq 2
let 5 = add 2 3
let 6 = mul 2 3
let 9 = sq 3
let 25 = sq 5
let 27 = mul 9 3
let 29 = add 27 2
let 32 = add 27 5
let 33 = succ 32
let 36 = sq 6
let 72 = mul 36 2
let 87 = mul 3 29
let 100 = mul 4 25
let 101 = succ 100
let 108 = mul 27 4
let 111 = add 108 3
let 114 = add 111 3

let chr = \n.$char (n $increment $counter)

let 'sp' = chr 32
let '!' = chr 33
let 'H' = chr 72
let 'W' = chr 87
let 'd' = chr 100
let 'e' = chr 101
let 'l' = chr 108
let 'o' = chr 111
let 'r' = chr 114

;

'H' 'e' 'l' 'l' 'o' 'sp' 'W' 'o' 'r' 'l' 'd' '!'