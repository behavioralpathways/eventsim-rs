<img src="https://behavioralpathways.com/images/logo/PNG.png" width="400" alt="Behavioral Pathways">

[![Tests](https://github.com/behavioralpathways/eventsim-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/behavioralpathways/eventsim-rs/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/behavioralpathways/eventsim-rs/graph/badge.svg)](https://codecov.io/gh/behavioralpathways/eventsim-rs)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)
[![Rust](https://img.shields.io/badge/Rust-2021_Edition-orange.svg)](https://www.rust-lang.org/)

Behavioral Pathways Event Sim is a Domain-agnostic specification for modeling individual psychology and social dynamics.

A Rust crate that simulates how entities think, feel, relate, and change over time.

**Website:** [behavioralpathways.com](https://behavioralpathways.com/)

## Documentation

- [Introduction](introduction.md) - Core concepts and theoretical foundations
- [Getting Started](getting-started.md) - Setup and basic usage
- [Longitudinal Tests](tests/longitudinal/) - Complex multi-year simulation examples
- [API Reference](api.md) - Complete API documentation

## Development

```bash
# Enable git hooks (auto-format on commit)
git config core.hooksPath .githooks

# Run tests
make test

# Run tests with coverage
make tests
```

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT License](LICENSE-MIT) at your option.
