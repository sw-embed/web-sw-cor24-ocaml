//! Integration test: run every non-interactive demo end-to-end through
//! the same `Session` the browser uses, and report any that don't halt
//! cleanly. Diagnoses "some demos produce errors" without needing the
//! user to point at specific ones.

use web_sw_cor24_ocaml::demos::{concat_with_aux, DEMOS};
use web_sw_cor24_ocaml::runner::{Session, DEFAULT_BATCH};

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
        // full_source() is a passthrough for single-file demos and
        // injects `let __module = "..."` directives + aux contents
        // for multi-file demos (modules-multifile).
        let full = demo.full_source();
        let mut s = Session::new(&full);
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
fn interactive_demos_reach_awaiting_input() {
    let interactive_demos: Vec<_> = DEMOS.iter().filter(|d| d.interactive).collect();
    assert!(
        !interactive_demos.is_empty(),
        "expected at least one interactive demo"
    );
    for demo in interactive_demos {
        let full = demo.full_source();
        let mut s = Session::new_interactive(&full);
        for _ in 0..MAX_TICKS {
            let r = s.tick();
            if r.done || s.is_awaiting_input() {
                break;
            }
        }
        assert!(
            !s.is_done(),
            "interactive demo '{}' should not have halted: {} (instrs={})",
            demo.name,
            s.stop_reason(),
            s.instructions()
        );
        assert!(
            s.is_awaiting_input(),
            "expected awaiting input after seeding source for '{}' ({} instrs, raw: {:?})",
            demo.name,
            s.instructions(),
            s.output()
        );
        let cleaned = s.clean_output();
        eprintln!("{} cleaned after seed: {cleaned:?}", demo.name);
    }
}

// Reference DEFAULT_BATCH so a future tweak that removes the public
// const surfaces here, not in a downstream consumer.
#[allow(dead_code)]
const _: u64 = DEFAULT_BATCH;

#[test]
fn concat_with_aux_edited_runs_through_session() {
    // Phase 2 invariant: when the App feeds Session a `concat_with_aux`
    // built from user-edited aux content, the result reflects the edit
    // -- not the demo's baked-in `auxiliary_files`.
    //
    // Simulate a user who edited math.ml to redefine Math.add as
    // multiplication. Main is unchanged: `Math.add 2 3` should now
    // print 6, not 5.
    let edited_math = "let add x y = x * y\nlet square x = x * x\nlet double x = add x x\n";
    let main = "Math.add 2 3";
    let aux = [("math.ml", edited_math)];
    let full = concat_with_aux(&aux, main);

    let mut s = Session::new(&full);
    for _ in 0..MAX_TICKS {
        let r = s.tick();
        if r.done {
            break;
        }
    }
    let cleaned = s.clean_output();
    assert!(s.is_done() && s.is_halted(), "session should halt cleanly");
    assert!(
        cleaned.trim() == "6",
        "edited Math.add should make 2 * 3 = 6, got cleaned={cleaned:?}\n  full source:\n{full}"
    );
}

#[test]
fn text_adventure_take_lamp_only_once() {
    // Regression test for the take bug: in the buggy version,
    // each `take` in the Cave produced "you take the lamp" again
    // and the cave's look still mentioned the lamp. Fix tracks
    // (room, inventory, lamp_taken, key_taken) in the loop state.
    let demo = DEMOS.iter().find(|d| d.name == "text-adventure").unwrap();
    let full = demo.full_source();
    let mut s = Session::new_interactive(&full);

    // Drain the seed.
    for _ in 0..MAX_TICKS {
        if s.is_awaiting_input() || s.is_done() {
            break;
        }
        s.tick();
    }
    assert!(s.is_awaiting_input(), "seed should reach awaiting_input");

    // 1st take: should pick up the lamp.
    s.feed_input("take");
    // Tick the full budget rather than bailing on is_awaiting_input --
    // rx_queue empties as soon as bytes flow into UART, so
    // is_awaiting_input flips true while the interpreter is mid-eval.
    // Once the interp blocks on the next read_line(), further ticks are
    // no-ops, so unbounded ticking is safe.
    for _ in 0..MAX_TICKS {
        s.tick();
        if s.is_done() {
            break;
        }
    }
    assert!(
        s.clean_output().contains("you take the lamp"),
        "expected 'you take the lamp' after first take\n  cleaned: {:?}",
        s.clean_output()
    );

    // 2nd take in the same room: should reject -- "nothing here."
    s.feed_input("take");
    // Tick the full budget rather than bailing on is_awaiting_input --
    // rx_queue empties as soon as bytes flow into UART, so
    // is_awaiting_input flips true while the interpreter is mid-eval.
    // Once the interp blocks on the next read_line(), further ticks are
    // no-ops, so unbounded ticking is safe.
    for _ in 0..MAX_TICKS {
        s.tick();
        if s.is_done() {
            break;
        }
    }
    let cleaned = s.clean_output();
    assert!(
        cleaned.contains("nothing here."),
        "expected 'nothing here.' on second take\n  cleaned: {cleaned:?}"
    );
    // And the second take must NOT have produced another "you take the lamp".
    let lamp_take_count = cleaned.matches("you take the lamp").count();
    assert_eq!(
        lamp_take_count, 1,
        "lamp should be takeable only once; cleaned: {cleaned:?}"
    );

    // look should describe the cave WITHOUT mentioning the lamp now.
    s.feed_input("look");
    for _ in 0..MAX_TICKS {
        s.tick();
        if s.is_awaiting_input() || s.is_done() {
            break;
        }
    }
    let cleaned = s.clean_output();
    // Find the most recent describe (after the second take).
    let after_second_take = cleaned
        .rfind("nothing here.")
        .map(|i| &cleaned[i..])
        .unwrap_or(&cleaned);
    assert!(
        !after_second_take.contains("A lamp lies here."),
        "post-take look should not mention the lamp\n  tail: {after_second_take:?}"
    );

    // Cleanup: quit.
    s.feed_input("quit");
    for _ in 0..MAX_TICKS {
        s.tick();
        if s.is_done() {
            break;
        }
    }
}

