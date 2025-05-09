#!/usr/bin/env -S just --justfile

_default:
  @just --list -u

alias r := ready

init:
  cargo binstall watchexec-cli typos-cli dprint -y

ready:
  git diff --exit-code --quiet
  typos
  pnpm install
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

watch *args='':
  watchexec --no-vcs-ignore {{args}}

watch-check:
  just watch "'cargo check; cargo clippy'"
