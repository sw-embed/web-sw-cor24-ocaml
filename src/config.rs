//! Compile-time resources for the live demo.
//!
//! `OCAML_P24M` is the linked multi-unit p-code image produced by
//! `scripts/vendor-artifacts.sh` (see `assets/ocaml.p24m`). The runner
//! parses its header at session start and lays out VM memory so the
//! baked absolute addresses (data refs, call targets after relocation,
//! globals) resolve correctly.
pub const OCAML_P24M: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/ocaml.p24m"));
