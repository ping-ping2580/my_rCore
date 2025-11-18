// src/main.rs
#![no_std]
#![no_main]
// ====================  println! 宏开始 ====================
use core::fmt::{self, Write};

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for &ch in s.as_bytes() {
            crate::sbi::console_putchar(ch as usize);
        }
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
}
// ==================== println! 宏结束 ====================


mod lang_items;
mod sbi;

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
            (ebss as *const () as usize) - (sbss as *const () as usize),
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
