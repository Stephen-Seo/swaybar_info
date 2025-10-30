use std::{ffi::c_int, mem::MaybeUninit, ptr::null_mut};

pub fn handle_signal(signal: c_int, handler: extern "C" fn(c_int)) -> c_int {
    let handler_ptr_usize: usize = handler as *const fn(c_int) as usize;

    let mut sigaction_struct: MaybeUninit<libc::sigaction> = MaybeUninit::zeroed();

    unsafe {
        let sigaction_ptr: *mut libc::sigaction = sigaction_struct.as_mut_ptr();
        (*sigaction_ptr).sa_sigaction = handler_ptr_usize;
        libc::sigemptyset(&mut (*sigaction_ptr).sa_mask as *mut libc::sigset_t);
        (*sigaction_ptr).sa_flags = 0;
        libc::sigaction(signal, sigaction_ptr, null_mut())
    }
}
