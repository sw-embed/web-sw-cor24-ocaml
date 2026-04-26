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

## modules

User-defined module namespaces via the reserved `let __module = "..."`
directive. Defines `Math.add` / `Math.double` inside module `Math`,
switches to module `Main`, then dispatches by qualified name. The
trailing line is intentional: from `Main`, an unqualified `add 1 2`
fails because `add` lives in `Math` -- the `EVAL ERROR` is the
namespace-isolation punchline. There is no language-level try/catch;
the REPL resets `eval_error` per line and continues, which is how
the corrective `Math.add 1 2` and the failing `add 1 2` can both
run.

The web variant diverges from the CLI source
(`tests/eval_module_namespace_directive.ml`): inline `(* ... *)`
comments and a corrective qualified call were added so the trailing
error reads as the educational climax rather than a bug.
`scripts/sync-demos.sh` lists `modules` under `LOCAL_OVERRIDE` so
re-syncing won't clobber it.

```ocaml
let __module = "Math"        (* enter module Math *)
let add x y = x + y          (* defines Math.add *)
let double x = add x x       (* Math.double; unqualified add resolves within Math *)
let __module = "Main"        (* switch to Main; Math's bindings now require qualification *)
Math.add 2 3                 (* qualified dispatch into Math --> 5 *)
Math.double 9                (* qualified --> 18 *)
Math.add 1 2                 (* the right way: qualify into Math --> 3 *)
add 1 2                      (* the wrong way: unqualified add lives in Math, not Main -- EVAL ERROR expected *)
```

**Expected output:**

```
5
18
3
EVAL ERROR
```

---

## modules-multifile

Phase 1 of the multi-file demos plan
(`docs/multiple-file-demos-plan.md`). Two `.ml` files vendored under
`examples/modules-multifile/`: `math.ml` defines `Math.add` /
`Math.square` / `Math.double`; `main.ml` calls them by qualified name.
The runner concatenates the files with synthesized
`let __module = "..."` directives, exactly mirroring what the CLI's
`scripts/run-ocaml.sh` does when invoked with multiple inputs. The
editor pane shows only `main.ml`; the aux file is baked in read-only
in this phase.

The capitalisation rule for module names: strip `.ml`, uppercase the
first character, leave the rest (so `math.ml` -> `Math`,
`game_state.ml` -> `Game_state`).

`main.ml`:

```ocaml
Math.add 2 3
Math.square 5
Math.double 7
```

`math.ml`:

```ocaml
let add x y = x + y
let square x = x * x
let double x = add x x
```

**Concatenated input fed to the REPL** (what `Demo::full_source`
produces, what tests assert against):

```ocaml
let __module = "Math"
let add x y = x + y
let square x = x * x
let double x = add x x
let __module = "Main"
Math.add 2 3
Math.square 5
Math.double 7
```

**Expected output:**

```
5
25
14
```

---

## higher-order-lists

The classic functional-programming trio over the built-in `List`
module: `map`, `filter`, `fold_left`, plus `iter` for side-effecting
traversal. Lambdas are passed inline; the last line reverses a list
via `fold_left` with `::`.

```ocaml
List.map (fun x -> x * 2) [1;2;3]
List.map (fun x -> x + 1) []
List.filter (fun x -> x mod 2 = 0) [1;2;3;4;5;6]
List.filter (fun x -> x > 10) [1;5;15;20;3]
List.fold_left (fun acc x -> acc + x) 0 [1;2;3;4]
List.fold_left (fun acc x -> acc * x) 1 [1;2;3;4;5]
List.iter (fun x -> print_int x) [10;20;30]
let inc x = x + 1 in List.map inc [1;2;3]
List.fold_left (fun acc x -> x :: acc) [] [1;2;3]
```

**Expected output:** each line's value is echoed; `List.iter`
prints the three integers via UART and then echoes its unit result.

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

Read the COR24 switch (S2) once and drive the LED (D2) to match.