#[test]
fn echo_loop_feed_and_check() {
    let demo = DEMOS.iter().find(|d| d.name == "echo-loop").unwrap();
    let full = demo.full_source();
    let mut s = Session::new_interactive(&full);

    // Tick until the seed is consumed and we're awaiting input.
    for _ in 0..MAX_TICKS {
        if s.is_awaiting_input() || s.is_done() {
            break;
        }
        s.tick();
    }
    assert!(
        s.is_awaiting_input(),
        "echo-loop should reach awaiting_input after seed (done={}, stop={}, instrs={})\n  raw: {:?}\n  cleaned: {:?}",
        s.is_done(),
        s.stop_reason(),
        s.instructions(),
        s.output(),
        s.clean_output(),
    );

    // Feed "hello" and tick until we get a response.
    s.feed_input("hello");
    for _ in 0..MAX_TICKS {
        s.tick();
        if s.is_awaiting_input() || s.is_done() {
            break;
        }
    }
    let cleaned = s.clean_output();
    eprintln!(
        "echo-loop after feeding 'hello':\n  raw:   {:?}\n  cleaned: {:?}\n  done={} halted={} stop={}\n  awaiting={} instrs={}",
        s.output(),
        cleaned,
        s.is_done(),
        s.is_halted(),
        s.stop_reason(),
        s.is_awaiting_input(),
        s.instructions(),
    );
    assert!(
        cleaned.contains("hello"),
        "expected 'hello' in cleaned output after feeding 'hello'\n  cleaned: {cleaned:?}"
    );

    // Feed "quit" and tick until done.
    s.feed_input("quit");
    for _ in 0..MAX_TICKS {
        s.tick();
        if s.is_done() {
            break;
        }
    }
    let cleaned = s.clean_output();
    eprintln!(
        "echo-loop after feeding 'quit':\n  raw:   {:?}\n  cleaned: {:?}\n  done={} halted={} stop={}\n  instrs={}",
        s.output(),
        cleaned,
        s.is_done(),
        s.is_halted(),
        s.stop_reason(),
        s.instructions(),
    );
    assert!(
        cleaned.contains("bye"),
        "expected 'bye' in cleaned output after feeding 'quit'\n  cleaned: {cleaned:?}"
    );
}

#[test]
fn read_line_simple() {
    let mut s = Session::new_interactive("let s = read_line () in print_endline s");
    for _ in 0..MAX_TICKS {
        if s.is_awaiting_input() || s.is_done() {
            break;
        }
        s.tick();
    }
    eprintln!(
        "read_line_simple after seed:\n  raw:   {:?}\n  cleaned: {:?}\n  done={} halted={} awaiting={} instrs={}",
        s.output(),
        s.clean_output(),
        s.is_done(),
        s.is_halted(),
        s.is_awaiting_input(),
        s.instructions(),
    );
    assert!(!s.is_done(), "read_line_simple should not halt after seed");
    s.feed_input("test");
    for _ in 0..MAX_TICKS {
        s.tick();
        if s.is_awaiting_input() || s.is_done() {
            break;
        }
    }
    let cleaned = s.clean_output();
    eprintln!(
        "read_line_simple after feeding 'test':\n  raw:   {:?}\n  cleaned: {:?}\n  done={} halted={} awaiting={} instrs={}",
        s.output(),
        cleaned,
        s.is_done(),
        s.is_halted(),
        s.is_awaiting_input(),
        s.instructions(),
    );
    assert!(
        cleaned.contains("test"),
        "expected 'test' in output after feeding 'test'\n  cleaned: {cleaned:?}"
    );
}

