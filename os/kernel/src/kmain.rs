#![feature(lang_items)]
#![feature(core_intrinsics)]
#![feature(const_fn)]
#![feature(asm)]
#![feature(optin_builtin_traits)]
#![feature(decl_macro)]
#![feature(repr_align)]
#![feature(attr_literals)]
#![feature(never_type)]
#![feature(ptr_internals)]

#[macro_use]
extern crate core;
extern crate pi;
extern crate stack_vec;

use pi::uart::MiniUart;
use core::fmt::Write;
use console::kprint;

pub mod lang_items;
pub mod mutex;
pub mod console;
pub mod shell;

#[no_mangle]
pub extern "C" fn kmain() {
    // FIXME: Start the shell.
//     shell::shell("> ")
    loop {
        kprint!("{}", "test");
    }
}