The web demo diverges from the CLI's `demo_led_toggle.ml` source
(which is a non-tail-call-optimised infinite `loop ()` recursion
that overflows the interp's call stack in a few thousand
iterations). Instead we ship a hand-edited, one-shot version so
the demo plays well with the live hardware panel: toggle **S2**
in the panel (bottom-right), hit **Run**, and see the output +
the **D2** indicator light up. `scripts/sync-demos.sh` has an
`LOCAL_OVERRIDE` allowlist that prevents future syncs from
clobbering this file.

```ocaml
let s = switch () in set_led s; print_int (if s then 1 else 0)
```

**Expected output:**

- With S2 **off**: `0` (and D2 stays dark).
- With S2 **on**: `1` (and D2 lights violet after Run).

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

## function-keyword

The `function` keyword is shorthand for `fun x -> match x with ...`.
Useful when the only thing a function does is pattern-match on its
sole argument. Also demonstrated inline at an application site
(`(function ...) 0`) and with richer patterns over lists and options.

```ocaml
let f = function 0 -> 100 | 1 -> 101 | _ -> 999 in f 0
let f = function 0 -> 100 | 1 -> 101 | _ -> 999 in f 1
let f = function 0 -> 100 | 1 -> 101 | _ -> 999 in f 5
let f = function [] -> 0 | h :: t -> h in f [42; 99]
(function 0 -> 100 | _ -> 0) 0
let classify = function Some n -> n | None -> 0 in classify (Some 42)
let classify = function Some n -> n | None -> 0 in classify None
```

**Expected output:**

```
100
101
999
42
100
42
0
```

---

## function-pattern-args

Destructuring patterns in the argument position of a function
definition. `let swap (x, y) = ...` binds `x` and `y` directly
from the incoming tuple, and the same works for nested tuples,
list cons, and unit.

```ocaml
let swap (x, y) = (y, x) in swap (1, 2)
let sum (a, (b, c)) = a + b + c in sum (1, (2, 3))
let head (h :: _) = h in head [1; 2; 3]
let f () = 42 in f ()
let add (x, y) = x + y in add (3, 4)
```

**Expected output:**

```
(2, 1)
6
1
42
7
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

## string-conversion

`string_of_int` and `int_of_string` round-trips. The REPL echoes
each value: integers print as decimals, strings print quoted.
`int_of_string` is lenient -- malformed inputs like `"abc"` and
the empty string return `0` rather than raising.

```ocaml
string_of_int 42
string_of_int 0
string_of_int (-7)
string_of_int 123456
int_of_string "100"
int_of_string "-42"
int_of_string "0"
print_endline (string_of_int (List.length [1;2;3]))
string_of_int (List.fold_left (fun a x -> a + x) 0 [1;2;3;4;5])
int_of_string (string_of_int 789)
int_of_string "abc"
int_of_string ""
```

**Expected output:**

```
"42"
"0"
"-7"
"123456"
100
-42
0
3
"15"
789
0
0
```

---

## string-equality

Structural `=` and `<>` on strings. `=` is true iff the strings
have identical bytes; `<>` is its negation. Both work inside `if`
conditions, which is the natural use in interactive demos that
read commands.

```ocaml
"abc" = "abc"
"abc" = "abd"
"abc" = "abcd"
"" = ""
"hello" <> "world"
"same" <> "same"
if "quit" = "quit" then print_endline "yes" else print_endline "no"
if "quit" = "keep" then print_endline "yes" else print_endline "no"
```

**Expected output:**

```
true
false
false
true
true
false
yes
no
```

---

## string-escapes

Escape sequences in string literals. The interpreter recognises
`\n` (newline), `\t` (tab), `\\` (backslash), and `\"` (double
quote) inside `"..."` strings. `String.length` counts bytes after
escapes are resolved, so `"a\nb"` has length 3.

```ocaml
let s = "line1\nline2" in print_endline s
let s = "tab\tbackslash\\quote\"" in print_endline s
String.length "a\nb"
```

**Expected output:** (the embedded `\n` becomes a real newline, the
`\t` becomes a tab character)

