#![no_std]
#![no_main]
#![allow(dead_code)]

use core::panic::PanicInfo;
extern crate rlibc;
mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
  println!("Hello World{}", "!");

  loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  println!("{}", info);

  loop {}
}
