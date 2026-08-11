# Viewpoint Support in YAML I/O

## Summary

Added support for specifying and validating ArchiMate viewpoints in YAML I/O. When a model is parsed from YAML, the `viewpoint` field is validated against the known ArchiMate viewpoints.

## Changes

### 1. Model Structure

Added `viewpoint` field to `YamlModelInner`:
```rust
struct YamlModelInner {
    name: String,
    #[serde(default)]
    viewpoint: Option<String>,
    #[serde(default)]
    elements: Vec<YamlElement>,
    #[serde(default)]
    relationships: Vec<YamlRelationship>,
}
```

### 2. Error Types

Added `InvalidViewpoint` variant to `SchemaError`:
```rust
pub enum SchemaError {
    // ... other variants
    InvalidViewpoint(String),
}
```

### 3. Validation

In `parse_yaml_with_ids`, viewpoint validation is performed:
```rust
if let Some(ref viewpoint_name) = viewpoint {
    if Viewpoint::from_yaml_viewpoint_name(viewpoint_name).is_none() {
        errors.push(SchemaError::InvalidViewpoint(viewpoint_name.clone()));
    }
}
```

### 4. Serialization

Both `model_to_yaml` and `model_to_yaml_with_ids` set `viewpoint: None` when serializing models to YAML.

## Usage

### Valid Viewpoint

```yaml
model:
  name: Business Process View
  viewpoint: Business
  elements:
    - id: e1
      name: Actor
      kind: BusinessActor
```

### Invalid Viewpoint

```yaml
model:
  name: Invalid Viewpoint Test
  viewpoint: UnknownViewpoint
  elements:
    - id: e1
      name: Actor
      kind: BusinessActor
```

This will return a `SchemaError::InvalidViewpoint("UnknownViewpoint")`.

## Test Coverage

- `test_valid_viewpoint`: Tests parsing a model with a valid viewpoint
- `test_invalid_viewpoint`: Tests parsing a model with an invalid viewpoint

## Supported Viewpoints

The validation uses `Viewpoint::from_yaml_viewpoint_name()` which checks against the following viewpoints:

- `Business`
- `Implementation and Migration`
- `Motivation`
- `Phasing`
- `Requirements and Stakeholders`
- `Stakeholder Roles and Responsibilities`
- `Strategy`

## Notes

- The viewpoint field is optional (defaults to `None`)
- Invalid viewpoints are caught during parsing and reported with the invalid name
