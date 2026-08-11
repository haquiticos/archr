# Archr Viewpoints Models Reference

## Overview

This document provides a quick reference for the viewpoint models created for the archr project.

## Available Models

### 1. model_complete.yaml

Basic complete model with standard elements and viewpoints.

**Size:** 1.9K  
**Elements:** 8 (e1-e8)  
**Relationships:** 8 (r1-r8)  
**Viewpoints:** 4

**Viewpoints:**
- Motivation Viewpoint (none kind)
- Motivation Layer Viewpoint (motivation kind)
- Business Viewpoint (business kind)
- Application Viewpoint (application kind)

**Key Features:**
- Minimal set of elements covering all ArchiMate layers
- Basic relationship types (Assignment, Association, Realization)
- Sample viewpoints for different architectural perspectives

### 2. model_full.yaml

Expanded model with additional elements and relationships.

**Size:** 3.3K  
**Elements:** 15 (e1-e15)  
**Relationships:** 17 (r1-r17)  
**Viewpoints:** 6

**Viewpoints:**
- Motivation Layer Viewpoint (motivation kind)
- Business Layer Viewpoint (business kind)
- Application Layer Viewpoint (application kind)
- Technology Viewpoint (business kind)
- Implementation Viewpoint (implementation kind)
- Physical Viewpoint (business kind)

**Key Features:**
- Additional elements from each ArchiMate layer
- More comprehensive relationship types
- Layer-specific viewpoints for better filtering

### 3. model_archimate_full.yaml

Full ArchiMate model with comprehensive layer-specific viewpoints.

**Size:** 3.6K  
**Elements:** 15 (e1-e15)  
**Relationships:** 17 (r1-r17)  
**Viewpoints:** 6

**Viewpoints:**
Same as model_full.yaml, but with layer-specific viewpoints focused on:
- Motivation layer (drivers, requirements, goals)
- Business layer (actors, processes)
- Application layer (components, services, data)
- Technology layer (nodes, networks)
- Implementation layer (work packages, deliverables)
- Physical layer (facilities)

**Key Features:**
- Most comprehensive set of viewpoints
- Layer-specific filtering
- Good balance between complexity and clarity

### 4. model_archimate_example.yaml

Example focused on archr core functionality.

**Size:** 2.2K  
**Elements:** 3 (e6, e9, e10)  
**Relationships:** 0  
**Viewpoints:** 1

**Viewpoints:**
- Archr Core Architecture Viewpoint (application kind)

**Key Features:**
- Minimal set focused on core archr elements
- Application layer viewpoint
- Example of a focused viewpoint

## Validation Status

All models pass validation:
```bash
python3 skill/scripts/archr.py validate <model_file>
```

## Usage

### Load and Use a Model

```python
import yaml
from skill.scripts.archr import load_model, validate_model

# Load model
with open('model_complete.yaml', 'r') as f:
    data = yaml.safe_load(f)

# Validate
result = validate_model(data)
print(f"Validation: {result['success']}")
```

### Create a New Viewpoint

```python
# Load existing model
with open('model_archimate_full.yaml', 'r') as f:
    data = yaml.safe_load(f)

# Create new viewpoint
new_viewpoint = {
    'id': 'vp_custom',
    'name': 'Custom Viewpoint',
    'kind': 'business',
    'elements': [
        {'id': 'e1', 'name': 'Driver', 'kind': 'Driver'},
        {'id': 'e5', 'name': 'BusinessActor', 'kind': 'BusinessActor'},
    ],
    'relationships': []
}

# Add to model
data['model']['viewpoints'].append(new_viewpoint)

# Save and validate
with open('model_custom.yaml', 'w') as f:
    yaml.dump(data, f, default_flow_style=False, sort_keys=False)

validate_model(data)
```

## Comparison

| Model | Elements | Relationships | Viewpoints | Best For |
|-------|----------|---------------|------------|----------|
| model_complete | 8 | 8 | 4 | Quick start, examples |
| model_full | 15 | 17 | 6 | Understanding full ArchiMate |
| model_archimate_full | 15 | 17 | 6 | Layer-specific analysis |
| model_archimate_example | 3 | 0 | 1 | Focused examples |

## Recommendations

1. **Start with** `model_complete.yaml` for learning
2. **Use** `model_archimate_full.yaml` for layer-specific analysis
3. **Reference** `model_archimate_example.yaml` for creating custom viewpoints

## Element and Relationship Reference

### Element Kinds by Layer

**Motivation:** Driver, Requirement, Goal, Stakeholder  
**Strategy:** Capability, Resource  
**Business:** BusinessActor, BusinessProcess, BusinessInterface  
**Application:** ApplicationComponent, ApplicationService, DataObject  
**Technology:** Node, CommunicationNetwork  
**Implementation:** WorkPackage, Deliverable  
**Physical:** Facility, Material  
**Other:** Product

### Relationship Kinds

- Assignment (motivation to strategy/business)
- Association (general connections)
- Realization (business to application/implementation)
- Access (application to technology)
- Serving (technology to technology)
