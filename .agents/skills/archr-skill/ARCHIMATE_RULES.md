# ArchiMate Rules

**This file is a redirect.** The authoritative spec is auto-generated from
archr's Rust source code at **[docs/SPEC.md](../docs/SPEC.md)**.

## Quick Reference

- **Layers:** 8 (Motivation, Strategy, Business, Application, Technology, Physical, Implementation, Other)
- **Elements:** 62
- **Relationships:** 11 (Composition, Aggregation, Assignment, Realization, Serving, Access, Influence, Association, Triggering, Flow, Specialization)
- **Derivability matrix:** 203 `(source_layer, relation_kind, target_layer)` triples in `validate.rs::ALLOWED`

## For Implementation Details

- [docs/SPEC.md](../docs/SPEC.md) — Full specification (single source of truth)
- [crates/archr-core/src/validate.rs](../crates/archr-core/src/validate.rs) — Rule implementation
- [crates/archr-core/src/model.rs](../crates/archr-core/src/model.rs) — Element and relationship definitions

## Regenerating

```bash
python3 gen_spec.py
```

CI rejects a PR if `docs/SPEC.md` is stale relative to the code.
