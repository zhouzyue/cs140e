#[cfg(not(feature="qemu"))]
mod interrupt;
#[cfg(not(feature="qemu"))]
pub use self::interrupt::*;

#[cfg(feature="qemu")]
mod qemu_interrupt;
#[cfg(feature="qemu")]
pub use self::qemu_interrupt::*;
