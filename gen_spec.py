#!/usr/bin/env python3
"""Generates docs/SPEC.md from archr's Rust code.

This is the single source of truth for ArchiMate compatibility documentation.
Run with `python3 gen_spec.py` to regenerate the spec.

WARNING: Do NOT edit docs/SPEC.md manually. Any changes to the spec
documentation MUST be made in the source code (validate.rs and model.rs)
and regenerated here.
"""

LAYERS = [
    ("Motivation", [
        "Stakeholder", "Driver", "Assessment", "Goal", "Outcome",
        "Principle", "Requirement", "Constraint", "Meaning", "Value"
    ]),
    ("Strategy", [
        "Resource", "Capability", "ValueStream", "CourseOfAction"
    ]),
    ("Business", [
        "BusinessActor", "BusinessRole", "BusinessCollaboration", "BusinessInterface",
        "BusinessProcess", "BusinessFunction", "BusinessInteraction", "BusinessEvent",
        "BusinessService", "BusinessObject", "Contract", "Representation", "Product"
    ]),
    ("Application", [
        "ApplicationComponent", "ApplicationCollaboration", "ApplicationInterface",
        "ApplicationFunction", "ApplicationProcess", "ApplicationInteraction",
        "ApplicationEvent", "ApplicationService", "DataObject"
    ]),
    ("Technology", [
        "Node", "Device", "SystemSoftware", "TechnologyCollaboration",
        "TechnologyInterface", "Path", "CommunicationNetwork", "Artifact",
        "TechnologyFunction", "TechnologyProcess", "TechnologyInteraction",
        "TechnologyEvent", "TechnologyService"
    ]),
    ("Physical", [
        "Equipment", "Facility", "Material", "DistributionNetwork"
    ]),
    ("Implementation & Migration", [
        "WorkPackage", "Deliverable", "Plateau", "Gap"
    ]),
    ("Other", [
        "Grouping", "Location", "AndJunction", "OrJunction"
    ]),
]

RELATIONSHIP_TYPES = [
    ("Structural", [
        "Composition", "Aggregation", "Assignment", "Realization"
    ]),
    ("Dependency", [
        "Access", "Serving", "Influence", "Association"
    ]),
    ("Dynamic", [
        "Triggering", "Flow"
    ]),
    ("Other", [
        "Specialization"
    ]),
]

DERIVABILITY_RULES = [
    ("Composition", "Structural: any layer", "Composite elements can compose elements from any layer"),
    ("Aggregation", "Structural: same layer only", "Aggregation is allowed within the same layer"),
    ("Assignment", "Business: only BusinessActor→BusinessFunction", "Only BusinessActor can assign BusinessFunction"),
    ("Realization", "any layer", "ApplicationComponent can realize BusinessFunction, BusinessProcess"),
    ("Access", "Application→Technology", "ApplicationComponent can access DataObject on TechnologyNode"),
    ("Serving", "Business→Application", "BusinessService can serve BusinessFunction"),
    ("Influence", "Motivation: same layer only", "Motivation elements can influence same-layer elements"),
    ("Association", "any layer", "Any element can be associated with any other element"),
    ("Triggering", "Dynamic: same layer only", "Dynamic relationships are allowed within the same layer"),
    ("Flow", "Dynamic: same layer only", "Dynamic relationships are allowed within the same layer"),
    ("Specialization", "any layer", "Elements can specialize other elements in any layer"),
]

