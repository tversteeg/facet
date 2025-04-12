# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0](https://github.com/facet-rs/facet/compare/facet-toml-v0.2.0...facet-toml-v0.3.0) - 2025-04-12

### Other

- Install cargo-tarpaulin in Docker, and collect + report coverage in CI ([#177](https://github.com/facet-rs/facet/pull/177))
- TOML enum with unit variant implementation ([#168](https://github.com/facet-rs/facet/pull/168))

## [0.2.0](https://github.com/facet-rs/facet/compare/facet-toml-v0.1.12...facet-toml-v0.2.0) - 2025-04-12

### Other

- different place in readme
- Sponsored by depot

## [0.1.12](https://github.com/facet-rs/facet/compare/facet-toml-v0.1.11...facet-toml-v0.1.12) - 2025-04-11

### Other

- Revert to facet-{core,derive,reflect} deps, closes #156 ([#159](https://github.com/facet-rs/facet/pull/159))
- Light deps ([#158](https://github.com/facet-rs/facet/pull/158))
- wip reflect ([#155](https://github.com/facet-rs/facet/pull/155))

## [0.1.11](https://github.com/facet-rs/facet/compare/facet-toml-v0.1.10...facet-toml-v0.1.11) - 2025-04-11

### Other

- Remove workspace dependencies
- Move the template files next to their output and improve the output of the facet-codegen crate.

## [0.1.10](https://github.com/facet-rs/facet/compare/facet-toml-v0.1.9...facet-toml-v0.1.10) - 2025-04-11

### Other

- Logo credit

## [0.1.8](https://github.com/facet-rs/facet/compare/facet-toml-v0.1.7...facet-toml-v0.1.8) - 2025-04-10

### Other

- PokeUninit / Poke

## [0.1.6](https://github.com/facet-rs/facet/compare/facet-toml-v0.1.5...facet-toml-v0.1.6) - 2025-04-10

### Other

- updated the following local packages: facet-core, facet-poke, facet-derive

## [0.1.5](https://github.com/facet-rs/facet/compare/facet-toml-v0.1.4...facet-toml-v0.1.5) - 2025-04-10

### Other

- Use put rather than write for all users of PokeValue
- rename pokevalue:: put into pokevalue:: write and provide a safe alternative
- introduces put in poke value which is safe

## [0.1.4](https://github.com/facet-rs/facet/releases/tag/facet-toml-v0.1.4) - 2025-04-10

### Added

- *(toml)* Add facet-toml crate

### Other

- Woops, use new everywhere
- A few fixups
