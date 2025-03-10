
check:
  cargo clippy --all-targets
  cargo nextest run
  cargo +nightly miri nextest run


publish:
  cargo +nightly publish --workspace -Zpackage-workspace
