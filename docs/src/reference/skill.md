# Agent Skill

The `skill/` directory contains an Agent Skill for AI assistants (Claude Code, VS Code Copilot, OpenAI Codex). The Python wrapper (`archr.py`) is self-contained — stdlib only, no `pip install`.

## Use

```bash
# Check the binary version is compatible
python3 skill/scripts/archr.py --version

# Validate through the wrapper
python3 skill/scripts/archr.py validate model.yaml

# Generate XML
python3 skill/scripts/archr.py generate model.yaml --output model.archimate
```

## Custom binary location

Set `ARCHR_BIN` if `archr` isn't in `PATH`:

```bash
export ARCHR_BIN=/usr/local/bin/archr
python3 skill/scripts/archr.py validate model.yaml
```

## What the skill gives an agent

A documented, executable interface to the engine — an LLM can read `skill/SKILL.md`, call the Python wrapper, and operate on ArchiMate models without learning the Rust CLI or the YAML schema by heart. The derivability ruleset is exposed via `skill/references/ARCHIMATE_RULES.md`.
