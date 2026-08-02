#!/usr/bin/env python3
"""Generate docs/SPEC.md from archr's Rust source.

Single source of truth: the spec is parsed directly from
  - crates/archr-core/src/model.rs     (element kinds, layers, variants)
  - crates/archr-core/src/validate.rs  (ALLOWED derivability matrix)
  - crates/archr-core/src/io/xml.rs     (namespace / version)

Run: python3 gen_spec.py

WARNING: Do NOT edit docs/SPEC.md by hand. Any structural change to the
metamodel or derivability rules MUST be made in the Rust code and
regenerated here. If the regeneration in CI diffs docs/SPEC.md, the
build fails — that is the safety net.
"""

from __future__ import annotations

import os
import re
import sys
from collections import defaultdict
from typing import Dict, List, Set, Tuple

ROOT = os.path.dirname(os.path.abspath(__file__))
MODEL_RS = os.path.join(ROOT, "crates", "archr-core", "src", "model.rs")
VALIDATE_RS = os.path.join(ROOT, "crates", "archr-core", "src", "validate.rs")
XML_RS = os.path.join(ROOT, "crates", "archr-core", "src", "io", "xml.rs")
OUTPUT = os.path.join(ROOT, "docs", "SPEC.md")

# Canonical ordering for layers, displayed everywhere consistently.
LAYER_ORDER = [
    "Motivation", "Strategy", "Business", "Application",
    "Technology", "Physical", "Implementation", "Other",
]

# Human-readable display name for "Implementation & Migration" layer.
LAYER_DISPLAY = {
    "Motivation": "Motivation",
    "Strategy": "Strategy",
    "Business": "Business",
    "Application": "Application",
    "Technology": "Technology",
    "Physical": "Physical",
    "Implementation": "Implementation & Migration",
    "Other": "Other",
}

REL_CATEGORY = {
    "Composition": "Structural",
    "Aggregation": "Structural",
    "Assignment": "Structural",
    "Realization": "Structural",
    "Access": "Dependency",
    "Serving": "Dependency",
    "Influence": "Dependency",
    "Association": "Dependency",
    "Triggering": "Dynamic",
    "Flow": "Dynamic",
    "Specialization": "Other",
}

REL_ORDER = [
    "Composition", "Aggregation", "Assignment", "Realization",
    "Access", "Serving", "Influence", "Association",
    "Triggering", "Flow", "Specialization",
]


def _read(path: str) -> str:
    with open(path, "r", encoding="utf-8") as f:
        return f.read()

def extract_layer_mapping() -> Tuple[Dict[str, List[str]], int]:
    """Parse `ElementKind::layer()` and `(layer, [kinds])` mapping.

    Returns (layers_ordered, total_element_count). Raises RuntimeError if the
    match is missing or variants cannot be attributed to a layer.
    """
    src = _read(MODEL_RS)
    m = re.search(
        r"pub const fn layer\(self\) -> ElementLayer \{(.+?)\n    \}",
        src, re.S,
    )
    if not m:
        raise RuntimeError("Could not locate ElementKind::layer() in model.rs")
    body = m.group(1)

    # Each clause: `// Optional comment\n <variants> => ElementLayer::Name,`
    clause = re.compile(
        r"(?://\s*([^\n]+)\n)?\s*([A-Za-z][\w\s|]*?)\s*=>\s*ElementLayer::(\w+)",
        re.M,
    )

    layers: Dict[str, List[str]] = defaultdict(list)
    seen: Set[str] = set()
    for mo in clause.finditer(body):
        variants_blob, layer = mo.group(2), mo.group(3)
        for name in variants_blob.split("|"):
            name = name.strip()
            if not name:
                continue
            if name in seen:
                raise RuntimeError(f"Duplicate ElementKind variant: {name}")
            seen.add(name)
            layers[layer].append(name)

    # Reorder per canonical LAYER_ORDER.
    ordered = {k: layers.get(k, []) for k in LAYER_ORDER if k in layers}
    total = sum(len(v) for v in ordered.values())
    return ordered, total


