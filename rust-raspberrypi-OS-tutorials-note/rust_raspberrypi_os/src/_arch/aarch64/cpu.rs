// global_asm!(include_str!("cpu.S"));

use crate::{bsp, cpu};
use cortex_a::{asm, regs::*};


#[naked]
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    use crate::runtime_init;

    if bsp::cpu::BOOT_CORE_ID == cpu::smp::core_id() {
        SP.set(bsp::memory::BOOT_CORE_STACK_START as u64);
        runtime_init::runtime_init();
    } else {
        wait_forever();
    }
    
}


#[inline(always)]
pub fn wait_forever() -> ! {
    loop {
        asm::wfe()
    }
}