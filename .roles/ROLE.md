# WITNESS Review Roles

Public changes are reviewed through four linked lenses:

- [Harness Runtime Reviewer](parliament/harness-runtime-reviewer.md) - event completeness, replay, and checkpoint behavior.
- [Context Boundary Reviewer](parliament/context-boundary-reviewer.md) - clean separation between WITNESS and LATTICE.
- [Session Safety Reviewer](stakeholders/session-safety-reviewer.md) - consent, credentials, privacy, and retention.
- [Provider Portability Reviewer](parliament/provider-portability-reviewer.md) - neutral core contracts and explicit fidelity loss.

## Productive tensions

| Pulls | Against | Because |
|---|---|---|
| Harness Runtime Reviewer | Session Safety Reviewer | Complete replay evidence can capture data that should not be retained or published. |
| Context Boundary Reviewer | Harness Runtime Reviewer | A convenient replay event can absorb closure semantics that belong to LATTICE. |
| Provider Portability Reviewer | Harness Runtime Reviewer | A neutral event model can omit provider detail needed for faithful replay. |

Safety and credential exposure block publication first. Boundary violations block the core
contract next. Resolve fidelity disputes with accepted and structured-failure replay fixtures,
recording any provider-specific loss rather than silently extending the neutral model.

## Review order

1. Use Session Safety Reviewer before accepting fixtures or capture fields.
2. Use Context Boundary Reviewer and Provider Portability Reviewer for event-schema changes.
3. Use Harness Runtime Reviewer to close replay, checkpoint, and CLI proof.
