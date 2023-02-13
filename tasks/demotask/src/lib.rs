wit_bindgen_guest_rust::generate!({
    world: "task",
    path: "../../wit/task.wit"
});

pub struct Task;

impl task::Task for Task {
    fn init() -> i32 {
        1
    }
    
    fn run() -> i32 {
        host::test();
        1
    }
    
    fn dispose() -> i32 {
        1
    }
}

export_task_world!(Task);