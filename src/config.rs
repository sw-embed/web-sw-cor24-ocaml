//! Compile-time resources baked by `build.rs`:
//!
//! - `PVM_BIN`: machine code of `asm/pvm.s` assembled with the
//!   cor24-emulator assembler. Loaded at address 0 of the emulator.
//! - `OCAML_P24M`: the linked multi-unit p-code image produced by the
//!   sw-cor24-ocaml toolchain (`scripts/vendor-artifacts.sh`). Loaded
//!   at the OCaml interpreter's reserved address (`OCAML_LOAD_ADDR`).
//! - `PVM_LABELS`: assembler symbol table; `label_addr("code_ptr")`
//!   tells the runner where to write the `.p24m` load address so pvm.s
//!   picks up the image at boot.

pub const PVM_BIN: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/pvm.bin"));
pub const OCAML_P24M: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/ocaml.p24m"));

include!(concat!(env!("OUT_DIR"), "/pvm_labels.rs"));

/// Runtime load address for the OCaml `.p24m` image. Matches
/// `../sw-cor24-ocaml/scripts/run-ocaml.sh` (and the `--load-addr`
/// passed to `p24-load`) so absolute data references baked into the
/// image resolve at the same memory locations the linker assumed.
pub const OCAML_LOAD_ADDR: u32 = 0x01_0000;

pub fn label_addr(name: &str) -> u32 {
    PVM_LABELS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(_, a)| *a)
        .unwrap_or(0)
}
