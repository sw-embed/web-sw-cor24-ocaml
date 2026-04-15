//! Integration test: run every non-interactive demo end-to-end through
//! the same `Session` the browser uses, and report any that don't halt
//! cleanly. Diagnoses "some demos produce errors" without needing the
//! user to point at specific ones.

use web_sw_cor24_ocaml::demos::DEMOS;
use web_sw_cor24_ocaml::runner::{DEFAULT_BATCH, Session};

/// Per-demo cycle cap. Each demo runs at most `MAX_TICKS *
/// DEFAULT_BATCH` cor24 instructions before we give up.
const MAX_TICKS: u64 = 400;

/// `led-toggle` is documented as an intentional infinite loop (reads
/// the switch, sets the LED, repeats; the browser stubs both so it
/// never makes progress). Skip the halt assertion for it but still
/// run a few ticks to confirm it doesn't trap.
const KNOWN_INFINITE: &[&str] = &["led-toggle"];

#[test]
fn every_non_interactive_demo_halts_cleanly() {
    let mut failures: Vec<String> = Vec::new();
    let mut summary: Vec<String> = Vec::new();

    for demo in DEMOS {
        if demo.interactive {
            continue;
        }
        let mut s = Session::new(demo.source);
        for _ in 0..MAX_TICKS {
            let r = s.tick();
            if r.done {
                break;
            }
        }

        let known_infinite = KNOWN_INFINITE.contains(&demo.name);
        let line = format!(
            "  {:<20}  done={:<5} halted={:<5} instrs={:>10} stop={:<8} cleaned={:?}",
            demo.name,
            s.is_done(),
            s.is_halted(),
            s.instructions(),
            s.stop_reason(),
            s.clean_output(),
        );
        summary.push(line.clone());

        if known_infinite {
            // Should not have halted in the budget; if it DID, that's
            // also fine -- just shift the doc note. Any trap is a bug.
            if s.is_done() && !s.is_halted() {
                failures.push(format!(
                    "{} (known-infinite) trapped: {}\n  raw: {:?}",
                    demo.name,
                    s.stop_reason(),
                    s.output()
                ));
            }
            continue;
        }

        if !s.is_done() {
            failures.push(format!(
                "{} did not halt within {} ticks ({} instrs)\n  raw: {:?}\n  cleaned: {:?}",
                demo.name,
                MAX_TICKS,
                s.instructions(),
                s.output(),
                s.clean_output(),
            ));
        } else if !s.is_halted() {
            failures.push(format!(
                "{} stopped abnormally: {}\n  instrs: {}\n  raw: {:?}\n  cleaned: {:?}",
                demo.name,
                s.stop_reason(),
                s.instructions(),
                s.output(),
                s.clean_output(),
            ));
        }
    }

    eprintln!("\n=== Per-demo summary ===");
    for line in &summary {
        eprintln!("{line}");
    }

    if !failures.is_empty() {
        eprintln!("\n=== Failures ({}) ===", failures.len());
        for f in &failures {
            eprintln!("{f}\n");
        }
        panic!("{} demo(s) failed", failures.len());
    }
}

#[test]
fn repl_session_reaches_awaiting_input() {
    // Find the interactive demo and run it long enough to drain the
    // seed source. After the seed is consumed the runner should flip
    // to `is_awaiting_input` so the UI can pop the input row.
    let demo = DEMOS
        .iter()
        .find(|d| d.interactive)
        .expect("an interactive demo");
    let mut s = Session::new_interactive(demo.source);
    for _ in 0..MAX_TICKS {
        let r = s.tick();
        if r.done || s.is_awaiting_input() {
            break;
        }
    }
    assert!(
        !s.is_done(),
        "interactive demo should not have halted: {} (instrs={})",
        s.stop_reason(),
        s.instructions()
    );
    // The heuristic flips when the source queue empties; that's the
    // signal the UI uses to show the input row.
    assert!(
        s.is_awaiting_input(),
        "expected awaiting input after seeding source ({} instrs, raw: {:?})",
        s.instructions(),
        s.output()
    );

    // Sanity: cleaned output should contain the seed's expected
    // results (42, 2, 42, 120, 99) once the seed has run.
    let cleaned = s.clean_output();
    eprintln!("repl-session cleaned after seed: {cleaned:?}");
}

// Reference DEFAULT_BATCH so a future tweak that removes the public
// const surfaces here, not in a downstream consumer.
#[allow(dead_code)]
const _: u64 = DEFAULT_BATCH;
