#![feature(asm, lang_items)]
#![feature(alloc_error_handler)]
#![feature(allocator_api)]
#![feature(optin_builtin_traits)]
//#![no_std]

extern crate xmodem;
extern crate pi;
#[macro_use]
extern crate core;
extern crate alloc;

pub mod allocator;
pub mod mutex;

use std::io;
use std::io::Cursor;
use core::fmt::Write;
use allocator::Allocator;

pub mod lang_items;

/// Start address of the binary to load and of the bootloader.
const BINARY_START_ADDR: usize = 0x80000;
const BOOTLOADER_START_ADDR: usize = 0x4000000;

/// Pointer to where the loaded binary expects to be laoded.
const BINARY_START: *mut u8 = BINARY_START_ADDR as *mut u8;

/// Free space between the bootloader and the loaded binary's start address.
const MAX_BINARY_SIZE: usize = BOOTLOADER_START_ADDR - BINARY_START_ADDR;

/// Branches to the address `addr` unconditionally.
fn jump_to(addr: *mut u8) -> ! {
    unsafe {
        asm!("br $0" : : "r"(addr as usize));
        loop { asm!("nop" :::: "volatile") }
    }
}

#[global_allocator]
pub static ALLOCATOR: Allocator = Allocator::uninitialized();

#[no_mangle]
pub extern "C" fn kmain() {
    let mut uart = pi::uart::MiniUart::new();
    uart.set_read_timeout(750);

    loop {
        let address = unsafe { core::slice::from_raw_parts_mut(BINARY_START, MAX_BINARY_SIZE) };
        match xmodem::Xmodem::receive(&mut uart, Cursor::new(address)) {
            Ok(_) => {
                jump_to(BINARY_START);
            },
            Err(err) => match err.kind() {
                io::ErrorKind::TimedOut => continue,
                e => {
                    uart.write_fmt(format_args!("err {}\r\n", "!")).unwrap();
                    continue
                }
            }
        }
    }
}
