# Estrutura YAML Aceita pelo archr-core

## Estrutura Completa

```yaml
model:
  name: "Nome do Modelo"
  viewpoint: business        # Opcional: business, application, implementation, motivation, compliance, ou null
  elements:                  # Obrigatório
    - id: "e1"                # Obrigatório: identificador único do elemento
      name: "Nome do Elemento" # Obrigatório: nome do elemento
      kind: "BusinessActor"   # Obrigatório: tipo do elemento (BusinessActor, BusinessRole, etc.)
    - id: "e2"
      name: "Outro Elemento"
      kind: "BusinessService"
  relationships:             # Obrigatório
    - id: "r1"                # Obrigatório: identificador único da relação
      kind: "Composition"     # Obrigatório: tipo da relação (Composition, Assignment, etc.)
      source: "e1"            # Obrigatório: ID do elemento de origem
      target: "e2"            # Obrigatório: ID do elemento de destino
```

## Campos por Nível

### 1. Nível Raiz: `model` (Obrigatório)
Um objeto YAML que contém todos os dados do modelo. Este é o nível esperado pelo parser.

### 2. Campo `name` (Obrigatório)
- **Tipo**: String
- **Descrição**: Nome do modelo
- **Exemplo**: `"Test Model"`

### 3. Campo `viewpoint` (Opcional)
- **Tipo**: String ou null
- **Valores aceitos**: `none`, `business`, `application`, `implementation`, `motivation`, `compliance`, ou `null`
- **Descrição**: Categoriza o modelo por uma perspectiva ArchiMate
- **Padrão**: `null` (será preenchido como `None` se não especificado)
- **Nota**: O campo é serializado como `null` quando não especificado

### 4. Campo `elements` (Obrigatório)
- **Tipo**: Array de objetos
- **Descrição**: Lista de elementos do modelo
- **Cada elemento deve ter**:
  - `id` (String): identificador único
  - `name` (String): nome do elemento
  - `kind` (String): tipo do elemento

**Tipos de elemento (`kind`)**:
- `BusinessActor`, `BusinessRole`, `BusinessCollaboration`, `BusinessInterface`
- `BusinessProcess`, `BusinessFunction`, `BusinessInteraction`, `BusinessEvent`
- `BusinessService`, `BusinessObject`, `Contract`, `Representation`, `Product`
- `ApplicationComponent`, `ApplicationCollaboration`, `ApplicationInterface`
- `ApplicationFunction`, `ApplicationProcess`, `ApplicationInteraction`, `ApplicationEvent`
- `ApplicationService`, `DataObject`
- `Node`, `Device`, `SystemSoftware`, `TechnologyCollaboration`, `TechnologyInterface`
- `Path`, `CommunicationNetwork`, `Artifact`
- `TechnologyFunction`, `TechnologyProcess`, `TechnologyInteraction`, `TechnologyEvent`, `TechnologyService`
- `Equipment`, `Facility`, `Material`, `DistributionNetwork`
- `WorkPackage`, `Deliverable`, `ImplementationEvent`, `Plateau`, `Gap`
- `Grouping`, `Location`, `AndJunction`, `OrJunction`

### 5. Campo `relationships` (Obrigatório)
- **Tipo**: Array de objetos
- **Descrição**: Lista de relações entre elementos
- **Cada relação deve ter**:
  - `id` (String): identificador único
  - `kind` (String): tipo da relação
  - `source` (String): ID do elemento de origem
  - `target` (String): ID do elemento de destino

**Tipos de relação (`kind`)**:
- `Composition`, `Aggregation`, `Assignment`, `Realization`
- `Access`, `Serving`, `Influence`, `Association`
- `Triggering`, `Flow`
- `Specialization`

## Exemplos

### Exemplo 1: Modelo Básico (sem viewpoint)
```yaml
model:
  name: "Modelo Simples"
  elements:
    - id: "e1"
      name: "Cliente"
      kind: "BusinessActor"
  relationships:
    - id: "r1"
      kind: "Assignment"
      source: "e1"
      target: "e1"
```

### Exemplo 2: Modelo com Viewpoint Business
```yaml
model:
  name: "Viewpoint Business"
  viewpoint: business
  elements:
    - id: "e1"
      name: "Time de Suporte"
      kind: "BusinessRole"
    - id: "e2"
      name: "Canal de Atendimento"
      kind: "BusinessService"
  relationships:
    - id: "r1"
      kind: "Assignment"
      source: "e1"
      target: "e2"
```

### Exemplo 3: Modelo com Viewpoint Application
```yaml
model:
  name: "Viewpoint Application"
  viewpoint: application
  elements:
    - id: "e1"
      name: "Portal Web"
      kind: "ApplicationComponent"
    - id: "e2"
      name: "API Gateway"
      kind: "ApplicationComponent"
  relationships:
    - id: "r1"
      kind: "Composition"
      source: "e1"
      target: "e2"
```

### Exemplo 4: Modelo Complexo com Múltiplos Elementos
```yaml
model:
  name: "Modelo Completo"
  viewpoint: implementation
  elements:
    - id: "e1"
      name: "Sistema CRM"
      kind: "ApplicationComponent"
    - id: "e2"
      name: "Banco de Dados"
      kind: "DataObject"
    - id: "e3"
      name: "Equipe de TI"
      kind: "BusinessRole"
    - id: "e4"
      name: "API REST"
      kind: "ApplicationComponent"
  relationships:
    - id: "r1"
      kind: "Composition"
      source: "e1"
      target: "e4"
    - id: "r2"
      kind: "Association"
      source: "e1"
      target: "e2"
    - id: "r3"
      kind: "Assignment"
      source: "e3"
      target: "e1"
```

## Regras Importantes

1. **Campos Obrigatórios**: Todos os campos indicados como obrigatórios devem estar presentes.
2. **Identificadores Únicos**: Cada `id` em `elements` e `relationships` deve ser único.
3. **Referências Válidas**: Cada referência em `relationships.source` e `relationships.target` deve existir em `elements`.
4. **Tipos Corretos**: Os valores de `kind` devem ser tipos válidos de ElementKind e RelationKind.
5. **Viewpoint Opcional**: O campo `viewpoint` é totalmente opcional e não afeta a validação do modelo.
6. **Case Insensitive**: Os valores do campo `viewpoint` são case-insensitive (serão convertidos para lowercase).

## Valores de Viewpoint

- **`none`** (ou não especificado): Sem viewpoint específico
- **`business`**: Perspectiva de negócio
- **`application`**: Perspectiva de aplicação
- **`implementation`**: Perspectiva de implementação
- **`motivation`**: Perspectiva de motivação
- **`compliance`**: Perspectiva de conformidade

## Formato de Saída

Quando um modelo é serializado de volta para YAML, o campo `viewpoint` sempre será incluído com o valor correspondente (ou `null` se for `None`):
```yaml
model:
  name: "Nome do Modelo"
  viewpoint: null  # ou o valor especificado
  elements: [...]
  relationships: [...]
```

## Notas sobre o Parser

O parser YAML do archr-core usa `serde` para (de)serialização e inclui validação de esquema no nível da estrutura. Qualquer erro de validação retornará um `ParseResult` com uma lista de `SchemaError`.
