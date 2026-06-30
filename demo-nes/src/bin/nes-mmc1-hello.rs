// Copyright (c) 2026 Matheus C. França
// SPDX-License-Identifier: Apache-2.0
//! NES MMC1 hello: displays static screen via NESlib.

#![no_std]
#![no_main]

extern crate demo_nes as _;

unsafe extern "C" {
    fn ppu_off();
    fn set_prg_bank(bank: u8);
    fn set_mirroring(mode: u8);
    fn pal_bright(bright: u8);
    fn pal_bg(data: *const u8);
    fn ppu_on_all();
    fn ppu_wait_nmi();
}

const MIRROR_VERTICAL: u8 = 0;

const BG_PAL: [u8; 16] = [
    0x1A, 0x1A, 0x2A, 0x3A, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    unsafe {
        ppu_off();
        set_prg_bank(0);
        set_mirroring(MIRROR_VERTICAL);
        pal_bright(4);
        pal_bg(&BG_PAL as *const u8);
        ppu_on_all();
    }
    loop {
        unsafe {
            ppu_wait_nmi();
        }
    }
}
