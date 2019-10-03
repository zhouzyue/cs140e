use volatile::{ReadVolatile, Volatile};
use volatile::prelude::*;

use interrupt::Interrupt::Timer1;

const CONTROL_BASE: usize = 0x4000_0040;

#[derive(Copy, Clone, PartialEq)]
pub enum Interrupt {
    CNTPSIRQ = 0,
    Timer1 = 1,
}

#[repr(C)]
#[allow(non_snake_case)]
struct Registers {
    CORE0_TIMER_CONTROL: Volatile<u32>,
    __OTHER: [ReadVolatile<u32>; 7],
    CORE_0_IRQ_SOURCE: Volatile<u32>,
}

/// An interrupt controller. Used to enable and disable interrupts as well as to
/// check if an interrupt is pending.
pub struct Controller {
    registers: &'static mut Registers
}

pub fn set_cntp_ctl_el0(x: u64) {
    unsafe {
        asm!("msr cntp_ctl_el0, $0" :: "r"(x));
    }
}

pub fn set_cntk_ctl_el1(x: u64) {
    unsafe {
        asm!("msr cntkctl_el1, $0" :: "r"(x));
    }
}

impl Controller {
    /// Returns a new handle to the interrupt controller.
    pub fn new() -> Controller {
        Controller {
            registers: unsafe { &mut *(CONTROL_BASE as *mut Registers) },
        }
    }

    // enable timer irq
    pub fn enable(&mut self, int: Interrupt) {
        set_cntk_ctl_el1(0b11); // D7.5.9
        set_cntp_ctl_el0(0b1); // D7.5.10
        self.registers.CORE0_TIMER_CONTROL.or_mask(1 << (int as u32));
    }

    /// Returns `true` if `int` is pending. Otherwise, returns `false`.
    pub fn is_pending(&self, int: Interrupt) -> bool {
        self.registers.CORE_0_IRQ_SOURCE.has_mask(1 << (int as u32))
    }
}
