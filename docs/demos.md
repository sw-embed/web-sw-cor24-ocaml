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
