pub mod cpu;
pub mod gdt;
pub mod interrupts;
pub mod memory;
pub mod tss;

use crate::arch;
use crate::println;

pub const ARCHITECTURE_INFO: arch::ArchitectureInfo = arch::ArchitectureInfo { name: "x86_64" };

pub fn init() {
  println!(" >> Initializing GDT...");
  gdt::init();

  println!(" >> Initializing IDT...");
  interrupts::init_idt();

  println!(" >> Initializing Interrupts...");
  unsafe { interrupts::PICS.lock().initialize() };

  println!(" >> Enable Interrupts...");
  x86_64::instructions::interrupts::enable();
}