#[test]
fn getc_simple() {
    let mut s = Session::new_interactive("let c = getc () in putc c");
    for _ in 0..MAX_TICKS {
        if s.is_awaiting_input() || s.is_done() {
            break;
        }
        s.tick();
    }
    eprintln!(
        "getc_simple after seed:\n  raw:   {:?}\n  cleaned: {:?}\n  done={} halted={} awaiting={} instrs={}",
        s.output(),
        s.clean_output(),
        s.is_done(),
        s.is_halted(),
        s.is_awaiting_input(),
        s.instructions(),
    );
    assert!(!s.is_done(), "getc_simple should not halt after seed");
    s.feed_input("A");
    for _ in 0..MAX_TICKS {
        s.tick();
        if s.is_done() || s.is_awaiting_input() {
            break;
        }
    }
    let cleaned = s.clean_output();
    eprintln!(
        "getc_simple after feeding 'A':\n  raw:   {:?}\n  cleaned: {:?}\n  done={} halted={} awaiting={} instrs={}",
        s.output(),
        cleaned,
        s.is_done(),
        s.is_halted(),
        s.is_awaiting_input(),
        s.instructions(),
    );
}

#[test]
fn read_line_var_ref() {
    let mut s = Session::new("read_line");
    for _ in 0..MAX_TICKS {
        s.tick();
        if s.is_done() {
            break;
        }
    }
    let cleaned = s.clean_output();
    eprintln!(
        "read_line_var_ref:\n  raw:   {:?}\n  cleaned: {:?}\n  done={} halted={} instrs={}",
        s.output(),
        cleaned,
        s.is_done(),
        s.is_halted(),
        s.instructions(),
    );
    assert!(
        cleaned.contains("<fun>"),
        "read_line should be recognized as a builtin and print <fun>\n  cleaned: {cleaned:?}"
    );
}

#[test]
fn print_int_var_ref() {
    let mut s = Session::new("print_int");
    for _ in 0..MAX_TICKS {
        s.tick();
        if s.is_done() {
            break;
        }
    }
    let cleaned = s.clean_output();
    eprintln!(
        "print_int_var_ref:\n  raw:   {:?}\n  cleaned: {:?}\n  done={} halted={} instrs={}",
        s.output(),
        cleaned,
        s.is_done(),
        s.is_halted(),
        s.instructions(),
    );
    assert!(
        cleaned.contains("<fun>"),
        "print_int should be recognized as a builtin\n  cleaned: {cleaned:?}"
    );
}

#[test]
fn print_endline_var_ref() {
    let mut s = Session::new("print_endline");
    for _ in 0..MAX_TICKS {
        s.tick();
        if s.is_done() {
            break;
        }
    }
    let cleaned = s.clean_output();
    eprintln!(
        "print_endline_var_ref:\n  raw:   {:?}\n  cleaned: {:?}\n  done={} halted={} instrs={}",
        s.output(),
        cleaned,
        s.is_done(),
        s.is_halted(),
        s.instructions(),
    );
    assert!(
        cleaned.contains("<fun>"),
        "print_endline should be recognized as a builtin\n  cleaned: {cleaned:?}"
    );
}

#[test]
fn builtin_spot_check() {
    let cases = [
        ("print_int", true),
        ("putc", true),
        ("set_led", true),
        ("led_on", true),
        ("led_off", true),
        ("switch", true),
        ("getc", true),
        ("let x = getc in x", true),
        ("read_line", true),
        ("let y = read_line in y", true),
        ("nil", false),
        ("hd", true),
        ("tl", true),
        ("fst", true),
        ("snd", true),
        ("print_endline", true),
    ];
    let mut failures = Vec::new();
    for (expr, expected_fun) in cases {
        let mut s = Session::new(expr);
        for _ in 0..MAX_TICKS {
            s.tick();
            if s.is_done() {
                break;
            }
        }
        let cleaned = s.clean_output();
        let has_fun = cleaned.contains("<fun>");
        if expected_fun && !has_fun {
            failures.push(format!(
                "  {:<15} expected <fun>, got cleaned={:?} done={} halted={}",
                expr,
                cleaned,
                s.is_done(),
                s.is_halted()
            ));
        } else if !expected_fun && has_fun {
            failures.push(format!(
                "  {:<15} expected NOT <fun>, got cleaned={:?}",
                expr, cleaned
            ));
        } else {
            eprintln!("  {:<15} OK (cleaned={:?})", expr, cleaned);
        }
    }
    if !failures.is_empty() {
        eprintln!("\nBuiltin failures:");
        for f in &failures {
            eprintln!("{f}");
        }
        panic!("{} builtin(s) failed", failures.len());
    }
}

#[test]
fn getc_var_ref() {
    let mut s = Session::new("getc");
    for _ in 0..MAX_TICKS {
        s.tick();
        if s.is_done() {
            break;
        }
    }
    let cleaned = s.clean_output();
    eprintln!(
        "getc_var_ref:\n  raw:   {:?}\n  cleaned: {:?}\n  done={} halted={} instrs={}",
        s.output(),
        cleaned,
        s.is_done(),
        s.is_halted(),
        s.instructions(),
    );
    assert!(
        cleaned.contains("<fun>"),
        "getc should be recognized as a builtin and print <fun>\n  cleaned: {cleaned:?}"
    );
}
