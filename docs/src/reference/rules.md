# ArchiMate 3.2 Rules

The authoritative ruleset lives in [`docs/SPEC.md`](https://github.com/haquiticos/archr/blob/main/docs/SPEC.md), generated from the Rust source in `crates/archr-core/src/{model,validate}.rs`. This page is a quick summary.

## At a glance

- **8** layers, **62** element kinds, **11** relationship types
- `ALLOWED` matrix: **203** `(source_layer, relation_kind, target_layer)` triples, encoded as a `const` slice in `validate.rs`
- Validation is **data-driven** — no hardcoded `match` arms; the runtime looks up the matrix directly

## Layers

| Layer | Example elements |
|-------|------------------|
| Motivation | Goal, Requirement, Driver, Stakeholder |
| Strategy | Resource, Capability, ValueStream, CourseOfAction |
| Business | BusinessActor, BusinessProcess, BusinessService, Product |
| Application | ApplicationComponent, ApplicationService, DataObject |
| Technology | Node, Device, Artifact, CommunicationNetwork |
| Physical | Equipment, Facility, Material, DistributionNetwork |
| Implementation | WorkPackage, Deliverable, Plateau, Gap |
| Other | Grouping, Location, Junctions |

## Relationship constraints (summary)

| Relationship | Rule |
|-------------|------|
| Composition, Aggregation, Assignment | Same layer |
| Realization | Same layer + cross-layer (App→Business, Tech→App, etc.) |
| Serving | Descending: Tech→App→Business→Strategy |
| Access | Application ↔ Technology, Application ↔ Business |
| Association, Influence | Any layer to any layer |
| Triggering, Flow | Same layer |
| Specialization | Same layer |
| Motivation → Core | Only Association |

For the full detailed matrix, read [SPEC.md](https://github.com/haquiticos/archr/blob/main/docs/SPEC.md).