def extract_variant_counts() -> Tuple[int, int]:
    """Read the const VARIANT_COUNT assertions for ElementKind and RelationKind.

    There are two `pub const VARIANT_COUNT: usize = N;` declarations — one in
    `impl ElementKind`, one in `impl RelationKind`. We pick them apart by the
    `type_name` that immediately precedes each impl block.
    """
    src = _read(MODEL_RS)
    # Find every `pub const VARIANT_COUNT: usize = N;` occurrence.
    counts = [int(m.group(1))
              for m in re.finditer(
                  r"pub const VARIANT_COUNT:\s*usize\s*=\s*(\d+)", src)]
    if len(counts) < 2:
        raise RuntimeError(
            "Could not find both VARIANT_COUNT constants in model.rs "
            f"(found {len(counts)})"
        )
    # The first declaration belongs to ElementKind, the second to RelationKind
    # (their impl blocks appear in that order in model.rs).
    return counts[0], counts[1]


def extract_allowed() -> Dict[str, Set[Tuple[str, str]]]:
    """Parse the `ALLOWED` const slice in validate.rs.

    Returns {relation_kind: {(source_layer, target_layer), ...}}.
    """
    src = _read(VALIDATE_RS)
    start = src.index("const ALLOWED:")
    end = src.index("\n];", start)
    block = src[start:end]
    pat = re.compile(
        r"\(\s*ElementLayer::(\w+),\s*RelationKind::(\w+),\s*ElementLayer::(\w+)\s*,?\s*\)"
    )
    by_rel: Dict[str, Set[Tuple[str, str]]] = defaultdict(set)
    found = False
    for mo in pat.finditer(block):
        found = True
        by_rel[mo.group(2)].add((mo.group(1), mo.group(3)))
    if not found:
        raise RuntimeError("ALLOWED matrix in validate.rs is empty or unreadable")
    return by_rel


def extract_xml_metadata() -> Tuple[str, str]:
    """Return (namespace, version) scraped from xml.rs.

    The values live inside Rust `writeln!`/`write!` macros as escaped
    backslash-quoted attributes (e.g. `name=\\\"...\\\"`). We tolerate
    either backslash-escaped or plain quotes.
    """
    src = _read(XML_RS)
    # Match the namespace URL whether wrapped by `\"`, `\\\"`, or backticks.
    ns = re.search(r'(?:\\["`])?(http://www\.archimatetool\.com/archimate)(?:\\["`])?',
                   src)
    # Match version=\\\"N.M.N\\\" — only digits/dots, on the `version=` attr.
    ver = re.search(r'version\s*=\s*\\?"(\d+\.\d+\.\d+)\\?"', src)
    return (ns.group(1) if ns else "unknown",
            ver.group(1) if ver else "unknown")


def _describe_rel(rel: str, pairs: Set[Tuple[str, str]]) -> str:
    if len(pairs) == LAYER_ORDER.__len__() ** 2:
        return "Any layer → any layer (fully permissive)"
    if rel in ("Composition", "Aggregation", "Assignment",
               "Triggering", "Flow", "Specialization"):
        return "Same layer only"
    if rel == "Realization":
        # Same layer + upward cross-layer
        return ("Same layer; upward crossing (lower layer realizes higher "
                "layer): Implementation→{Strategy,Business,Application,"
                "Technology,Physical}, Technology→{Application,Business}, "
                "Application→Business")
    if rel == "Serving":
        return ("Descending chain Physical→Technology, Technology→{Application,"
                "Business}, Application→{Business,Strategy}, Business→Strategy")
    if rel == "Access":
        return ("Bidirectional: Application↔Technology, "
                "Application↔Business, Application↔Application")
    if rel == "Influence":
        return "Any layer → any layer (fully permissive)"
    return ""


