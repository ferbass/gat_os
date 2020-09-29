#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(gat_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use gat_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Meow! {}", "MEOOOOOOOOW!");
    println!("CATO!");

    gat_os::init();

    fn stack_overflow() {
        stack_overflow();
    }

    // uncomment the line bellow to simulate stack overflow
    // stack_overflow();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    gat_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
