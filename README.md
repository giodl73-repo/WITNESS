# WITNESS

**Replayable AI harness event and context-control contracts.**

WITNESS records user turns, source reads, edits, validation obligations,
checkpoints, context deltas, and context operations as typed events. Those
events can be replayed and handed to
[LATTICE](https://github.com/giodl73-repo/LATTICE) for deterministic context
closure and receipt generation.

This public edition contains the product-neutral event model, deterministic
fixtures, and a small CLI. It does not contain private provider sessions,
customer data, funding material, or organization-specific integrations.

See [`docs/MAINTENANCE.md`](docs/MAINTENANCE.md) for the public/private
promotion and compatibility boundary.

## Context & Harness family

WITNESS is the capture-and-replay layer in a four-stage context-control family:

```text
Sources → FLETCH → MDCROP → LATTICE → WITNESS
           fetch     select     close       replay
```

| Repo | Responsibility |
|------|----------------|
| [FLETCH](https://github.com/giodl73-repo/FLETCH) | Acquire, verify, cache, partition, and bundle source material. |
| [MDCROP](https://github.com/giodl73-repo/MDCROP) | Index and select bounded, provenance-aware candidate context. |
| [LATTICE](https://github.com/giodl73-repo/LATTICE) | Apply closure, meet/join, budgets, frontiers, packs, and receipts. |
| **WITNESS** | Capture harness events, checkpoints, context deltas, and deterministic replay. |

WITNESS records how context is used. It does not own acquisition, candidate
selection, or semantic closure.

## Quick start

```powershell
cargo run -p witness-cli -- status
cargo run -p witness-cli -- replay
```

## Crates

| Crate | Role |
|---|---|
| `witness-core` | Typed harness events, deltas, checkpoints, handoff records, and deterministic fixtures. |
| `witness-cli` | Minimal public status and replay front door. |

## Design principles

- Capture durable events rather than treating raw chat logs as truth.
- Keep provider-specific adapters outside the core event model.
- Make validation obligations and unresolved frontiers explicit.
- Preserve enough information for deterministic replay and audit.
- Let LATTICE own semantic closure; WITNESS owns harness capture and projection.

## Development

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

### Retained CLI proof

```powershell
cargo test -p witness-cli --test proof_surface
```

The fixture records an accepted deterministic replay with seven events and a
structured exit-code 2 usage failure for an unsupported command.

## Status

This is an early public core extracted from a larger incubation codebase.
Provider adapters and live-session capture are intentionally out of scope for
the first public release.

## License

MIT. See [`LICENSE`](LICENSE).
