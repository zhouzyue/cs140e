#![feature(decl_macro)]
#![allow(safe_packed_borrows)]
//#![no_std]

#[cfg(not(target_endian="little"))]
compile_error!("only little endian platforms supported");

//extern crate std;

#[macro_use]
extern crate core;

#[cfg(test)]
mod tests;
mod mbr;
mod util;

pub mod vfat;
pub mod traits;

pub use mbr::*;
