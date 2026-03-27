//! Implementation of syscalls
//!
//! The single entry point to all system calls, [`syscall()`], is called
//! whenever userspace wishes to perform a system call using the `ecall`
//! instruction. In this case, the processor raises an 'Environment call from
//! U-mode' exception, which is handled as one of the cases in
//! [`crate::trap::trap_handler`].
//!
//! For clarity, each single syscall is implemented as its own function, named
//! `sys_` then the name of the syscall. You can find functions like this in
//! submodules, and you should also implement syscalls this way.

/// write syscall
const SYSCALL_WRITE: usize = 64;
/// exit syscall
const SYSCALL_EXIT: usize = 93;
/// yield syscall
const SYSCALL_YIELD: usize = 124;
/// gettime syscall
const SYSCALL_GET_TIME: usize = 169;
/// trace syscall
const SYSCALL_TRACE: usize = 410;

// trace request for sys_trace
mod called_times {
    pub static mut WRITE: usize = 0;
    pub static mut EXIT: usize = 0;
    pub static mut YIELD: usize = 0;
    pub static mut GET_TIME: usize = 0;
    pub static mut TRACE: usize = 0;
}

mod fs;
mod process;

use fs::*;
use process::*;

/// handle syscall exception with `syscall_id` and other arguments
pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    match syscall_id {
        SYSCALL_WRITE => {
            unsafe {
                called_times::WRITE += 1;
            }
            sys_write(args[0], args[1] as *const u8, args[2])
        }
        SYSCALL_EXIT => {
            unsafe {
                called_times::EXIT += 1;
            }
            sys_exit(args[0] as i32)
        }
        SYSCALL_YIELD => {
            unsafe {
                called_times::YIELD += 1;
            }
            sys_yield()
        }
        SYSCALL_GET_TIME => {
            unsafe {
                called_times::GET_TIME += 1;
            }
            sys_get_time(args[0] as *mut TimeVal, args[1])
        }
        SYSCALL_TRACE => {
            unsafe {
                called_times::TRACE += 1;
            }
            sys_trace(args[0], args[1], args[2])
        }
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}
