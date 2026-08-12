# Formato Correto do YAML para Viewpoints

## Estrutura Correcta

O formato do YAML para viewpoints deve seguir esta estrutura:

```yaml
model:
  name: "Nome do Modelo"
  elements:
    - id: "e1"
      name: "Elemento 1"
      kind: "BusinessRole"
    - id: "e2"
      name: "Elemento 2"
      kind: "BusinessService"
  relationships:
    - id: "r1"
      kind: "Assignment"
      source: "e1"
      target: "e2"
  viewpoints:
    - id: "vp1"
      name: "Nome do Viewpoint"
      kind: "business"
      elements:
        - id: "e1"  # Referência ao elemento global
        - id: "e2"  # Referência ao elemento global
```

## Regras Importantes

### 1. Elements no Nível Global

Os elements devem ser definidos no nível global com todos os campos obrigatórios:
- `id`: ID único do elemento
- `name`: Nome do elemento (opcional, mas recomendado)
- `kind`: Tipo do elemento (BusinessRole, BusinessService, ApplicationComponent, etc.)

```yaml
elements:
  - id: "e1"
    name: "Customer Service"
    kind: "BusinessRole"
```

### 2. Relationships no Nível Global

As relationships devem ser definidas no nível global com todos os campos obrigatórios:
- `id`: ID único da relationship
- `kind`: Tipo da relationship (Assignment, Realization, Composition, etc.)
- `source`: ID do elemento de origem
- `target`: ID do elemento de destino

```yaml
relationships:
  - id: "r1"
    kind: "Assignment"
    source: "e1"
    target: "e2"
```

### 3. Viewpoints

Os viewpoints são listas de elementos e relationships globais:

```yaml
viewpoints:
  - id: "vp1"
    name: "Nome do Viewpoint"
    kind: "business"  # business, application, implementation, physical, motivation
    elements:
      - id: "e1"  # Referência a element global
      - id: "e2"  # Referência a element global
    relationships:
      - id: "r1"  # Referência a relationship global
```

#### Regras dos Viewpoints:

- **`id`**: ID único do viewpoint (opcional, mas recomendado)
- **`name`**: Nome do viewpoint (obrigatório)
- **`kind`**: Tipo do viewpoint (business, application, implementation, physical, motivation)
- **`elements`**: Lista de IDs de elementos globais que este viewpoint inclui
- **`relationships`**: Lista de IDs de relationships globais que este viewpoint inclui

#### Diferença Importante:

1. **Elements e Relationships no viewpoint NÃO devem ser duplicados** - eles são apenas referências aos elements e relationships globais
2. **Elements e Relationships no viewpoint NÃO precisam de `kind`** - pois já estão definidos no nível global
3. **Elements e Relationships no viewpoint NÃO precisam de `source` e `target`** - pois já estão definidos no nível global

### 4. Campos Opcionais

Nos elements do viewpoint, os campos podem ser omitidos:

```yaml
viewpoints:
  - id: "vp1"
    elements:
      - id: "e1"  # name e kind são opcionais aqui
```

Isso é suportado porque os campos `name` e `kind` têm `#[serde(default)]` nos structs.

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

### Exemplo 3: Viewpoints com Distintos Tipos

```yaml
model:
  name: "Complete Architecture Model"
  elements:
    - id: "e1"
      name: "Web Server"
      kind: "ApplicationComponent"
    - id: "e2"
      name: "Database"
      kind: "ApplicationComponent"
    - id: "e3"
      name: "Security System"
      kind: "ApplicationComponent"
  relationships:
    - id: "r1"
      kind: "Serving"
      source: "e1"
      target: "e2"
    - id: "r2"
      kind: "Realization"
      source: "e3"
      target: "e1"
  viewpoints:
    - id: "vp1"
      name: "Application Viewpoint"
      kind: "application"
      elements:
        - id: "e1"
        - id: "e2"
        - id: "e3"
```

## Erros Comuns

### Erro 1: Duplicar Elements/Relationships no Viewpoint

```yaml
# ❌ INCORRETO - duplica a definition
viewpoints:
  - id: "vp1"
    elements:
      - id: "e1"
        name: "Customer Service"
        kind: "BusinessRole"
    relationships:
      - id: "r1"
        kind: "Assignment"
        source: "e1"
        target: "e2"
```

### Erro 2: Tentar referenciar elementos que não existem

```yaml
# ❌ INCORRETO - e3 não existe no nível global
viewpoints:
  - id: "vp1"
    elements:
      - id: "e1"
    relationships:
      - id: "r1"
        source: "e1"
        target: "e3"  # e3 não está definido no nível global
```

### Erro 3: Usar `kind` em relationships do viewpoint

```yaml
# ❌ INCORRETO - kind não é necessário
viewpoints:
  - id: "vp1"
    relationships:
      - id: "r1"
        kind: "Assignment"  # kind não é necessário aqui
        source: "e1"
        target: "e2"
```

## Gerar XML

Use o comando abaixo para gerar o XML a partir de um modelo YAML com viewpoints:

```bash
cargo run -- generate --input example_viewpoint.yaml --output output.archimate
```

O XML gerado incluirá múltiplos diagramas, um para cada viewpoint definido no modelo.

## Gerar XML de Teste

```bash
cargo run -- generate --input example_viewpoint_final.yaml --output output_test.xml
```

## Validar o XML

O XML gerado pode ser carregado no ArchiMate tool. Certifique-se de que:

1. Os IDs referenciados nos elementos do diagrama correspondem aos IDs dos elementos no modelo
2. Não há referências circulares ou inválidas
3. Todos os `archimateElementRef` e `archimateRelationshipRef` referenciam elementos que existem
