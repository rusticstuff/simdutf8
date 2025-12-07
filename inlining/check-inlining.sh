#!/usr/bin/env bash
set -euo pipefail

target="$1"
expected_fns="$2"
build_args="${3:-}"
cargo clean --quiet
cargo build --quiet --release --target $target $build_args
LLVM_NM=$(rustc --print sysroot)/lib/rustlib/$(rustc -vV | sed -n 's|host: ||p')/bin/llvm-nm
nm_output=$($LLVM_NM "${CARGO_TARGET_DIR:-../target}/$target/release/libsimdutf8.rlib")
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
inline_ignore_pattern=\
'__aeabi_unwind_cpp_pr(0|1)|'\
'drop_in_place|'\
'core::str::converts::from_utf8|'\
'std_detect::detect::|'\
'::fmt::|'\
'^\$x\.|'\
'^<T as core::convert::From<T>>::from$|'\
'^core::str::Utf8Error::error_len$|'\
'^core::str::Utf8Error::valid_up_to$|'\
'^core::str::from_utf8$|'\
'^core::result::Result<T,E>::map_err$'
if [[ $target == *wasm* ]]; then
    inline_ignore_pattern="$inline_ignore_pattern|ct_function_table|pointer|r::converts::from_utf8|t::Formatter::write_str|t::write|mt::Formatter>::write_str"
fi
echo "$nm_output" | rustfilt | egrep "$pattern" | cut -c "$cut_arg"- | grep -Ev "$inline_ignore_pattern" | sort -u | diff -u $expected_fns -
