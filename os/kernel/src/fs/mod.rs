pub use fat32::traits;
use fat32::vfat::{Shared, VFat};
use mutex::Mutex;

use self::sd::Sd;
use console::kprint;

pub mod sd;

#[derive(Debug)]
pub struct FileSystem(Mutex<Option<Shared<VFat>>>);

impl FileSystem {
    /// Returns an uninitialized `FileSystem`.
    ///
    /// The file system must be initialized by calling `initialize()` before the
    /// first memory allocation. Failure to do will result in panics.
    pub const fn uninitialized() -> Self {
        FileSystem(Mutex::new(None))
    }

    /// Initializes the file system.
    ///
    /// # Panics
    ///
    /// Panics if the underlying disk or file sytem failed to initialize.
    pub fn initialize(&self) {
        kprint!("loading sd\r\n");
        let r = VFat::from(Sd::new().unwrap()).unwrap();
        kprint!("sd card loaded\r\n");
        *self.0.lock() = Some(r);
        kprint!("{:?}\r\n", *self.0.lock());
    }

    pub fn get(&self) -> Shared<VFat> {
        self.0.lock().as_mut().expect("").clone()
    }
}
