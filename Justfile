
check:
  rustup toolchain install nightly || true
  cargo +nightly clippy --all-targets
  cargo +nightly nextest run
  cargo +nightly test --doc
  cargo +nightly miri nextest run


publish:
  cargo +nightly publish --workspace -Zpackage-workspace
