// Teste simples para múltiplos viewpoints

use archr_core::io::yaml::{parse_yaml, SchemaError};
use std::fs;

fn main() {
    // Ler o arquivo YAML de exemplo
    let yaml_content = fs::read_to_string("tmp/multiple_viewpoints.yaml")
        .expect("Não foi possível ler o arquivo YAML");
    
    println!("=== Testando YAML com Múltiplos Viewpoints ===");
    println!("\nConteúdo do YAML:\n{}", yaml_content);
    
    // Tentar parsear o YAML
    match parse_yaml(&yaml_content) {
        Ok(model) => {
            println!("\n✅ Parse bem-sucedido!");
            println!("Nome do modelo: {}", model.name);
            println!("Total de elementos: {}", model.element_count());
            println!("Total de relações: {}", model.relation_count());
        }
        Err(errors) => {
            println!("\n❌ Erros de parsing:");
            for error in errors {
                println!("  - {:?}", error);
            }
        }
    }
    
    // Tentar serializar de volta para YAML
    match parse_yaml(&yaml_content) {
        Ok(model) => {
            use archr_core::io::yaml::model_to_yaml;
            let serialized = model_to_yaml(&model);
            println!("\n=== YAML Serializado ===\n{}", serialized);
        }
        Err(e) => {
            println!("\nErro ao serializar: {:?}", e);
        }
    }
}
