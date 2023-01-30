tasks = demotask
tasks: $(tasks)

demotask: tasks/demotask/src/lib.rs
	cd tasks/demotask && cargo build --target=wasm32-wasi --release --out-dir ../compiled -Z unstable-options

