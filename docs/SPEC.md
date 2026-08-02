# ArchiMate 3.2 Compatibility Specification

**Single source of truth:** generated from `crates/archr-core/src`.

**License:** MIT (compatible with Archi's MIT license).

**Reference:** ArchiMate 3.2 Specification (The Open Group, C193).

> ⚠️ Do **not** edit this file by hand. Run `python3 gen_spec.py` to regenerate; CI rejects a stale spec. Any metamodel or derivability change must be made in `model.rs` / `validate.rs` first.

## Element Layers

The `archr` engine implements **8** layers as defined in ArchiMate 3.2, totalling **61** element kinds.

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

`archr` implements **11** relationship types with derivability rules.

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

## Derivability Rules (`ALLOWED` Matrix)

Rules are encoded in `validate.rs::ALLOWED` as a const slice of `(source_layer, relation_kind, target_layer)` triples — the runtime validator looks them up directly.

### Summary

| Relationship | Category | Allowed (source → target) | Count |
|--------------|----------|----------------------------|-------|
| `Composition` | Structural | same layer | 8 |
| `Aggregation` | Structural | same layer | 8 |
| `Assignment` | Structural | same layer | 8 |
| `Realization` | Structural | Application → Application, Application → Business, Business → Business, Implementation → Application, Implementation → Business, Implementation → Implementation, Implementation → Physical, Implementation → Strategy, Implementation → Technology, Motivation → Motivation, Other → Other, Physical → Physical, Strategy → Strategy, Technology → Application, Technology → Business, Technology → Technology | 16 |
| `Access` | Dependency | Application → Application, Application → Business, Application → Technology, Business → Application, Technology → Application | 5 |
| `Serving` | Dependency | Application → Business, Application → Strategy, Business → Strategy, Physical → Technology, Technology → Application, Technology → Business | 6 |
| `Influence` | Dependency | any → any | 64 |
| `Association` | Dependency | any → any | 64 |
| `Triggering` | Dynamic | same layer | 8 |
| `Flow` | Dynamic | same layer | 8 |
| `Specialization` | Other | same layer | 8 |

### Detailed Matrix

#### `Composition` (Structural)

Same layer only

| Source Layer | Target Layer |
|--------------|--------------|
| Application | Application |
| Business | Business |
| Implementation | Implementation |
| Motivation | Motivation |
| Other | Other |
| Physical | Physical |
| Strategy | Strategy |
| Technology | Technology |

#### `Aggregation` (Structural)

Same layer only

| Source Layer | Target Layer |
|--------------|--------------|
| Application | Application |
| Business | Business |
| Implementation | Implementation |
| Motivation | Motivation |
| Other | Other |
| Physical | Physical |
| Strategy | Strategy |
| Technology | Technology |

#### `Assignment` (Structural)

Same layer only

| Source Layer | Target Layer |
|--------------|--------------|
| Application | Application |
| Business | Business |
| Implementation | Implementation |
| Motivation | Motivation |
| Other | Other |
| Physical | Physical |
| Strategy | Strategy |
| Technology | Technology |

#### `Realization` (Structural)

Same layer; upward crossing (lower layer realizes higher layer): Implementation→{Strategy,Business,Application,Technology,Physical}, Technology→{Application,Business}, Application→Business

| Source Layer | Target Layer |
|--------------|--------------|
| Application | Application |
| Application | Business |
| Business | Business |
| Implementation | Application |
| Implementation | Business |
| Implementation | Implementation |
| Implementation | Physical |
| Implementation | Strategy |
| Implementation | Technology |
| Motivation | Motivation |
| Other | Other |
| Physical | Physical |
| Strategy | Strategy |
| Technology | Application |
| Technology | Business |
| Technology | Technology |

#### `Access` (Dependency)

Bidirectional: Application↔Technology, Application↔Business, Application↔Application

| Source Layer | Target Layer |
|--------------|--------------|
| Application | Application |
| Application | Business |
| Application | Technology |
| Business | Application |
| Technology | Application |

#### `Serving` (Dependency)

Descending chain Physical→Technology, Technology→{Application,Business}, Application→{Business,Strategy}, Business→Strategy

| Source Layer | Target Layer |
|--------------|--------------|
| Application | Business |
| Application | Strategy |
| Business | Strategy |
| Physical | Technology |
| Technology | Application |
| Technology | Business |

#### `Influence` (Dependency)

Any layer → any layer (fully permissive)

| Source Layer | Target Layer |
|--------------|--------------|
| Application | Application |
| Application | Business |
| Application | Implementation |
| Application | Motivation |
| Application | Other |
| Application | Physical |
| Application | Strategy |
| Application | Technology |
| Business | Application |
| Business | Business |
| Business | Implementation |
| Business | Motivation |
| Business | Other |
| Business | Physical |
| Business | Strategy |
| Business | Technology |
| Implementation | Application |
| Implementation | Business |
| Implementation | Implementation |
| Implementation | Motivation |
| Implementation | Other |
| Implementation | Physical |
| Implementation | Strategy |
| Implementation | Technology |
| Motivation | Application |
| Motivation | Business |
| Motivation | Implementation |
| Motivation | Motivation |
| Motivation | Other |
| Motivation | Physical |
| Motivation | Strategy |
| Motivation | Technology |
| Other | Application |
| Other | Business |
| Other | Implementation |
| Other | Motivation |
| Other | Other |
| Other | Physical |
| Other | Strategy |
| Other | Technology |
| Physical | Application |
| Physical | Business |
| Physical | Implementation |
| Physical | Motivation |
| Physical | Other |
| Physical | Physical |
| Physical | Strategy |
| Physical | Technology |
| Strategy | Application |
| Strategy | Business |
| Strategy | Implementation |
| Strategy | Motivation |
| Strategy | Other |
| Strategy | Physical |
| Strategy | Strategy |
| Strategy | Technology |
| Technology | Application |
| Technology | Business |
| Technology | Implementation |
| Technology | Motivation |
| Technology | Other |
| Technology | Physical |
| Technology | Strategy |
| Technology | Technology |

_All 8×8 layer combinations are permitted._

#### `Association` (Dependency)

Any layer → any layer (fully permissive)

| Source Layer | Target Layer |
|--------------|--------------|
| Application | Application |
| Application | Business |
| Application | Implementation |
| Application | Motivation |
| Application | Other |
| Application | Physical |
| Application | Strategy |
| Application | Technology |
| Business | Application |
| Business | Business |
| Business | Implementation |
| Business | Motivation |
| Business | Other |
| Business | Physical |
| Business | Strategy |
| Business | Technology |
| Implementation | Application |
| Implementation | Business |
| Implementation | Implementation |
| Implementation | Motivation |
| Implementation | Other |
| Implementation | Physical |
| Implementation | Strategy |
| Implementation | Technology |
| Motivation | Application |
| Motivation | Business |
| Motivation | Implementation |
| Motivation | Motivation |
| Motivation | Other |
| Motivation | Physical |
| Motivation | Strategy |
| Motivation | Technology |
| Other | Application |
| Other | Business |
| Other | Implementation |
| Other | Motivation |
| Other | Other |
| Other | Physical |
| Other | Strategy |
| Other | Technology |
| Physical | Application |
| Physical | Business |
| Physical | Implementation |
| Physical | Motivation |
| Physical | Other |
| Physical | Physical |
| Physical | Strategy |
| Physical | Technology |
| Strategy | Application |
| Strategy | Business |
| Strategy | Implementation |
| Strategy | Motivation |
| Strategy | Other |
| Strategy | Physical |
| Strategy | Strategy |
| Strategy | Technology |
| Technology | Application |
| Technology | Business |
| Technology | Implementation |
| Technology | Motivation |
| Technology | Other |
| Technology | Physical |
| Technology | Strategy |
| Technology | Technology |

_All 8×8 layer combinations are permitted._

#### `Triggering` (Dynamic)

Same layer only

| Source Layer | Target Layer |
|--------------|--------------|
| Application | Application |
| Business | Business |
| Implementation | Implementation |
| Motivation | Motivation |
| Other | Other |
| Physical | Physical |
| Strategy | Strategy |
| Technology | Technology |

#### `Flow` (Dynamic)

Same layer only

| Source Layer | Target Layer |
|--------------|--------------|
| Application | Application |
| Business | Business |
| Implementation | Implementation |
| Motivation | Motivation |
| Other | Other |
| Physical | Physical |
| Strategy | Strategy |
| Technology | Technology |

#### `Specialization` (Other)

Same layer only

| Source Layer | Target Layer |
|--------------|--------------|
| Application | Application |
| Business | Business |
| Implementation | Implementation |
| Motivation | Motivation |
| Other | Other |
| Physical | Physical |
| Strategy | Strategy |
| Technology | Technology |

---

## Implementation Details

### XML Format

- XML Namespace: `http://www.archimatetool.com/archimate`
- Version Attribute: `5.0.0` (Archi native format, forward-compatible)
- File Extension: `.archimate`

### Element & Relationship Counts

- Total Elements: **61** (matches `ElementKind::VARIANT_COUNT = 61`)
- Total Layers: **8**
- Total Relationships: **11** (matches `RelationKind::VARIANT_COUNT = 11`)
- `ALLOWED` matrix size: **203** triples

---

## Limitations

`archr` implements a subset of ArchiMate 3.2 semantics:

- **Composition exclusivity:** full compositional exclusivity is not enforced — composition graphs may share children.
- **Strategy abstraction:** the Strategy layer is treated as a separate layer rather than a mixin hierarchy.
- **Motivation semantics:** full causal reasoning and goal/value dependencies are not modeled.
- **Physical abstraction:** the Physical layer is treated as separate from the Technology layer.
- **XML dialects:** only Archi native XML (`.archimate`) is supported for read/write; the Open Group Exchange File (`.model`) format is not parsed.

---

## References

- [ArchiMate 3.2 Specification](https://www.opengroup.org/publications/catalog/C193) (The Open Group)
- [Open Group ArchiMate Exchange File Format](https://www.opengroup.org/xsd/archimate/) (XSD and samples)
- [Archi Tool Repository](https://github.com/archimatetool/archi) (MIT-licensed metamodel)

