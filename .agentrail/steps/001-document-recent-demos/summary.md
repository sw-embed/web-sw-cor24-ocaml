Added 10 missing entries to docs/demos.md (the 8 demos from refresh-interp-add-seven-demos + add-modules-demo-and-multifile-plan plus echo-loop and guess that were already missing). Inserted topically next to related demos (string-* near strings, modules near list-module, variants-with-payload near named-adts, etc.) rather than alphabetically, matching the existing doc's flow. Expected outputs captured from 
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test every_non_interactive_demo_halts_cleanly ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 9 filtered out; finished in 43.66s (which prints cleaned=... for every demo) so the doc reflects exactly what the live UI panel shows; interactive entries (echo-loop, guess) follow the text-adventure pattern with a sample interaction transcript. Doc now has 34 ## sections matching the catalog.