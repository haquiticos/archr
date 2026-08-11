# Relatório: Suporte a Viewpoints no Arkr

## Problemas Identificados

### 1. Enum Viewpoint Existe no Código Rust

**Localização:** `crates/archr-core/src/model.rs:461-526`

O enum `Viewpoint` está completamente implementado com **14 tipos**:

```rust
pub enum Viewpoint {
    // Layer-based viewpoints
    Motivation, Strategy, Business, Application, Technology, 
    Physical, Implementation, Other,
    
    // Special viewpoints
    EnterpriseStructure, ValueStream,
    
    // Mixin viewpoints
    Organization, BusinessProcessCooperation, Product,
    ApplicationCooperation, ApplicationUsage,
    
    NONE,  // "Layered" viewpoint
}
```

Métodos disponíveis:
- `from_yaml_viewpoint_name(name: &str) -> Option<Self>`
- `to_yaml_viewpoint_name(self) -> &'static str`
- `from_xml_viewpoint_name(name: &str) -> Option<Self>`
- `to_xml_viewpoint_name(self) -> &'static str`
- `layer_filter(self) -> Option<ElementLayer>`

### 2. YAML Não Suporta Viewpoint

**Localização:** `crates/archr-core/src/io/yaml.rs:16-23`

```rust
struct YamlModelInner {
    name: String,
    #[serde(default)]
    elements: Vec<YamlElement>,
    #[serde(default)]
    relationships: Vec<YamlRelationship>,
}
```

**Problema:** Não há campo `viewpoint` em `YamlModelInner`.

### 3. XML Gerado Não Inclui Atributo Viewpoint

**Localização:** `crates/archr-core/src/io/xml.rs:208-213**

```rust
let _ = writeln!(
    xml,
    "    <element xsi:type=\"archimate:ArchimateDiagramModel\" \
     name=\"Default View\" id=\"{}\">",  // ← Sem atributo viewpoint
    diagram_id
);
```

**Resultado atual:** O XML sempre gera `<element xsi:type="archimate:ArchimateDiagramModel" name="Default View" id="..."/>` sem o atributo `viewpoint`.

### 4. Erros do Arki (Archi)

Quando o arquivo .archimate é gerado sem o atributo viewpoint:
- **Erro:** Arki não aplica nenhum filtro de viewpoint
- **Resultado:** Tudo o que está no modelo aparece no diagrama, não filtrado por viewpoint
- **Comportamento:** Equivalente ao viewpoint "Layered" (tudo), mas sem essa especificação

## Melhorias Sugeridas

### 1. Adicionar Campo `viewpoint` ao Schema YAML

**Arquivo:** `crates/archr-core/src/io/yaml.rs`

```rust
struct YamlModelInner {
    name: String,
    #[serde(default)]  // ← Definir um viewpoint padrão
    viewpoint: Option<String>,  // ← NOVO CAMPO
    #[serde(default)]
    elements: Vec<YamlElement>,
    #[serde(default)]
    relationships: Vec<YamlRelationship>,
}
```

**Valores possíveis (segundo Viewpoint enum):**
```yaml
viewpoint: Business       # Layer-based: Business layer filter
viewpoint: Motivation      # Layer-based: Motivation layer filter
viewpoint: Strategy        # Layer-based: Strategy layer filter
viewpoint: Technology      # Layer-based: Technology layer filter
viewpoint: EnterpriseStructure  # Special: Enterprise Structure
viewpoint: ValueStream     # Special: Value Stream
viewpoint: Organization    # Mixin: Location + Business elements
viewpoint: BusinessProcessCooperation  # Mixin: Business + Application
viewpoint: Product         # Mixin: Business + Application + Artifact + Service
viewpoint: ApplicationCooperation   # Mixin: Location + Application
viewpoint: ApplicationUsage        # Mixin: Business elements + Application
viewpoint: Layered         # NONE: All elements (default)
```

**Validação a adicionar em `parse_yaml`:**
```rust
// Verificar se o viewpoint é válido
if let Some(vp_name) = &model.viewpoint {
    if !Viewpoint::from_yaml_viewpoint_name(vp_name).is_some() {
        return Err(vec![SchemaError::InvalidViewpoint(
            vp_name.clone()
        )]);
    }
}
```

### 2. Emitir Atributo Viewpoint no XML

**Arquivo:** `crates/archr-core/src/io/xml.rs`

```rust
fn emit_diagram(...) {
    let _ = writeln!(
        xml,
        "    <element xsi:type=\"archimate:ArchimateDiagramModel\" \
         name=\"{}\" id=\"{}\"{}>",  // ← Adicionar viewpoint
        "Default View", diagram_id,
        // Se houver viewpoint, adicionar atributo
        if let Some(vp_name) = model.viewpoint_name() {
            format!(" viewpoint=\"{}\"", vp_name)
        } else {
            String::new()
        }
    );
}
```

**Nota:** É necessário adicionar um método `viewpoint_name()` ao struct `Model` que retorne o nome do viewpoint atual.

### 3. Adicionar Viewpoint ao Struct Model

**Arquivo:** `crates/archr-core/src/model.rs`

```rust
pub struct Model {
    pub name: String,
    pub viewpoint: Option<Viewpoint>,  // ← NOVO CAMPO
    // ... restante dos campos
}
```

E atualizar `Model::new`:
```rust
pub fn new(name: &str) -> Self {
    Self {
        name: name.to_string(),
        viewpoint: None,  // ← Default: NONE (Layered)
        // ...
    }
}
```

### 4. Atualizar Documentação da Skill

**Arquivo:** `skill/SKILL.md`

Adicionar exemplos de uso de viewpoint:

```yaml
model:
  name: My Architecture
  viewpoint: Business       # ← NOVO CAMPO
  elements:
    - id: actor_001
      name: Customer
      kind: BusinessActor
    # ... outros elementos
```

Documentar os 14 tipos de viewpoint e seus significados.

## Resumo

**Erros do Arki:**
1. Arki não aplica filtros de viewpoint porque o XML não tem o atributo `viewpoint`
2. O viewpoint "Default View" sempre aparece sem especificação de filtro

**Necessário mudar:**
1. Adicionar campo `viewpoint: Option<String>` em `YamlModelInner`
2. Adicionar campo `viewpoint: Option<Viewpoint>` em `Model`
3. Adicionar validação de viewpoint no parser YAML
4. Emitir atributo `viewpoint` no XML quando definido
5. Documentar os 14 tipos de viewpoint na skill

**Impacto:**
- Com essas mudanças, o Arki será capaz de aplicar os filtros corretos de viewpoint
- O usuário poderá especificar diferentes viewpoints no YAML para gerar diagramas filtrados
- Compatibilidade com o padrão ArchiMate 3.2 para viewpoints
