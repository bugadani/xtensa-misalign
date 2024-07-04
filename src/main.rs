#![no_std]
#![no_main]

use defmt_rtt as _;
use esp_backtrace as _;

#[repr(align(64))]
#[derive(Default)]
struct Aligned {
    _a: [u32; 16],
}

#[esp_hal::entry]
fn main() -> ! {
    call();
    loop {}
}

#[inline(never)]
#[cold]
fn call() {
    let aligned = Aligned::default();

    let ptr = core::ptr::addr_of!(aligned);
    defmt::info!(
        "0x{:x} (aligned to 0x{:x}) {:b}",
        ptr as usize,
        core::mem::align_of_val(&aligned),
        ptr.is_aligned()
    );

    let ptr = (ptr as usize) as *const Aligned;
    defmt::info!(
        "0x{:x} (aligned to 0x{:x}) {:b}",
        ptr as usize,
        core::mem::align_of_val(&aligned),
        ptr.is_aligned()
    );
}
