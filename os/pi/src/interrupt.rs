use common::IO_BASE;
use volatile::prelude::*;
use volatile::{Volatile, ReadVolatile};

const INT_BASE: usize = IO_BASE + 0xB000 + 0x200;

#[derive(Copy, Clone, PartialEq)]
pub enum Interrupt {
    Timer1 = 1,
    Timer3 = 3,
    Usb = 9,
    Gpio0 = 49,
    Gpio1 = 50,
    Gpio2 = 51,
    Gpio3 = 52,
    Uart = 57,
}

#[repr(C)]
#[allow(non_snake_case)]
struct Registers {
    BASIC_PENDING: ReadVolatile<u32>,
    PENDING: [ReadVolatile<u32>; 2],
    FIQ: Volatile<u32>,
    INTERRUPT_ENABLE_1: Volatile<u32>,
    INTERRUPT_ENABLE_2: Volatile<u32>,
    BASE_INTERRUPT_ENABLE: Volatile<u32>,
    INTERRUPT_DISABLE_1: Volatile<u32>,
    INTERRUPT_DISABLE_2: Volatile<u32>,
    BASE_DISABLE: Volatile<u32>,
}

/// An interrupt controller. Used to enable and disable interrupts as well as to
/// check if an interrupt is pending.
pub struct Controller {
    registers: &'static mut Registers
}

impl Controller {
    /// Returns a new handle to the interrupt controller.
    pub fn new() -> Controller {
        Controller {
            registers: unsafe { &mut *(INT_BASE as *mut Registers) },
        }
    }

    /// Enables the interrupt `int`.
    pub fn enable(&mut self, int: Interrupt) {
        let bit = int as u32;
        if bit < 32 {
            self.registers.INTERRUPT_ENABLE_1.or_mask(1 << bit);
        } else {
            self.registers.INTERRUPT_ENABLE_2.or_mask(1 << (bit - 32));
        }
    }

    /// Disables the interrupt `int`.
    pub fn disable(&mut self, int: Interrupt) {
        let bit = int as u32;
        if bit < 32 {
            self.registers.INTERRUPT_DISABLE_1.or_mask(1 << bit);
        } else {
            self.registers.INTERRUPT_DISABLE_2.or_mask(1 << (bit - 32));
        }
    }

    /// Returns `true` if `int` is pending. Otherwise, returns `false`.
    pub fn is_pending(&self, int: Interrupt) -> bool {
        let bit = int as u32;
        if bit < 32 {
            self.registers.PENDING[0].has_mask(1 << bit)
        } else {
            self.registers.PENDING[1].has_mask(1 << (bit - 32))
        }
    }
}