```
line1
line2
tab	backslash\quote"
3
```

---

## text-adventure

Marked `interactive: true`. A tiny text adventure demonstrating
variant types (rooms, items), pattern matching, string equality,
and `read_line ()` for runtime user input.

The seed is one long expression that starts by describing the Cave,
then enters a command loop. Once the seed finishes evaluating the
initial `describe Cave; loop (Cave, [])`, the interpreter blocks on
`read_line ()` and the input row appears.

**Available commands:**

| Command | Effect |
|---------|--------|
| `look` | Re-describe the current room |
| `inventory` | List collected items |
| `take` | Pick up the item in the current room (if any) |
| `n` / `s` / `e` / `w` | Move in that direction |
| `quit` | End the game |

**Map:** Cave →(n) Hall →(e) Garden, Hall →(n) Outside.

**Expected interaction:**

```
Damp cave. A lamp lies here. Exits: n.
look
Damp cave. A lamp lies here. Exits: n.
take
you take the lamp
inventory
lamp
n
Pillared hall. Exits: s, e, n (out).
e
Sunlit garden. A key glints here. Exits: w.
take
you take the key
w
Pillared hall. Exits: s, e, n (out).
n
You step into daylight. The end!
```

---

## echo-loop

Marked `interactive: true`. Reads a line, prints it back, repeats.
Type `quit` to exit cleanly via `exit 0`. Originally added to debug
`read_line` buffering; remains as the simplest possible
interactive demo and a smoke test for the input-row plumbing.

```ocaml
let rec loop = fun u -> let s = read_line () in if s = "quit" then (print_endline "bye"; exit 0) else (print_endline s; loop ()) in loop ()
```

**Expected interaction:**

```
hello
hello
ocaml is fun
ocaml is fun
quit
bye
```

---

## guess

Marked `interactive: true`. Number-guessing game with target 42.
`int_of_string` (lenient) parses the user's input; the loop
replies `too low`, `too high`, or `correct!` and exits via
`exit 0`.

The web variant of this demo is hand-edited (`LOCAL_OVERRIDE`)
because the CLI's branching version overflows the browser stack
on the losing path -- there's no Ctrl-C in the browser to abort
a runaway recursion.

```ocaml
let rec loop = fun u -> let g = int_of_string (read_line ()) in if g = 42 then (print_endline "correct!"; exit 0) else if g < 42 then (print_endline "too low"; loop ()) else (print_endline "too high"; loop ()) in (print_endline "Guess a number 1..100."; loop ())
```

**Expected interaction:**

```
Guess a number 1..100.
50
too high
30
too low
42
correct!
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

## variants-with-payload

Variant constructors that carry payloads of different types
(`TInt of int`, `TIdent of string`) alongside nullary ones
(`TLArrow`, `TEOF`). `match` dispatches by constructor and binds
the payload. The bare `TInt 7` line is variant constructor
application as an expression -- the REPL echoes the constructed
value.

```ocaml
type token = TInt of int | TIdent of string | TLArrow | TEOF
let dump tok = match tok with TInt n -> string_of_int n | TIdent s -> s | TLArrow -> "<-" | TEOF -> "EOF"
let _ = print_endline (dump (TInt 42))
let _ = print_endline (dump (TIdent "abc"))
let _ = print_endline (dump TLArrow)
let _ = print_endline (dump TEOF)
TInt 7
type color = Red | Green | Blue
let color_name c = match c with Red -> "red" | Green -> "green" | Blue -> "blue"
let _ = print_endline (color_name Green)
```

**Expected output:**

```
42
abc
<-
EOF
TInt 7
green
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

## tuple-arity

Pattern matching against tuples of different arities (3-tuples and
4-tuples) with literal subpatterns and wildcards. Each `match` arm
must match the same arity as the subject. The bare `(1, 2, 3)`
is a 3-tuple constructor whose value the REPL echoes.

