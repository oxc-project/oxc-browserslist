#!/usr/bin/env -S just --justfile

_default:
  @just --list -u

alias r := ready

init:
  cargo binstall cargo-watch typos-cli dprint -y

ready:
  git diff --exit-code --quiet
  typos
  cargo codegen
  just fmt
  cargo check
  just lint
  cargo test

fmt:
  cargo fmt
  dprint fmt

lint:
  cargo clippy --all-targets --all-features -- -D warnings
