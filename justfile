# list recipes
help:
  just --list

develop:
  nix develop -c zsh

run:
  cargo run

release:
  cargo build --release

build:
  nix build