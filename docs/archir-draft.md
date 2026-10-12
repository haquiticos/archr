Boa separação — e ela é o que define a arquitetura de longo prazo. Vou propor um critério de fronteira objetivo, classificar cada requisito e destacar os \*\*requisitos novos que essa separação revela para o core\*\* (a GUI "puxa" requisitos que o CLI sozinho não puxaria).

## Critério de fronteira

&gt; \*\*Core (archr)\*\*: tudo que é testável sem abrir janela — regras, estado persistido no arquivo, transformações. 

&gt; \*\*GUI\*\*: tudo que é estado de interação ou percepção visual. 

&gt; \*\*Teste do tailing\*\*: se faz sentido numa linguagem natural tipo "dado este modelo, retorne X", é core. Se depende de "o usuário arrastou", é GUI.

A GUI deve ser um \*\*cliente burro da lib\*\* — assim como o CLI é hoje. Isso garante que o MCP server, Web API e GUI compartilhem as mesmas regras 【turn0fetch0】.

## Classificação dos requisitos

### ✅ Core (archr) — implementável headless

| Requisito | Status / Nota |

|---|---|

| Metamodelo + validação de derivabilidade | ✅ Já existe, data-driven 【turn0fetch0】 |

| YAML ↔ XML Exchange bidirecional | ✅ Já existe 【turn0fetch0】 |

| Diff de modelos | ✅ Já existe 【turn0fetch0】 |

| Layout automático + persistência de coordenadas | ✅ Já existe (grid topológico) 【turn0fetch0】 |

| \*\*Projeção por viewpoint\*\* — dado um viewpoint, retornar o subgrafo válido | ✅ **Implementado** — `project(model, Viewpoint) -> Model` + `project_view` (viewpoints salvas); 4 viewpoints mixin indefinidos são recusados, não filtrados errado |

| \*\*Query engine\*\* (`archr query "...")` — impacto, dependências, caminhos, órfãos | ✅ **Implementado** — `impact`/`dependencies`/`path`/`orphans` no core + subcomando `archr query` |

| \*\*`can_connect(src, tgt, kind)`\*\* — validação granular de uma única conexão | ✅ **Implementado** — `archr_core::can_connect`, derivada da matriz `ALLOWED` 【turn0fetch0】 |

| **API transacional** (`apply(model, Command)`) | ✅ **Implementado** — `Command` serde-replayable, ids mintados pelo cliente; undo via snapshot stack |

| **Merge de modelos** (3-way, além do diff) | ⏸️ Adiado — YAGNI até haver segundo editor concorrente; diff cobre review |

| Verificação de integridade (refs quebradas, ciclos, elementos órfãos) | ❌ Novo |

| ~~Refactorings headless~~ | 🔀 Composição, não módulo — renomear com propagação = `query` (achar refs) + `apply` (N comandos), 5 linhas acima do core. Sai da fase 1.5 |

| Export SVG/HTML report (SVG é XML — headless puro) | ❌ Novo |

| Validação de regras de viewpoint (quais elementos/relações cada viewpoint permite) | ❌ Novo |

### 🖱️ GUI — só faz sentido interativo

| Requisito | Nota |

|---|---|

| Canvas: zoom, pan, marquee, seleção | Estado de UI puro |

| Drag &amp; drop de elementos / criação de conexão | O gesto é GUI; a \*\*validação é `can_connect` do core\*\* |

| Handles de resize, snapping, guides | Snapping à grid pode ser regra core exposta como dado |

| Bendpoints (pontos de dobra das arestas) | Dados persistem no XML (core); manipulação é GUI |

| Model tree visual | Estrutura já está no modelo; render é GUI |

| Properties panel + inline editing | Escrita via commands do core |

| Indicadores visuais de erro (borda vermelha em conexão ilegal) | Erro vem do core; apresentação é GUI |

| Undo/redo — \*\*a pilha\*\* | O core fornece commands/aplicação; histórico é da sessão GUI |

| Sketch views, wizards, preferências, workspace layout | 100% GUI |

### ⚠️ Armadilhas — o que \*\*não\*\* deve vazar para o core

- \*\*Seleção / multi-seleção\*\* → estado de UI, não do modelo

- \*\*Zoom / scroll\*\* → transformação de viewport

- \*\*Pilha de undo\*\* → a GUI mantém o histórico de commands; o core só sabe aplicar

- \*\*Cores, temas, fontes\*\* → apresentação

Se um dia `archr-core` compilar dependendo de algo de renderização, a fronteira quebrou.

## O contrato mínimo core ↔ GUI

### Identidade (decisão 2026-10-11)

A identidade pública do core é o **`original_id`** (o id que já round-tripa nos formatos YAML/XML), exposto como newtype `RefId(String)` — **decidido: opção (a)**. O `ElementId(usize)` (índice de arena) vira detalhe de implementação; o `Model` ganha um map interno `RefId → índice`.

Motivo: o índice de arena não sobrevive a delete — qualquer mutação renumera tudo. Com `RefId` estável: seleção da GUI, `Command` serializável/replayable e requisições stateless de MCP/API funcionam sem que o caller conheça regras de invalidação.

```rust
// O que a GUI consome do archr-core (sketch de API)

Model::from_yaml(...) / from_archimate(...)

core::can_connect(&model, src: RefId, tgt: RefId, kind) -> Result<(), ConnError>

core::apply(&model, Command) -> Result<Model, Error>   // Command é dado serde; ids mintados pelo cliente (uuid → RefId)

core::project(&model, Viewpoint) -> Model              // retorna Model filtrado, não um tipo SubGraph novo

core::layout(&model, engine) -> Vec<(RefId, Coord)>    // chaveado por id

core::validate(&model) -> Vec<Diagnostic>              // com posição, para mapear no canvas
```

Com isso, o undo/redo da GUI vira trivial: **snapshot stack** (`Vec<Model>`, clone antes de cada apply — modelos têm KBs). Sem `Command::invert`, sem inversão de diff.

## Visão arquitetural

\`\`\`mermaid

flowchart LR

    subgraph Core\["archr-core (lib headless)"\]

        M\[Model / Arena Graph\] --&gt; V\[Validate / can\_connect\]

        M --&gt; Q\[Query / Project\]

        M --&gt; T\[Apply Commands\]

        M --&gt; IO\[YAML ↔ XML\]

        M --&gt; L\[Layout\]

    end

    CLI\[CLI archr\] --&gt; Core

    MCP\[MCP Server\] --&gt; Core

    API\[Web API\] --&gt; Core

    GUI\[GUI Desktop/Web\] --&gt; Core

\`\`\`

Tudo que está no diagrama à esquerda é testável com `cargo test` — esse é o teste da fronteira.

## Efeito prático no roadmap do archr

A separação enriquece o "core complete" atual com uma **fase 1.5 antes da GUI**:

1. `RefId` como identidade pública (map interno `RefId → índice` no `Model`) + `can_connect` + `apply`/`Command` (desbloqueia qualquer GUI e melhora o skill dos agentes)

2. `project` por viewpoint + query engine (desbloqueia viewer, relatórios e refatorações por composição — ex.: renomear com propagação)

3. Aí sim o viewer GUI vira "só" renderizar o Model filtrado + coordenadas que o core já persiste

