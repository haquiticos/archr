# Mudança de Estrutura de Viewpoints

## Estrutura Antiga (Single Viewpoint)
```yaml
model:
  name: "Nome do Modelo"
  viewpoint: business        # único viewpoint
  elements: [...]
  relationships: [...]
```

## Estrutura Nova (Multiple Viewpoints)
```yaml
model:
  name: "Nome do Modelo"
  viewpoints:                 # múltiplos viewpoints
    - id: "vp1"
      name: "Business Viewpoint"
      kind: "business"
      elements:               # elementos específicos deste viewpoint
        - id: "e1"
          name: "Nome1"
          kind: "BusinessActor"
      relationships:          # relações específicas deste viewpoint
        - id: "r1"
          kind: "Composition"
          source: "e1"
          target: "e2"
    - id: "vp2"
      name: "Application Viewpoint"
      kind: "application"
      elements: [...]         # elementos específicos deste viewpoint
      relationships: [...]    # relações específicas deste viewpoint
  elements: [...]             # elementos globais (comuns a todos)
  relationships: [...]        # relações globais (comuns a todos)
```

## Mudanças de Código

### 1. Criar novo enum `YamlViewpointKind` (antes de `YamlViewpoint`)
```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum YamlViewpointKind {
    None,
    Business,
    Application,
    Implementation,
    Motivation,
    Compliance,
}
```

### 2. Criar novo struct `YamlViewpointDefinition` (após `YamlViewpoint`)
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
struct YamlViewpointDefinition {
    id: String,
    name: String,
    #[serde(rename_all = "lowercase")]
    kind: YamlViewpointKind,
    #[serde(default)]
    elements: Vec<YamlElement>,
    #[serde(default)]
    relationships: Vec<YamlRelationship>,
}
```

### 3. Atualizar `YamlModelInner`
Remover:
```rust
viewpoint: Option<YamlViewpoint>,
```

Adicionar:
```rust
#[serde(default)]
viewpoints: Vec<YamlViewpointDefinition>,
```

### 4. Atualizar functions de parsing e serialization
- `parse_yaml_with_ids` - iterar sobre viewpoints e processar cada um
- `model_to_yaml` - serializar todos os viewpoints
- `model_to_yaml_with_ids` - serializar todos os viewpoints

## Impacto na Lógica

- Cada viewpoint terá seus próprios elementos e relações específicos
- Mantém-se a possibilidade de elementos e relações globais
- Cada viewpoint pode ter elementos/relações que não existem em outros viewpoints
- Cada viewpoint deve ter `id`, `name`, `kind`, `elements`, `relationships`
