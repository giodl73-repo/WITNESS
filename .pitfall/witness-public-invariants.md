# WITNESS Public Core Invariants

## WITPUB-I-01: Replay Fixture Is Deterministic

**Status:** VERIFIED

**Invariant:** The retained `claude-session` replay reports seven events, a
stable active cut, checkpoint, and frontier count.

**Why it matters:** Public consumers need a stable harness baseline that does
not depend on live provider state.

**Test:** `cargo test --workspace`.

**Evidence:** `crates/witness-core/src/lib.rs`,
`crates/witness-cli/tests/proof_surface.rs`, and `README.md`.

## WITPUB-I-02: CLI Usage Failures Are Structured

**Status:** VERIFIED

**Invariant:** Unsupported public CLI commands exit with code 2 and emit a
stable usage string.

**Why it matters:** Downstream scripts need a predictable failure mode instead
of silent success or provider invocation.

**Test:** `cargo test -p witness-cli --test proof_surface`.

**Evidence:** `crates/witness-cli/tests/proof_surface.rs` and
`crates/witness-cli/tests/fixtures/replay_cli_proof.txt`.

## WITPUB-I-03: Public CLI Emits JSON Status And Replay

**Status:** VERIFIED

**Invariant:** `witness-cli status --json` emits `witness.status.v1`, and
`witness-cli replay --json` emits `witness.harness.v1` for the retained replay.

**Why it matters:** Agent and tracker workflows need machine-readable evidence
without scraping human CLI text.

**Test:** `cargo test -p witness-cli --test proof_surface`.

**Evidence:** `crates/witness-cli/src/main.rs`,
`crates/witness-cli/tests/proof_surface.rs`, and `README.md`.

## WITPUB-I-04: Public Core Has No Provider Orchestration

**Status:** VERIFIED

**Invariant:** The public workspace exposes typed fixtures and a minimal CLI,
not live provider sessions, credentials, raw provider transcripts, or provider
orchestration.

**Why it matters:** The public core must remain reusable and publishable without
leaking private incubation material.

**Test:** `cargo test --workspace --locked`.

**Evidence:** `README.md`, `PRODUCT_PLAN.md`, `docs/MAINTENANCE.md`, and
`Cargo.toml`.

## WITPUB-I-05: Publication Scan Gates Promotion

**Status:** VERIFIED

**Invariant:** Public-core promotion requires formatting, linting, tests, CLI
smokes, and publication-boundary review.

**Why it matters:** Private sessions, customer material, credentials, or
organization-specific claims must not enter the public repo by accident.

**Test:** `cargo fmt --all -- --check`,
`cargo clippy --workspace --all-targets -- -D warnings`, and
`cargo test --workspace --locked`.

**Evidence:** `docs/MAINTENANCE.md`,
`context/waves/2026-07-20-public-core/WAVE.md`, and `CONTRIBUTING.md`.
