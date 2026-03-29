//! Types related to task management

use super::TaskContext;

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// trace syscall count
    pub called_times: CalledTimes,
}

/// trace syscall count
#[derive(Copy, Clone, Default)]
pub struct CalledTimes {
    /// count of write syscalls
    pub write_c: usize,
    /// count of exit syscalls
    pub exit_c: usize,
    /// count of yield syscalls
    pub yield_c: usize,
    /// count of get_time syscalls
    pub get_time_c: usize,
    /// count of trace syscalls
    pub trace_c: usize,
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}
