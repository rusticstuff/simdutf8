#!/usr/bin/env bash
set -euo pipefail

INLINE_IGNORE_PATTERN="drop_in_place|::fmt::"

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

echo "$nm_output" | rustfilt | egrep "$pattern" | cut -c "$cut_arg"- | grep -Ev $INLINE_IGNORE_PATTERN | sort | diff -u $expected_fns -
