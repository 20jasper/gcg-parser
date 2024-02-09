# build and run example query on save
dev:
    RUSTFLAGS="-A warnings" cargo watch --poll -q -c -w examples/ -x "run --example quick_dev -- --nocapture"

# build and run tests on save
test:
    RUSTFLAGS="-A warnings" cargo watch --poll -q -c -w src/ -x "test"

# format rust, justfile, and markdown
format:
    cargo fmt --all
    just --fmt --unstable
    npx -y prettier './**/*.{md,yaml}' --write

# check formatting for rust, justfile, and markdown
format-check:
    cargo fmt --all -- --check
    just --fmt --unstable --check
    npx -y prettier './**/*.{md,yaml}' --check

# lint rust
lint:
    cargo clippy --all-targets --all-features

# commands to run before making a pull request
prepublish:
    RUSTFLAGS="-D warnings" just lint
    just format-check
    cargo diet -r --dry-run
    cargo publish --dry-run
    cargo semver-checks