```ocaml
let t = (1, 42, "hello") in match t with (0, _, s) -> print_endline ("IDENT " ^ s) | (1, n, _) -> print_endline ("INT " ^ string_of_int n) | (_, _, _) -> print_endline "OTHER"
let q = (1, 2, 3, 4) in match q with (1, _, 3, n) -> print_int n | (_, _, _, _) -> print_int 0
(1, 2, 3)
match (0, "name", 9) with (0, s, _) -> print_endline s | (_, _, _) -> print_endline "miss"
```

**Expected output:** (3-tuples display as right-nested pairs in the
REPL: `(1, 2, 3)` is internally `(1, (2, 3))`)

```
INT 42
4
(1, (2, 3))
name
```

---

## when-guards

`match` arms can be qualified with a `when <bool-expr>` guard, so
the arm fires only if the pattern matches *and* the guard is true.
Used below to implement `abs` and `sign`, and to chain guarded
arms against a fresh match subject.

```ocaml
let abs x = match x with n when n < 0 -> -n | n -> n in abs (-5)
let abs x = match x with n when n < 0 -> -n | n -> n in abs 7
let sign x = match x with n when n < 0 -> -1 | 0 -> 0 | _ -> 1 in sign (-10)
let sign x = match x with n when n < 0 -> -1 | 0 -> 0 | _ -> 1 in sign 0
let sign x = match x with n when n < 0 -> -1 | 0 -> 0 | _ -> 1 in sign 99
match 7 with n when n > 10 -> "big" | n when n > 5 -> "mid" | _ -> "small"
```

**Expected output:**

```
5
7
-1
0
1
"mid"
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

## toplevel-let

Top-level `let` bindings -- the form without a trailing `in`. The
binding lives until the next `let __module = ...` switch or end
of program. Demonstrated with: a simple lambda binding, a
sugared multi-arg function, `let rec`, tuple destructuring on the
LHS, and the unit-pattern shorthand `let () = ...` for running an
expression purely for its side effect. Top-level lets evaluate
to unit and the REPL emits a blank line for them; `clean_output`
strips those, so only the `print_int` / `print_endline` calls
inside each binding show in the panel.

```ocaml
let greet = fun name -> print_endline ("hello, " ^ name)
let _ = greet "world"
let _ = greet "tuplet"
let add x y = x + y
let _ = print_int (add 20 22)
let rec fact n = if n = 0 then 1 else n * fact (n - 1)
let _ = print_int (fact 5)
let (a, b) = (3, 4)
let _ = print_int (a + b)
let () = print_endline "done"
```

**Expected output:**

```
hello, world
hello, tuplet
42
120
7
done
```

---

## tco-countdown

Tail-call optimisation showcase. `count n` recurses until `n = 0`,
printing each integer with a trailing newline (the next REPL line
emits one) before the recursive call. The recursion is in tail
position -- without TCO this would overflow the call stack at
~100 frames. The vendored interpreter implements TCO in
`eval_expr`, so the demo runs to completion cleanly from 100
down to 1, then prints `done`.

```ocaml
let rec count = fun n -> if n = 0 then print_endline "done" else (print_int n; count (n - 1)) in count 100
```

**Expected output:** (101 lines total -- elided here for brevity)

```
100
99
98
...
2
1
done
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

### Typing your own input

The Pascal OCaml interpreter parses each line of REPL input as a
single expression that reduces to a value. It does **not** accept
bare top-level `let` bindings (those are a concession in real
OCaml's toplevel that this interpreter doesn't make).

**Works:**

```ocaml
42
1 + 1
let x = 42 in x
let f x = x * 2 in f 21
let rec fact n = if n = 0 then 1 else n * fact (n - 1) in fact 5
print_int 99
```

**Fails** (each produces `PARSE ERROR`):

```ocaml
let x = 42
let add x y = x + y
let f x = x + 1
```

If you want the effect of a top-level binding, wrap the rest of
your session in `let ... in` and put the value you want on the
right-hand side. For function definitions you'll test repeatedly,
just edit the demo in the source pane on the left -- that's what
the source editor is for.
