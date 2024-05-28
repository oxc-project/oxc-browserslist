#!/usr/bin/env -S just --justfile

_default:
  @just --list -u

alias r := ready

ready:
  cargo check
