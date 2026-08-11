# Estrutura YAML com Múltiplos Viewpoints

## Estrutura Atualizada

A estrutura YAML foi atualizada para suportar múltiplos viewpoints, cada um com seus próprios elementos e relações:

```yaml
model:
  name: "Nome do Modelo"              # Obrigatório
  
  elements:                          # Elementos globais (comuns a todos os viewpoints)
    - id: "e1"                        # Obrigatório
      name: "Nome do Elemento"        # Obrigatório
      kind: "BusinessActor"           # Obrigatório
  
  relationships:                      # Relações globais (comuns a todos os viewpoints)
    - id: "r1"                        # Obrigatório
      kind: "Composition"             # Obrigatório
      source: "e1"                    # Obrigatório
      target: "e2"                    # Obrigatório
  
  viewpoints:                        # Múltiplos viewpoints (opcional)
    - id: "vp1"                       # Obrigatório para cada viewpoint
      name: "Viewpoint Name"          # Obrigatório para cada viewpoint
      kind: "business"                # Obrigatório para cada viewpoint (lowercase)
      elements:                       # Elementos específicos deste viewpoint (opcional)
        - id: "vp_e1"                 # Pode ter IDs únicos para cada viewpoint
          name: "Elemento Específico"
          kind: "BusinessRole"
      relationships:                  # Relações específicas deste viewpoint (opcional)
        - id: "vp_r1"
          kind: "Assignment"
          source: "vp_e1"
          target: "vp_e2"
    - id: "vp2"
      name: "Outro Viewpoint"
      kind: "application"
      elements: [...]                 # Pode ter elementos diferentes
      relationships: [...]            # Pode ter relações diferentes
```

## Regras Importantes

### 1. Ordem de Leitura
- **Viewpoints devem ser lidos por último**
- Os elementos e relações globais são criados primeiro
- Os viewpoints podem referenciar elementos globais

### 2. Campos do Model
- `name`: Nome do modelo (obrigatório)
- `elements`: Elementos globais (obrigatório se houver elementos)
- `relationships`: Relações globais (obrigatório se houver relações)
- `viewpoints`: Lista de viewpoints (opcional)

### 3. Campos de Cada Viewpoint
- `id`: Identificador único do viewpoint (obrigatório)
- `name`: Nome descritivo do viewpoint (obrigatório)
- `kind`: Tipo do viewpoint (obrigatório):
  - `none`, `business`, `application`, `implementation`, `motivation`, `compliance`
  - Case-insensitive (será convertido para lowercase)
- `elements`: Elementos específicos deste viewpoint (opcional, padrão: [])
- `relationships`: Relações específicas deste viewpoint (opcional, padrão: [])

### 4. IDs Únicos
- Cada viewpoint tem seus próprios IDs para elementos
- Um elemento global pode ser referenciado por múltiplos viewpoints
- Não há conflito de IDs entre viewpoints

### 5. Validação
- IDs vazios ou com espaços são inválidos
- IDs duplicados são inválidos
- Referências inexistentes causam erro
- Tipos desconhecidos causam erro

## Exemplos Práticos

### Exemplo 1: Modelos Simples (Sem Viewpoints)
```yaml
model:
  name: "Modelo Simples"
  elements:
    - id: "e1"
      name: "Actor"
      kind: "BusinessActor"
    - id: "e2"
      name: "Componente"
      kind: "ApplicationComponent"
  relationships:
    - id: "r1"
      kind: "Serving"
      source: "e1"
      target: "e2"
```

### Exemplo 2: Múltiplos Viewpoints
```yaml
model:
  name: "Arquitetura Completa"
  elements:
    - id: "e1"
      name: "Cliente"
      kind: "BusinessActor"
    - id: "e2"
      name: "API"
      kind: "ApplicationComponent"
  relationships:
    - id: "r1"
      kind: "Assignment"
      source: "e1"
      target: "e2"
  viewpoints:
    - id: "vp1"
      name: "Viewpoint de Negócio"
      kind: "business"
      elements:
        - id: "e1"
          name: "Cliente"
          kind: "BusinessActor"
      relationships:
        - id: "r1"
          kind: "Assignment"
          source: "e1"
          target: "e2"
    - id: "vp2"
      name: "Viewpoint de Aplicação"
      kind: "application"
      elements:
        - id: "e2"
          name: "API"
          kind: "ApplicationComponent"
      relationships:
        - id: "r1"
          kind: "Assignment"
          source: "e2"
          target: "e1"
```

### Exemplo 3: Elementos Compartilhados
```yaml
model:
  name: "Elementos Compartilhados"
  elements:
    - id: "e1"
      name: "Recurso Comum"
      kind: "Resource"
    - id: "e2"
      name: "Componente"
      kind: "ApplicationComponent"
  relationships:
    - id: "r1"
      kind: "Composition"
      source: "e1"
      target: "e2"
  viewpoints:
    - id: "vp1"
      name: "Viewpoint de Estratégia"
      kind: "motivation"
      elements:
        - id: "e1"
          name: "Recurso Comum"
          kind: "Resource"
      relationships:
        - id: "r1"
          kind: "Composition"
          source: "e1"
          target: "e2"
    - id: "vp2"
      name: "Viewpoint de Implementação"
      kind: "implementation"
      elements:
        - id: "e2"
          name: "Componente"
          kind: "ApplicationComponent"
      relationships:
        - id: "r1"
          kind: "Composition"
          source: "e1"
          target: "e2"
```

## Valores do Campo `kind` em Viewpoints

- **`none`** - Sem viewpoint específico
- **`business`** - Perspectiva de negócio
- **`application`** - Perspectiva de aplicação
- **`implementation`** - Perspectiva de implementação
- **`motivation`** - Perspectiva de motivação
- **`compliance`** - Perspectiva de conformidade

## Exemplo de Código Rust

```rust
use archr_core::io::yaml::{parse_yaml, model_to_yaml};

fn main() {
    // Parsear YAML com múltiplos viewpoints
    let yaml = r#"
model:
  name: "Teste"
  elements: [...]
  relationships: [...]
  viewpoints:
    - id: "vp1"
      name: "Viewpoint"
      kind: "business"
      elements: [...]
      relationships: [...]
"#;
    
    let model = parse_yaml(yaml).expect("Falha ao parsear YAML");
    
    // Serializar de volta para YAML
    let yaml_output = model_to_yaml(&model);
    println!("{}", yaml_output);
}
```

## Transição de Single para Multiple Viewpoints

### Antes (Single Viewpoint)
```yaml
model:
  name: "Modelo"
  viewpoint: business
  elements: [...]
  relationships: [...]
```

### Depois (Multiple Viewpoints)
```yaml
model:
  name: "Modelo"
  elements: [...]        # Elementos globais
  relationships: [...]    # Relações globais
  viewpoints:            # Viewpoints específicos
    - id: "vp1"
      name: "Viewpoint"
      kind: "business"
      elements: [...]     # Elementos específicos deste viewpoint
      relationships: [...] # Relações específicas deste viewpoint
```

## Notas

- Viewpoints são opcionais
- Elementos e relações globais são opcionais
- Cada viewpoint pode ter elementos e relações diferentes
- IDs podem ser compartilhados entre viewpoints (mas não dentro de um mesmo viewpoint)
- O campo `kind` em viewpoints é case-insensitive
