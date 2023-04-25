#![no_std]
#![no_main]

extern crate alloc;
use alloc::format;
use core::{
    alloc::{GlobalAlloc, Layout},
    arch::asm,
};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        asm!(
            "lw a0, 0(sp)",
            "addi a1, sp, 8",
            "li a2, 0",
            "call {}",
            "li a7, 93",
            "ecall",
            sym main,
            options(noreturn)
        );
    }
}

#[panic_handler]
fn panic_handler(panic_info: &core::panic::PanicInfo) -> ! {
    let mut s = alloc::string::String::new();
    if let Some(p) = panic_info.payload().downcast_ref::<&str>() {
        s.push_str(&format!("panic occurred: {:?}", p));
    } else {
        s.push_str(&format!("panic occurred:"));
    }
    if let Some(location) = panic_info.location() {
        s.push_str(&format!(
            ", in file {}:{}",
            location.file(),
            location.line()
        ));
    } else {
        s.push_str(&format!(", but can't get location information..."));
    }
    debug(s);
    exit(-1)
}

unsafe fn syscall(
    mut a0: u64,
    a1: u64,
    a2: u64,
    a3: u64,
    a4: u64,
    a5: u64,
    a6: u64,
    a7: u64,
) -> u64 {
    asm!(
      "ecall",
      inout("a0") a0,
      in("a1") a1,
      in("a2") a2,
      in("a3") a3,
      in("a4") a4,
      in("a5") a5,
      in("a6") a6,
      in("a7") a7
    );
    a0
}

pub const SYS_EXIT: u64 = 93;
pub const SYS_DEBUG: u64 = 2177;

pub fn exit(code: i8) -> ! {
    unsafe { syscall(code as u64, 0, 0, 0, 0, 0, 0, SYS_EXIT) };
    loop {}
}

pub fn debug(mut s: alloc::string::String) {
    s.push('\0');
    let c_str = s.into_bytes();
    unsafe {
        syscall(c_str.as_ptr() as u64, 0, 0, 0, 0, 0, 0, SYS_DEBUG);
    }
}

const HEAP_SIZE: usize = 512 * 1024;
static mut HEAP: [u8; HEAP_SIZE] = [0u8; HEAP_SIZE];
static mut USED_HEAP_SIZE: usize = 0;

struct NeverFreeAlloc {}

unsafe impl GlobalAlloc for NeverFreeAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let size = if size < 4 {
            4
        } else {
            if size % 4 != 0 {
                (size / 4 + 1) * 4
            } else {
                size
            }
        };
        let p = (&mut HEAP[USED_HEAP_SIZE..USED_HEAP_SIZE + size]).as_mut_ptr();
        USED_HEAP_SIZE += layout.size();
        p
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[global_allocator]
static ALLOC: NeverFreeAlloc = NeverFreeAlloc {};

pub fn main() -> i8 {
    debug(format!("hello, {}", "world"));
    0
}
