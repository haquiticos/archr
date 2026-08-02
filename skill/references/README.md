# ArchiMate Compatibility References

This directory contains external references used to validate `archr`'s conformance
with ArchiMate 3.2.

- `archimate.ecore` — Archi tool's EMF metamodel (MIT license, source: 
  https://github.com/archimatetool/archi/blob/master/com.archimatetool.model/model/archimate.ecore)
- `ARCHIMATE_RULES.md` — Manual derivability rules reference (outdated, see SPEC.md)
- `spec.md` — Auto-generated reference derived from `archr` code (single source of truth)

**Compatibility guarantees:**
- `archr` emits XML compatible with Open Exchange 3.0 format (namespace:
  http://www.archimatetool.com/archimate, version attribute can be 5.0.0)
- `archr` uses the same 8 layers and 61 element types as ArchiMate 3.2
- `archr` implements 11 relationship types with derivability rules
  defined in `validate.rs::ALLOWED`

**Limitations:**
- `archr` does not implement full semantics (e.g., composition exclusivity)
- No automatic validation against the official spec; use these references for
  manual conformance checking.
