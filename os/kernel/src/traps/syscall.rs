use traps::TrapFrame;
use pi::timer;
use SCHEDULER;
use process::{State, Process};

/// Sleep for `ms` milliseconds.
///
/// This system call takes one parameter: the number of milliseconds to sleep.
///
/// In addition to the usual status value, this system call returns one
/// parameter: the approximate true elapsed time from when `sleep` was called to
/// when `sleep` returned.
pub fn sleep(ms: u32, tf: &mut TrapFrame) {
    let start_time = timer::current_time();
    let end_time = start_time + (ms as u64) * 1000;
    let boxed_fnmut = Box::new(move |p: &mut Process| {
        let now = timer::current_time();
        if now < end_time {
            false
        } else {
            p.trap_frame.x0 = (now - start_time) / 1000;
            p.trap_frame.x7 = 0;
            true
        }
    });
    SCHEDULER.switch(State::Waiting(boxed_fnmut), tf).unwrap();
}

pub fn handle_syscall(num: u16, tf: &mut TrapFrame) {
    if num == 1 {
        sleep(tf.x0 as u32, tf);
    }
}
