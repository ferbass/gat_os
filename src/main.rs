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
    use gat_os::memory::active_level_4_table;
    use x86_64::VirtAddr;

    println!("Meow! {}", "MEOOOOOOOOW!");
    println!("CATO!");

    gat_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let l4_table = unsafe { active_level_4_table(phys_mem_offset) };

    for (i, entry) in l4_table.iter().enumerate() {
        use x86_64::structures::paging::PageTable;

        if !entry.is_unused() {
            println!("L4 Entry {}: {:?}", i, entry);

            // get the physical address from the entry and convert it
            let phys = entry.frame().unwrap().start_address();
            let virt = phys.as_u64() + boot_info.physical_memory_offset;
            let ptr = VirtAddr::new(virt).as_mut_ptr();
            let l3_table: &PageTable = unsafe { &*ptr };

            // print non-empty entries of the level 3 table
            for (i, entry) in l3_table.iter().enumerate() {
                if !entry.is_unused() {
                    println!("  L3 Entry {}: {:?}", i, entry);
                }
            }
        }
    }
    //fn stack_overflow() {
    //stack_overflow();
    //}

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
