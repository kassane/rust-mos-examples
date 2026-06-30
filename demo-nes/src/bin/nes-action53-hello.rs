// Copyright (c) 2026 Matheus C. França
// SPDX-License-Identifier: Apache-2.0
//! NES Action53 hello: displays static screen via NESlib.

#![no_std]
#![no_main]

extern crate demo_nes as _;

const PALETTE: [u8; 32] = [
    0x1A, 0x00, 0x10, 0x20, 0x1A, 0x00, 0x10, 0x20, 0x1A, 0x00, 0x10, 0x20, 0x1A, 0x00, 0x10, 0x20,
    0x1A, 0x00, 0x10, 0x20, 0x1A, 0x00, 0x10, 0x20, 0x1A, 0x00, 0x10, 0x20, 0x1A, 0x00, 0x10, 0x20,
];

unsafe extern "C" {
    fn ppu_off();
    fn pal_bright(bright: i8);
    fn pal_all(data: *const u8);
    fn ppu_on_all();
    fn ppu_wait_nmi();
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    unsafe {
        ppu_off();
        pal_bright(4);
        pal_all(&PALETTE as *const u8);
        ppu_on_all();
    }
    loop {
        unsafe { ppu_wait_nmi() }
    }
}
