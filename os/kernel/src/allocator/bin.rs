use std::fmt;

use allocator::util::*;
use alloc::alloc::{AllocErr, Layout};
use core::fmt::{Error, Formatter};
use std::cmp::{max, min};
use std::mem::size_of;

use allocator::linked_list::LinkedList;

/// A simple allocator that allocates based on size classes.
pub struct Allocator {
    list: [LinkedList; 32],
    allocated: usize,
    length: usize,
    total: usize
}

const BIN_SIZE: usize = size_of::<usize>();

pub fn prev_power_of_two(num: usize) -> usize {
    1 << (8 * (size_of::<usize>()) - num.leading_zeros() as usize - 1)
}

impl Allocator {
    /// Creates a new bin allocator that will allocate memory from the region
    /// starting at address `start` and ending at address `end`.
    pub fn new(start: usize, end: usize) -> Allocator {
        let mut list = [LinkedList::new(); 32];
        let mut cur = start;
        let mut total = 0;
        while cur + BIN_SIZE <= end {
            let max_allocable = prev_power_of_two(end - cur);
            let cur_aligned = cur & (!cur + 1);
            let size = min(cur_aligned, max_allocable);
            total += size;
            unsafe {
                list[(size / BIN_SIZE).trailing_zeros() as usize].push(cur as *mut usize);
            }
            cur += size;
        }
        Allocator {
            list,
            allocated: 0,
            length: end - start,
            total,
        }
    }

    /// Allocates memory. Returns a pointer meeting the size and alignment
    /// properties of `layout.size()` and `layout.align()`.
    ///
    /// If this method returns an `Ok(addr)`, `addr` will be non-null address
    /// pointing to a block of storage suitable for holding an instance of
    /// `layout`. In particular, the block will be at least `layout.size()`
    /// bytes large and will be aligned to `layout.align()`. The returned block
    /// of storage may or may not have its contents initialized or zeroed.
    ///
    /// # Safety
    ///
    /// The _caller_ must ensure that `layout.size() > 0` and that
    /// `layout.align()` is a power of two. Parameters not meeting these
    /// conditions may result in undefined behavior.
    ///
    /// # Errors
    ///
    /// Returning `Err` indicates that either memory is exhausted
    /// (`AllocError::Exhausted`) or `layout` does not meet this allocator's
    /// size or alignment constraints (`AllocError::Unsupported`).
    pub fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
        let align = max(layout.align(), BIN_SIZE);
        let size = max(layout.size().next_power_of_two(), align);

        let bin = (size / BIN_SIZE).trailing_zeros() as usize;

        for i in bin..self.list.len() {
            if self.list[i].peek().is_some() {
                for j in (bin + 1..i + 1).rev() {
                    let addr = self.list[j].pop().expect("reverse order should have value");
                    unsafe {
                        self.list[j - 1].push(addr);
                        self.list[j - 1].push((addr as usize + (BIN_SIZE << (j - 1))) as *mut usize);
                    }
                }

                self.allocated += size;

                let result = self.list[bin].pop().expect("this bin should have space now");
                return Ok(result as *mut u8);
            }
        }
        Err(AllocErr)
    }

    /// Deallocates the memory referenced by `ptr`.
    ///
    /// # Safety
    ///
    /// The _caller_ must ensure the following:
    ///
    ///   * `ptr` must denote a block of memory currently allocated via this
    ///     allocator
    ///   * `layout` must properly represent the original layout used in the
    ///     allocation call that returned `ptr`
    ///
    /// Parameters not meeting these conditions may result in undefined
    /// behavior.
    pub fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        let align = max(layout.align(), BIN_SIZE);
        let size = max(layout.size().next_power_of_two(), align);

        let bin = (size / BIN_SIZE).trailing_zeros() as usize;

        unsafe {
            self.list[bin].push(ptr as *mut usize);

            let next_bin_start = BIN_SIZE << (bin + 1);
            let n = next_bin_start >> 2;
            let pair = ptr as usize ^ n;

            let mut exists = false;
            for node in self.list[bin].iter_mut() {
                if node.value() as usize == pair {
                    node.pop();
                    exists = true;
                    break;
                }
            }

            if exists {
                self.list[bin].pop();
                self.list[bin + 1].push(min(ptr as usize, pair) as *mut usize);
            }
        }

        self.allocated -= size;
    }
}

impl core::fmt::Debug for Allocator {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Allocator {{ length: {}, allocate length: {}, allocated: {}, list: {:?} }}\n",
               self.length, self.total, self.allocated, self.list)
    }
}

