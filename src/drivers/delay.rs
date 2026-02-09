#[inline(always)]
pub fn nsdelay(count: u32) {
    for _ in 0..count {
        unsafe { core::arch::asm!("nop") };
    }
}
