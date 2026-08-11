# Archr Viewpoints - Practical Examples

## Overview

This document provides practical examples of creating and using viewpoints in archr.

## Example 1: Basic Motivation Viewpoint

Create a simple viewpoint focusing on the motivation layer:

```yaml
# motivation_viewpoint.yaml
model:
  name: "Architecture Model"
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
    - id: e4
      name: Stakeholder
      kind: Stakeholder
  
  relationships:
    - id: r1
      source: e1
      target: e2
      kind: Assignment
    - id: r2
      source: e2
      target: e3
      kind: Assignment
  
  viewpoints:
    - id: vp_motivation
      name: Motivation Viewpoint
      kind: motivation
      elements:
        - id: e1
        - id: e2
        - id: e3
      relationships:
        - id: r1
        - id: r2
```

**Validation:**
```bash
python3 skill/scripts/archr.py validate motivation_viewpoint.yaml
```

## Example 2: Business Layer Viewpoint

Create a viewpoint focusing on business actors and processes:

```yaml
# business_viewpoint.yaml
model:
  name: "Business Model"
  elements:
    - id: e1
      name: BusinessActor
      kind: BusinessActor
    - id: e2
      name: BusinessProcess
      kind: BusinessProcess
    - id: e3
      name: BusinessInterface
      kind: BusinessInterface
  
  relationships:
    - id: r1
      source: e1
      target: e2
      kind: Assignment
  
  viewpoints:
    - id: vp_business
      name: Business Viewpoint
      kind: business
      elements:
        - id: e1
        - id: e2
        - id: e3
      relationships:
        - id: r1
```

## Example 3: Application Layer Viewpoint

Create a viewpoint focusing on application components:

```yaml
# application_viewpoint.yaml
model:
  name: "Application Model"
  elements:
    - id: e1
      name: ApplicationComponent
      kind: ApplicationComponent
    - id: e2
      name: ApplicationService
      kind: ApplicationService
    - id: e3
      name: DataObject
      kind: DataObject
  
  relationships:
    - id: r1
      source: e1
      target: e2
      kind: Realization
  
  viewpoints:
    - id: vp_application
      name: Application Viewpoint
      kind: application
      elements:
        - id: e1
        - id: e2
        - id: e3
      relationships:
        - id: r1
```

## Example 4: Mixed Layer Viewpoint

Create a viewpoint that combines elements from multiple layers:

```yaml
# mixed_viewpoint.yaml
model:
  name: "Cross-Layer Model"
  elements:
    # Business Layer
    - id: e1
      name: BusinessActor
      kind: BusinessActor
    - id: e2
      name: BusinessProcess
      kind: BusinessProcess
    # Application Layer
    - id: e3
      name: ApplicationComponent
      kind: ApplicationComponent
    # Technology Layer
    - id: e4
      name: Node
      kind: Node
    # Implementation Layer
    - id: e5
      name: WorkPackage
      kind: WorkPackage
  
  relationships:
    - id: r1
      source: e1
      target: e2
      kind: Assignment
    - id: r2
      source: e2
      target: e3
      kind: Assignment
    - id: r3
      source: e3
      target: e4
      kind: Access
  
  viewpoints:
    - id: vp_cross_layer
      name: Cross-Layer Viewpoint
      kind: none
      elements:
        - id: e1
        - id: e2
        - id: e3
        - id: e4
        - id: e5
      relationships:
        - id: r1
        - id: r2
        - id: r3
```

## Example 5: Minimal Viewpoint

Create the smallest possible valid viewpoint:

```yaml
# minimal_viewpoint.yaml
model:
  name: "Minimal Model"
  elements:
    - id: e1
      name: Driver
      kind: Driver
  
  viewpoints:
    - id: vp_minimal
      name: Minimal Viewpoint
      kind: none
      elements:
        - id: e1
      relationships: []
```

## Example 6: Viewpoint with Relationships

Create a viewpoint that includes relationships between elements:

```yaml
# relationships_viewpoint.yaml
model:
  name: "Relationship Model"
  elements:
    - id: e1
      name: BusinessActor
      kind: BusinessActor
    - id: e2
      name: ApplicationComponent
      kind: ApplicationComponent
    - id: e3
      name: DataObject
      kind: DataObject
  
  relationships:
    - id: r1
      source: e1
      target: e2
      kind: Realization
    - id: r2
      source: e2
      target: e3
      kind: Realization
  
  viewpoints:
    - id: vp_with_rels
      name: With Relationships Viewpoint
      kind: business
      elements:
        - id: e1
        - id: e2
      relationships:
        - id: r1
```

## Example 7: Using Multiple Viewpoints

Create a model with multiple viewpoints on the same architecture:

