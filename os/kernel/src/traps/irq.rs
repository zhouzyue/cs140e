use pi::interrupt::Interrupt;
use pi::timer::tick_in;
use process::{State, TICK};
use SCHEDULER;
use traps::TrapFrame;

pub fn handle_irq(interrupt: Interrupt, tf: &mut TrapFrame) {
    match interrupt {
        Interrupt::Timer1 => {
            tick_in(TICK);
            SCHEDULER.switch(State::Running, tf).unwrap();
        }
        _ => {}
    }

    tf.spsr &= !(1 << 7)
}
