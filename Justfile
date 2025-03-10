
check:
  cargo clippy --all-targets
  RUST_LOG=trace cargo nextest run

publish:
  cargo +nightly publish --workspace -Zpackage-workspace
