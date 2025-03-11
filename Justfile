check:
  just install-nightly
  just clippy
  just nextest
  just doc-tests
  just absolve

install-nightly:
  echo -e "\033[1;32m🔧 Installing nightly Rust toolchain...\033[0m"
  rustup toolchain install nightly || true

absolve:
  #!/bin/bash
  echo -e "\033[1;34m🌳 Checking dependency tree...\033[0m"
  if ! cargo +nightly tree -i syn 2>/dev/null | grep -q .; then
  echo -e "\033[38;2;255;255;255;48;2;0;0;0m free of \033[38;2;255;255;255;48;2;255;105;180m syn \033[38;2;255;255;255;48;2;0;0;0m\033[0m"
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
