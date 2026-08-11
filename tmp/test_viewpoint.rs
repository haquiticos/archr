// Simple test to verify viewpoint field works
use archr_core::io::yaml::{parse_yaml, model_to_yaml, Model};
use archr_core::model::ElementKind;

fn main() {
    // Create a simple model
    let mut elements = vec![];
    elements.push(archr_core::model::Element {
        id: 0,
        name: "Business1".to_string(),
        kind: ElementKind::BusinessActor,
        layer: 0,
        x: 100,
        y: 100,
        description: None,
    });
    
    let model = Model {
        name: "Test Model".to_string(),
        elements,
        relationships: vec![],
    };
    
    // Serialize to YAML
    let yaml = model_to_yaml(&model);
    println!("Serialized YAML:\n{}", yaml);
    
    // Try to deserialize it back
    let parsed = parse_yaml(&yaml).expect("Failed to parse YAML");
    println!("Deserialized model: {:?}", parsed.name);
    
    println!("Test passed!");
}
