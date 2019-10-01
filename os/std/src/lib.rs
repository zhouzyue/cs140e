#![stable(feature = "rust1", since = "1.0.0")]

// Don't link to std. We are std.
#![no_std]

// std may use features in a platform-specific way
#![allow(unused_features)]

// std is implemented with unstable features, many of which are internal
// compiler details that will never be stable
#![feature(allocator_api)]
#![feature(allocator_internals)]
#![feature(alloc_layout_extra)]
#![feature(allow_internal_unsafe)]
#![feature(allow_internal_unstable)]
#![feature(array_error_internals)]
#![feature(asm)]
#![feature(box_syntax)]
#![feature(cfg_target_has_atomic)]
#![feature(cfg_target_thread_local)]
#![feature(char_error_internals)]
//#![feature(compiler_builtins_lib)]
#![feature(const_fn)]
#![feature(core_intrinsics)]
#![feature(dropck_eyepatch)]
#![feature(exact_size_is_empty)]
#![feature(fixed_size_array)]
#![feature(fn_traits)]
#![feature(fused)]
#![feature(i128)]
#![feature(int_error_internals)]
#![feature(integer_atomics)]
#![feature(lang_items)]
#![feature(libc)]
#![feature(link_args)]
#![feature(linkage)]
#![feature(needs_panic_runtime)]
#![feature(never_type)]
#![feature(on_unimplemented)]
#![feature(optin_builtin_traits)]
#![feature(prelude_import)]
#![feature(ptr_internals)]
#![feature(raw)]
#![feature(rustc_attrs)]
#![feature(slice_internals)]
#![feature(slice_patterns)]
#![feature(staged_api)]
#![feature(stmt_expr_attributes)]
#![feature(str_internals)]
#![feature(test, rustc_private)]
#![feature(thread_local)]
#![feature(unboxed_closures)]
#![feature(untagged_unions)]
#![feature(unwind_attributes)]
#![feature(doc_cfg)]
#![feature(doc_masked)]
#![feature(doc_spotlight)]

//#![feature(shared)] //- added due to no NonNull changes
//#![feature(unique)] //- added due to no NonNull changes

#![feature(custom_attribute)]
#![feature(hashmap_internals)]
#![feature(try_reserve)]
#![feature(toowned_clone_into)]

#![feature(core_panic)]
#![feature(core_panic_info)]
#![default_lib_allocator]

#[prelude_import]
#[allow(unused)]
use prelude::v1::*;

// We want to re-export a few macros from core but libcore has already been
// imported by the compiler (via our #[no_std] attribute) In this case we just
// add a new crate name so we can attach the re-exports to it.
//- #[macro_reexport(assert, assert_eq, assert_ne, debug_assert, debug_assert_eq,
//#[macro_reexport(panic, assert, assert_eq, assert_ne, debug_assert, debug_assert_eq,
//                 debug_assert_ne, unreachable, unimplemented, write, writeln, try)]
//#[]
//pub use core::{panic,  assert_eq, assert_ne, debug_assert, debug_assert_eq,
//                 debug_assert_ne, unreachable, unimplemented, write, writeln, try};
#[macro_use]
extern crate core as __core;

#[macro_use]
//#[macro_reexport(vec, format)]
extern crate alloc;

// The standard macros that are not built-in to the compiler.
#[macro_use]
mod macros;

// The Rust prelude
pub mod prelude;

// Public module declarations and re-exports
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::any;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::cell;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::clone;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::cmp;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::convert;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::default;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::hash;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::intrinsics;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::iter;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::marker;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::mem;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::ops;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::ptr;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::raw;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::result;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::option;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::isize;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::i8;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::i16;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::i32;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::i64;
#[unstable(feature = "i128", issue = "35118")]
pub use core::i128;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::usize;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::u8;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::u16;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::u32;
#[stable(feature = "rust1", since = "1.0.0")]
pub use core::u64;
#[stable(feature = "rust1", since = "1.0.0")]
pub use alloc::boxed;
#[stable(feature = "rust1", since = "1.0.0")]
pub use alloc::rc;
#[stable(feature = "rust1", since = "1.0.0")]
pub use alloc::borrow;
#[stable(feature = "rust1", since = "1.0.0")]
pub use alloc::fmt;
#[stable(feature = "rust1", since = "1.0.0")]
pub use alloc::slice;
#[stable(feature = "rust1", since = "1.0.0")]
pub use alloc::str;
#[stable(feature = "rust1", since = "1.0.0")]
pub use alloc::string;
#[stable(feature = "rust1", since = "1.0.0")]
pub use alloc::vec;
#[unstable(feature = "i128", issue = "35118")]
pub use core::u128;

pub mod ascii;
pub mod collections;
pub mod error;
pub mod ffi;
pub mod io;
pub mod num;
pub mod os;
pub mod path;
pub mod sync;

//- // Platform-abstraction modules
#[macro_use]
mod sys_common;
mod sys;

// Private support modules
//- mod panicking;
mod memchr;
//mod env;

// Include a number of private modules that exist solely to provide
// the rustdoc documentation for primitive types. Using `include!`
// because rustdoc only looks for these modules at the crate level.
include!("primitive_docs.rs");
