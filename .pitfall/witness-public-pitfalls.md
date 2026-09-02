# WITNESS Public Core Pitfalls

## WITPUB-PF-01: Raw Session Logs Enter Public Fixtures

**Status:** MITIGATED

**Pattern:** Private provider transcripts, customer material, credentials,
employee content, or raw source payloads are committed as replay or checkpoint
fixtures.

**Domain:** fixtures, docs, CLI proof output, promotion from private incubation,
and future provider adapters.

**Detection difficulty:** A real session can look like valuable proof while
quietly carrying private material or provider-specific state.

**Structural solution:** Keep public fixtures synthetic/product-neutral, require
session-safety review, and treat private history as non-mergeable into public
core.

**Evidence:** `docs/MAINTENANCE.md`, `SECURITY.md`, and
`.roles/stakeholders/session-safety-reviewer.md`.

## WITPUB-PF-02: WITNESS Becomes A Second LATTICE

**Status:** MITIGATED

**Pattern:** Replay or handoff events start computing closure, budgets,
frontiers, source acquisition, candidate selection, or receipt validation that
belongs to FLETCH, MDCROP, or LATTICE.

**Domain:** core fixtures, context operations, handoff records, display reports,
and future scenario examples.

**Detection difficulty:** Displaying LATTICE-shaped facts can look like owning
their semantics unless the boundary is explicit in fields and docs.

**Structural solution:** Keep WITNESS as capture/replay/handoff surface and
route semantic ownership through the context-boundary reviewer.

**Evidence:** `README.md`, `.roles/parliament/context-boundary-reviewer.md`,
and `crates/witness-core/src/lib.rs`.

## WITPUB-PF-03: Provider Projection Loss Is Silent

**Status:** MITIGATED

**Pattern:** Future provider adapters normalize sessions into public events
without declaring unsupported provider behavior, missing fields, redactions,
ordering loss, or fidelity gaps.

**Domain:** planned provider projection examples, compatibility reports,
fidelity-loss reports, safe adapters, and public claims.

**Detection difficulty:** A normalized event stream can replay cleanly while
concealing the provider details that were dropped or approximated.

**Structural solution:** Public WITNESS now exposes a synthetic
`witness.provider-projection.v1` report through `witness-cli
provider-projection --json`, with fixture-backed unsupported-behavior,
redaction, ordering-loss, and fidelity-gap rows before accepting live or adapter
claims.

**Evidence:** `docs/PROVIDER_PROJECTION.md`, `crates/witness-core/src/lib.rs`,
`crates/witness-cli/src/main.rs`, `crates/witness-cli/tests/proof_surface.rs`,
`tests/check-provider-projection-boundary.ps1`, `PRODUCT_PLAN.md`,
`context/waves/PHASES.md`, and `.roles/parliament/provider-portability-reviewer.md`.

## WITPUB-PF-04: CLI Machine Output Lags Core Fixtures

**Status:** MITIGATED

**Pattern:** Core fixtures expose structured JSON internally, but the public CLI
only emits human text, forcing agents and tracker scripts to scrape output.

**Domain:** `witness-cli`, proof fixtures, README quick start, status/replay
smokes, and downstream automation.

**Detection difficulty:** Human CLI output passes smoke tests until another
tool needs stable machine-readable replay evidence.

**Structural solution:** Add retained `--json` proof coverage for public
`status` and `replay` commands while preserving text output.

**Evidence:** PITFALL adoption added `witness-cli status --json`,
`witness-cli replay --json`, README examples, and proof-fixture assertions.

## WITPUB-PF-05: Public Readiness Outruns Proof Surface

**Status:** MITIGATED

**Pattern:** The public core is described as a general harness or customer-ready
integration before schemas, compatibility, provider projection, safety gates,
and retained CLI proof support the claim.

**Domain:** README, product plan, public/private promotion, provider adapters,
customer language, and phase tracking.

**Detection difficulty:** The event model and fixture surface are broad enough
to look more mature than the minimal public CLI and active public-core phase.

**Structural solution:** Keep status language scoped to early public core,
track schema stabilization and safe adapters as planned phases, and require
proof-backed promotion before customer or provider-readiness claims.

**Evidence:** `README.md`, `PRODUCT_PLAN.md`, `context/waves/PHASES.md`, and
`context/waves/2026-07-20-public-core/WAVE.md`.
