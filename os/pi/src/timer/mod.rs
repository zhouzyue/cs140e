#[cfg(not(feature="qemu"))]
mod timer;
#[cfg(not(feature="qemu"))]
pub use self::timer::*;

#[cfg(feature="qemu")]
mod qemu_timer;
#[cfg(feature="qemu")]
pub use self::qemu_timer::*;
