# ArchiMate 3.2 Derivability Rules

## Overview

ArchiMate 3.2 defines 8 layers and 11 relationship types. Derivability rules govern which relationships are valid between which layers.

## 8 Layers

1. **Motivation** - Goals, requirements, drivers, constraints
2. **Strategy** - Roadmaps, principles, KPIs, goals
3. **Business** - Business actors, functions, processes, events
4. **Application** - Application components, interfaces, services
5. **Technology** - Infrastructure components, systems, networks
6. **Physical** - Hardware, locations, facilities, devices
7. **Implementation** - Projects, deliverables, migration plans
8. **Other** - Concepts, artifacts, principles

## 11 Relationship Types

### Structural Relationships (4)
- **Composition** - Whole-part relationship
- **Aggregation** - Cluster relationship
- **Assignment** - Usage/ownership relationship
- **Realization** - Implementation of behavior

### Dependency Relationships (4)
- **Serving** - Serves a behavior/function
- **Access** - Provides access to a service/resource
- **Influence** - Affects the course of events
- **Association** - General relationship

### Dynamic Relationships (2)
- **Triggering** - Initiates a behavior
- **Flow** - Transfers something between elements

### Other Relationships (1)
- **Specialization** - Generalization relationship

## Derivability Rules Summary

### Same Layer Combinations
All 11 relationship types are allowed between elements of the **same layer**:
- Business → Business
- Application → Application
- Technology → Technology
- Physical → Physical
- Implementation → Implementation
- Other → Other

### Motivation Layer Combinations
**Motivation** can only relate to **Core** layers (Business, Application, Technology) using **Association**:

| Source Layer | Target Layer | Allowed Relations |
|--------------|--------------|-------------------|
| Motivation   | Motivation   | Association       |
| Motivation   | Business     | Association       |
| Motivation   | Application  | Association       |
| Motivation   | Technology   | Association       |

### Motivation → Core (Any Relation)
**Motivation** elements can be related to **Core** elements (Business, Application, Technology) using **only Association**:
- Goal → BusinessFunction (Association)
- Requirement → ApplicationComponent (Association)
- Driver → TechnologyInfrastructure (Association)

### Core Layer Combinations (Business, Application, Technology)
All 11 relationship types are allowed between **Core** layers:
- Business → Business (all 11)
- Business → Application (all 11)
- Business → Technology (all 11)
- Application → Business (all 11)
- Application → Application (all 11)
- Application → Technology (all 11)
- Technology → Business (all 11)
- Technology → Application (all 11)
- Technology → Technology (all 11)

### Serving Relationship
**Serving** (Dependency) must be directed **downward** from **Core** to **Core**:
- BusinessActor → BusinessFunction (Serves)
- ApplicationComponent → BusinessFunction (Serves)
- BusinessFunction → ApplicationComponent (Accesses)
- ApplicationFunction → ApplicationComponent (Accesses)

### Access Relationship
**Access** (Dependency) must be directed **downward** from **Application** to **Technology**:
- ApplicationInterface → TechnologyInterface (Access)
- ApplicationService → TechnologyService (Access)

### Influence Relationship
**Influence** (Dependency) must be directed **downward** from **Motivation** or **Core**:
- MotivationElement → AnyCoreElement (Influences)
- BusinessActor → BusinessFunction (Influences)
- ApplicationComponent → BusinessFunction (Influences)

### Association Relationship
**Association** (Dependency) is the most permissive and can be used:
- Between **any** two layers
- In **any direction**
- For most relationship types where other rules don't apply

### Structural Relationships (Composition, Aggregation, Assignment, Realization)
Structural relationships follow the layer rules above but have specific semantics:
- **Composition** - Exclusive whole/part (must not overlap)
- **Aggregation** - Optional whole/part (may overlap)
- **Assignment** - Usage/ownership of a behavior
- **Realization** - Implementation of behavior by structure

### Dynamic Relationships (Triggering, Flow)
Dynamic relationships also follow the layer rules above but apply to behavioral elements:
- **Triggering** - Initiates a behavior (same layer)
- **Flow** - Transfers value/data (same layer)

### Specialization Relationship
**Specialization** (Generalization) can only relate **same layer** elements:
- Same layer, different specialization levels
