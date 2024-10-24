#!/usr/bin/env bash
set -euo pipefail

target="$1"
expected_fns="$2"
build_args="${3:-}"
cargo clean --quiet
cargo build --quiet --release --target $target $build_args
LLVM_NM=$(rustc --print sysroot)/lib/rustlib/$(rustc -vV | sed -n 's|host: ||p')/bin/llvm-nm
nm_output=$($LLVM_NM ../target/$target/release/libsimdutf8.rlib)
if [[ $target == *darwin* ]]; then
    pattern=" (t|T|U) _"
    cut_arg=21
elif [[ $target == *armv7* ]]; then
    pattern=" (t|T|U) "
    cut_arg=12
else
    pattern=" (t|T|U) "
    cut_arg=20
fi
inline_ignore_pattern='drop_in_place|core::str::converts::from_utf8|::fmt::|^\$x\.|^<T as core::convert::From<T>>::from$|^core::result::Result<T,E>::map_err$'
echo "$nm_output" | rustfilt | egrep "$pattern" | cut -c "$cut_arg"- | grep -Ev "$inline_ignore_pattern" | sort | diff -u $expected_fns -
