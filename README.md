# ckb-stable-rust-demo
A demonstration showcasing the seamless process of writing contracts on CKB,
using the officially supported Rust and Clang toolchains.

## Requirements
- Official Clang with 16 or above
- Official Rust with 1.69 or above

## Build
Install ckb-debugger, Rust target:
```
make install
```

Build:
```
git submodule update --init --recursive
make all
```

Build & Run:
```
make ci
```
