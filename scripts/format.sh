#!/usr/bin/env bash

HERE=$(dirname "$(readlink -f "$0")")
ROOT=$(dirname "$HERE")

cd "$ROOT" || exit 1

rustup override set nightly
cargo fix --allow-dirty --allow-staged --allow-no-vcs
cargo fix --lib -p bzauth-rs --allow-dirty --allow-staged --allow-no-vcs
cargo fmt --all
rustup override unset
