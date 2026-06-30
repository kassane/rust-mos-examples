#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]

extern crate demo_c64 as _;

const VIC_BORDER: *mut u8 = 0xD020 as *mut _;
const VIC_BG: *mut u8 = 0xD021 as *mut _;

fn putchar(c: u8) {
    unsafe { core::arch::asm!("jsr $ffd2", in("a") c, options(nostack)) }
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    unsafe { *VIC_BG = 0 }
    for &c in b"Hello Rust!\r" {
        putchar(c);
    }
    loop {
        unsafe { *VIC_BORDER = (*VIC_BORDER).wrapping_add(1) }
    }
}
