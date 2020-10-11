use x86_64::structures::gdt::{SegmentSelector, GlobalDescriptorTable, Descriptor};
use lazy_static::lazy_static;
use crate::arch::x86_64::tss::TSS;

lazy_static! {
  static ref GDT: (GlobalDescriptorTable, Selectors) = {
    let mut gdt = GlobalDescriptorTable::new();
    let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
    let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
    (
      gdt,
      Selectors {
        code_selector,
        tss_selector,
      },
    )
  };
}

struct Selectors {
  code_selector: SegmentSelector,
  tss_selector: SegmentSelector,
}

pub fn init() {
  use x86_64::instructions::segmentation::set_cs;
  use x86_64::instructions::tables::load_tss;
  GDT.0.load();
  unsafe {
    set_cs(GDT.1.code_selector);
    load_tss(GDT.1.tss_selector);
  }
}
