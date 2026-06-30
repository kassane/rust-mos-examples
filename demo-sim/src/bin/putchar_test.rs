#![no_std]
#![no_main]

extern crate demo_sim as _;

use core::ptr;

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    let msg = b"Hello!\n";
    for &c in msg {
        unsafe { ptr::write_volatile(0xFFF9 as *mut u8, c) }
    }
    unsafe { ptr::write_volatile(0xFFF8 as *mut u8, 0) }
    loop {}
}
