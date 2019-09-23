mod linked_list;
mod util;

#[path = "bin.rs"]
mod imp;

#[cfg(test)]
mod tests;

use mutex::Mutex;
use core::alloc::{Alloc, AllocErr, Layout, GlobalAlloc};
use core::ptr::NonNull;
use std::cmp::max;

/// Thread-safe (locking) wrapper around a particular memory allocator.
#[derive(Debug)]
pub struct Allocator(Mutex<Option<imp::Allocator>>);

impl Allocator {
    /// Returns an uninitialized `Allocator`.
    ///
    /// The allocator must be initialized by calling `initialize()` before the
    /// first memory allocation. Failure to do will result in panics.
    pub const fn uninitialized() -> Self {
        Allocator(Mutex::new(None))
    }

    /// Initializes the memory allocator.
    ///
    /// # Panics
    ///
    /// Panics if the system's memory map could not be retrieved.
    pub fn initialize(&self) {
        let (start, end) = memory_map().expect("failed to find memory map");
        *self.0.lock() = Some(imp::Allocator::new(start, end));
    }
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.0.lock().as_mut().expect("allocator uninitialized").alloc(layout).unwrap()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.0.lock().as_mut().expect("allocator uninitialized").dealloc(ptr, layout);
    }
}

extern "C" {
    static _end: u8;
}

use pi::atags::Atags;

/// Returns the (start address, end address) of the available memory on this
/// system if it can be determined. If it cannot, `None` is returned.
///
/// This function is expected to return `Some` under all normal cirumstances.
fn memory_map() -> Option<(usize, usize)> {
    let binary_end = unsafe { (&_end as *const u8) as u32 };

    let mut atags = Atags::get();
    while let Some(atag) = atags.next() {
        if let Some(mem) = atag.mem() {
            return Some((binary_end as usize, (mem.start + mem.size) as usize));
        }
    }
    None
}
