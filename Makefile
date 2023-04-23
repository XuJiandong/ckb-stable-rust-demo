

CKB_DEBUGGER ?= ckb-debugger-2023


all:
	cargo build --release --target riscv64imac-unknown-none-elf
	cargo build --release --bin panic --target riscv64imac-unknown-none-elf
	cargo build --release --bin oom --target riscv64imac-unknown-none-elf

run:
	$(CKB_DEBUGGER) --bin target/riscv64imac-unknown-none-elf/release/ckb-stable-rust-demo

run-panic:
	$(CKB_DEBUGGER) --bin target/riscv64imac-unknown-none-elf/release/panic

run-oom:
	$(CKB_DEBUGGER) --bin target/riscv64imac-unknown-none-elf/release/oom

install:
	rustup target add riscv64imac-unknown-none-elf
	cargo install --rev 6bb45f3 --git https://github.com/nervosnetwork/ckb-standalone-debugger ckb-debugger
	mv ~/.cargo/bin/ckb-debugger ~/.cargo/bin/ckb-debugger-2023
