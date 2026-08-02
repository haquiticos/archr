# Contributing to archr

Thank you for your interest in contributing to `archr` — a headless ArchiMate 3.2 validation, manipulation, and export engine.

## Getting Started

### Prerequisites

- Rust 1.74 or later
- Git
- A code editor

### Build from Source

```bash
# Clone the repository
git clone https://github.com/haquiticos/archr.git
cd archr

# Build the project
cargo build --release
```

### Run Tests

```bash
# Run unit tests
cargo test -p archr-core --lib

# Run end-to-end tests
bash tests/e2e.sh
```

## Development Workflow

### Finding Issues

- **Good first issues:** Look for the `good first issue` label on open GitHub issues.
- **Bug reports:** File issues in the [Issues section](https://github.com/haquiticos/archr/issues).
- **Feature requests:** Use the [Feature Request template](.github/ISSUE_TEMPLATE/feature_request.yml).

### Branch Conventions

We follow the following branch naming patterns:

- `issue-<N>-<slug>` for fixes and changes related to a specific issue (e.g., `issue-12-remove-dead-module`)
- `feat/<description>` for new features (e.g., `feat/add-llm-support`)

### Pull Request Process

1. Create a branch from `main` following the naming convention above.
2. Make your changes.
3. Ensure CI passes:
   ```bash
   cargo build --release
   cargo test -p archr-core --lib
   cargo clippy --workspace --all-targets -- -D warnings
   cargo fmt --all --check
   ```
4. Update tests if you added or modified functionality.
5. Submit a pull request.

### PR Checklist

- [ ] CI builds and tests pass
- [ ] `cargo clippy` is clean
- [ ] `cargo fmt` is clean
- [ ] Tests added/updated for new behavior
- [ ] Code follows existing style
- [ ] Related issue closed with `Closes #<N>` in the commit message

### Linking Issues

If your PR closes an issue, include it in your commit message using `Closes #<N>` (case-sensitive, # is required).

## Project Structure

`archr` is a single-crate workspace (`crates/archr-core/`) that produces both the library and the `archr` binary (CLI). The entry point for understanding the architecture is the [`README.md`](README.md), which includes a "Project Structure" block describing the layout.

## Design Documents

The `docs/` directory contains historical design documents (strategy, implementation plan, implementation guide) describing the original architecture decisions. These are optional reading for contributors who want deeper context.

## Code of Conduct

Please see [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) for details on our community standards.

## Reporting Security Issues

See [SECURITY.md](SECURITY.md) for how to report security vulnerabilities responsibly.

## Getting Help

- GitHub Discussions: [https://github.com/haquiticos/archr/discussions](https://github.com/haquiticos/archr/discussions)
- Open an issue for bugs or questions
- Look for existing issues before creating a new one

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
