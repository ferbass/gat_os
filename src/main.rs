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

    // uncommment the lines bellow to cause a page fault
    // cause_page_fault();

    // uncoment the block to see the read operation working
    // cause_page_fault_on_write();

    use x86_64::registers::control::Cr3;

    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table at: {:?}", level_4_page_table.start_address());

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    gat_os::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    gat_os::hlt_loop();
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

fn cause_page_fault() {
    let ptr = 0xdeadbeaf as *mut u32;
    unsafe { *ptr = 42; }
}

fn cause_page_fault_on_write() {
    let ptr = 0x20398c as *mut u32;
    // read from a code page
    unsafe { let x = *ptr; }
    println!("=^.^= Read worked");

    // write to a code page
    unsafe { *ptr = 42; }
    println!("Write worked");
}