def generate_spec() -> str:
    layers, total_elements = extract_layer_mapping()
    ek_count, rk_count = extract_variant_counts()
    by_rel = extract_allowed()
    namespace, version = extract_xml_metadata()

    # Cross-check: layers' count must equal the declared VARIANT_COUNT.
    if total_elements != ek_count:
        raise RuntimeError(
            f"Element count mismatch: layer() yields {total_elements} but "
            f"VARIANT_COUNT is {ek_count}"
        )
    if set(by_rel) != set(REL_ORDER):
        raise RuntimeError(
            f"ALLOWED relation kinds {sorted(by_rel)} differ from "
            f"canonical set {sorted(REL_ORDER)}"
        )
    actual_rel_count = len(by_rel)
    if actual_rel_count != rk_count:
        raise RuntimeError(
            f"ALLOWED has {actual_rel_count} relation kinds but "
            f"RelationKind::VARIANT_COUNT is {rk_count}"
        )

    out: List[str] = []

    out.append("# ArchiMate 3.2 Compatibility Specification\n\n")
    out.append(
        "**Single source of truth:** generated from `crates/archr-core/src`.\n\n"
    )
    out.append("**License:** MIT (compatible with Archi's MIT license).\n\n")
    out.append(
        "**Reference:** ArchiMate 3.2 Specification (The Open Group, C193).\n\n"
    )
    out.append(
        "> ⚠️ Do **not** edit this file by hand. Run `python3 gen_spec.py` to "
        "regenerate; CI rejects a stale spec. Any metamodel or derivability "
        "change must be made in `model.rs` / `validate.rs` first.\n\n"
    )

    # ── Element Layers ────────────────────────────────────────────────────
    out.append("## Element Layers\n\n")
    out.append(
        f"The `archr` engine implements **{len(layers)}** layers as defined "
        f"in ArchiMate 3.2, totalling **{total_elements}** element kinds.\n\n"
    )
    for layer in LAYER_ORDER:
        if layer not in layers:
            continue
        names = layers[layer]
        display = LAYER_DISPLAY[layer]
        out.append(f"### {display} Layer ({len(names)} elements)\n\n")
        for name in names:
            out.append(f"- `{name}`\n")
        out.append("\n")

    # ── Relationship Types ───────────────────────────────────────────────
    out.append("---\n\n## Relationship Types\n\n")
    out.append(
        f"`archr` implements **{actual_rel_count}** relationship types with "
        "derivability rules.\n\n"
    )
    by_cat: Dict[str, List[str]] = defaultdict(list)
    for rel in REL_ORDER:
        by_cat[REL_CATEGORY[rel]].append(rel)
    for cat in ("Structural", "Dependency", "Dynamic", "Other"):
        rels = by_cat.get(cat, [])
        out.append(f"### {cat} ({len(rels)} relations)\n\n")
        for rel in rels:
            out.append(f"- `{rel}`\n")
        out.append("\n")

    # ── Derivability Rules ───────────────────────────────────────────────
    out.append("---\n\n## Derivability Rules (`ALLOWED` Matrix)\n\n")
    out.append(
        "Rules are encoded in `validate.rs::ALLOWED` as a const slice of "
        "`(source_layer, relation_kind, target_layer)` triples — the runtime "
        "validator looks them up directly.\n\n"
    )

    # First a per-relation summary table (clean prose, no broken cells).
    out.append("### Summary\n\n")
    out.append("| Relationship | Category | Allowed (source → target) | Count |\n")
    out.append("|--------------|----------|----------------------------|-------|\n")
    for rel in REL_ORDER:
        pairs = sorted(by_rel[rel])
        if len(pairs) == len(LAYER_ORDER) ** 2:
            cells = "any → any"
        elif rel in ("Composition", "Aggregation", "Assignment",
                     "Triggering", "Flow", "Specialization"):
            cells = "same layer"
        else:
            cells = ", ".join(f"{s} → {t}" for s, t in pairs)
        out.append(f"| `{rel}` | {REL_CATEGORY[rel]} | {cells} | "
                   f"{len(pairs)} |\n")
    out.append("\n")

    # Then the full enumerated matrix, one section per relation.
    out.append("### Detailed Matrix\n\n")
    for rel in REL_ORDER:
        pairs = sorted(by_rel[rel])
        out.append(f"#### `{rel}` ({REL_CATEGORY[rel]})\n\n")
        out.append(_describe_rel(rel, set(pairs)) + "\n\n")
        if rel in ("Composition", "Aggregation", "Assignment",
                   "Triggering", "Flow", "Specialization"):
            out.append("| Source Layer | Target Layer |\n")
            out.append("|--------------|--------------|\n")
            for s, t in pairs:
                out.append(f"| {s} | {t} |\n")
        else:
            out.append("| Source Layer | Target Layer |\n")
            out.append("|--------------|--------------|\n")
            for s, t in pairs:
                out.append(f"| {s} | {t} |\n")
            if len(pairs) == len(LAYER_ORDER) ** 2:
                out.append("\n_All 8×8 layer combinations are permitted._\n")
        out.append("\n")

    # ── Implementation Details ───────────────────────────────────────────
    out.append("---\n\n## Implementation Details\n\n")
    out.append("### XML Format\n\n")
    out.append(f"- XML Namespace: `{namespace}`\n")
    out.append(
        f"- Version Attribute: `{version}` (Archi native format, "
        "forward-compatible)\n"
    )
    out.append("- File Extension: `.archimate`\n\n")

    out.append("### Element & Relationship Counts\n\n")
    out.append(f"- Total Elements: **{total_elements}** "
               f"(matches `ElementKind::VARIANT_COUNT = {ek_count}`)\n")
    out.append(f"- Total Layers: **{len(layers)}**\n")
    out.append(f"- Total Relationships: **{actual_rel_count}** "
               f"(matches `RelationKind::VARIANT_COUNT = {rk_count}`)\n")
    out.append(f"- `ALLOWED` matrix size: **"
               f"{sum(len(v) for v in by_rel.values())}** triples\n\n")

    # ── Limitations ──────────────────────────────────────────────────────
    out.append("---\n\n## Limitations\n\n")
    out.append(
        "`archr` implements a subset of ArchiMate 3.2 semantics:\n\n"
    )
    out.append(
        "- **Composition exclusivity:** full compositional exclusivity is not "
        "enforced — composition graphs may share children.\n"
    )
    out.append(
        "- **Strategy abstraction:** the Strategy layer is treated as a "
        "separate layer rather than a mixin hierarchy.\n"
    )
    out.append(
        "- **Motivation semantics:** full causal reasoning and goal/value "
        "dependencies are not modeled.\n"
    )
    out.append(
        "- **Physical abstraction:** the Physical layer is treated as separate "
        "from the Technology layer.\n"
    )
    out.append(
        "- **XML dialects:** only Archi native XML (`.archimate`) is "
        "supported for read/write; the Open Group Exchange File (`.model`) "
        "format is not parsed.\n\n"
    )

    # ── References ───────────────────────────────────────────────────────
    out.append("---\n\n## References\n\n")
    out.append(
        "- [ArchiMate 3.2 Specification](https://www.opengroup.org/publications/catalog/C193) "
        "(The Open Group)\n"
    )
    out.append(
        "- [Open Group ArchiMate Exchange File Format]"
        "(https://www.opengroup.org/xsd/archimate/) (XSD and samples)\n"
    )
    out.append(
        "- [Archi Tool Repository](https://github.com/archimatetool/archi) "
        "(MIT-licensed metamodel)\n"
    )
    out.append("\n")

    return "".join(out)


def main() -> int:
    try:
        spec = generate_spec()
    except RuntimeError as exc:
        print(f"❌ Spec generation failed: {exc}", file=sys.stderr)
        return 1

    os.makedirs(os.path.dirname(OUTPUT), exist_ok=True)
    with open(OUTPUT, "w", encoding="utf-8") as f:
        f.write(spec)

    layers, total = extract_layer_mapping()
    by_rel = extract_allowed()
    print("✅ Generated docs/SPEC.md")
    print(f"   Elements: {total}")
    print(f"   Layers:   {len(layers)}")
    print(f"   Relations: {len(by_rel)}")
    print(f"   ALLOWED triples: {sum(len(v) for v in by_rel.values())}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
