#![no_std]
#![no_main]

use alloc::format;
use alloc::vec;
use ckb_std::default_alloc;
use ckb_std::syscalls::debug;

ckb_std::entry!(program_entry);
default_alloc!();

fn program_entry() -> i8 {
    debug(format!("will panic"));
    let v = vec![1, 2, 3, 4];
    let value = v[100];
    debug(format!("v[100] is out of index: {}", value));
    0
}
