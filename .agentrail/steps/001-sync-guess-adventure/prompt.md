# Sync demo_guess and demo_adventure into web UI

Update scripts/sync-demos.sh MAPPING to include demo_guess -> guess. The demo_adventure entry is already in place but the upstream source has evolved (now includes help command, inv alias, quit exit); re-run sync so examples/text-adventure.ml matches upstream.

Run ./scripts/sync-demos.sh to refresh examples/*.ml.

Edit src/demos.rs to insert a new Demo entry for 'guess' in alphabetical order between 'functions' and 'hello'. Set interactive: true, add a description (it's a number-guessing game; target is 42; enter integers).

Verify: cargo test (demos_are_alphabetised, names_are_unique, interactive_demos_exist).

Commit examples, sync script, src/demos.rs, and .agentrail changes.