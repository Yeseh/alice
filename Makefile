all: host tasks 

host: src/main.rs
	cargo build

tasks: demotask

demotask: tasks/demotask/src/lib.rs
	cd tasks/demotask && cargo build --target=wasm32-wasi --release --out-dir ../compiled/demotask -Z unstable-options
	wasm-tools component new ./tasks/compiled/demotask/demotask.wasm -o ./tasks/compiled/demotask/demotask.component.wasm

.PONY: run
run: 
	cargo run

