#![feature(lang_items)]
#![feature(core_intrinsics)]
#![feature(const_fn)]
#![feature(asm)]
#![feature(optin_builtin_traits)]
#![feature(decl_macro)]
#![feature(repr_align)]
#![feature(attr_literals)]
#![feature(exclusive_range_pattern)]
#![feature(alloc, allocator_api, global_allocator)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]

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

#[cfg(not(test))]
use allocator::Allocator;
use fs::FileSystem;

#[cfg(not(test))]
#[global_allocator]
pub static ALLOCATOR: Allocator = Allocator::uninitialized();

pub static FILE_SYSTEM: FileSystem = FileSystem::uninitialized();

#[no_mangle]
#[cfg(not(test))]
pub extern "C" fn kmain() {
    loop {
        kprint!("1");
    }
    // ALLOCATOR.initialize();
}
