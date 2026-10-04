# Public browser replay fixture

`witness-web` constructs seven provider-neutral `HarnessEvent` records and
projects a prefix through native `HarnessReplay` counts and JSON serialization.
The synthetic trace loads sample instructions, receives intent, reads/edits a
sample source, validates, checkpoints, then resumes. No private session,
provider transcript, credential, protected source, or live capture is included.

Step 4 displays validation owed; step 5 attaches fixture validation. Checkpoint
and resume labels remain pending until their events occur. Counters include all
observed prefix events even when the display filter hides some rows. Source
pointers and receipt identifiers are demonstration values, not fetched content
or proof of external execution. No semantic closure is executed here.

A worker runs real WASM; bounded URLs share a step; JSON exports the exact frame.
The UI supports keyboard range controls, previous/next, timed play/pause, and
source/evidence filters. Actions runs native tests, adapter lint, release WASM,
actual browser checks, a 5 MB budget, then default-branch Pages deployment.
