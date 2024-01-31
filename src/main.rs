#![no_main]
#![no_std]

use panic_halt as _;

extern "C" {
    fn assert_eq_i32(actual: i32, expected: i32);
    fn allocate(size: usize);
}
// fn assert_eq_i32(actual: i32, expected: i32) {
//     assert_eq!(actual, expected);
// }

// use wasmi::*;

use core::{
    alloc::{GlobalAlloc, Layout},
};

struct BumpPointerAlloc;

unsafe impl GlobalAlloc for BumpPointerAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe {
            allocate(layout.size());
        }
        return core::ptr::null_mut();
        // sys_alloc_aligned(layout.size(), layout.align())
    }

    unsafe fn dealloc(&self, _: *mut u8, _: Layout) {
        // this allocator never deallocates memory
    }
}

#[global_allocator]
static HEAP: BumpPointerAlloc = BumpPointerAlloc;

#[no_mangle]
fn main() {
    // First step is to create the Wasm execution engine with some config.
    // In this example we are using the default configuration.
    let engine = wasmi::Engine::default();
    // let wat = r#"
    //     (module
    //         (import "host" "hello" (func $host_hello (param i32)))
    //         (func (export "hello")
    //             (call $host_hello (i32.const 3))
    //         )
    //     )
    // "#;
    // Wasmi does not yet support parsing `.wat` so we have to convert
    // out `.wat` into `.wasm` before we compile and validate it.
    // let wasm = wat::parse_str(&wat).unwrap();
    let wasm = include_bytes!("../add.wasm");
    let module = wasmi::Module::new(&engine, &wasm[..]).unwrap();

    // All Wasm objects operate within the context of a `Store`.
    // Each `Store` has a type parameter to store host-specific data,
    // which in this case we are using `42` for.
    type HostState = ();
    let mut store = wasmi::Store::new(&engine, ());
    let host_assert = wasmi::Func::wrap(
        &mut store,
        |_caller: wasmi::Caller<'_, HostState>, lhs: i32, rhs: i32| {
            unsafe {
                assert_eq_i32(lhs, rhs);
            }
        },
    );

    // In order to create Wasm module instances and link their imports
    // and exports we require a `Linker`.
    let mut linker = <wasmi::Linker<HostState>>::new(&engine);
    // Instantiation of a Wasm module requires defining its imports and then
    // afterwards we can fetch exports by name, as well as asserting the
    // type signature of the function with `get_typed_func`.
    //
    // Also before using an instance created this way we need to start it.
    linker.define("env", "assert_eq", host_assert).unwrap();
    let _ = linker.instantiate(&mut store, &module).unwrap().start(&mut store).unwrap();
}
