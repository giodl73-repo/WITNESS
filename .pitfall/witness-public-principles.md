# WITNESS Public Core Principles

## WITPUB-P-01: Typed Events Beat Raw Chat

**Status:** ACTIVE

**Statement:** WITNESS public core records harness activity as typed events,
deltas, checkpoints, handoff records, and replay fixtures rather than treating
raw chat logs or provider-native state as the durable truth.

**Decision rule:** A new public fixture or CLI surface must expose the event,
checkpoint, frontier, validation, or handoff shape it preserves.

**Evidence:** `README.md`, `PRODUCT_PLAN.md`, and
`.roles/parliament/harness-runtime-reviewer.md`.

## WITPUB-P-02: LATTICE Owns Semantic Closure

**Status:** ACTIVE

**Statement:** WITNESS captures and replays context use; FLETCH acquires,
MDCROP selects, and LATTICE owns closure, budgets, frontiers, packs, and
receipts.

**Decision rule:** WITNESS may display or hand off LATTICE-owned facts, but must
not recompute acquisition, selection, closure, or semantic validation.

**Evidence:** `README.md`, `docs/MAINTENANCE.md`, and
`.roles/parliament/context-boundary-reviewer.md`.

## WITPUB-P-03: Public Promotion Requires Safety Review

**Status:** ACTIVE

**Statement:** Public-core changes must be useful without private context and
must exclude private sessions, customer data, employee data, credentials,
protected source material, and organization-specific positioning.

**Decision rule:** Any fixture or capture field promoted from private incubation
must pass the documented promotion boundary and session-safety role review.

**Evidence:** `docs/MAINTENANCE.md`, `SECURITY.md`, and
`.roles/stakeholders/session-safety-reviewer.md`.

## WITPUB-P-04: Provider Portability Requires Fidelity Labels

**Status:** ACTIVE

**Statement:** Provider-specific details belong in adapters unless repeated
public consumers prove a neutral core contract; projection loss must be visible.

**Decision rule:** Provider projection examples must name unsupported behavior,
redaction, ordering loss, or fidelity gaps before being treated as compatible.

**Evidence:** `PRODUCT_PLAN.md` and
`.roles/parliament/provider-portability-reviewer.md`.

## WITPUB-P-05: CLI Proof Is A Public Contract

**Status:** ACTIVE

**Statement:** The small public CLI is part of the reusable proof surface, so
accepted replay, JSON output, and structured usage failures must stay stable
enough for downstream smoke checks.

**Decision rule:** CLI output changes require retained proof fixture updates and
should preserve both human-readable and JSON forms unless a compatibility note
explains the change.

**Evidence:** `README.md`, `crates/witness-cli/tests/proof_surface.rs`, and
`crates/witness-cli/tests/fixtures/replay_cli_proof.txt`.
