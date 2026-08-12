# Estrutura YAML Completa com Tipos Válidos

## Estrutura Básica

```yaml
model:
  name: "Nome do Modelo"
  viewpoint: business        # Opcional: none, business, application, implementation, motivation, compliance
  elements:                  # Obrigatório
    - id: "e1"                # Identificador único
      name: "Nome do Elemento" # Nome do elemento
      kind: "BusinessActor"   # Tipo do elemento
  relationships:             # Obrigatório
    - id: "r1"                # Identificador único
      kind: "Composition"     # Tipo da relação
      source: "e1"            # ID do elemento de origem
      target: "e2"            # ID do elemento de destino
```

## ElementKind - Tipos de Elemento Válidos

### Motivation (10)
- `Stakeholder`
- `Driver`
- `Assessment`
- `Goal`
- `Outcome`
- `Principle`
- `Requirement`
- `Constraint`
- `Meaning`
- `Value`

### Strategy (4)
- `Resource`
- `Capability`
- `ValueStream`
- `CourseOfAction`

### Business (13)
- `BusinessActor`
- `BusinessRole`
- `BusinessCollaboration`
- `BusinessInterface`
- `BusinessProcess`
- `BusinessFunction`
- `BusinessInteraction`
- `BusinessEvent`
- `BusinessService`
- `BusinessObject`
- `Contract`
- `Representation`
- `Product`

### Application (9)
- `ApplicationComponent`
- `ApplicationCollaboration`
- `ApplicationInterface`
- `ApplicationFunction`
- `ApplicationProcess`
- `ApplicationInteraction`
- `ApplicationEvent`
- `ApplicationService`
- `DataObject`

### Technology (13)
- `Node`
- `Device`
- `SystemSoftware`
- `TechnologyCollaboration`
- `TechnologyInterface`
- `Path`
- `CommunicationNetwork`
- `Artifact`
- `TechnologyFunction`
- `TechnologyProcess`
- `TechnologyInteraction`
- `TechnologyEvent`
- `TechnologyService`

## RelationKind - Tipos de Relação Válidos

### Structural (4)
- `Composition`
- `Aggregation`
- `Assignment`
- `Realization`

### Dependency (4)
- `Serving`
- `Access`
- `Influence`
- `Association`

### Dynamic (2)
- `Triggering`
- `Flow`

### Other (1)
- `Specialization`

## Viewpoint - Valores Válidos

- `none` (ou não especificado)
- `business`
- `application`
- `implementation`
- `motivation`
- `compliance`

## Exemplo Completo

```yaml
model:
  name: "Modelo Arquitetural Completo"
  viewpoint: business

  elements:
    # Motivation Layer
    - id: "m1"
      name: "Cliente Principal"
      kind: "Stakeholder"
    - id: "m2"
      name: "Meta de Crescimento"
      kind: "Goal"

    # Strategy Layer
    - id: "s1"
      name: "Recurso CRM"
      kind: "Resource"
    - id: "s2"
      name: "Capacidade de Automação"
      kind: "Capability"

    # Business Layer
    - id: "b1"
      name: "Equipe de Suporte"
      kind: "BusinessRole"
    - id: "b2"
      name: "Portal de Serviços"
      kind: "BusinessService"
    - id: "b3"
      name: "Time de Projetos"
      kind: "BusinessActor"

    # Application Layer
    - id: "a1"
      name: "Sistema CRM"
      kind: "ApplicationComponent"
    - id: "a2"
      name: "API de Autenticação"
      kind: "ApplicationInterface"
    - id: "a3"
      name: "Base de Dados"
      kind: "DataObject"

    # Technology Layer
    - id: "t1"
      name: "Servidor Principal"
      kind: "Node"
    - id: "t2"
      name: "Load Balancer"
      kind: "Device"

  relationships:
    # Motivation Layer
    - id: "r1"
      kind: "Specialization"
      source: "m1"
      target: "s2"

    # Business Layer
    - id: "r2"
      kind: "Assignment"
      source: "b1"
      target: "a1"
    - id: "r3"
      kind: "Serving"
      source: "b2"
      target: "b1"
    - id: "r4"
      kind: "Composition"
      source: "b3"
      target: "b1"

    # Application Layer
    - id: "r5"
      kind: "Composition"
      source: "a1"
      target: "a2"
    - id: "r6"
      kind: "Association"
      source: "a1"
      target: "a3"

    # Technology Layer
    - id: "r7"
      kind: "Serving"
      source: "t1"
      target: "a1"
    - id: "r8"
      kind: "Composition"
      source: "t2"
      target: "t1"
```

## Regras de Validação

1. **Todos os campos obrigatórios devem estar presentes**
   - `model.name`
   - `model.elements` (array não vazio)
   - `model.relationships` (array não vazio)
   - Cada elemento: `id`, `name`, `kind`
   - Cada relação: `id`, `kind`, `source`, `target`

2. **Identificadores únicos**
   - Todos os `id` em `elements` devem ser únicos
   - Todos os `id` em `relationships` devem ser únicos

3. **Referências válidas**
   - Todos os `source` e `target` devem referenciar elementos existentes em `elements`
   - IDs devem estar entre aspas duplas: `"e1"`

4. **Tipos válidos**
   - `kind` em elementos deve ser um valor de `ElementKind`
   - `kind` em relações deve ser um valor de `RelationKind`

5. **Viewpoint opcional**
   - Pode ser omitido
   - Se presente, deve ser um dos valores válidos (case-insensitive)
   - O valor `none` é equivalente a não especificar

6. **Caminhos de elementos**
   - Opcional: `elements` pode estar vazio `[]`
   - Opcional: `relationships` pode estar vazio `[]`

## Formato de Saída

Quando um modelo é serializado de volta para YAML:

```yaml
model:
  name: "Nome do Modelo"
  viewpoint: null           # Será null se viewpoint for None
  elements: []
  relationships: []
```

ou

```yaml
model:
  name: "Nome do Modelo"
  viewpoint: business       # Será convertido para lowercase
  elements: [...]
  relationships: [...]
```

## Comparação com Formato XML

A estrutura YAML é equivalente ao formato XML do ArchiMate:

**XML:**
```xml
<model name="Nome do Modelo">
  <elements>
    <element id="e1" name="Nome" kind="BusinessActor"/>
  </elements>
  <relationships>
    <relationship id="r1" kind="Composition" source="e1" target="e2"/>
  </relationships>
</model>
```

**YAML:**
```yaml
model:
  name: "Nome do Modelo"
  elements:
    - id: "e1"
      name: "Nome"
      kind: "BusinessActor"
  relationships:
    - id: "r1"
      kind: "Composition"
      source: "e1"
      target: "e2"
```
