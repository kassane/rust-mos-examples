// Copyright (c) 2026 Matheus C. França
// SPDX-License-Identifier: Apache-2.0
//! NES UNROM color cycle: animated palette demo.

#![no_std]
#![no_main]

extern crate demo_nes as _;

unsafe extern "C" {
    fn ppu_off();
    fn set_prg_bank(bank: u8);
    fn pal_bg(data: *const u8);
    fn ppu_on_bg();
    fn ppu_wait_nmi();
    fn pal_col(index: u8, color: u8);
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    unsafe {
        ppu_off();
        set_prg_bank(0);
    }
    let pal: [u8; 16] = [0x0f; 16];
    unsafe {
        pal_bg(&pal as *const u8);
        ppu_on_bg();
    }
    let mut color: u8 = 0;
    loop {
        for _ in 0..30 {
            unsafe { ppu_wait_nmi() }
        }
        color = (color.wrapping_add(1)) & 0x3f;
        unsafe { pal_col(0, color) }
    }
}
