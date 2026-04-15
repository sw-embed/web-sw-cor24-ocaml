# Demos

One section per entry in `src/demos.rs`. Source is reproduced verbatim
from `examples/`. Expected output is what the OCaml interpreter prints
to UART (after stripping the pvm boot banner via `Session::clean_output`),
matching what you see in the live UI's output panel.

OCaml integer-subset semantics: every top-level expression's value is
echoed in the REPL, and `print_int` writes a decimal representation
without a trailing newline.

---

## hello

The default demo. Shortest possible program.

```ocaml
print_int 42
```

**Expected output:**

```
42
```

---

## factorial

Recursive `let rec` with an `if`/`else` base case. Computes 5! = 120.

```ocaml
let rec fact = fun n -> if n = 0 then 1 else n * fact (n - 1) in print_int (fact 5)
```

**Expected output:**

```
120
```

---

## functions

A first-class function bound with `let`, then applied. Result `42` is
echoed because it's the value of the top-level expression.

```ocaml
let f = fun x -> x + 1 in f 41
```

**Expected output:**

```
42
```

---

## multi-arg

Curried two-argument function via `fun x y -> ...`, then applied with
juxtaposition.

```ocaml
let add = fun x y -> x + y in print_int (add 20 22)
```

**Expected output:**

```
42
```

---

## pairs

Tuples and the `fst` / `snd` accessors. Each top-level expression
prints its value.

```ocaml
(1, 2)
fst (1, 2)
snd (10, 20)
let p = (10, 20) in fst p + snd p
(1, [2; 3])
(fst (1, 2), snd (3, 4))
```

**Expected output:**

```
(1, 2)
1
20
30
(1, [2; 3])
(1, 4)
```

---

## lists

List literals (`[]`, `[1]`, `[1; 2; 3]`), cons (`::`), and the
built-in primitives `is_empty`, `hd`, `tl`.

```ocaml
[]
[1]
[1; 2; 3]
1 :: nil
is_empty nil
is_empty [1]
hd [42]
tl [1; 2; 3]
```

**Expected output:**

```
[]
[1]
[1; 2; 3]
[1]
true
false
42
[2; 3]
```

---

## list-module

Qualified-name lookups into the `List` module. Demonstrates that
dotted identifiers (`List.length`) resolve at parse time.

```ocaml
List.length [1;2;3]
List.length []
List.rev [1;2;3]
List.rev []
List.hd [42; 99]
List.tl [1; 2; 3]
List.is_empty nil
List.is_empty [1]
```

**Expected output:**

```
3
0
[3; 2; 1]
[]
42
[2; 3]
true
false
```

---

## lists-pairs-demo

A larger program mixing recursion, lists, pairs, and qualified names.

```ocaml
let rec sum = fun l -> if is_empty l then 0 else hd l + sum (tl l) in sum [1;2;3;4;5]
let rec length = fun l -> if is_empty l then 0 else 1 + length (tl l) in length [10;20;30]
let rec map = fun f l -> if is_empty l then [] else (f (hd l)) :: (map f (tl l)) in map (fun x -> x * 2) [1;2;3]
let p = (3, 4) in fst p * fst p + snd p * snd p
List.length [1;2;3;4;5]
List.rev [1;2;3;4;5]
let swap = fun p -> (snd p, fst p) in swap (1, 2)
let rec countdown = fun n -> if n = 0 then [] else n :: countdown (n - 1) in countdown 5
```

**Expected output:**

```
15
3
[2; 4; 6]
25
5
[5; 4; 3; 2; 1]
(2, 1)
[5; 4; 3; 2; 1]
```

---

## sequencing

Semicolon `;` sequences expressions left-to-right; only side effects
are observable.

```ocaml
print_int 1; print_int 2; print_int 3
```

**Expected output:**

```
123
```

---

## print

Bind a value with `let` and print it.

```ocaml
let x = 41 + 1 in print_int x
```

**Expected output:**

```
42
```

---

## led-blink

Drive the COR24 board LED via the `led_on` / `led_off` primitives.
On the real board the LED toggles; in the browser the runner stubs
the LED I/O (`SYS 3` is a no-op), so you only see the `print_int`
side effects, but the program runs through cleanly.

(`scripts/sync-demos.sh` collapses this demo's newlines into one
REPL line so the semicolon-sequenced expression is parsed as a
single input -- the OCaml REPL treats each line as an independent
top-level expression.)

```ocaml
led_on (); print_int 1; led_off (); print_int 0; led_on (); print_int 1
```

**Expected output:** (one number per `print_int`, on its own line)

```
1
0
1
```

---

## led-toggle

Tail-recursive loop that reads the COR24 switch and reflects it on
the LED. In the browser `switch ()` always returns `false` (`SYS 6`
returns 0). The OCaml interpreter is *not* tail-call-optimized, so
each `loop ()` call grows the interp's call stack until it
exhausts -- the runtime catches that and prints `EVAL ERROR` instead
of trapping the VM. On the real COR24 board with a working switch
this would still loop; on the web emulator it terminates cleanly
after a few thousand iterations.

(`scripts/sync-demos.sh` collapses this demo's newlines for the
same REPL-line-parsing reason as `led-blink`.)

```ocaml
let rec loop = fun u -> let s = switch () in set_led s; loop () in loop ()
```

**Expected output:**

