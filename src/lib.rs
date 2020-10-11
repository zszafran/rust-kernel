#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(const_mut_refs)]
#![feature(const_in_array_repeat_expressions)]
#![feature(wake_trait)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate rlibc;
extern crate alloc;

pub mod allocator;
pub mod arch;
pub mod devices;
pub mod task;
pub mod test;

pub fn init() {
  println!(" >> Architecture: {}", arch::ARCHITECTURE_INFO.name);

  arch::init();
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
  panic!("allocation error: {:?}", layout)
}
