#![no_std]

use core::panic::PanicInfo;
use core::ptr;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[repr(C)]
pub struct SimRegs {
    pub clock: [u8; 4],  // $FFF0-$FFF3
    _pad1: u8,           // $FFF4
    pub getchar: u8,     // $FFF5
    pub eof: u8,         // $FFF6
    pub abort: u8,       // $FFF7
    pub exit: u8,        // $FFF8
    pub putchar: u8,     // $FFF9
}

pub const SIM_REGS: *mut SimRegs = 0xFFF0 as *mut SimRegs;

pub unsafe fn reg_write(reg: *mut u8, val: u8) {
    unsafe { ptr::write_volatile(reg, val) }
}

pub unsafe fn reg_read(reg: *const u8) -> u8 {
    unsafe { ptr::read_volatile(reg) }
}

#[unsafe(no_mangle)]
pub extern "C" fn abort() -> ! {
    unsafe { reg_write(&mut (*SIM_REGS).exit, 1) }
    loop {}
}
