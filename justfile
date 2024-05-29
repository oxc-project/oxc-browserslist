#!/usr/bin/env -S just --justfile

_default:
  @just --list -u

alias r := ready

init:
  cargo binstall cargo-watch taplo-cli -y

ready:
  git diff --exit-code --quiet
  just fmt
  cargo check

fmt:
  cargo fmt
  taplo format
