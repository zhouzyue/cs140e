//use core::alloc::{Layout, GlobalAlloc};
//
//#[derive(Debug)]
//pub struct Allocator {
//    current: usize,
//    end: usize,
//}
//
//impl Allocator {
//    /// Returns an uninitialized `Allocator`.
//    ///
//    /// The allocator must be initialized by calling `initialize()` before the
//    /// first memory allocation. Failure to do will result in panics.
//    pub const fn uninitialized() -> Self {
//        Allocator {
//            current: 0,
//            end: 0,
//        }
//    }
//
//    /// Initializes the memory allocator.
//    ///
//    /// # Panics
//    ///
//    /// Panics if the system's memory map could not be retrieved.
//    pub fn initialize(&self) {
////        let (start, end) = memory_map().expect("failed to find memory map");
////        *self.0.lock() = Some(imp::Allocator::new(start, end));
////        unimplemented!("bump allocator")
//
//    }
//}
//
//unsafe impl GlobalAlloc for Allocator {
//    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
////        self.0.lock().as_mut().expect("allocator uninitialized").alloc(layout).unwrap()
////        unimplemented!("bump allocator")
////        self.alloc(layout).unwrap()
//        0 as *mut u8
//    }
//
//    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
////        self.0.lock().as_mut().expect("allocator uninitialized").dealloc(ptr, layout);
////        unimplemented!("bump allocator")
//    }
//}