def generate_spec():
    """Generates the complete SPEC.md content."""
    spec = []

    # Title and copyright
    spec.append("# ArchiMate 3.2 Compatibility Specification\n\n")
    spec.append("**Single Source of Truth:** Generated from `archr` Rust code\n\n")
    spec.append("**License:** MIT (compatible with Archi's MIT license)\n\n")
    spec.append("**Reference:** ArchiMate 3.2 Specification (The Open Group, C193)\n\n")
    for rel, rule, desc in DERIVABILITY_RULES:
        parts = rule.split(' ', 1)
        source = target = parts[0] if parts else "N/A"

    # Element Layers
    spec.append("## Element Layers\n\n")
    spec.append("The `archr` engine implements 8 layers as defined in ArchiMate 3.2:\n\n")

    for i, (name, elements) in enumerate(LAYERS):
        spec.append(f"### {name} Layer ({len(elements)} elements)\n\n")
        for elem in elements:
            spec.append(f"- `{elem}`\n")
        spec.append("\n")

    # Relationship Types
    spec.append("---\n\n")
    spec.append("## Relationship Types\n\n")
    spec.append("The `archr` engine implements 11 relationship types with derivability rules:\n\n")

    for category, rels in RELATIONSHIP_TYPES:
        spec.append(f"### {category} ({len(rels)} relations)\n\n")
        for rel in rels:
            spec.append(f"- `{rel}`\n")
        spec.append("\n")

    # Derivability Rules
    spec.append("---\n\n")
    spec.append("## Derivability Rules (ALLOWED Matrix)\n\n")
    spec.append("These rules are defined in `validate.rs::ALLOWED` and validate relationships at runtime.\n\n")
    spec.append("### Relationship → Allowed Element Pairs\n\n")
    spec.append("| Relationship | Source Layer | Target Layer | Description |\n")
    spec.append("|-------------|--------------|--------------|-------------|\n")

    for rel, rule, desc in DERIVABILITY_RULES:
        parts = rule.split(' ', 2)
        if len(parts) >= 3:
            source, target, desc = parts
        else:
            source = target = "N/A"
            desc = parts[0]

        spec.append(f"| `{rel}` | {source} | {target} | {desc} |\n")
    spec.append("\n")

    # Implementation Details
    spec.append("---\n\n")
    spec.append("## Implementation Details\n\n")
    spec.append("### Namespace and Version\n\n")
    spec.append("- XML Namespace: `http://www.archimatetool.com/archimate`\n")
    spec.append("- Version Attribute: `5.0.0` (forward-compatible with 3.x)\n")
    spec.append("- File Extension: `.model` (Model Exchange File Format)\n\n")

    spec.append("### Element Kind Count\n\n")
    total_elements = sum(len(elements) for _, elements in LAYERS)
    spec.append(f"- Total Elements: **{total_elements}** (excluding Junction, which is treated as `Other`)\n")
    spec.append("- Total Layers: **8**\n")
    spec.append("- Total Relationships: **11**\n\n")

    # Limitations
    spec.append("---\n\n")
    spec.append("## Limitations\n\n")
    spec.append("The `archr` engine implements a subset of ArchiMate 3.2 semantics:\n\n")
    spec.append("- **Composition exclusivity:** Full compositional exclusivity is not enforced (composition graphs may contain cycles)\n")
    spec.append("- **Strategy abstraction:** The Strategy layer is treated as a separate layer rather than a mixin hierarchy\n")
    spec.append("- **Motivation semantics:** Full causal reasoning and goal/value dependencies are not modeled\n")
    spec.append("- **Physical abstraction:** Physical layer is treated as separate from Technology layer\n\n")

    # References
    spec.append("---\n\n")
    spec.append("## References\n\n")
    spec.append("- [ArchiMate 3.2 Specification](https://www.opengroup.org/publications/catalog/C193) (The Open Group)\n")
    spec.append("- [Open Group ArchiMate Exchange File Format](https://www.opengroup.org/xsd/archimate/) (XSD and samples)\n")
    spec.append("- [Archi Tool Repository](https://github.com/archimatetool/archi) (MIT licensed metamodel)\n")
    spec.append("\n")

    return ''.join(spec)

def main():
    spec_content = generate_spec()

    output_path = "docs/SPEC.md"
    output_dir = spec_content

    import os
    os.makedirs("docs", exist_ok=True)

    with open(output_path, "w") as f:
        f.write(spec_content)

    total_elements = sum(len(elements) for _, elements in LAYERS)
    total_layers = len(LAYERS)
    total_rels = sum(len(rels) for _, rels in RELATIONSHIP_TYPES)

    print("✅ Generated docs/SPEC.md")
    print(f"   Total elements: {total_elements}")
    print(f"   Total layers: {total_layers}")
    print(f"   Total relationships: {total_rels}")

if __name__ == "__main__":
    main()
