

CKB_DEBUGGER ?= ckb-debugger


all:
	cargo build --release --bin vanilla --target riscv64imac-unknown-none-elf
	cargo build --release --target riscv64imac-unknown-none-elf
	cargo build --release --bin panic --target riscv64imac-unknown-none-elf
	cargo build --release --bin oom --target riscv64imac-unknown-none-elf
	cargo build --release --bin bytes --target riscv64imac-unknown-none-elf
run:
	$(CKB_DEBUGGER) --bin target/riscv64imac-unknown-none-elf/release/ckb-stable-rust-demo

run-panic:
	$(CKB_DEBUGGER) --bin target/riscv64imac-unknown-none-elf/release/panic

run-oom:
	$(CKB_DEBUGGER) --bin target/riscv64imac-unknown-none-elf/release/oom

run-vanilla:
	$(CKB_DEBUGGER) --bin target/riscv64imac-unknown-none-elf/release/vanilla	

run-bytes:
	$(CKB_DEBUGGER) --bin target/riscv64imac-unknown-none-elf/release/bytes

ci: all
	make run 
	make run-vanilla
	make run-bytes
	make run-panic || echo "ignore"
	make run-oom  || echo "ignore"


install:
	rustup target add riscv64imac-unknown-none-elf
	wget 'https://github.com/nervosnetwork/ckb-standalone-debugger/releases/download/v0.108.1/ckb-debugger-linux-x64.tar.gz'
	tar -xzvf ckb-debugger-linux-x64.tar.gz && mv ckb-debugger ~/.cargo/bin
	wget https://apt.llvm.org/llvm.sh && chmod +x llvm.sh && sudo ./llvm.sh 16 && rm llvm.sh
