# ArchiMate Strategy (Design History)

## 1. Core Thesis

- **The Thesis:** Automating the ArchiMate model lifecycle requires a headless, performant, and native engine for AI, isolating structural complexity from natural language semantics.
- **The Context:** Current tools rely on heavy runtimes (JVM/Eclipse) and focus on human interaction. The rise of autonomous agents creates demand for a standalone CLI that optimizes token costs, processes batch files, and provides structured feedback for auto-correction in pipelines.

## 2. Decision Tree

- **Decision:** Develop in Rust as a standalone binary. | **Discarded:** Extend Archi (Java/EMF). | **Reason:** Eliminate JVM initialization overhead (~seconds to ~ms), drastically reduce artifact size, and ensure memory safety.
- **Decision:** Expose functionality via CLI. | **Discarded:** REST API (Axum). | **Reason:** AI agents natively interact with CLIs in sandboxes; avoids idle server persistence and reduces integration friction.
- **Decision:** Adopt YAML as intermediate format. | **Discarded:** Direct XML manipulation or incremental API calls. | **Reason:** Reduces token cost by ~58%, enables contextual batch validation, and facilitates human auditing.
- **Decision:** Manage state via Arena with typed indices. | **Discarded:** `Rc<RefCell<>>` or string `HashMaps`. | **Reason:** Maximum O(1) access performance, zero *data races*, and compile-time type error prevention.

## 3. Synthesis (PRD Lite)

- **Assumptions:** AI agents are primary consumers; ArchiMate 3.2 specification is mappable to rigid Enums; approximate automatic layout is sufficient for initial export.
- **Constraints:** Single binary without external dependencies; need to resolve geometric positioning (X,Y) in export to Open Exchange format; prohibition of breaking model integrity during incremental edits.
- **Implicit Risks:** Low adoption by traditional architects stuck in the visual ecosystem; technical debt in auto-layout algorithm (NP-Hard problem); LLMs failing to generate valid YAMLs without exhaustive correction loops.

## 4. Next Moves

- Validate LLM efficiency in generating intermediate YAML without schema hallucinations or ID breaks.
- Prototype layout resolution algorithm to ensure generated XML doesn't overlap elements when opened in Archi. [Information Gap: Exact layout library to be used].
- Test existing XML parser robustness against real-world models with proprietary extensions.