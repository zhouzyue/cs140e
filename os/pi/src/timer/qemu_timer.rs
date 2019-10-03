use volatile::{ReadVolatile, Volatile};
use volatile::{Readable, Writeable, ReadableWriteable};

// Quad-A7 control
const TIMER_REG_BASE: usize = 0x4000_0000;

#[repr(C)]
#[allow(non_snake_case)]
struct Registers {
    CONTROL: Volatile<u32>,
    __OTHER: [ReadVolatile<u32>; 15],
    TIMER_INTERRUPT_CONTROL: [ReadVolatile<u32>; 4],
    __MAIL: [ReadVolatile<u32>; 4],
    CORE_IRQ_SOURCE: [Volatile<u32>; 4],
}

pub struct Timer {
    registers: &'static mut Registers,
}

pub fn set_cntp_tval_el0(x: u64) {
    unsafe {
        asm!("msr cntp_tval_el0, $0" :: "r"(x));
    }
}

pub fn read_cntfrq_el0() -> u64 {
    let x: u64;
    unsafe {
        asm!("mrs $0, cntfrq_el0"
            : "=r"(x));
    }
    x
}

pub fn read_cntpct_el0() -> u64 {
    let x: u64;
    unsafe {
        asm!("isb
              mrs $0, cntpct_el0"
            : "=r"(x) ::: "volatile");
    }
    x
}

impl Timer {
    /// Returns a new instance of `Timer`.
    pub fn new() -> Timer {
        Timer {
            registers: unsafe { &mut *(TIMER_REG_BASE as *mut Registers) },
        }
    }

    /// Reads the system timer's counter and returns the 64-bit counter value.
    /// The returned value is the number of elapsed microseconds.
    pub fn read(&self) -> u64 {
        1_000_000 * read_cntpct_el0() / read_cntfrq_el0()
    }

    /// Sets up a match in timer 1 to occur `us` microseconds from now. If
    /// interrupts for timer 1 are enabled and IRQs are unmasked, then a timer
    /// interrupt will be issued in `us` microseconds.
    pub fn tick_in(&mut self, us: u32) {
        let count = us as u64 * read_cntfrq_el0() / 1_000_000;
        set_cntp_tval_el0(count);
    }
}

/// Returns the current time in microseconds.
pub fn current_time() -> u64 {
    Timer::new().read()
}

/// Spins until `us` microseconds have passed.
pub fn spin_sleep_us(us: u64) {
    let end = current_time() + us;
    loop {
        if current_time() >= end {
            break;
        }
    }
}

/// Spins until `ms` milliseconds have passed.
pub fn spin_sleep_ms(ms: u64) {
    spin_sleep_us(ms * 1000)
}

/// Sets up a match in timer 1 to occur `us` microseconds from now. If
/// interrupts for timer 1 are enabled and IRQs are unmasked, then a timer
/// interrupt will be issued in `us` microseconds.
pub fn tick_in(us: u32) {
    Timer::new().tick_in(us)
}



