#!/usr/bin/env -S just --justfile

_default:
  @just --list -u

alias r := ready

init:
  cargo binstall watchexec-cli typos-cli cargo-shear -y

ready:
  git diff --exit-code --quiet
  typos
  vp install
  cargo codegen
  just fmt
  cargo check
  just lint
  cargo test

fmt:
  -cargo shear --fix # remove all unused dependencies
  cargo fmt
  vp fmt

lint:
  cargo clippy --all-targets --all-features -- -D warnings

watch *args='':
  watchexec --no-vcs-ignore {{args}}

watch-check:
  just watch "'cargo check; cargo clippy'"
