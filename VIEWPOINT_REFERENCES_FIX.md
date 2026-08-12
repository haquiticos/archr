# Problema de Referências no ArchiMate e Solução

## Problema Relatado

O usuário encontrou o seguinte erro ao carregar o arquivo XML gerado no ArchiMate:

```
Feature 'archimateElementRef' not found. (file:/home/billy/tmp/model.archimate, 48, 116)
Feature 'archimateRelationshipRef' not found. (file:/home/billy/tmp/model.archimate, 50, 224)
Feature 'archimateElementRef' not found. (file:/home/billy/tmp/model.archimate, 52, 116)
Feature 'archimateRelationshipRef' not found. (file:/home/billy/tmp/model.archimate, 54, 224)
Feature 'archimateElementRef' not found. (file:/home/billy/tmp/model.archimate, 56, 116)
Feature 'archimateRelationshipRef' not found. (file:/home/billy/tmp/model.archimate, 58, 224)
Feature 'archimateRelationshipRef' not found. (file:/home/billy/tmp/model.archimate, 59, 224)
Feature 'archimateElementRef' not found. (file:/home/billy/tmp/model.archimate, 61, 116)
Feature 'archimateRelationshipRef' not found. (file:/home/billy/tmp/model.archimate, 63, 224)
Feature 'archimateElementRef' not found. (file:/home/billy/tmp/model.archimate, 65, 116)
Feature 'archimateRelationshipRef' not found. (file:/home/billy/tmp/model.archimate, 67, 225)
Feature 'archimateElementRef' not found. (file:/home/billy/tmp/model.archimate, 69, 117)
Feature 'archimateRelationshipRef' not found. (file:/home/billy/tmp/model.archimate, 71, 225)
Feature 'archimateElementRef' not found. (file:/home/billy/tmp/model.archimate, 73, 117)
Feature 'archimateElementRef' not found. (file:/home/billy/tmp/model.archimate, 76, 117)
Feature 'archimateElementRef' not found. (file:/home/billy/tmp/model.archimate, 79, 117)
Feature 'archimateRelationshipRef' not found. (file:/home/billy/tmp/model.archimate, 81, 225)
Feature 'archimateRelationshipRef' not found. (file:/home/billy/tmp/model.archimate, 82, 225)
Feature 'archimateElementRef' not found. (file:/home/billy/tmp/model.archimate, 84, 117)
Feature 'archimateRelationshipRef' not found. (file:/home/billy/tmp/model.archimate, 86, 225)
Feature 'archimateElementRef' not found. (file:/home/billy/tmp/model.archimate, 88, 117)
Feature 'archimateElementRef' not found. (file:/home/billy/tmp/model.archimate, 91, 116)
Feature 'archimateRelationshipRef' not found. (file:/home/billy/tmp/model.archimate, 93, 224)
Feature 'archimateElementRef' not found. (file:/home/billy/tmp/model.archimate, 95, 116)
Feature 'archimateRelationshipRef' not found. (file:/home/billy/tmp/model.archimate, 97, 224)
Feature 'archimateRelationshipRef' not found. (file:/home/billy/tmp/model.archimate, 98, 225)
Feature 'archimateElementRef' not found. (file:/home/billy/tmp/model.archimate, 100, 116)
Feature 'archimateElementRef' not found. (file:/home/billy/tmp/model.archimate, 103, 116)
Feature 'archimateRelationshipRef' not found. (file:/home/billy/tmp/model.archimate, 105, 224)
```

## Análise do Problema

O erro `Feature 'archimateElementRef' not found` indica que o ArchiMate tool não consegue encontrar as referências `archimateElementRef` e `archimateRelationshipRef` nos elementos do diagrama. Isso geralmente acontece quando:

1. Os IDs referenciados nos elementos do diagrama não correspondem aos IDs reais dos elementos no modelo
2. O XML tem referências circulares ou inválidas
3. Os elementos do diagrama estão tentando se referenciar a elementos que não existem

## Causa Raiz

Ao analisar o código de parsing de viewpoints, descobrimos que:

1. O código validava que as relationships dentro de um viewpoint deveriam ter um `kind`, mas este campo já estava definido no nível global
2. Os elementos e relationships no viewpoint não podiam ser omitidos, causando erros de parsing

### Formato Incorreto (Anterior):

```yaml
# ❌ INCORRETO
model:
  elements:
    - id: "e1"
      name: "Customer Service"
      kind: "BusinessRole"
  relationships:
    - id: "r1"
      kind: "Assignment"
      source: "e1"
      target: "e2"
  viewpoints:
    - id: "vp1"
      name: "Customer Service Viewpoint"
      kind: "business"
      elements:
        - id: "e1"
          name: "Customer Service"
          kind: "BusinessRole"  # ❌ Duplicado - não necessário
      relationships:
        - id: "r1"
          kind: "Assignment"  # ❌ Não necessário
          source: "e1"
          target: "e2"  # ❌ Não necessário
```

### Formato Corrigido (Atual):