```
EVAL ERROR
```

---

## function-form-let

The sugared `let f x y = body` shorthand for curried function
definitions, equivalent to `let f = fun x -> fun y -> body`.
Each expression demonstrates a different use of the form
(application, recursion, higher-order composition, options).

```ocaml
let f x = x + 1 in f 5
let square x = x * x in square 7
let add x y = x + y in add 20 22
let rec fact n = if n = 0 then 1 else n * fact (n - 1) in fact 5
let rec fib n = if n < 2 then n else fib (n-1) + fib (n-2) in fib 7
let compose f g x = f (g x) in compose (fun x -> x + 1) (fun x -> x * 2) 10
let safe_div x y = if y = 0 then None else Some (x / y) in safe_div 42 6
```

**Expected output:**

```
6
49
42
120
13
21
Some 7
```

---

## strings

String literals, `^` concatenation, `print_endline` (which writes
the string and a newline), and `String.length`.

```ocaml
"Hello"
"OCaml" ^ " rocks"
print_endline "Hello, World!"
String.length "abcde"
String.length ""
"x" ^ "y" ^ "z"
let greeting = "Hello, " ^ "World!" in print_endline greeting
```

**Expected output:** (string values display with quotes;
`print_endline` writes raw bytes)

```
"Hello"
"OCaml rocks"
Hello, World!
5
0
"xyz"
Hello, World!
```

---

## named-adts

Define a sum type with `type T = C1 | C2 | ...`, then construct
values and pattern-match over them. The last expression matches
across two different ADTs to show that constructor names share one
namespace.

```ocaml
type color = Red | Green | Blue
Red
Green
Blue
match Red with Red -> 1 | Green -> 2 | Blue -> 3
match Green with Red -> 1 | Green -> 2 | Blue -> 3
match Blue with Red -> 1 | Green -> 2 | Blue -> 3
let name = function Red -> "red" | Green -> "green" | Blue -> "blue" in name Blue
type shape = Circle | Square | Triangle
match Circle with Red -> 1 | Green -> 2 | Blue -> 3 | Circle -> 10 | Square -> 11 | Triangle -> 12
```

**Expected output:**

```
Red
Green
Blue
1
2
3
"blue"
10
```

---

## options

The built-in `option` type with `None` and `Some x` constructors.
Demonstrates options carrying ints, lists, tuples, and other
options (nested `Some (Some n)`).

```ocaml
None
Some 42
Some [1; 2; 3]
Some (1, 2)
let x = Some 7 in x
Some (Some 3)
```

**Expected output:**

```
None
Some 42
Some [1; 2; 3]
Some (1, 2)
Some 7
Some Some 3
```

---

## patterns

Pattern matching across lists (`[]` / `h :: t`), tuples,
the `option` type, and literal integers (with `_` wildcard).

```ocaml
let rec sum = fun l -> match l with [] -> 0 | h :: t -> h + sum t in sum [1;2;3;4;5]
let rec length = fun l -> match l with [] -> 0 | _ :: t -> 1 + length t in length [10;20;30]
let rec map = fun f l -> match l with [] -> [] | h :: t -> f h :: map f t in map (fun x -> x * 2) [1;2;3]
let rec filter = fun f l -> match l with [] -> [] | h :: t -> if f h then h :: filter f t else filter f t in filter (fun x -> x mod 2 = 0) [1;2;3;4;5;6]
let safe_div = fun x y -> if y = 0 then None else Some (x / y) in safe_div 10 3
let safe_div = fun x y -> if y = 0 then None else Some (x / y) in safe_div 10 0
match Some 7 with None -> 0 | Some n -> n + 1
let classify = fun n -> match n with 0 -> 100 | 1 -> 101 | _ -> 999 in classify 0
let classify = fun n -> match n with 0 -> 100 | 1 -> 101 | _ -> 999 in classify 5
let swap = fun p -> match p with (a, b) -> (b, a) in swap (1, 2)
```

**Expected output:**

```
15
3
[2; 4; 6]
[2; 4; 6]
Some 3
None
8
100
999
(2, 1)
```

---

## let-destructure

Destructuring patterns on the LHS of `let`: tuples, list cons,
list literals, and `option` constructors. Useful for binding
multiple names at once when you know the shape of the value.

```ocaml
let (a, b) = (1, 2) in a + b
let (x, y) = (10, 20) in let (p, q) = (3, 4) in x + y + p + q
let h :: t = [1; 2; 3] in h
let [a; b; c] = [10; 20; 30] in a + b + c
let Some n = Some 99 in n + 1
let (x, [a; b]) = (1, [2; 3]) in x + a + b
```

**Expected output:**

```
3
37
1
60
100
6
```

---

## repl-session

Marked `interactive: true` in the catalog. The seed source contains
several REPL inputs separated by newlines; once they're consumed the
runner pauses with `awaiting input` and the input row appears under
the output panel. Type a fresh OCaml expression and press Enter (or
**Send**) to feed it into the running interpreter.

Seed source:

```ocaml
42
let x = 1 + 1 in x
let f = fun x -> x * 2 in f 21
let rec fact = fun n -> if n = 0 then 1 else n * fact (n - 1) in fact 5
print_int 99
```

**Expected output (after the seed runs):**

```
42
2
42
120
99
```

The session stays open; further inputs print their values inline.
