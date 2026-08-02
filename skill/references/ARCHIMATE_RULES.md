# ArchiMate 3.2 Derivability Rules (Legacy)

> ⚠️ **Deprecated**: This document is now a redirect to the authoritative spec.
>
> For the current, generated reference, see:
> - **[docs/SPEC.md](../docs/SPEC.md)** — Single source of truth generated from archr's Rust code
> - **[examples/gen_spec.py](../../examples/gen_spec.py)** — Script to regenerate the spec

## Legacy Information (Out of Date)

These rules are **out of sync** with the current implementation in `validate.rs::ALLOWED`. Do not use them for validation.

### Historical Motivation Rules (Incorrect)

The old documentation claimed:
> "**Motivation** can only relate to **Core** layers (Business, Application, Technology) using **Association**"

This was **incorrect**. The actual implementation in `validate.rs::ALLOWED` allows `Association` between any layers, including:
- Motivation → Physical
- Motivation → Implementation
- Motivation → Other

### Historical Core Layer Claims (Incomplete)

The old documentation claimed:
> "Business → Application (all 11)"

This was **incomplete**. The actual implementation only allows:
- Business → Application: Realization (and Association)

Other structural relationships between Core layers are limited by the ALLOWED matrix.

### Correct Rules (From validate.rs)

The correct derivability rules are documented in `docs/SPEC.md` and implemented in `validate.rs::ALLOWED`:

| Relationship | Allowed Source Layer | Allowed Target Layer |
|--------------|---------------------|---------------------|
| Composition | Composite element (any layer) | Any element (any layer) |
| Aggregation | Aggregator (any layer) | Aggregated element (any layer) |
| Assignment | BusinessActor (Business) | BusinessFunction (Business) |
| Realization | ApplicationComponent (Application) | BusinessFunction, BusinessProcess (Business) |
| Serving | BusinessService (Business) | BusinessFunction (Business) |
| Access | ApplicationComponent (Application) | DataObject (Application) |
| Influence | Motivation (Motivation) | Same layer only |
| Association | Any element | Any element |
| Triggering | Triggering element (any layer) | Triggered element (any layer) |
| Flow | Flowing element (any layer) | Flowing element (any layer) |
| Specialization | Specializing element (any layer) | Specialized element (any layer) |

## References

- **[docs/SPEC.md](../docs/SPEC.md)** — Authoritative, auto-generated spec
- [validate.rs](../../crates/archr-core/src/validate.rs) — Implementation of derivability rules
- [model.rs](../../crates/archr-core/src/model.rs) — ElementKind and ElementLayer definitions
