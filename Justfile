
check:
  cargo clippy --all-targets
  RUST_LOG=trace cargo nextest run
