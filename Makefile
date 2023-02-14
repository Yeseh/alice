all: host tasks 

host: src/main.rs
	cargo build

tasks: demotask validate

demotask: tasks/demotask/src/lib.rs
	cd tasks/demotask && cargo build --target=wasm32-wasi --out-dir ../compiled/demotask -Z unstable-options
	wasm-tools component new ./tasks/compiled/demotask/demotask.wasm -o ./tasks/compiled/demotask/demotask.component.wasm --adapt ./common/wasi_snapshot_preview1.wasm

validate: tasks/compiled/demotask/demotask.component.wasm
	wasm-tools validate tasks/compiled/demotask/demotask.component.wasm --features component-model
	wasm-tools component wit tasks/compiled/demotask/demotask.component.wasm

.PONY: run
run: 
	cargo run