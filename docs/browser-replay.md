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

## Self-service delivery candidate

The original public fixture/schema and native CLI/core stay unchanged. A current
frame is tied to the selected step, request and worker generation; step changes
clear metrics/timeline/checkpoint and both exports immediately. Presentation
filters cannot restore a prior result. Pause trace stops timed stepping; Stop
engine also terminates initialization/computation and keeps the selected step
and filter. Retry engine starts a fresh worker. Initialization is bounded to
15 seconds and projection to10 seconds on the main thread. Fatal runtime errors
retire the engine; ordinary native error strings preserve the warm worker.
These bounds are recovery policies for this tiny fixed fixture, not performance
or live-session claims. Injected CPU stalls/traps are explicit test faults.

Download replay frame retains the original pretty-JSON format and filename;
Download raw Rust frame preserves the exact native string. All8 public prefixes
and8 portable typed invalid steps match full native/WASM strings/errors. Raw JS
u32 coercions remain separate. The source builder also publishes all8 exact
native fixture frames as NDJSON, for inspection without running WebAssembly.
No imported transcript, new capture schema or private session is supported.

The footer links to a no-JavaScript build guide, complete source.tar.xz,
receiver.zip, full original notices, native frames and BUILD-MANIFEST.json.
Runtime app/session/worker/module routes share assets/<actual-source-commit>/.
The source receiver verifies a trusted external SHA and full Git/file/mode
identity before writing a new directory. Its ZIP includes both applicable MIT
grants. Source/input/compiler custody helpers and lifecycle code are adapted
from public TERRAIN; original notices are retained in notices/COMPONENTS.json.

Python3.11+ stdlib, Rust1.95.0 with wasm32-unknown-unknown and wasm-bindgen0.2.127
are external prerequisites. From a trusted complete source receipt:

```sh
python tools/build-pages.py --inputs /new/witness-inputs --cargo-home /new/witness-home --target /new/witness-target
python tools/check-frames.py --site dist/WITNESS --output /new/frame-proof --inputs /new/witness-inputs --cargo-home /new/witness-home --target /new/witness-target
python tools/check-source-replay.py --site dist/WITNESS --output /new/source-proof --browser
```

Inputs/home must be new or verified, outside the source/target. All20 original
locked registry archives and740 files are included; no implicit default cache,
TRACKER-relative override or online Cargo acquisition is part of the build.
The complete site remains bounded below5,000,000 bytes. The replay runner uses
received source outside Git, new owned inputs/home and an initially absent
target, compares every published file/manifest, then runs native/frame/browser
validation. Installed tools and original-checkout npm/Playwright dependencies
are reused explicitly; cross-platform compiler-byte identity is not promised.

Set WITNESS_DIST to a built site's parent directory for local browser checks.
Set WITNESS_SITE_URL=https://giodl73-repo.github.io for the same committed suite
against the anonymous live origin; this disables the local server. The archive
replay clears that live override. Current full-package/default/live qualification
is recorded in context/waves/2026-11-06-public-replay-self-service/WAVE.md; pending
gates are not inferred complete from the earlier local lifecycle test.
