// src/main.rs
#![no_std]
#![no_main]

mod lang_items;
mod sbi;
mod lib;

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));

unsafe extern "C" {
    fn sbss();
    fn ebss();
}

fn clear_bss() {
    unsafe {
        core::ptr::write_bytes(
            sbss as *mut u8,
            0,
            (ebss as usize) - (sbss as usize),
        );
    }
}


#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    clear_bss();
    println!("Hello rCore!");
    panic!("Shutdown machine!");
    crate::sbi::shutdown(false);
}
