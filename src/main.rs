#![no_std]
#![no_main]
// #![feature(lang_items)]
// #![feature(alloc_error_handler)]
// #![feature(panic_info_message)]

use alloc::format;
use ckb_std::default_alloc;
use ckb_std::syscalls::debug;

ckb_std::entry!(program_entry);
default_alloc!();

fn program_entry() -> i8 {
    debug(format!("hello, {}", "world"));
    0
}
