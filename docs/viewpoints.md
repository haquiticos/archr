# Viewpoints

Viewpoints filtram um modelo ArchiMate para focar em um aspecto específico (camada, processo, domínio). Definidos em YAML, validados pelo mesmo `archr validate`.

> Veja [`docs/schema.yaml`](schema.yaml) para o schema completo de referência do formato YAML.

## Kinds suportados

| Kind | Descrição |
|------|-----------|
| `none` | Geral — todos os elementos |
| `motivation` | Camada de motivação (drivers, requirements, goals, stakeholders) |
| `business` | Camada de negócio (actors, roles, processes) |
| `application` | Camada de aplicação (components, services, data) |
| `implementation` | Camada de implementação/migração (work packages, deliverables) |
| `compliance` | Conformidade |

## Estrutura YAML

```yaml
model:
  name: My Architecture
  elements:
    - id: e1
      name: Customer
      kind: BusinessActor
    - id: e2
      name: CRM
      kind: ApplicationComponent
    - id: e3
      name: Order Process
      kind: BusinessProcess
  relationships:
    - id: r1
      source: e1
      target: e3
      kind: Assignment
    - id: r2
      source: e2
      target: e3
      kind: Serving
  viewpoints:
    - id: vp_business
      name: Business Viewpoint
      kind: business
      elements:
        - e1
        - e3
      relationships:
        - r1
    - id: vp_app
      name: Application Viewpoint
      kind: application
      elements:
        - e2
      relationships: []
```

### Campos de viewpoint

| Campo | Obrigatório | Descrição |
|-------|-------------|-----------|
| `id` | sim | Identificador único (sem espaços) |
| `name` | sim | Nome legível |
| `kind` | sim | Um dos kinds da tabela acima (lowercase) |
| `elements` | sim | Lista de **IDs** de elementos (strings) — cada ID deve existir em `model.elements` |
| `relationships` | não | Lista de **IDs** de relacionamentos (strings) — cada ID deve existir em `model.relationships` |

### Regras

- `elements` e `relationships` são **listas de strings** (IDs de referência), não objetos.
- IDs em `elements` devem existir em `model.elements`.
- IDs em `relationships` devem existir em `model.relationships`.
- Elementos/relacionamentos não listados na viewpoint não são visíveis.
- `kind` determina o filtro semântico; `none` inclui tudo.

## Validação

```bash
archr validate --input model.yaml
```

Exit 0 = válido, 1 = erros de validação (JSON em stdout), 2 = I/YAML.

## Exemplos

### Viewpoint minimal

```yaml
model:
  name: Minimal
  elements:
    - id: e1
      name: Driver
      kind: Driver
  viewpoints:
    - id: vp1
      name: Minimal Viewpoint
      kind: none
      elements:
        - e1
      relationships: []
```

### Múltiplas viewpoints

```yaml
model:
  name: Multi-VP
  elements:
    - id: e1
      name: Driver
      kind: Driver
    - id: e2
      name: Customer
      kind: BusinessActor
    - id: e3
      name: CRM
      kind: ApplicationComponent
  relationships:
    - id: r1
      source: e1
      target: e2
      kind: Assignment
  viewpoints:
    - id: vp_motiv
      name: Motivation Viewpoint
      kind: motivation
      elements:
        - e1
      relationships: []
    - id: vp_biz
      name: Business Viewpoint
      kind: business
      elements:
        - e2
      relationships: []
    - id: vp_app
      name: Application Viewpoint
      kind: application
      elements:
        - e3
      relationships: []
```

## Checklist de validação

- [ ] Todos os IDs em `viewpoints[].elements` existem em `model.elements`
- [ ] Todos os IDs em `viewpoints[].relationships` existem em `model.relationships`
- [ ] `kind` é válido (`none`, `motivation`, `business`, `application`, `implementation`, `compliance`)
- [ ] `archr validate --input model.yaml` retorna exit 0
