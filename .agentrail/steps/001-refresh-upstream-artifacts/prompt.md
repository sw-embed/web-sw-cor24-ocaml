# Refresh upstream pvm.s and ocaml.p24m

Pull the latest `asm/pvm.s` and `assets/ocaml.p24m` from the sibling
repos so subsequent steps can use the new language features (strings,
named ADTs, function-form let).

## Work

1. Run `./scripts/vendor-artifacts.sh`. Confirm:
   - `asm/pvm.s` updated (or unchanged -- VM hasn't moved, this just
     re-vendors).
   - `assets/ocaml.p24m` size grows from ~30961 B to ~39870 B.
2. Run `cargo test --lib` and `cargo test --test demos`. The
   existing demos must all still halt cleanly against the new image
   (the runtime is backward compatible per upstream commit history).
   The `print_int 42` measurement may shift slightly -- record the
   new figure in the commit message.
3. Run `trunk build` to confirm the WASM bundle still links.

## Validation

- `git diff --stat assets/` shows ocaml.p24m grew.
- `cargo test` exits zero across all suites.
- `trunk build` clean.

## Out of scope

- Adding new demos that exercise the new features (next step).
- Hardware panel work (third step).
