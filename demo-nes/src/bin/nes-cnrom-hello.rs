// Copyright (c) 2026 Matheus C. França
// SPDX-License-Identifier: Apache-2.0
//! NES CNROM hello: displays static screen via NESlib.

#![no_std]
#![no_main]

extern crate demo_nes as _;

#[used]
#[unsafe(link_section = ".chr_rom")]
static CHR_ROM: [u8; 8192] = *include_bytes!("../../chr/Alpha.chr");

unsafe extern "C" {
    fn ppu_off();
    fn set_chr_bank(bank: u8);
    fn pal_bright(bright: u8);
    fn pal_bg(data: *const u8);
    fn vram_adr(adr: u16);
    fn vram_put(c: u8);
    fn ppu_on_all();
    fn ppu_wait_nmi();
}

fn ntadr_a(col: u8, row: u8) -> u16 {
    0x2000 | ((row as u16) << 5) | (col as u16)
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    unsafe {
        ppu_off();
        set_chr_bank(0);
    }
    let bg_pal: [u8; 16] = [0x11, 0x00, 0x10, 0x30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    unsafe {
        pal_bright(4);
        pal_bg(&bg_pal as *const u8);
        vram_adr(ntadr_a(4, 14));
    }
    for &c in b"CNROM Hello!" {
        unsafe { vram_put(c) }
    }
    unsafe { ppu_on_all() }
    loop {
        unsafe { ppu_wait_nmi() }
    }
}
