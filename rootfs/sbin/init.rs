#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::ffi::CStr;

extern "C" {
    fn syscall(num: usize, ...) -> isize;
}

const SYS_MOUNT: usize = 165;
const SYS_EXECVE: usize = 59;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        let proc_fs = CStr::from_bytes_with_nul_unchecked(b"proc\0");
        let proc_mount = CStr::from_bytes_with_nul_unchecked(b"/proc\0");
        let proc_type = CStr::from_bytes_with_nul_unchecked(b"proc\0");

        syscall(SYS_MOUNT, proc_fs.as_ptr(), proc_mount.as_ptr(), proc_type.as_ptr(), 0, 0);

        let sys_fs = CStr::from_bytes_with_nul_unchecked(b"sysfs\0");
        let sys_mount = CStr::from_bytes_with_nul_unchecked(b"/sys\0");

        syscall(SYS_MOUNT, sys_fs.as_ptr(), sys_mount.as_ptr(), sys_fs.as_ptr(), 0, 0);

        let shell = CStr::from_bytes_with_nul_unchecked(b"/bin/sh\0");
        syscall(SYS_EXECVE, shell.as_ptr(), 0, 0);
    }

    loop {} // Prevents exiting
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
