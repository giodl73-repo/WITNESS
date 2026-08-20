# WITNESS Product Plan

## Thesis

AI harnesses need a durable, provider-neutral event model for context changes,
tool activity, validation, checkpoints, and replay. WITNESS makes those
transitions explicit and prepares them for LATTICE-backed semantic closure.

## Public scope

- Typed harness events and response deltas.
- Checkpoint and replay contracts.
- Product-neutral LATTICE handoff records.
- Deterministic fixtures and a minimal CLI.

## Non-goals

- No private session ingestion.
- No credentials or raw provider transcripts.
- No provider orchestration in the core.
- No organization-specific product positioning.
- No reimplementation of LATTICE semantics.

## Near-term work

1. Stabilize event and handoff schemas.
2. Add JSON output to the public CLI.
3. Add fixture-backed provider projection examples.
4. Define compatibility and fidelity-loss reports.
5. Add optional live adapters only after safety and consent review.
