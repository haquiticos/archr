# Atualização de Viewpoint no YAML

## Mudança de Estrutura

O viewpoint no YAML deve mudar de `Option<String>` para um objeto com:
```yaml
model:
  name: Business Process View
  viewpoint:
    name: Business
    elements:
      - BusinessActor
      - BusinessRole
      - BusinessService
      - Location
  elements:
    - id: e1
      name: Org
      kind: BusinessActor
    - id: e2
      name: Service
      kind: BusinessService
```

## ElementKind por Viewpoint (baseado em viewpoints.xml)

### Business-layer based
- **Business**: BusinessActor, BusinessCollaboration, BusinessInterface, BusinessRole, BusinessProcess, BusinessFunction, BusinessInteraction, BusinessEvent, BusinessService, BusinessObject, Contract, Representation
- **Other**: Grouping, Location, AndJunction, OrJunction

### Application-layer based  
- **Application**: ApplicationComponent, ApplicationCollaboration, ApplicationInterface, ApplicationFunction, ApplicationProcess, ApplicationInteraction, ApplicationEvent, ApplicationService, DataObject
- **Implementation and Migration**: ApplicationComponent, ApplicationCollaboration, ApplicationInterface, ApplicationFunction, ApplicationProcess, ApplicationInteraction, ApplicationEvent, ApplicationService, DataObject, Artifact, Path, SystemSoftware, TechnologyFunction, TechnologyInteraction, TechnologyInterface, TechnologyProcess, TechnologyService

### Technology-layer based
- **Technology**: Node, Device, SystemSoftware, TechnologyCollaboration, TechnologyInterface, Path, CommunicationNetwork, Artifact, TechnologyFunction, TechnologyProcess, TechnologyInteraction, TechnologyEvent, TechnologyService
- **Physical**: Node, Device, SystemSoftware, TechnologyCollaboration, TechnologyInterface, Path, CommunicationNetwork, Artifact, TechnologyFunction, TechnologyProcess, TechnologyInteraction, TechnologyEvent, TechnologyService, Equipment, Facility, Material, DistributionNetwork, Location

### Special
- **Motivation**: Stakeholder, Driver, Assessment, Goal, Outcome, Principle, Requirement, Constraint, Meaning, Value
- **Strategy**: Resource, Capability, ValueStream, CourseOfAction, Outcome
- **EnterpriseStructure**: Business-layer + Application-layer elements
- **ValueStream**: Capability, Outcome, Stakeholder, ValueStream

### Mixin viewpoints (add extra elements to base)
- **Organization**: BusinessActor, BusinessCollaboration, BusinessInterface, BusinessRole, Location
- **BusinessProcessCooperation**: Business-layer + Application-layer elements + Location + Representation
- **Product**: Business-layer + Application-layer + Artifact + TechnologyService + Value
- **ApplicationCooperation**: Application-layer + Location
- **ApplicationUsage**: Application-layer + Business-layer elements
- **Layered**: all elements allowed

## Validação

Durante o parse, deve-se validar que TODOS os elementos do modelo pertencem aos ElementKind definidos no viewpoint (ou Layered que permite todos).
