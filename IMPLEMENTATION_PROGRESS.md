# Implementação de Suporte a Viewpoints - Progresso

## Passo 1: Adicionar campo viewpoints ao Model (model.rs) - COMPLETO ✓

### Mudanças Realizadas:

1. **Adicionar importação** ✓
   - `use crate::io::yaml::YamlViewpointDefinition;`

2. **Atualizar struct Model** ✓
   - Adicionado campo: `viewpoints: Vec<YamlViewpointDefinition>,`

3. **Atualizar Model::new()** ✓
   - Adicionado inicialização: `viewpoints: Vec::new(),`

4. **Atualizar element_count()** ✓
   - Agora retorna: `self.elements.len() + self.viewpoints.len()`

5. **Tornar YamlViewpointDefinition pública** ✓
   - `struct YamlViewpointDefinition` → `pub struct YamlViewpointDefinition`

### Status:
- Compilação: OK ✓ (apenas 1 warning: variável viewpoints não usada ainda)

## Passo 2: Preservar viewpoints durante parsing (yaml.rs) - COMPLETO ✓

### Mudanças Realizadas:

1. **Atualizar YamlParseResult type** ✓
   - Agora retorna: `(Model, HashMap<String, ElementId>, HashMap<String, RelationId>, Vec<YamlViewpointDefinition>)`

2. **Atualizar parse_yaml_with_ids()** ✓
   - Agora retorna os viewpoints no resultado

3. **Atualizar parse_yaml()** ✓
   - Agora retorna os viewpoints no resultado

4. **Atualizar main.rs** ✓
   - Agora recebe a variável viewpoints, mas ainda não a usa

5. **Corrigir testes** ✓
   - Corrigido test_malformed_yaml_not_invalid_id que estava corrompido

### Status:
- Compilação: OK ✓
- Próximo: Passo 3

## Passo 3: Modificar XML emission para múltiplos diagramas (xml.rs) - INICIANDO

### Mudanças Necessárias:

1. **Modificar emit_diagram()** 
   - Criar função `emit_viewpoint_diagram()` para gerar diagramas por viewpoint
   - Adicionar loop para iterar sobre viewpoints
   - Para cada viewpoint, filtrar elementos e relationships
   - Gerar um ArchimateDiagramModel por viewpoint

2. **Atualizar função de parsing do YAML no xml.rs**
   - Usar viewpoints para filtrar elementos e relationships

### Status:
- Em progresso
