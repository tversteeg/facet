
check:
  #!/bin/bash -exu
  echo -e "\033[1;32m🔧 Installing nightly Rust toolchain...\033[0m"
  rustup toolchain install nightly || true
  echo -e "\033[1;34m🌳 Checking dependency tree...\033[0m"
  if cargo +nightly tree | grep -q syn; then
    echo -e "\033[1;31m❌ 'syn' found in dependency tree. Here's the full tree:\033[0m"
    cargo +nightly tree
    echo -e "\033[1;31m❌ Here's what's using 'syn':\033[0m"
    cargo +nightly tree -i syn
    exit 1
  else
    echo -e "\033[1;32m✅ 'syn' not found in dependency tree.\033[0m"
  fi
  echo -e "\033[1;35m🔍 Running Clippy on all targets...\033[0m"
  cargo +nightly clippy --all-targets
  echo -e "\033[1;33m🏃 Running tests with nextest...\033[0m"
  cargo +nightly nextest run
  echo -e "\033[1;36m📚 Running documentation tests...\033[0m"
  cargo +nightly test --doc
  echo -e "\033[1;31m🧪 Running tests with Miri...\033[0m"
  cargo +nightly miri nextest run


publish:
  cargo +nightly publish --workspace -Zpackage-workspace
