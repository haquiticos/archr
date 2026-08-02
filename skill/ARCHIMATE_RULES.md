# ArchiMate Rules

**Auto-generated from archr implementation**  
**Last generated:** 2026-08-02  
**Source:** [docs/SPEC.md](../docs/SPEC.md)

This file redirects to the authoritative specification generated from the archr Rust code.

## Quick Reference

- **Layers:** 8 (Motivation, Strategy, Business, Application, Technology, Physical, Implementation, Other)
- **Elements:** 61 (excluding Junction)
- **Relationships:** 11 (Composition, Aggregation, Assignment, Realization, Serving, Access, Influence, Association, Triggering, Flow, Specialization)

## For Implementation Details

See the complete auto-generated specification at:
- [docs/SPEC.md](../docs/SPEC.md) — Full specification
- [crates/archr-core/src/validate.rs](../crates/archr-core/src/validate.rs) — Rule implementation

## Regenerating

To regenerate this spec from the Rust code:

```bash
python3 gen_spec.py
```

Or using Rust:

```bash
cargo run -- gen_spec
```
