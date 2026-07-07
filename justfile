set shell := ["bash", "-eu", "-o", "pipefail", "-c"]

default:
    @just --list

# format the code
fmt:
    cargo fmt --all

# check formatting without writing changes
fmt-check:
    cargo fmt --all -- --check

# lint the code
lint:
    cargo clippy --all-targets --all-features -- -D warnings

# run tests
test:
    cargo test --all-targets --all-features

# build the crate
build:
    cargo build --all-targets

# build the crate for release
release:
    cargo build --release

# run the app
run:
    cargo run

# enter the Nix development shell
develop:
    nix develop -c zsh

# run checks and build
ci: fmt-check lint test build

# build with Nix
nix-build:
    nix build
