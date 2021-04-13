# Issues
* nostd
* neon support
* avx512 support


# TODO
* benchmark: on bulldog as well
* restructure implementations -> implementation get_available_implementations(), set_implementation(), default: autodetect, check e.g. memchr impl
* from_utf8_mut()/run_validation() restructuring
* is_valid_until() implementation
* fuzz testing
* implementation traits: overhead?
* macro_export needed?
* test cpu feature detection overhead


# NEXT
* emoij
* benchmark: restructure baselines: std/std-avx2/base-avx2/base-sse4.2/change-avx2/change-sse4.2 (?)
* use critcmp export