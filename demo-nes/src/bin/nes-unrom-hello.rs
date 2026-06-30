// Copyright (c) 2026 Matheus C. França
// SPDX-License-Identifier: Apache-2.0
//! NES UNROM hello: displays static screen via NESlib.

#![no_std]
#![no_main]

extern crate demo_nes as _;

unsafe extern "C" {
    fn ppu_off();
    fn set_prg_bank(bank: u8);
    fn pal_bright(bright: u8);
    fn pal_bg(data: *const u8);
    fn ppu_on_all();
    fn ppu_wait_nmi();
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    unsafe {
        ppu_off();
        set_prg_bank(0);
    }
    let bg_pal: [u8; 16] = [0x16, 0x16, 0x27, 0x30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    unsafe {
        pal_bright(4);
        pal_bg(&bg_pal as *const u8);
        ppu_on_all();
    }
    loop {
        unsafe { ppu_wait_nmi() }
    }
}