```yaml
# multiple_viewpoints.yaml
model:
  name: "Multi-Viewpoint Model"
  elements:
    - id: e1
      name: Driver
      kind: Driver
    - id: e2
      name: BusinessActor
      kind: BusinessActor
    - id: e3
      name: ApplicationComponent
      kind: ApplicationComponent
  
  relationships:
    - id: r1
      source: e1
      target: e2
      kind: Assignment
  
  viewpoints:
    - id: vp1
      name: Motivation Viewpoint
      kind: motivation
      elements:
        - id: e1
      relationships: []
    
    - id: vp2
      name: Business Viewpoint
      kind: business
      elements:
        - id: e2
      relationships: []
    
    - id: vp3
      name: Application Viewpoint
      kind: application
      elements:
        - id: e3
      relationships: []
```

## Example 8: Custom Viewpoint with Documentation

Create a comprehensive viewpoint with metadata:

```yaml
# documented_viewpoint.yaml
model:
  name: "Documented Model"
  elements:
    - id: e1
      name: Core Component
      kind: ApplicationComponent
    - id: e2
      name: Core Service
      kind: ApplicationService
  
  relationships:
    - id: r1
      source: e1
      target: e2
      kind: Realization
  
  viewpoints:
    - id: vp_custom
      name: Core Architecture Viewpoint
      kind: application
      description: "View of the core architecture components and their relationships"
      purpose: "Understand the core functionality and structure"
      elements:
        - id: e1
        - id: e2
      relationships:
        - id: r1
```

## Python Examples

### Loading and Using a Model

```python
import yaml
from skill.scripts.archr import load_model, validate_model

# Load model from file
with open('model_complete.yaml', 'r') as f:
    model_data = yaml.safe_load(f)

# Validate model
result = validate_model(model_data)
print(f"Model is valid: {result['success']}")

# Get all elements
elements = model_data['model']['elements']
for elem in elements:
    print(f"- {elem['name']} ({elem['kind']})")

# Get all viewpoints
viewpoints = model_data['model']['viewpoints']
for vp in viewpoints:
    print(f"\nViewpoint: {vp['name']}")
    print(f"Kind: {vp['kind']}")
    print(f"Elements: {[e['id'] for e in vp['elements']]}")
```

### Creating a New Viewpoint Programmatically

```python
import yaml
from skill.scripts.archr import validate_model

# Load existing model
with open('model_full.yaml', 'r') as f:
    model_data = yaml.safe_load(f)

# Create new viewpoint
new_viewpoint = {
    'id': 'vp_performance',
    'name': 'Performance Focus Viewpoint',
    'kind': 'business',
    'description': 'View focusing on performance-critical components',
    'elements': [
        {'id': 'e6', 'name': 'ApplicationComponent', 'kind': 'ApplicationComponent'},
        {'id': 'e9', 'name': 'ApplicationService', 'kind': 'ApplicationService'},
        {'id': 'e10', 'name': 'DataObject', 'kind': 'DataObject'},
    ],
    'relationships': [
        {'id': 'r10', 'source': 'e9', 'target': 'e10', 'kind': 'Realization'},
    ]
}

# Add to model
model_data['model']['viewpoints'].append(new_viewpoint)

# Save to file
with open('model_with_performance_viewpoint.yaml', 'w') as f:
    yaml.dump(model_data, f, default_flow_style=False, sort_keys=False)

# Validate
result = validate_model(model_data)
print(f"New model is valid: {result['success']}")
```

### Filtering Elements by Viewpoint

```python
import yaml

# Load model
with open('model_archimate_full.yaml', 'r') as f:
    model_data = yaml.safe_load(f)

# Get a specific viewpoint
viewpoint_name = 'Motivation Layer Viewpoint'
for vp in model_data['model']['viewpoints']:
    if vp['name'] == viewpoint_name:
        print(f"Viewpoint: {vp['name']}")
        print(f"Elements in this viewpoint:")
        
        # Get element IDs from viewpoint
        element_ids = [e['id'] for e in vp['elements']]
        
        # Get full element details
        for elem_id in element_ids:
            for elem in model_data['model']['elements']:
                if elem['id'] == elem_id:
                    print(f"  - {elem['name']} ({elem['kind']})")
        
        # Get relationship details
        print(f"\nRelationships in this viewpoint:")
        for rel in vp.get('relationships', []):
            for relationship in model_data['model']['relationships']:
                if relationship['id'] == rel['id']:
                    print(f"  - {relationship['source']} -> {relationship['target']} ({relationship['kind']})")
        
        break
```

## Tips and Best Practices

1. **Use descriptive names** for viewpoints that clearly indicate their focus
2. **Keep viewpoints focused** on a single aspect or layer
3. **Validate after creating** viewpoints to catch errors early
4. **Use consistent ID naming** (e.g., e1, e2, e3 for elements; r1, r2, r3 for relationships)
5. **Document your viewpoints** with descriptions and purposes
6. **Start simple** and add complexity gradually
7. **Test viewpoints** with validation before using them in analysis
8. **Reuse existing elements** from the model rather than duplicating

## Validation Checklist

Before using a viewpoint, ensure:
- ✓ All element IDs exist in the model's `elements` section
- ✓ All relationship IDs exist in the model's `relationships` section
- ✓ Viewpoint kind is one of the supported values
- ✓ Model passes validation with no errors
- ✓ Viewpoint has meaningful name and purpose
