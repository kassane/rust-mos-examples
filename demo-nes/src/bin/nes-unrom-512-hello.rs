// Copyright (c) 2026 Matheus C. França
// SPDX-License-Identifier: Apache-2.0
//! NES UNROM-512 hello: displays static screen via NESlib.

#![no_std]
#![no_main]

extern crate demo_nes as _;

const BG_PAL: [u8; 16] = [
    0x1A, 0x1A, 0x27, 0x30, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

unsafe extern "C" {
    fn ppu_off();
    fn set_prg_bank(bank: i8) -> i8;
    fn set_chr_bank(bank: i8);
    fn pal_bright(bright: i8);
    fn pal_bg(data: *const u8);
    fn ppu_on_all();
    fn ppu_wait_nmi();
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    unsafe {
        ppu_off();
        set_prg_bank(0);
        set_chr_bank(0);
        pal_bright(4);
        pal_bg(&BG_PAL as *const u8);
        ppu_on_all();
    }
    loop {
        unsafe { ppu_wait_nmi() }
    }
}
