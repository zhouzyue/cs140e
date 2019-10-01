use alloc::collections::VecDeque;

use mutex::Mutex;
use process::{Process, State, Id};
use traps::TrapFrame;
use shell::{run_shell, run_shell2};
use pi::timer::tick_in;
use pi::interrupt::{Controller, Interrupt};
use aarch64;

/// The `tick` time.
// FIXME: When you're ready, change this to something more reasonable.
//pub const TICK: u32 = 2 * 1000 * 1000;
pub const TICK: u32 = 1000;

/// Process scheduler for the entire machine.
#[derive(Debug)]
pub struct GlobalScheduler(Mutex<Option<Scheduler>>);

impl GlobalScheduler {
    /// Returns an uninitialized wrapper around a local scheduler.
    pub const fn uninitialized() -> GlobalScheduler {
        GlobalScheduler(Mutex::new(None))
    }

    /// Adds a process to the scheduler's queue and returns that process's ID.
    /// For more details, see the documentation on `Scheduler::add()`.
    pub fn add(&self, process: Process) -> Option<Id> {
        self.0.lock().as_mut().expect("scheduler uninitialized").add(process)
    }

    /// Performs a context switch using `tf` by setting the state of the current
    /// process to `new_state`, saving `tf` into the current process, and
    /// restoring the next process's trap frame into `tf`. For more details, see
    /// the documentation on `Scheduler::switch()`.
    #[must_use]
    pub fn switch(&self, new_state: State, tf: &mut TrapFrame) -> Option<Id> {
        self.0.lock().as_mut().expect("scheduler uninitialized").switch(new_state, tf)
    }

    /// Initializes the scheduler and starts executing processes in user space
    /// using timer interrupt based preemptive scheduling. This method should
    /// not return under normal conditions.
    pub fn start(&self) {
        *self.0.lock() = Some(Scheduler::new());

        Controller::new().enable(Interrupt::Timer1);
        tick_in(TICK);

        let mut process = Process::new().unwrap();
        process.trap_frame.sp = process.stack.top().as_u64();
        process.trap_frame.spsr = 0x0;
        process.trap_frame.elr = run_shell as u64;
        self.add(process);

//        let mut process2 = Process::new().unwrap();
//        process2.trap_frame.sp = process2.stack.top().as_u64();
//        process2.trap_frame.spsr = 0x0;
//        process2.trap_frame.elr = run_shell2 as u64;
//        self.add(process2);

        unsafe {
            asm!("mov sp, $0
                  bl context_restore

                  adr lr, _start
                  mov sp, lr
                  mov lr, xzr

                  eret
                  "
                  ::"r"(&*(self.0.lock().as_mut().unwrap()).processes.get(0).unwrap().trap_frame)
                  ::"volatile");
        }
    }
}

#[derive(Debug)]
struct Scheduler {
    processes: VecDeque<Process>,
    current: Option<Id>,
    last_id: Option<Id>,
}

impl Scheduler {
    /// Returns a new `Scheduler` with an empty queue.
    fn new() -> Scheduler {
        Scheduler {
            processes: VecDeque::new(),
            current: None,
            last_id: None,
        }
    }

    /// Adds a process to the scheduler's queue and returns that process's ID if
    /// a new process can be scheduled. The process ID is newly allocated for
    /// the process and saved in its `trap_frame`. If no further processes can
    /// be scheduled, returns `None`.
    ///
    /// If this is the first process added, it is marked as the current process.
    /// It is the caller's responsibility to ensure that the first time `switch`
    /// is called, that process is executing on the CPU.
    fn add(&mut self, mut process: Process) -> Option<Id> {
        if self.current.is_none() {
            self.last_id = Some(1);
            self.current = Some(1);
        } else {
            let next_id = self.last_id.unwrap().wrapping_add(1);
            if next_id < self.last_id.unwrap() {
                return None;
            }
            self.last_id = Some(next_id);
        }

        process.trap_frame.tpidr = self.last_id.unwrap();
        self.processes.push_back(process);

        self.last_id
    }

    /// Sets the current process's state to `new_state`, finds the next process
    /// to switch to, and performs the context switch on `tf` by saving `tf`
    /// into the current process and restoring the next process's trap frame
    /// into `tf`. If there is no current process, returns `None`. Otherwise,
    /// returns `Some` of the process ID that was context switched into `tf`.
    ///
    /// This method blocks until there is a process to switch to, conserving
    /// energy as much as possible in the interim.
    fn switch(&mut self, new_state: State, tf: &mut TrapFrame) -> Option<Id> {
        if self.current.is_none() {
            return None;
        }

        let mut curr = self.processes.pop_front().unwrap();
        *curr.trap_frame = *tf;
        curr.state = new_state;
        self.processes.push_back(curr);

        loop {
            for _ in 0..self.processes.len() {
                let mut next = self.processes.pop_front().unwrap();

                if next.is_ready() {
                    self.current = Some(next.trap_frame.tpidr);
                    *tf = *next.trap_frame;
                    next.state = State::Running;
                    self.processes.push_front(next);
                    return self.current;
                } else {
                    self.processes.push_back(next);
                }
            }
            aarch64::wfi();
        }
    }
}
