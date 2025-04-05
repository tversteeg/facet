quickcheck:
    just rustfmt
    just clippy
    just test
    just doc-tests
    just absolve

ci:
    just quickcheck
    just miri
    echo -e "\033[1;34m📝 Running cargo fmt in check mode...\033[0m"
    cargo fmt --all -- --check

rustfmt:
    echo -e "\033[1;34m📝 Checking code formatting...\033[0m"
    cargo fmt --all -- --check

clippy:
    echo -e "\033[1;35m🔍 Running Clippy on all targets...\033[0m"
    cargo clippy --all-targets -- -D warnings

test *args:
    #!/bin/bash -euo pipefail
    echo -e "\033[1;33m🏃 Running all but doc-tests with nextest...\033[0m"
    cargo nextest run {{args}}

doc-tests:
    echo -e "\033[1;36m📚 Running documentation tests...\033[0m"
    cargo test --doc


codegen:
    cargo run -p shapely-codegen

rustfmt-fix:
    echo -e "\033[1;34m📝 Fixing code formatting...\033[0m"
    cargo fmt --all

miri *args:
    #!/bin/bash -euo pipefail
    echo -e "\033[1;31m🧪 Running tests under Miri...\033[0m"
    cargo miri nextest run {{args}}


absolve:
    #!/bin/bash
    if ! cargo tree -i syn 2>/dev/null | grep -q .; then
    echo -e "\033[38;2;255;255;255;48;2;0;0;0m free of \033[38;2;255;255;255;48;2;255;105;180m syn \033[38;2;255;255;255;48;2;0;0;0m\033[0m"
    else
    echo -e "\033[1;31m❌ 'syn' found in dependency tree. Here's what's using 'syn':\033[0m"
    cargo tree -i syn -e features
    exit 1
    fi
