# Contributing

Contributions should keep WITNESS's core provider-neutral, replayable, and
explicit about consent, safety, validation, and LATTICE handoff boundaries.

## Before opening a pull request

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --locked
cargo run -p witness-cli -- status
cargo run -p witness-cli -- replay
```

Do not commit credentials, private sessions, customer data, raw provider logs,
or organization-specific product positioning.
