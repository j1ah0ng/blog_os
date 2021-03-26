#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use blog_os::*;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_div_by_zero();
    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

#[allow(unconditional_panic)]
fn test_div_by_zero() {
    serial_print!("should_panic::test_div_by_zero...\t");
    let _invalid_op = 1 / 0;
}
