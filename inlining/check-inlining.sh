#!/usr/bin/env bash
set -euo pipefail

INLINE_IGNORE_PATTERN="drop_in_place|::fmt::"

if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <target> <build_args>"
    exit 1
fi

target="$1"
expected_fns="$2"
build_args="${3:-}"
cargo clean --quiet
cargo build --quiet --release --target $target $build_args
nm_output=$(nm -U ../target/$target/release/libsimdutf8.rlib 2>/dev/null)
if [[ $target == *darwin* ]]; then
    pattern=" (t|T) _"
    cut_arg=21
else
    pattern=" (t|T) "
    cut_arg=20
fi

echo "$nm_output" | rustfilt | egrep "$pattern" | cut -c "$cut_arg"- | grep -Ev $INLINE_IGNORE_PATTERN | diff -u $expected_fns -
