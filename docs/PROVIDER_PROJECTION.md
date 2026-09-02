# Provider Projection Compatibility

This public-core report mitigates `WITPUB-PF-03`.

Provider adapters may normalize sessions into WITNESS public events only when
they declare what was dropped, redacted, approximated, or left outside the
neutral event core. A clean replay is not enough evidence of compatibility.

## Public Fixture

`witness-cli provider-projection --json` emits
`witness.provider-projection.v1` from a synthetic fixture. It performs no live
provider call and contains no private transcript, customer data, credentials, or
provider-native hidden context.

The report names:

- unsupported provider behavior;
- redactions;
- ordering loss;
- fidelity gaps;
- compatibility status;
- adapter-owned repair path.

## Acceptance Boundary

A future adapter may claim public compatibility only after it produces a
provider projection report with explicit loss categories. Provider-specific
fields stay in the adapter until multiple public consumers prove they belong in
the WITNESS core contract.
