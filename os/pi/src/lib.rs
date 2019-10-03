#![feature(core_intrinsics)]
#![feature(const_fn)]
#![feature(asm)]
#![feature(decl_macro)]
#![feature(never_type)]
#![no_std]

extern crate std;
extern crate volatile;

pub mod atags;
pub mod common;
pub mod gpio;
pub mod timer;
pub mod uart;
pub mod interrupt;
