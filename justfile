set positional-arguments

default:
  @just --list

# Cargo build everything.
build:
  cargo build --workspace --all-targets --all-features

# Cargo check everything.
check:
  cargo check --workspace --all-targets --all-features

# Lint everything.
lint:
  cargo +nightly clippy --workspace --all-targets --all-features -- --deny warnings
  # lint warnings get inhibited unless we use `--nocapture`
  cargo test --quiet --workspace --doc -- --nocapture

# Run cargo fmt
fmt:
  cargo fmt --all

# Check the formatting
format:
  cargo fmt --all --check

# Quick and dirty CI useful for pre-push checks.
sane: lint
  cargo test --quiet --workspace --all-targets --no-default-features > /dev/null || exit 1
  cargo test --quiet --workspace --all-targets > /dev/null || exit 1
  cargo test --quiet --workspace --all-targets --all-features > /dev/null || exit 1

  # Make an attempt to catch feature gate problems in doctests
  cargo test --manifest-path bitcoin/Cargo.toml --doc --no-default-features > /dev/null || exit 1
