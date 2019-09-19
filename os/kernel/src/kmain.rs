#![feature(lang_items)]
#![feature(core_intrinsics)]
#![feature(const_fn)]
#![feature(asm)]
#![feature(optin_builtin_traits)]
#![feature(decl_macro)]
#![feature(exclusive_range_pattern)]
#![feature(never_type)]
#![feature(naked_functions)]
#![feature(allocator_api, global_allocator)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]
#![feature(raw_vec_internals)]
#![feature(try_reserve)]
#![feature(ptr_internals)]

#[macro_use]
#[allow(unused_imports)]
extern crate alloc;
#[macro_use]
extern crate core;
extern crate pi;
extern crate stack_vec;
extern crate fat32;

pub mod allocator;
use pi::uart::MiniUart;
use console::{kprint, CONSOLE};

pub mod lang_items;
pub mod mutex;
pub mod console;
pub mod shell;
pub mod fs;
pub mod traps;
pub mod aarch64;
pub mod process;
pub mod vm;

#[cfg(not(test))]
use allocator::Allocator;
use fs::FileSystem;
use process::GlobalScheduler;

#[cfg(not(test))]
#[global_allocator]
pub static ALLOCATOR: Allocator = Allocator::uninitialized();

pub static FILE_SYSTEM: FileSystem = FileSystem::uninitialized();

pub static SCHEDULER: GlobalScheduler = GlobalScheduler::uninitialized();

#[no_mangle]
#[cfg(not(test))]
pub extern "C" fn kmain() {
    let mut uart = MiniUart::new();
    loop {
//        kprint!("1");
        uart.write_byte(0x42);
    }
    // ALLOCATOR.initialize();
}

