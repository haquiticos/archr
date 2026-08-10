# Quick Start Guide - Archr Viewpoints

## 5-Minute Quick Start

### 1. Load a Model

```bash
# Load a model in Python
python3 -c "
import yaml
with open('model_complete.yaml', 'r') as f:
    data = yaml.safe_load(f)
print('Model loaded:', data['model']['name'])
print('Elements:', len(data['model']['elements']))
print('Viewpoints:', len(data['model']['viewpoints']))
"
```

### 2. View All Viewpoints

```python
import yaml

with open('model_complete.yaml', 'r') as f:
    data = yaml.safe_load(f)

print('Available Viewpoints:')
for vp in data['model']['viewpoints']:
    print(f'  - {vp["name"]} ({vp["kind"]})')
```

### 3. Create a Viewpoint

```yaml
viewpoints:
  - id: my_viewpoint
    name: My Viewpoint
    kind: business
    elements:
      - id: e1
        name: BusinessActor
        kind: BusinessActor
    relationships: []
```

### 4. Validate a Model

```bash
python3 skill/scripts/archr.py validate model_complete.yaml
```

### 5. Add to an Existing Model

```python
import yaml
from skill.scripts.archr import validate_model

# Load existing model
with open('model_complete.yaml', 'r') as f:
    data = yaml.safe_load(f)

# Create new viewpoint
new_vp = {
    'id': 'vp_example',
    'name': 'Example Viewpoint',
    'kind': 'business',
    'elements': [
        {'id': 'e1', 'name': 'BusinessActor', 'kind': 'BusinessActor'},
    ],
    'relationships': []
}

# Add and save
data['model']['viewpoints'].append(new_vp)

with open('model_with_example.yaml', 'w') as f:
    yaml.dump(data, f, default_flow_style=False, sort_keys=False)

# Validate
result = validate_model(data)
print(f"Valid: {result['success']}")
```

## Supported Viewpoint Kinds

- `none` - General (all elements)
- `motivation` - Motivation layer
- `business` - Business layer
- `application` - Application layer
- `implementation` - Implementation layer

## Element Kinds

- **Motivation:** Driver, Requirement, Goal, Stakeholder
- **Strategy:** Capability, Resource
- **Business:** BusinessActor, BusinessProcess, BusinessInterface
- **Application:** ApplicationComponent, ApplicationService, DataObject
- **Technology:** Node, CommunicationNetwork
- **Implementation:** WorkPackage, Deliverable
- **Physical:** Facility, Material

## Relationship Kinds

- Assignment, Association, Realization, Access, Serving

## Common Commands

```bash
# Validate all models
for f in model_*.yaml; do python3 skill/scripts/archr.py validate $f; done

# Load a model
python3 -c "import yaml; print(yaml.safe_load(open('model_complete.yaml')))"

# Count elements in a viewpoint
python3 -c "
import yaml
data = yaml.safe_load(open('model_complete.yaml'))
vp = data['model']['viewpoints'][0]
print(f'Elements: {len(vp[\"elements\"])}')
print(f'Relationships: {len(vp[\"relationships\"])}')
"

# Check validation status
python3 skill/scripts/archr.py validate model_complete.yaml 2>&1 | grep -E '(success|errors)'
```

## Need Help?

1. Read [INDEX.md](INDEX.md) for navigation
2. See [VIEWPOINTS_GUIDE.md](VIEWPOINTS_GUIDE.md) for detailed guide
3. Check [EXAMPLES.md](EXAMPLES.md) for practical examples
4. Reference [MODELS_REFERENCE.md](MODELS_REFERENCE.md) for model details

## Validation Checklist

Before using a viewpoint:
- [ ] All element IDs exist in the model
- [ ] All relationship IDs exist in the model
- [ ] Viewpoint kind is valid (none, motivation, business, application, implementation)
- [ ] Model validates without errors

## Models Reference

| Model | Elements | Viewpoints | Best For |
|-------|----------|------------|----------|
| model_complete.yaml | 8 | 4 | Quick start |
| model_full.yaml | 15 | 6 | Learning |
| model_archimate_full.yaml | 15 | 6 | Layer-specific analysis |
| model_archimate_example.yaml | 3 | 1 | Focused examples |

---
**Status:** Complete ✓  
**Date:** 2026-08-10
