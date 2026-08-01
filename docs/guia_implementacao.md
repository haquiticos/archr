# GUIA DE IMPLEMENTAÇÃO: `archr` — Motor Headless ArchiMate + Skill de IA

> **Versão 2.1** — Motor Rust (CLI) + Skill de IA em **Monorepo** com isolamento estrito de código. A Skill segue a [Agent Skills Specification](https://agentskills.io/specification): `SKILL.md` + `scripts/archr.py` autocontido (PEP 723), compatível com VS Code Copilot, Claude Code e OpenAI Codex.

---

## 1. VISÃO GERAL DA ARQUITETURA

O `archr` é uma ferramenta **standalone**, escrita em Rust, que atua como motor de validação, manipulação e exportação de modelos ArchiMate 3.2, projetada desde o início para integração nativa com **Agentes de IA** via uma **Skill** (wrapper Python/Node).

A decisão central da v2: **motor Rust e Skill de IA vivem no mesmo repositório (Monorepo)**, com isolamento estrito de código. O acoplamento de schema entre os dois é o maior risco da arquitetura — qualquer drift entre o que a Skill ensina e o que o Rust valida produz erros de validação difíceis de debugar. Monorepo torna cada release atômica.

**Fluxo de Dados Hierárquico:**

```text
[Usuário]
   |
   v
[Agente IA] --> Gera [Arquivo YAML Intermediário]
   |
   v
[Skill: archr.py]     --> 1. Escreve YAML em disco
   |                      2. Executa `archr validate --format json`
   |                      3. Captura stdout (JSON) e stderr
   v
[Rust CLI: archr]    --> a. Parse YAML -> Arena/Índices
                          b. Validação de regras de derivabilidade
                          c. Diff com modelo existente (se aplicável)
                          d. Resolução de layout automático (grafo)
                          e. Serialização para XML (Open Exchange) / YAML / Mermaid
   |
   v
[Arquivo .archimate (XML)] --> Aberto no editor Archi
   |
   v
[Usuário]
```

### Decisão: Monorepo com isolamento estrito

- **Por que junto:** O maior risco não é bug em Rust, é **schema mismatch**. A Skill ensina a IA a gerar YAML; o Rust valida esse YAML. Se o enum `ElementKind` mudar no Rust, o `SKILL.md` precisa atualizar no mesmo commit. Repos separados = *version drift* clássico.
- **Por que isolado:** A Skill **não** importa Rust via FFI (PyO3) nem compila em runtime. Comunicação é estritamente CLI (`subprocess.run`). Isolamento perfeito, zero acoplamento de memória/ABI.
- **Por que seguir a spec Agent Skills:** interoperabilidade. Qualquer cliente spec-compliant (VS Code Copilot, Claude Code, OpenAI Codex, etc.) descobre e ativa a Skill sem código de cola. Sem spec, cada integração é custom.

---

## 2. ESTRUTURA DO MONOREPO

A Skill segue a [Agent Skills Specification](https://agentskills.io/specification): um diretório contendo `SKILL.md` (frontmatter YAML + instruções no body) e um subdir `scripts/` com código executável autocontido (PEP 723 para Python).

```text
archr/                            # Repositório principal (GitHub)
├── Cargo.toml                    # Workspace Rust
├── crates/
│   └── archr-core/               # O motor em Rust (CLI, Parser, Validator, Layout)
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs           # Entry point do CLI (clap)
│           ├── model.rs          # Arena + Element/Relationship
│           ├── validate.rs       # Regras de derivabilidade ArchiMate 3.2
│           ├── diff.rs           # ModelDiffAnalyzer
│           ├── layout.rs         # LayoutResolver (petgraph)
│           └── io/
│               ├── yaml.rs       # serde_yaml <-> Model
│               └── xml.rs        # quick-xml <-> Open Exchange
├── skill/                        # Agent Skill (spec-compliant)
│   ├── SKILL.md                  # Obrigatório: frontmatter + instruções para a IA
│   ├── scripts/
│   │   └── archr.py              # CLI Python autocontido (PEP 723): invoca o binário Rust
│   └── references/
│       └── ARCHIMATE_RULES.md    # Documentação de regras (progressive disclosure)
├── tests/                        # Testes E2E (IA -> Skill -> Rust)
├── docs/
│   └── guia_implementacao.md     # Este documento
└── README.md
```

> **Onde a Skill é instalada:** copie o diretório `skill/` para `.agents/skills/archr-skill/` no projeto do usuário. VS Code Copilot, Claude Code, OpenAI Codex e qualquer cliente compatível com a spec descobrem skills nesse caminho automaticamente. O `name: archr-skill` no frontmatter deve bater com o nome do diretório.

### Regras do Monorepo

1. **Sem dependência direta de código.** Skill (Python) nunca importa Rust via FFI. Comunicação só via CLI.
2. **Skill = diretório autocontido.** Sem `pyproject.toml`, sem `pip install`. O script Python usa apenas stdlib + declaração inline PEP 723, e é invocado pelo agente diretamente via terminal.
3. **CI separado por linguagem.** GitHub Actions com jobs distintos:
   - `build-rust`: roda `cargo test` no workspace.
   - `test-skill`: roda `python skill/scripts/archr.py --help` + testes do CLI da Skill (mockando o binário Rust).
   - `e2e-test`: baixa o binário Rust compilado, roda a Skill e testa o fluxo completo.
4. **Versionamento do binário.** O `SKILL.md` declara `compatibility:` com a versão mínima do `archr`. O script `archr.py` checa a versão real do binário no início de cada execução e falha rápido se houver mismatch.

### Quando separar os repositórios no futuro?

Somente se a comunidade criar **múltiplas Skills** (uma para LangChain, outra para Semantic Kernel, outra como MCP Server nativo). Aí o `archr` (Rust) fica puro, e as Skills migram para uma organização GitHub `archr-skills/langchain`, `archr-skills/mcp`, etc. Para o MVP, monorepo reduz drasticamente a fricção de manutenção.

---

## 3. PRÉ-REQUISITOS E DEPENDÊNCIAS

**Linguagem & Build:**

- Rust (Edition 2021 ou superior)
- Cargo (gerenciador de pacotes)
- Python 3.10+ (apenas para a Skill)

**`crates/archr-core/Cargo.toml`:**

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_yaml = "0.9"
quick-xml = "0.31"
clap = { version = "4.5", features = ["derive"] }
petgraph = "0.6"
uuid = { version = "1.8", features = ["v4", "serde"] }
thiserror = "1.0"
```

**Skill — sem dependências externas.** O `skill/scripts/archr.py` é autocontido: usa apenas stdlib (`subprocess`, `json`, `argparse`, `os`, `sys`, `tempfile`) e declara inline via [PEP 723](https://peps.python.org/pep-0723/) caso precise de algo a mais. Não há `pyproject.toml`, `requirements.txt` ou `pip install` — o agente executa `python skill/scripts/archr.py ...` diretamente.

---

## 4. CONFIGURAÇÃO DE AMBIENTE

A ferramenta é **stateless** e roda em modo sandbox. Não há variáveis de ambiente obrigatórias nem serviços de backend. Configuração via argumentos de CLI.

**Estrutura de Uso (binário Rust):**

```bash
# Validar
./archr validate --input model.yaml --format json

# Gerar XML final
./archr generate --input model.yaml --output model.archimate

# Converter XML existente para YAML (input de LLM)
./archr parse --input model.archimate --output model.yaml

# Diff entre modelo existente e novo YAML
./archr diff --old model.archimate --new model.yaml
```

**Estrutura de Uso (Skill Python — invocada diretamente pelo agente via terminal):**

```bash
# Validar (agente sempre chama isto PRIMEIRO)
python skill/scripts/archr.py validate model.yaml

# Gerar XML final (só após validate retornar success=true)
python skill/scripts/archr.py generate model.yaml --output out.archimate
```

O `SKILL.md` instrui o agente a chamar esses comandos; o agente lê o JSON do stdout e decide o próximo passo. Não há wrapper de Function Calling — a spec Agent Skills trata o agente como executor de terminal.

---

## 5. MODELO DE DADOS / ESQUEMA

A estrutura interna usa o padrão **Arena com Índices Tipados** para gerenciar o grafo sem conflitos de *Borrow Checker*.

### Schema Interno (Rust Structs)

| Estrutura       | Campos                                                                      | Descrição                                                          |
|:----------------|:----------------------------------------------------------------------------|:-------------------------------------------------------------------|
| `Model`         | `name: String`, `elements: Vec<Element>`, `relations: Vec<Relationship>`    | Contêiner raiz. Posse exclusiva de todos os dados.                 |
| `Element`       | `id: ElementId`, `name: String`, `kind: ElementKind`                        | Nó do grafo.                                                       |
| `Relationship`  | `id: RelationId`, `source: ElementId`, `target: ElementId`, `kind: RelationKind` | Aresta direcionada.                                                |
| `ElementId`     | `pub struct ElementId(pub usize);`                                          | Índice forte para acessar `Vec` em O(1).                           |
| `RelationId`    | `pub struct RelationId(pub usize);`                                         | Índice forte para acessar `Vec` em O(1).                           |

### Schema YAML Intermediário (Input/Output de IA)

```yaml
model:
  name: "String"
  elements:
    - id: "String"        # Deve ser único; snake_case, sem espaços
      name: "String"
      kind: "String"      # Ex: BusinessActor, ApplicationComponent, Node
  relationships:
    - id: "String"
      source: "String"    # Referência a element.id existente
      target: "String"    # Referência a element.id existente
      kind: "String"      # Ex: Serving, Realization, Assignment
```

> **Contrato crítico:** este YAML é o ponto de acoplamento entre Skill (ensina a IA) e Rust (valida). Qualquer mudança no enum `ElementKind`/`RelationKind` exige atualização simultânea do body do `skill/SKILL.md`.

---

## 6. CONTRATOS DE API / ENDPOINTS

Como a decisão final foi por **CLI** (descartando API REST devido ao overhead de servidores ociosos e friction para Agentes IA), os "endpoints" são comandos do `clap`.

### Endpoints do binário Rust

| Comando     | Argumentos                                   | Response (stdout)                              |
|:------------|:---------------------------------------------|:-----------------------------------------------|
| `validate`  | `--input <file>`, `--format json`            | JSON com sucesso ou lista estruturada de erros. |
| `generate`  | `--input <yaml>`, `--output <xml>`           | Mensagem de status em texto.                   |
| `parse`     | `--input <xml>`, `--output <yaml>`           | Mensagem de status em texto.                   |
| `diff`      | `--old <xml>`, `--new <yaml>`                | JSON com elementos adicionados/removidos.      |
| `--version` | —                                            | Versão semântica (ex: `archr 1.0.0`).          |

### Endpoint exposto pela Skill (CLI Python autocontido)

A Skill expõe duas ações via `skill/scripts/archr.py`. O agente invoca diretamente no terminal, seguindo as instruções do `SKILL.md`. Saída estruturada (JSON) sempre em stdout; diagnósticos em stderr.

| Comando Skill                                         | Ação                                                  | Saída (stdout)                                  |
|:------------------------------------------------------|:------------------------------------------------------|:------------------------------------------------|
| `python skill/scripts/archr.py validate <file.yaml>`  | Validar YAML intermediário contra regras ArchiMate.   | JSON `{"success": bool, "errors": [...]}`       |
| `python skill/scripts/archr.py generate <file.yaml>`  | Gerar XML final após validação implícita.             | JSON `{"success": bool, "message\|error": ...}` |
| `python skill/scripts/archr.py --help`                | Documentação de uso para o agente.                    | Texto com flags e exit codes.                   |

**Exit codes documentados (padrão spec Agent Skills):**

| Código | Significado                                  |
|:-------|:---------------------------------------------|
| 0      | Sucesso.                                     |
| 1      | Erro de validação (YAML válido, regras quebradas). stdout tem JSON de erros. |
| 2      | YAML malformado ou erro de I/O.              |
| 3      | Versão do binário `archr` incompatível com `compatibility:` do SKILL.md. |
| 4      | Timeout do subprocess.                        |
| 64     | Argumentos inválidos (uso incorreto).         |

### Exemplo de Response de Erro (Validação)

```json
{
  "success": false,
  "errors": [
    {
      "code": "INVALID_RELATIONSHIP",
      "message": "BusinessActor não pode ter relação de Realization com ApplicationComponent",
      "element_source": "cliente_001",
      "suggestion": "Considere usar 'Serving' ou inverta a direção."
    }
  ]
}
```

---

## 7. FLUXO DE IMPLEMENTAÇÃO PASSO A PASSO

Abaixo, a sequência lógica e trechos de código da versão final escolhida.

### 7.1. Padrão Arena para Posse de Grafo

*Justificativa:* Evita `Rc<RefCell<>>` (lento e propenso a panic) e `HashMap` de strings (lento). Acesso O(1) direto na memória.

```rust
pub struct Model {
    pub name: String,
    elements: Vec<Element>,
    relations: Vec<Relationship>,
}

impl Model {
    pub fn add_element(&mut self, name: &str, kind: ElementKind) -> ElementId {
        let id = ElementId(self.elements.len());
        self.elements.push(Element { id, name: name.to_string(), kind });
        id
    }

    pub fn element(&self, id: ElementId) -> &Element {
        &self.elements[id.0] // Indexação direta O(1)
    }

    pub fn link(&mut self, source: ElementId, target: ElementId, kind: RelationKind) -> RelationId {
        let id = RelationId(self.relations.len());
        self.relations.push(Relationship { id, source, target, kind });
        id
    }
}
```

### 7.2. Validação de Regras de ArchiMate

*Justificativa:* Garantir que a IA não crie modelos semanticamente inválidos.

```rust
pub fn validate_relationship(
    source: &Element,
    target: &Element,
    rel: &RelationKind,
) -> Result<(), String> {
    match (source.kind, target.kind, rel) {
        (ElementKind::ApplicationComponent, ElementKind::Node, RelationKind::Realization) => Ok(()),
        (ElementKind::BusinessActor, ElementKind::ApplicationComponent, RelationKind::Serving) => Ok(()),
        // ... outras regras válidas
        _ => Err(format!(
            "Regra violada: {:?} não pode {:?} {:?}",
            source.kind, rel, target.kind
        )),
    }
}
```

### 7.3. Análise de Diff (Edição de Modelos)

*Justificativa:* Necessário para atualizar modelos sem quebrar referências de IDs existentes.

```rust
use std::collections::HashSet;

pub struct ModelDiffAnalyzer {
    existing_ids: HashSet<String>,
    new_ids: HashSet<String>,
    referenced_ids: HashSet<String>,
}

impl ModelDiffAnalyzer {
    pub fn analyze_update(&mut self, yaml: &str) -> Result<DiffReport, Vec<ReferenceError>> {
        // Parse YAML e popula new_ids e referenced_ids...

        // Encontra referências para IDs não definidos em lugar nenhum
        let undefined_refs: Vec<_> = self.referenced_ids
            .difference(&self.new_ids)
            .filter(|id| !self.existing_ids.contains(*id))
            .cloned()
            .collect();

        if !undefined_refs.is_empty() {
            return Err(undefined_refs
                .into_iter()
                .map(ReferenceError::UndefinedId)
                .collect());
        }

        // Calcula adicionados, removidos, modificados...
        Ok(DiffReport::default())
    }
}
```

### 7.4. Resolvedor de Layout Automático

*Justificativa:* O Archi precisa de coordenadas X/Y para renderizar. A IA não calcula geometria.

```rust
use petgraph::graphmap::UnGraphMap;
use std::collections::HashMap;

pub struct LayoutResolver {
    graph: UnGraphMap<String, ()>,
    positions: HashMap<String, (f64, f64)>,
}

impl LayoutResolver {
    pub fn calculate_layout(&mut self) -> Result<(), LayoutError> {
        // 1. Detectar componentes conexos (petgraph::algo::kosaraju_scc)
        // 2. Aplicar layout hierárquico simplificado por componente (Sugiyama)
        // 3. Popular HashMap de posições (x, y)
        Ok(())
    }

    pub fn export_to_archi_xml(&self, model: &Model) -> String {
        // Iterar sobre elementos, injetar xpos, ypos, width, height...
        String::new()
    }
}
```

### 7.5. Skill do Agente IA (`skill/SKILL.md` + `skill/scripts/archr.py`)

*Justificativa:* A spec Agent Skills define uma Skill como um diretório com `SKILL.md` (frontmatter + instruções) e opcionalmente `scripts/`. O agente (VS Code Copilot, Claude Code, OpenAI Codex, etc.) lê o `SKILL.md`, decide se aplica, e invoca os scripts diretamente via terminal. Não há Function Calling wrapper — a spec trata o agente como executor de CLI.

#### 7.5.1. `skill/SKILL.md`

```markdown
---
name: archr-skill
description: Create, validate, and generate ArchiMate 3.2 architecture models as
  .archimate files. Use when the user asks to draw, build, or generate an
  ArchiMate model, business/application/technology architecture diagram, or
  needs to validate ArchiMate relationships. Requires the `archr` binary on
  PATH (Linux/macOS) or `archr.exe` (Windows).
compatibility: Requires `archr` v1.0.x. Set ARCHR_BIN env var to override the
  default `./archr` path. Python 3.10+ required to run the script.
license: MIT
---

You are an Enterprise Architecture assistant. Use the `archr` CLI via the
`archr.py` wrapper to create ArchiMate 3.2 models. ALWAYS validate BEFORE
generating.

## Workflow

1. Generate the model in the YAML intermediate format below.
2. Write it to a file (e.g. `model.yaml`) and run:
   `python skill/scripts/archr.py validate model.yaml`
3. If the JSON response has `"success": false`, read the `"errors"` array,
   fix the YAML, and re-run `validate`. Do NOT call `generate` yet.
4. Only after `"success": true`, run:
   `python skill/scripts/archr.py generate model.yaml --output model.archimate`
5. Return the path of the generated `.archimate` file to the user.

## YAML schema (mandatory)

\`\`\`yaml
model:
  name: "String"
  elements:
    - id: "String"        # snake_case, unique, no spaces
      name: "String"
      kind: "String"      # BusinessActor | ApplicationComponent | Node | ...
  relationships:
    - id: "String"
      source: "String"    # must be an existing element id
      target: "String"    # must be an existing element id
      kind: "String"      # Serving | Realization | Assignment | ...
\`\`\`

## ArchiMate rules (do NOT break)

- `BusinessActor` may only have `Serving` to `ApplicationComponent`.
- `ApplicationComponent` may only `Realization` to `Node` / `SystemSoftware`.
- Every relationship must reference existing element ids.

For deeper rules (full derivability matrix), see `references/ARCHIMATE_RULES.md`.

## Exit codes

| Code | Meaning                            |
|:-----|:-----------------------------------|
| 0    | Success.                           |
| 1    | Validation error (read JSON).      |
| 2    | YAML malformed or I/O error.       |
| 3    | Binary version mismatch.           |
| 4    | Subprocess timeout.                |
| 64   | Invalid CLI arguments.             |
```

> **Regras da spec atendidas:** `name` bate com o nome do diretório (`archr-skill`), 1-64 chars, lowercase + hífen; `description` descreve o que faz E quando usar (necessário para ativação por similaridade semântica); `compatibility` documenta pré-requisitos; body tem as instruções que o agente segue.

#### 7.5.2. `skill/scripts/archr.py` (CLI autocontido PEP 723)

```python
# /// script
# requires-python = ">=3.10"
# ///
"""archr.py — Agent Skills wrapper for the archr Rust binary.

Exit codes:
  0  Success
  1  Validation error (stdout has JSON with errors[])
  2  YAML malformed or I/O error
  3  Binary version mismatch
  4  Subprocess timeout
  64 Invalid arguments
"""
import argparse
import json
import os
import shutil
import subprocess
import sys
import tempfile

EXPECTED_ARCHR_VERSION = "1.0."  # compat: aceita qualquer 1.0.x
ARCHR_BIN = os.environ.get("ARCHR_BIN", shutil.which("archr") or "./archr")
TIMEOUT_SECONDS = 10


def _check_version() -> None:
    """Fail fast if the binary is incompatible. Exit 3."""
    try:
        result = subprocess.run(
            [ARCHR_BIN, "--version"],
            capture_output=True, text=True, timeout=5,
        )
    except (FileNotFoundError, subprocess.TimeoutExpired) as e:
        print(json.dumps({"error": f"archr binary not runnable: {e}"}), file=sys.stderr)
        sys.exit(3)
    if EXPECTED_ARCHR_VERSION not in result.stdout:
        print(
            json.dumps({
                "error": "version mismatch",
                "expected": EXPECTED_ARCHR_VERSION + "x",
                "got": result.stdout.strip(),
            }),
            file=sys.stderr,
        )
        sys.exit(3)


def _run(args: list[str]) -> subprocess.CompletedProcess:
    try:
        return subprocess.run(
            args, capture_output=True, text=True, timeout=TIMEOUT_SECONDS,
        )
    except subprocess.TimeoutExpired:
        print(json.dumps({"error": "archr subprocess timed out"}), file=sys.stderr)
        sys.exit(4)


def cmd_validate(yaml_path: str) -> None:
    result = _run([ARCHR_BIN, "validate", "--input", yaml_path, "--format", "json"])
    # Sempre escreve o JSON do Rust no stdout (decisão ou erro)
    sys.stdout.write(result.stdout)
    if result.returncode != 0:
        # exit 1 = erro de validação; 2 = yaml malformed (stdin devolvido pelo Rust)
        sys.exit(1 if result.returncode == 1 else 2)


def cmd_generate(yaml_path: str, output: str) -> None:
    result = _run([ARCHR_BIN, "generate", "--input", yaml_path, "--output", output])
    if result.returncode == 0:
        print(json.dumps({"success": True, "message": f"Model generated at {output}"}))
    else:
        print(json.dumps({"success": False, "error": result.stderr.strip()}))
        sys.exit(1 if "validation" in result.stderr.lower() else 2)


def main() -> None:
    p = argparse.ArgumentParser(
        prog="archr.py",
        description="Agent Skills wrapper for the archr ArchiMate engine. "
                    "Always run `validate` BEFORE `generate`.",
    )
    sub = p.add_subparsers(dest="cmd", required=True)

    pv = sub.add_parser("validate", help="Validate YAML against ArchiMate rules.")
    pv.add_argument("yaml", help="Path to the intermediate YAML file.")

    pg = sub.add_parser("generate", help="Generate final .archimate XML.")
    pg.add_argument("yaml", help="Path to the intermediate YAML file.")
    pg.add_argument("--output", "-o", required=True, help="Output .archimate path.")

    args = p.parse_args()
    _check_version()

    if args.cmd == "validate":
        cmd_validate(args.yaml)
    elif args.cmd == "generate":
        cmd_generate(args.yaml, args.output)


if __name__ == "__main__":
    main()
```

> **Regras da spec atendidas:** script é autocontido (PEP 723 inline metadata, sem `pip install`); apenas stdlib; sem prompts interativos; `--help` documentado via `argparse`; saída estruturada JSON em stdout; diagnósticos em stderr; exit codes distintos e documentados no `SKILL.md`.

---

## 8. FLUXO DE FEEDBACK (Loop de Auto-correção)

O diferencial desta arquitetura é como o motor Rust (crítico para performance) dialoga com a IA (criativa mas falha):

1. **IA cria:** gera YAML conectando `BusinessActor` -> `Node` com `Realization`.
2. **Skill executa:** `archr validate` rejeita em <10ms.
3. **Rust responde:** `{"success": false, "errors": [{"code": "INVALID_RELATIONSHIP", "message": "BusinessActor não pode Realization Node"}]}`
4. **IA corrige:** lê o erro, muda a relação para `Serving` ou troca o alvo para `ApplicationComponent`.
5. **Skill re-executa:** `archr validate` aprova em <10ms.
6. **IA finaliza:** chama `archr generate` e entrega o arquivo `.archimate` ao usuário.

```text
[usuário] -> [LLM Agent] --(1. Gera YAML)--> [Skill: archr.py]
                                                  |
                                                  v
                                           (2. Escreve YAML em disco)
                                           (3. Executa `archr validate --format json`)
                                           (4. Captura stdout JSON + stderr)
                                                  |
                                                  v
[LLM Agent] <--(5. Retorna JSON de erros)--- [Skill: archr.py]
     |
     +--> (6. LLM lê erros, corrige YAML, volta para passo 1)
     |
     +--> (7. Se válido, Skill executa `archr generate`)
     |
[usuário] <--(8. Retorna arquivo .archimate final)
```

---

## 9. SEGURANÇA E OBSERVABILIDADE

### Segurança

- A ferramenta é isolada (não expõe portas de rede).
- **Sandbox de arquivos:** a Skill só deve ter permissão de escrita no diretório de output definido pelo usuário, prevenindo que a IA sobrescreva arquivos do sistema.
- **Sanitização de paths:** [PENDENTE] Definir estratégia contra directory traversal caso a CLI seja envolvida por um wrapper web no futuro.

### Observabilidade

- Logs estruturados devem ser emitidos em `stderr`.
- O output de dados (resultados, JSON) deve ir **exclusivamente** para `stdout` para não poluir a resposta parseada pela IA.

### Premissas da Skill

- **Timeout rigoroso:** 10s no `archr.py`. O Rust resolve em milissegundos, mas a IA pode gerar um YAML catastrófico que entre em loop no parser.
- **Versionamento do binário:** `_check_version()` em `archr.py` falha rápido (exit 3) se a versão do binário não bater com `EXPECTED_ARCHR_VERSION`. O frontmatter `compatibility:` do `SKILL.md` documenta o requisito para o agente saber antes de instalar.
- **Sem instalação Python:** `archr.py` é autocontido via PEP 723; apenas stdlib; o agente roda `python skill/scripts/archr.py ...` sem `pip install`.

---

## 10. ESTRATÉGIA DE TESTES

### Comandos de Validação Local

```bash
# Rust
cargo test --workspace -- --nocapture

# Skill (sem pytest; só valida CLI + --help)
python skill/scripts/archr.py --help
python skill/scripts/archr.py validate tests/fixtures/valid.yaml
python skill/scripts/archr.py validate tests/fixtures/orphan_id.yaml  # espera exit 1

# E2E
cargo build --release
./tests/e2e.sh   # Baixa o binário, roda a Skill, valida o fluxo completo
```

### Casos de Borda Críticos

1. **IDs órfãos:** YAML que referencia `id: "db_001"` inexistente. Deve falhar com `UndefinedId`.
2. **Relacionamentos inválidos:** `BusinessActor` fazendo `Realization` de `SystemSoftware`. Deve falhar com `INVALID_RELATIONSHIP`.
3. **Ciclos no grafo:** verificar se o `LayoutResolver` não entra em loop infinito com dependências circulares (A -> B -> A).
4. **XML malformado:** parser `quick-xml` deve tratar gracefully tags inesperadas.
5. **Drift de versão:** Skill com `compatibility: "Requires archr v1.0.x"` contra binário `0.9.0` deve falhar rápido (exit 3) antes de qualquer `subprocess.run` no `archr.py`.
6. **Timeout da Skill:** YAML catastrófico que exceda o parser — `archr.py` deve matar o processo em 10s e retornar exit 4 com JSON de erro estruturado em stderr.
7. **Saída estruturada:** stdout deve sempre conter JSON parseável, mesmo em erro (validação espec: agente lê stdout para decidir próximo passo). Mensagens humanas vão só em stderr.

---

## 11. PASSOS PARA DEPLOY

A solução é distribuída como um único binário estático Rust + um diretório de Skill self-contained.

### Build

```bash
# Release otimizado do motor Rust
cargo build --release
# Binário: target/release/archr

# Skill não tem build — é um diretório com SKILL.md + scripts/archr.py.
# Para validar localmente:
python skill/scripts/archr.py --help
```

### Distribuição

- Empacotar o binário Rust para alvos específicos: Linux x86_64, macOS ARM, Windows.
- **Distribuir a Skill como diretório** (spec Agent Skills): o usuário copia `skill/` para `.agents/skills/archr-skill/` no projeto, OU instala via mecanismo futuro de marketplace de skills. Não há `pip install` — `archr.py` é autocontido.
- Alternativamente, publicar o monorepo no GitHub e instruir o usuário a clonar ou copiar o diretório `skill/`.
- Não há necessidade de Dockerfile, pois não há runtime dependente.
- **[PENDENTE]** Definir pipeline de CI/CD GitHub Actions com três jobs: `build-rust`, `test-skill` (valida `SKILL.md` parseable + roda `archr.py --help` + testes de fixture), `e2e-test`. Publicar binários via GitHub Releases.

---

## 12. PONTOS DE ATENÇÃO E ARMADILHAS COMUNS

1. **Algoritmo de Layout é NP-Hard.** O `LayoutResolver` usa Sugiyama simplificado. Para modelos com >500 elementos, o cálculo pode tornar-se caro. *Mitigação:* cache de layout baseado em hash do estado do modelo.
2. **Custo de Tokens de IA.** Trabalhar diretamente com XML do Archi consome muitos tokens de LLM. *Mitigação:* o formato YAML intermediário é obrigatório e reduz o payload em ~70%.
3. **Especificação ArchiMate 3.2 extensiva.** *Risco:* mapear todas as regras de derivabilidade em `match` statements gera dívida técnica. *Mitigação:* modelar as regras de forma data-driven (matriz de adjacência) em vez de hardcoded, se crescerem.
4. **Validação de Schema YAML.** Confiar apenas no `serde_yaml` gera mensagens crípticas para a IA. *Mitigação:* validação manual pós-deserialização para produzir o JSON estruturado da Seção 6.
5. **Acoplamento de Schema (Rust <-> Skill).** *Risco:* mudar `ElementKind` no Rust sem atualizar o `SKILL.md` quebra a IA silenciosamente. *Mitigação:* Monorepo + release atômica + teste E2E que valida o `SKILL.md` contra o enum Rust.
6. **Drift de versão do binário.** *Risco:* usuário com binário `0.9.0` e Skill esperando `1.0.x`. *Mitigação:* frontmatter `compatibility:` no `SKILL.md` + `_check_version()` em `archr.py` (exit 3).
7. **Aderência à spec Agent Skills.** *Risco:* clientes (VS Code Copilot, Claude Code, Codex) só descobrem a Skill se as regras formais forem seguidas. *Mitigação:* validar no CI que (a) `name` no frontmatter bate com o nome do diretório, (b) `description` tem ≤1024 chars e descreve quando usar, (c) `scripts/archr.py` roda com `--help` e respeita os exit codes documentados. Usar o parser YAML da própria spec se disponível.
8. **Ativação por similaridade semântica.** *Risco:* `description` mal escrita faz a Skill não ser acionada quando deveria, ou ser acionada quando não deveria. *Mitigação:* seguir [Optimizing skill descriptions](https://agentskills.io/skill-creation/optimizing-descriptions) — incluir verbos de ação ("create", "validate", "generate"), domínio ("ArchiMate", "architecture") e gatilhos explícitos ("when the user asks to...").

---

*v2.1 — Skill agora segue a [Agent Skills Specification](https://agentskills.io/specification): diretório com `SKILL.md` (frontmatter + body) + `scripts/archr.py` (PEP 723 autocontido). Compatível com VS Code Copilot, Claude Code, OpenAI Codex e qualquer cliente spec-compliant.*
