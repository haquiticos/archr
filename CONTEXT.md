# archr — Domain Glossary

Domain terms for the headless ArchiMate 3.2 engine (`crates/archr-core`). Use these
names in code, docs, and architecture discussions.

## Model

The root container (`model::Model`). Owns all Elements and Relationships in an
arena (`Vec` + typed indices) and the Model's Viewpoints. The Model is
self-describing: it carries every fact an adapter needs to serialize, so no
conversion thread side-channel data around it.

## Element / Relationship

A node / directed edge of the ArchiMate graph. Each carries:

- **arena index** — `ElementId`/`RelationId`, a typed `usize` position for O(1)
  traversal. Internal to the Model; never serialized.
- **original id** — the identifier as written in the source file (YAML `id`,
  XML `id`). Preserved verbatim through any conversion; models built
  programmatically synthesize `e_N` / `r_N`. This is what round-trip fidelity
  means: same original ids in, same original ids out.

## Viewpoint

A named scope of Elements and Relationships (`model::ViewpointDefinition`) with
an Archi **viewpoint kind** (`model::ViewpointKind`: business, application,
implementation, motivation, compliance, none), rendered as one diagram per
viewpoint. Domain type owned by the Model; both adapters (YAML, XML) read and
write it. A diagram without a `viewpoint` attribute is an anonymous view and
carries no Viewpoint state.

## Adapters

- **YAML adapter** (`io/yaml`) — archr's canonical input format, with schema
  validation (`SchemaError`) and accumulated errors.
- **XML adapter** (`io/xml`) — Archi native `.archimate` (Open Exchange-style)
  for interop with the Archi tool.

CLI verbs: `validate` (YAML → rules), `generate` (YAML → XML), `parse`
(XML → YAML), `diff` (existing XML vs new YAML, matched by element name).
