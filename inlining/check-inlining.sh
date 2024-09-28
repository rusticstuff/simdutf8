#!/usr/bin/env bash
set -euo pipefail

target="$1"
expected_fns="$2"
build_args="${3:-}"
cargo clean --quiet
cargo build --quiet --release --target $target $build_args
nm_output=$(nm --defined-only ../target/$target/release/libsimdutf8.rlib)
if [[ $target == *darwin* ]]; then
    pattern=" (t|T) _"
    cut_arg=21
else
    pattern=" (t|T) "
    cut_arg=20
fi
inline_ignore_pattern='drop_in_place|::fmt::|^\$x\.|^<T as core::convert::From<T>>::from$'
echo "$nm_output" | rustfilt | egrep "$pattern" | cut -c "$cut_arg"- | grep -Ev "$inline_ignore_pattern" | sort | diff -u $expected_fns -
