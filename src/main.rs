#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate rlibc;
use core::panic::PanicInfo;
use rust_kernel::println;
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
  use rust_kernel::memory;
  use rust_kernel::memory::BootInfoFrameAllocator;
  use x86_64::{structures::paging::Page, VirtAddr};

  println!("Hello World{}", "!");

  rust_kernel::init();

  let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

  #[cfg(test)]
  test_main();

  println!("It did not crash!");
  rust_kernel::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  println!("{}", info);
  rust_kernel::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  rust_kernel::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
  assert_eq!(1, 1);
}
