#![no_std]
#![no_main]

use alloc::format;
use alloc::vec;
use ckb_std::default_alloc;
use ckb_std::syscalls::debug;

ckb_std::entry!(program_entry);
default_alloc!();

fn program_entry() -> i8 {
    let mut v = vec![];
    debug(format!("will panic because of oom"));
    for i in 1..10000000 {
        v.push(format!("This is the string {}", i));
    }
    debug(format!("there are totally {} items in vector", v.len()));
    0
}
