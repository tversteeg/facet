check:
  just install-nightly
  just clippy
  just nextest
  just doc-tests
  just miri
  just check-syn

install-nightly:
  echo -e "\033[1;32m🔧 Installing nightly Rust toolchain...\033[0m"
  rustup toolchain install nightly || true

check-syn:
  #!/bin/bash
  echo -e "\033[1;34m🌳 Checking dependency tree...\033[0m"
  if ! cargo +nightly tree -i syn 2>/dev/null | grep -q .; then
    echo -e "\033[1;32m✅ 'syn' not found in dependency tree.\033[0m"
    echo -e "\033[1;34m┌─────────────┐\033[0m"
    echo -e "\033[1;34m│ \033[1;90mfree of \033[1;95msyn \033[1;34m│\033[0m"
    echo -e "\033[1;34m└─────────────┘\033[0m"
  else
    echo -e "\033[1;31m❌ 'syn' found in dependency tree. Here's what's using 'syn':\033[0m"
    cargo +nightly tree -i syn -e features
    exit 1
  fi

clippy:
  echo -e "\033[1;35m🔍 Running Clippy on all targets...\033[0m"
  cargo +nightly clippy --all-targets

nextest:
  echo -e "\033[1;33m🏃 Running all but doc-tests with nextest...\033[0m"
  cargo +nightly nextest run

doc-tests:
  echo -e "\033[1;36m📚 Running documentation tests...\033[0m"
  cargo +nightly test --doc

miri:
  echo -e "\033[1;31m🧪 Running tests under Miri...\033[0m"
  cargo +nightly miri nextest run

publish:
  cargo +nightly publish --workspace -Zpackage-workspace
