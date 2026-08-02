# ArchiMate 3.2 Compatibility Specification

**Single Source of Truth:** Generated from `archr` Rust code

**License:** MIT (compatible with Archi's MIT license)

**Reference:** ArchiMate 3.2 Specification (The Open Group, C193)

## Element Layers

The `archr` engine implements 8 layers as defined in ArchiMate 3.2:

### Motivation Layer (10 elements)

- `Stakeholder`
- `Driver`
- `Assessment`
- `Goal`
- `Outcome`
- `Principle`
- `Requirement`
- `Constraint`
- `Meaning`
- `Value`

### Strategy Layer (4 elements)

- `Resource`
- `Capability`
- `ValueStream`
- `CourseOfAction`

### Business Layer (13 elements)

- `BusinessActor`
- `BusinessRole`
- `BusinessCollaboration`
- `BusinessInterface`
- `BusinessProcess`
- `BusinessFunction`
- `BusinessInteraction`
- `BusinessEvent`
- `BusinessService`
- `BusinessObject`
- `Contract`
- `Representation`
- `Product`

### Application Layer (9 elements)

- `ApplicationComponent`
- `ApplicationCollaboration`
- `ApplicationInterface`
- `ApplicationFunction`
- `ApplicationProcess`
- `ApplicationInteraction`
- `ApplicationEvent`
- `ApplicationService`
- `DataObject`

### Technology Layer (13 elements)

- `Node`
- `Device`
- `SystemSoftware`
- `TechnologyCollaboration`
- `TechnologyInterface`
- `Path`
- `CommunicationNetwork`
- `Artifact`
- `TechnologyFunction`
- `TechnologyProcess`
- `TechnologyInteraction`
- `TechnologyEvent`
- `TechnologyService`

### Physical Layer (4 elements)

- `Equipment`
- `Facility`
- `Material`
- `DistributionNetwork`

### Implementation & Migration Layer (4 elements)

- `WorkPackage`
- `Deliverable`
- `Plateau`
- `Gap`

### Other Layer (4 elements)

- `Grouping`
- `Location`
- `AndJunction`
- `OrJunction`

---

## Relationship Types

The `archr` engine implements 11 relationship types with derivability rules:

### Structural (4 relations)

- `Composition`
- `Aggregation`
- `Assignment`
- `Realization`

### Dependency (4 relations)

- `Access`
- `Serving`
- `Influence`
- `Association`

### Dynamic (2 relations)

- `Triggering`
- `Flow`

### Other (1 relations)

- `Specialization`

---

## Derivability Rules (ALLOWED Matrix)

These rules are defined in `validate.rs::ALLOWED` and validate relationships at runtime.

### Relationship → Allowed Element Pairs

| Relationship | Source Layer | Target Layer | Description |
|-------------|--------------|--------------|-------------|
| `Composition` | Structural: | any | layer |
| `Aggregation` | Structural: | same | layer only |
| `Assignment` | Business: | only | BusinessActor→BusinessFunction |
| `Realization` | N/A | N/A | any |
| `Access` | N/A | N/A | Application→Technology |
| `Serving` | N/A | N/A | Business→Application |
| `Influence` | Motivation: | same | layer only |
| `Association` | N/A | N/A | any |
| `Triggering` | Dynamic: | same | layer only |
| `Flow` | Dynamic: | same | layer only |
| `Specialization` | N/A | N/A | any |

---

## Implementation Details

### Namespace and Version

- XML Namespace: `http://www.archimatetool.com/archimate`
- Version Attribute: `5.0.0` (forward-compatible with 3.x)
- File Extension: `.model` (Model Exchange File Format)

### Element Kind Count

- Total Elements: **61** (excluding Junction, which is treated as `Other`)
- Total Layers: **8**
- Total Relationships: **11**

---

## Limitations

The `archr` engine implements a subset of ArchiMate 3.2 semantics:

- **Composition exclusivity:** Full compositional exclusivity is not enforced (composition graphs may contain cycles)
- **Strategy abstraction:** The Strategy layer is treated as a separate layer rather than a mixin hierarchy
- **Motivation semantics:** Full causal reasoning and goal/value dependencies are not modeled
- **Physical abstraction:** Physical layer is treated as separate from Technology layer

---

## References

- [ArchiMate 3.2 Specification](https://www.opengroup.org/publications/catalog/C193) (The Open Group)
- [Open Group ArchiMate Exchange File Format](https://www.opengroup.org/xsd/archimate/) (XSD and samples)
- [Archi Tool Repository](https://github.com/archimatetool/archi) (MIT licensed metamodel)

