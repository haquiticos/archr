# Archr Viewpoints Guide

## Overview

Viewpoints in ArchiMate allow you to filter and focus on specific aspects of an architecture model. They define which elements and relationships are visible from a particular perspective.

## Supported Viewpoint Types

The archr system supports the following viewpoint kinds:

- `none` - General viewpoint (all elements)
- `motivation` - Focus on motivation layer (drivers, requirements, goals, stakeholders)
- `business` - Focus on business layer (actors, roles, processes)
- `application` - Focus on application layer (components, services, data)
- `implementation` - Focus on implementation/migration layer (work packages, deliverables)

## Creating a Viewpoint

A viewpoint definition includes:

- `id` - Unique identifier
- `name` - Human-readable name
- `kind` - Viewpoint type (one of the supported types above)
- `elements` - List of elements to include
- `relationships` - List of relationships to include (optional)

## Example Viewpoint

```yaml
viewpoints:
  - id: vp_motivation
    name: Motivation Layer Viewpoint
    kind: motivation
    elements:
      - id: e1
        name: Driver
        kind: Driver
      - id: e2
        name: Requirement
        kind: Requirement
      - id: e3
        name: Goal
        kind: Goal
    relationships:
      - id: r1
        source: e1
        target: e2
        kind: Assignment
      - id: r2
        source: e2
        target: e3
        kind: Assignment
```

## Available Models

- `model_complete.yaml` - Basic model with complete elements and viewpoints
- `model_full.yaml` - Expanded model with more elements and viewpoints
- `model_archimate_full.yaml` - Full ArchiMate model with layer-specific viewpoints

## Validation

All models can be validated using:

```bash
python3 skill/scripts/archr.py validate model.yaml
```

## Notes

- Relationship IDs must match IDs in the model's `relationships` section
- Element IDs must match IDs in the model's `elements` section
- Elements not in a viewpoint are not visible from that viewpoint
- Relationships without elements in a viewpoint are not visible from that viewpoint
