#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(gat_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use gat_os::println;
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use gat_os::memory;
    use gat_os::memory::BootInfoFrameAllocator;
    use x86_64::{structures::paging::Page, VirtAddr};

    println!("Meow! {}", "MEOOOOOOOOW!");
    println!("CATO!");

    gat_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = memory::EmptyFrameAllocator;

    // map unused page
    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();

    unsafe { page_ptr.offset(400).write_volatile(0x_f06f_f074_f061_f063)};

    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    //fn stack_overflow() {
    //stack_overflow();
    //}

    // uncomment the line bellow to simulate stack overflow
    // stack_overflow();

    // uncommment the lines bellow to cause a page fault
    // cause_page_fault();

    // uncoment the block to see the read operation working
    // cause_page_fault_on_write();

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
