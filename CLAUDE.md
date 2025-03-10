# Shapely Project Guide

## Build, Test, and Lint Commands
- Build: `cargo build`
- Check all: `just check` (runs clippy and tests)
- Lint: `cargo clippy --all-targets`
- Run all tests: `RUST_LOG=trace cargo nextest run`
- Run single test: `cargo nextest run <test_name>`
- Run specific package test: `cargo nextest run -p shapely <test_name>`
- Publish workspace: `cargo +nightly publish --workspace -Zpackage-workspace`

## Code Style Guidelines
- Follow Rust snake_case for functions/variables, PascalCase for types/traits
- Group imports: std first, external crates second, internal imports last
- Document public APIs with `///` comments (explain behavior, parameters, return values)
- Use Result<T, E> with descriptive error types that implement std::error::Error
- Provide context in error messages (including position information where relevant)
- Wrap unsafe code in safe abstractions with clear documentation
- Write thorough unit tests for all functionality
- Use explicit panic messages that explain invariant violations
- Document any undefined behavior with clear warnings