mod task;

use crate::task::*;

// wit_bindgen_guest_rust::generate!("task");
// use wasm_bindgen::*;

export_task!(dyn Task);

pub struct TaskImpl;

impl task::Task for TaskImpl {
    fn init() -> i32 {
        1
    }

    fn run() -> i32 {
        task::host::test();
        1
    }

    fn dispose() -> i32 {
        1
    }
}
