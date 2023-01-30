wit_bindgen_guest_rust::generate!("task");
// use wasm_bindgen::*;

use host::*;

pub struct TaskImpl;

impl Task for TaskImpl {
    #[no_mangle]
    fn init() -> i32 {
        1
    }

    #[no_mangle]
    fn run() -> i32 {
        host::test();
        1
    }

    #[no_mangle]
    fn dispose() -> i32 {
        1
    }
}
