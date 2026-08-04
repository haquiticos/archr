# ArchiMate Compatibility References

This directory contains external references used to validate `archr`'s conformance
with ArchiMate 3.2.

- `archimate.ecore` — Archi tool's EMF metamodel (MIT license, source:
  https://github.com/archimatetool/archi/blob/master/com.archimatetool.model/model/archimate.ecore)

**Compatibility guarantees:**
- `archr` emits XML in Archi native format (namespace:
  `http://www.archimatetool.com/archimate`, version `5.0.0`)
- `archr` uses the same 8 layers and 62 element types as ArchiMate 3.2
- `archr` implements 11 relationship types with derivability rules
  defined in `validate.rs::ALLOWED` (203 triples)

**Authoritative spec:**
- [docs/SPEC.md](../../docs/SPEC.md) — Auto-generated from `model.rs` and `validate.rs`

**Limitations:**
- `archr` does not implement full semantics (e.g., composition exclusivity)
- `archr` only reads/writes Archi native XML (`.archimate`); the Open Group
  Exchange File format (`.model`) is not parsed
