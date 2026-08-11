# Correção de Suporte a Viewpoints

## Problema
O comando `archr generate` não gera os vários viewpoints definidos no arquivo YAML `model_archimate_full.yaml`.

## Causa Raiz

### 1. Estrutura Model Não Suporta Viewpoints
```rust
pub struct Model {
    pub name: String,
    elements: Vec<Element>,
    relations: Vec<Relationship>,
}
// Falta campo: viewpoints: Vec<YamlViewpointDefinition>
```

### 2. Emit_Diagram Ignora Viewpoints
A função `emit_diagram` cria apenas um único diagrama "Default View" com todos os elementos, ignorando completamente os viewpoints definidos no YAML.

### 3. XML Não Tem Suporte para Múltiplos Diagramas
Cada viewpoint deveria gerar um `ArchimateDiagramModel` separado, mas o código não faz isso.

## Dados do YAML

O arquivo `model_archimate_full.yaml` define 6 viewpoints:

```yaml
viewpoints:
- id: vp_motivation
  name: Motivation Layer Viewpoint
  kind: Motivation
  description: Focus on drivers, requirements, and goals
  elements:
  - id: e1  # Driver
  - id: e2  # Requirement
  - id: e3  # Goal

- id: vp_business
  name: Business Layer Viewpoint
  kind: Business
  description: Focus on business actors, roles, and processes
  elements:
  - id: e5  # BusinessActor
  - id: e7  # WorkPackage

- id: vp_application
  name: Application Layer Viewpoint
  kind: Application
  description: Focus on application components, services, and data objects
  elements:
  - id: e6  # ApplicationComponent
  - id: e9  # ApplicationService
  - id: e10 # DataObject

- id: vp_technology
  name: Technology Layer Viewpoint
  kind: Business  # ⚠️ Erro no YAML
  description: Focus on technology nodes and communication networks
  elements:
  - id: e11 # Node
  - id: e12 # CommunicationNetwork

- id: vp_implementation
  name: Implementation Viewpoint
  kind: Implementation
  description: Focus on work packages and deliverables
  elements:
  - id: e13 # WorkPackage
  - id: e14 # Deliverable

- id: vp_physical
  name: Physical Layer Viewpoint
  kind: Business  # ⚠️ Erro no YAML
  description: Focus on facilities and physical resources
  elements:
  - id: e15 # Facility
```

## Solução Proposta

### Passo 1: Adicionar Viewpoints ao Model
Adicionar campo para armazenar viewpoints no struct Model:

```rust
pub struct Model {
    pub name: String,
    elements: Vec<Element>,
    relations: Vec<Relationship>,
    viewpoints: Vec<YamlViewpointDefinition>,
}
```

### Passo 2: Atualizar Parse_YAML
Preservar viewpoints durante o parsing:

```rust
let (model, elem_ids, rel_ids, viewpoints) = yaml::parse_yaml_with_viewpoints(&yaml_str)?;
```

### Passo 3: Modificar Emit_Diagram
Gerar múltiplos diagramas, um para cada viewpoint:

```rust
fn emit_diagram(
    xml: &mut String,
    model: &Model,
    positions: &HashMap<ElementId, (f64, f64, f64, f64)>,
    elem_ids: &HashMap<ElementId, String>,
    rel_ids: &HashMap<RelationId, String>,
    child_ids: &HashMap<ElementId, String>,
) {
    // Para cada viewpoint, criar um ArchimateDiagramModel
    for vp_def in model.viewpoints.iter() {
        emit_viewpoint_diagram(xml, model, positions, elem_ids, rel_ids, child_ids, vp_def)?;
    }
}
```

### Passo 4: Criar Emit_Viewpoint_Diagram
Função para gerar um diagram específico para um viewpoint:

```rust
fn emit_viewpoint_diagram(
    xml: &mut String,
    model: &Model,
    positions: &HashMap<ElementId, (f64, f64, f64, f64)>,
    elem_ids: &HashMap<ElementId, String>,
    rel_ids: &HashMap<RelationId, String>,
    child_ids: &HashMap<ElementId, String>,
    viewpoint: &YamlViewpointDefinition,
) {
    let folder_id = Uuid::new_v4();
    let diagram_id = Uuid::new_v4();
    
    let _ = writeln!(
        xml,
        "  <folder name=\"Views\" id=\"{}\" type=\"diagrams\">",
        folder_id
    );
    
    let _ = writeln!(
        xml,
        "    <element xsi:type=\"archimate:ArchimateDiagramModel\" \
         name=\"{}\" id=\"{}\">",
        viewpoint.name, diagram_id
    );
    
    // Filtrar elementos do viewpoint
    for elem_id in &viewpoint.elements {
        if let Some(elem) = model.elements.iter().find(|e| e.id.to_string() == elem_id.id) {
            emit_diagram_object(xml, elem, positions, elem_ids, rel_ids, child_ids, viewpoint)?;
        }
    }
    
    let _ = writeln!(xml, "    </element>");
    let _ = writeln!(xml, "  </folder>");
}
```

## Testes Necessários

1. Testar geração de XML com viewpoints múltiplos
2. Verificar que cada viewpoint gera um diagrama separado
3. Verificar que elementos são filtrados corretamente por viewpoint
4. Verificar que relationships são filtradas corretamente por viewpoint

## Implementação

A implementação requer modificações em:
1. `crates/archr-core/src/model.rs` - Adicionar campo viewpoints ao Model
2. `crates/archr-core/src/io/yaml.rs` - Preservar viewpoints durante parsing
3. `crates/archr-core/src/io/xml.rs` - Modificar emit_diagram para gerar múltiplos diagramas

