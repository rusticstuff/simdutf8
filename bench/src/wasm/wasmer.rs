//! Runtime support for embedding WASM simdutf8 shim into [Wasmer](https://wasmer.io/).

use std::convert::TryInto;
use wasmer::{imports, Instance, Module, NativeFunc, Store};

use super::{WasmValidator, PAGE_SIZE, WASM_SHIM_CODE};

type ValidatorFunc = NativeFunc<(i32, i32), i32>;

pub struct WasmerValidator {
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

impl WasmValidator for WasmerValidator {
    fn new(input: &[u8]) -> Self {
        // TODO consider cleaning this up to do better result handling

        // The code here is similar to its wasmtime counterpart but the APIs are subtly different

        // we could be smarter and do the compilation once, but as long as we don't benchmark
        // the compilation it doesn't really matter
        let store = Store::default();
        let module = Module::new(&store, WASM_SHIM_CODE).expect("Invalid WASM shim");
        let imports_obj = imports![];
        let instance = Instance::new(&module, &imports_obj).expect("Unable to load WASM shim");
        let exports = &instance.exports;

        // get the memory--there should be a single exported memory
        let memory = exports
            .get_memory("memory")
            .expect("Could not get exported memory from WASM shim");
        let start: i32 = memory
            .size()
            .bytes()
            .0
            .try_into()
            .expect("Slice position too big for WASM");

        // grow it to hold the input
        let len = input
            .len()
            .try_into()
            .expect("Slice length too big for WASM");
        memory
            .grow(((input.len() as u32) / PAGE_SIZE as u32) + 1)
            .expect("Unable to grow memory for WASM shim");

        // copy input into linear memory
        unsafe {
            memory.data_unchecked_mut()[start as usize..(start + len) as usize]
                .copy_from_slice(input)
        }

        // get the function bindings
        let std_func = exports
            .get_native_function::<(i32, i32), i32>("std_from_utf8")
            .expect("Could not get std_from_utf8");
        let compat_func = exports
            .get_native_function::<(i32, i32), i32>("compat_from_utf8")
            .expect("Could not get compat_from_utf8");
        let basic_func = exports
            .get_native_function::<(i32, i32), i32>("basic_from_utf8")
            .expect("Could not get basic_from_utf8");

        // return an "environment" from which we can invoke the validation routine
        Self {
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
            .call(self.start, self.len)
            .expect("Could not evaluate WASM std_from_utf8");
        return len == self.len;
    }

    #[inline]
    fn compat_from_utf8(&mut self) -> bool {
        let len = self
            .compat_func
            .call(self.start, self.len)
            .expect("Could not evaluate WASM compat_from_utf8");
        len == self.len
    }

    #[inline]
    fn basic_from_utf8(&mut self) -> bool {
        let res = self
            .basic_func
            .call(self.start, self.len)
            .expect("Could not evaluate WASM basic_from_utf8");
        res != 0
    }
}