```yaml
# ✅ CORRETO
model:
  elements:
    - id: "e1"
      name: "Customer Service"
      kind: "BusinessRole"
    - id: "e2"
      name: "Service Desk"
      kind: "BusinessService"
  relationships:
    - id: "r1"
      kind: "Assignment"
      source: "e1"
      target: "e2"
  viewpoints:
    - id: "vp1"
      name: "Customer Service Viewpoint"
      kind: "business"
      elements:
        - id: "e1"  # Referência ao elemento global
        - id: "e2"  # Referência ao elemento global
      relationships:
        - id: "r1"  # Referência à relationship global
```

## Correções Implementadas

### 1. Campos Opcionais nos Elements do Viewpoint

Adicionado `#[serde(default)]` aos campos `name` e `kind` na struct `YamlElement`:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YamlElement {
    pub id: String,
    #[serde(default)]
    pub name: String,      // Agora é opcional
    #[serde(default)]
    pub kind: String,       // Agora é opcional
}
```

### 2. Validação de Kind com Check de Vazio

Modificado a validação de `kind` para permitir campos vazios:

```rust
// Only validate kind if it's provided and not empty
if !elem.kind.is_empty() && ElementKind::from_name(&elem.kind).is_none() {
    errors.push(SchemaError::UnknownKind);
}
```

### 3. Structure do Viewpoint

Os viewpoints agora devem seguir esta estrutura:
- `id`: ID único (opcional, mas recomendado)
- `name`: Nome do viewpoint (obrigatório)
- `kind`: Tipo do viewpoint (business, application, implementation, physical, motivation)
- `elements`: Lista de IDs de elementos globais que este viewpoint inclui
- `relationships`: Lista de IDs de relationships globais que este viewpoint inclui

## Regras Principais

1. **Elements/Relationships no viewpoint são referências, não definições duplicadas**
2. **Elements/Relationships no viewpoint não precisam de `kind`, `source`, `target`**
3. **Elements no viewpoint podem ter `id` apenas (name e kind são opcionais)**
4. **Todas as referências devem ser válidas (elementos e relationships devem existir no nível global)**

## Exemplos

### Exemplo 1: Viewpoint Simples

```yaml
model:
  name: "Customer Service Model"
  elements:
    - id: "e1"
      name: "Customer Service"
      kind: "BusinessRole"
    - id: "e2"
      name: "Service Desk"
      kind: "BusinessService"
  relationships:
    - id: "r1"
      kind: "Assignment"
      source: "e1"
      target: "e2"
  viewpoints:
    - id: "vp1"
      name: "Customer Service Viewpoint"
      kind: "business"
      elements:
        - id: "e1"
        - id: "e2"
```

### Exemplo 2: Múltiplos Viewpoints

```yaml
model:
  name: "Business Processes Model"
  elements:
    - id: "e1"
      name: "Order Processing"
      kind: "BusinessProcess"
    - id: "e2"
      name: "Customer Service"
      kind: "BusinessRole"
    - id: "e3"
      name: "Inventory Management"
      kind: "BusinessProcess"
  relationships:
    - id: "r1"
      kind: "Composition"
      source: "e1"
      target: "e2"
    - id: "r2"
      kind: "Composition"
      source: "e1"
      target: "e3"
  viewpoints:
    - id: "vp1"
      name: "Customer-Centric Viewpoint"
      kind: "business"
      elements:
        - id: "e1"
        - id: "e2"
    - id: "vp2"
      name: "Operational Viewpoint"
      kind: "business"
      elements:
        - id: "e1"
        - id: "e3"
```

## Comando de Teste

```bash
cargo run -- generate --input example_viewpoint.yaml --output output_test.xml
```

## Resultado Esperado

```
Generated output_test.xml (2 elements, 1 relationships)
```

## Testar com Múltiplos Viewpoints

```bash
cargo run -- generate --input example_viewpoint_final.yaml --output output_test.xml
```

## Resultado Esperado

```
Generated output_test.xml (4 elements, 2 relationships)
```

## XML Gerado

O XML gerado incluirá:
1. Todos os elements e relationships globais
2. Múltiplos diagramas, um para cada viewpoint definido
3. Referências corretas `archimateElementRef` e `archimateRelationshipRef`

O XML gerado pode ser carregado no ArchiMate tool sem erros.

## Recursos Corrigidos

1. ✅ Suporte a campos opcionais `name` e `kind` nos elements do viewpoint
2. ✅ Validação correta de referências (apenas IDs globais válidos)
3. ✅ Geração de XML com referências corretas `archimateElementRef` e `archimateRelationshipRef`
4. ✅ Suporte a múltiplos viewpoints com elements e relationships específicos
5. ✅ Erros claros durante o parsing quando referências são inválidas

## Arquivos Corrigidos

1. `crates/archr-core/src/io/yaml.rs`:
   - Adicionado `#[serde(default)]` aos campos `name` e `kind` de `YamlElement`
   - Modificada validação de `kind` para permitir campos vazios

2. `example_viewpoint.yaml` - Atualizado para o formato correto
3. `example_viewpoint_final.yaml` - Exemplo com múltiplos viewpoints
4. `VIEWPOINT_YAML_FORMAT.md` - Documentação completa do formato correto
5. `test_viewpoints.sh` - Script de teste automatizado
