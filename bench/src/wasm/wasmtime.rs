//! Runtime support for embedding WASM simdutf8 shim into [Wasmtime](https://wasmtime.dev/).

use std::convert::TryInto;
use wasmtime::{Config, Engine, Instance, Module, Store, TypedFunc};

use super::{WasmValidator, PAGE_SIZE, WASM_SHIM_CODE};

type ValidatorFunc = TypedFunc<(i32, i32), i32>;

pub struct WasmtimeValidator {
    /// The underlying storage for our WASM components.
    store: Store<()>,
    /// The guest function for std implementation.
    std_func: ValidatorFunc,
    /// The guest function for compat implementation.
    compat_func: ValidatorFunc,
    /// The guest function for basic implementation.
    basic_func: ValidatorFunc,
    /// The position in linear memory where the input resides
    start: i32,
    /// The length of the input
    len: i32,
}

impl WasmValidator for WasmtimeValidator {
    fn new(input: &[u8]) -> Self {
        // TODO consider cleaning this up to do better result handling

        // The code here is similar to its wasmer counterpart but the APIs are subtly different

        // we could be smarter and do the compilation once, but as long as we don't benchmark
        // the compilation it doesn't really matter
        let mut config = Config::new();
        config.wasm_simd(true);
        let engine =
            Engine::new(Config::new().wasm_simd(true)).expect("Could not initialize WASM engine");
        let mut store = Store::new(&engine, ());
        let module = Module::new(store.engine(), WASM_SHIM_CODE).expect("Invalid WASM shim");
        let instance = Instance::new(&mut store, &module, &[]).expect("Unable to load WASM shim");

        // get the memory--there should be a single exported memory
        let memory = instance
            .get_memory(&mut store, "memory")
            .expect("Could not get exported memory from WASM shim");
        let start: i32 = (memory.size(&store) * (PAGE_SIZE as u64))
            .try_into()
            .expect("Slice position too big for WASM");

        // grow it to hold the input
        let len = input
            .len()
            .try_into()
            .expect("Slice length too big for WASM");
        memory
            .grow(&mut store, ((input.len() as u64) / (PAGE_SIZE as u64)) + 1)
            .expect("Unable to grow memory for WASM shim");

        // copy input into linear memory
        memory.data_mut(&mut store)[start as usize..(start + len) as usize].copy_from_slice(input);

        // get the function bindings
        let std_func = instance
            .get_typed_func::<(i32, i32), i32, _>(&mut store, "std_from_utf8")
            .expect("Could not get std_from_utf8");
        let compat_func = instance
            .get_typed_func::<(i32, i32), i32, _>(&mut store, "compat_from_utf8")
            .expect("Could not get compat_from_utf8");
        let basic_func = instance
            .get_typed_func::<(i32, i32), i32, _>(&mut store, "basic_from_utf8")
            .expect("Could not get basic_from_utf8");

        Self {
            store,
            std_func,
            compat_func,
            basic_func,
            start,
            len,
        }
    }

    #[inline]
    fn std_from_utf8(&mut self) -> bool {
        let len = self
            .std_func
            .call(&mut self.store, (self.start, self.len))
            .expect("Could not evaluate WASM std_from_utf8");
        return len == self.len;
    }

    #[inline]
    fn compat_from_utf8(&mut self) -> bool {
        let len = self
            .compat_func
            .call(&mut self.store, (self.start, self.len))
            .expect("Could not evaluate WASM compat_from_utf8");
        len == self.len
    }

    #[inline]
    fn basic_from_utf8(&mut self) -> bool {
        let res = self
            .basic_func
            .call(&mut self.store, (self.start, self.len))
            .expect("Could not evaluate WASM basic_from_utf8");
        res != 0
    }
}
