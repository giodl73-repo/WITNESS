# Public Core Wave

## Goal

Publish the product-neutral WITNESS event and replay core without private
sessions, customer material, funding content, or organization-specific
integrations.

## Pulses

| Pulse | Status | Deliverable |
|---|---|---|
| 01 | complete | Extract the core event model and deterministic fixtures. |
| 02 | complete | Add a minimal CLI, public documentation, roles, license, and CI. |
| 03 | complete | Validate formatting, linting, tests, CLI smoke, and publication scan. |

## Validation

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --locked
cargo run -p witness-cli -- status
cargo run -p witness-cli -- replay
git diff --check
```
