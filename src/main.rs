#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate rlibc;
extern crate alloc;

use core::panic::PanicInfo;
use rust_kernel::println;
use rust_kernel::task::{Task, executor::Executor};
use rust_kernel::task::keyboard;
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
  use rust_kernel::allocator;
  use rust_kernel::memory;
  use rust_kernel::memory::BootInfoFrameAllocator;
  use x86_64::VirtAddr;

  println!("Hello World{}", "!");

  rust_kernel::init();

  let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
  let mut mapper = unsafe { memory::init(phys_mem_offset) };
  let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

  allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

  #[cfg(test)]
  test_main();

  let mut executor = Executor::new();
  executor.spawn(Task::new(keyboard::print_keypresses()));
  executor.run();
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
