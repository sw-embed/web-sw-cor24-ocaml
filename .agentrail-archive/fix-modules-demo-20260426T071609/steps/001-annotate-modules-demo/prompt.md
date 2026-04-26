# Annotate modules demo with comments + corrective line

## Tasks

1. Add `modules` to LOCAL_OVERRIDE in `scripts/sync-demos.sh`
   (alongside `led-toggle` and `guess`) and document the rationale
   in the LOCAL_OVERRIDE comment block: the CLI test ends on a
   bare EVAL ERROR which reads as broken; the web override adds
   inline `(* ... *)` annotations and a corrective `Math.add 1 2`
   before the deliberate-failure line.

2. Rewrite `examples/modules.ml` with inline trailing comments:

       let __module = "Math"        (* enter module Math *)
       let add x y = x + y          (* defines Math.add *)
       let double x = add x x       (* Math.double; unqualified add resolves within Math *)
       let __module = "Main"        (* switch to Main; Math's bindings now require qualification *)
       Math.add 2 3                 (* qualified dispatch into Math --> 5 *)
       Math.double 9                (* qualified --> 18 *)
       Math.add 1 2                 (* the right way: qualify into Math --> 3 *)
       add 1 2                      (* the wrong way: unqualified add lives in Math, not Main -- EVAL ERROR expected *)

3. Update the description for `modules` in `src/demos.rs` to make
   the educational frame explicit: namespaces, qualified dispatch
   working three times, then a deliberate unqualified call to show
   isolation. Note that the language has no try/catch -- the REPL
   simply resets eval_error per line and continues.

4. Run `cargo test` (full suite). The cleanly-halts integration
   test will execute the augmented demo end-to-end against the
   vendored interpreter; it must still pass.

5. Commit as `fix(demos): annotate modules demo, add corrective
   line, document local override` covering:
     - scripts/sync-demos.sh
     - examples/modules.ml
     - src/demos.rs

Stop after committing. Pages rebuild + push is step 002.