//! Rust runtime initialization code.

use crate::memory;
use core::ops::Range;

unsafe fn bss_range() -> Range<*mut usize> {
    extern "C" {
        static mut __bss_start: usize;
        static mut __bss_end: usize;
    }

    Range {
        start: &mut __bss_start,
        end: &mut __bss_start,
    }
}

#[inline(always)]
unsafe fn zero_bss() {
    memory::zero_volatile(bss_range());
}



#[no_mangle]
pub unsafe extern "C" fn runtime_init() -> ! {
    zero_bss();

    crate::kernel_init()
}