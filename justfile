#!/usr/bin/env -S just --justfile

_default:
  @just --list -u

alias r := ready

init:
  cargo binstall cargo-watch typos-cli taplo-cli -y

ready:
  git diff --exit-code --quiet
  typos
  cargo codegen
  just fmt
  cargo check
  cargo test

fmt:
  cargo fmt
  taplo format
