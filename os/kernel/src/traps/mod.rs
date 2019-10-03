use pi::interrupt::{Controller, Interrupt};
use shell::shell;

use self::irq::handle_irq;
use self::syndrome::Syndrome;
use self::syscall::handle_syscall;
pub use self::trap_frame::TrapFrame;

mod irq;
mod trap_frame;
mod syndrome;
mod syscall;

use aarch64;

#[repr(u16)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Kind {
    Synchronous = 0,
    Irq = 1,
    Fiq = 2,
    SError = 3,
}

#[repr(u16)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Source {
    CurrentSpEl0 = 0,
    CurrentSpElx = 1,
    LowerAArch64 = 2,
    LowerAArch32 = 3,
}

#[repr(C)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Info {
    source: Source,
    kind: Kind,
}

use console::kprint;

/// This function is called when an exception occurs. The `info` parameter
/// specifies the source and kind of exception that has occurred. The `esr` is
/// the value of the exception syndrome register. Finally, `tf` is a pointer to
/// the trap frame for the exception.
#[no_mangle]
pub extern fn handle_exception(info: Info, esr: u32, tf: &mut TrapFrame) {
//    kprint!("exception {:?}\r\n", info);
    let syndrome = Syndrome::from(esr);
    if info.kind == Kind::Synchronous {
        match syndrome {
            Syndrome::Svc(num) => {
                handle_syscall(num, tf)
            }
            Syndrome::Brk(_) => {
                shell("! ");
                tf.elr += 4;
                return;
            }
            other => {
                kprint!("syndrome {:?}", other);
            }
        }
    } else if info.kind == Kind::Irq {
        if Controller::new().is_pending(Interrupt::Timer1) {
//            use pi::timer::current_time;
//            kprint!("handling timer irq {}\r\n", current_time() / 1000);
            handle_irq(Interrupt::Timer1, tf);
        }
        return;
    }
    loop {
        aarch64::nop();
    }
}
