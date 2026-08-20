# Public Core Maintenance

This repository is the canonical public home for product-neutral WITNESS
harness events, deltas, checkpoints, replay, and LATTICE handoff contracts.

## Promotion boundary

Related private incubation may explore protected provider sessions, competitive
positioning, customer scenarios, and organization-specific integrations. That
material is not part of the public core.

A change may move into this repository only when it:

1. is useful without private context;
2. contains no session, customer, employee, credential, or protected source
   material;
3. uses provider-neutral core contracts;
4. includes deterministic public fixtures and documentation;
5. passes the repository's formatting, lint, test, and CLI smoke gates.

Private history must never be merged, rebased, or force-pushed into this
repository. Promote a reviewed change as a new public commit with fresh public
authorship metadata. Public-core fixes may be ported back into private
incubation, but the public event and handoff contracts remain the compatibility
reference.
