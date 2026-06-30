// Copyright (c) 2026 Matheus C. França
// SPDX-License-Identifier: Apache-2.0
//! NES NROM hello: writes "Hello Rust!" to the nametable via NESlib.

#![no_std]
#![no_main]

extern crate demo_nes as _;

#[used]
#[unsafe(link_section = ".chr_rom")]
static CHR_ROM: [u8; 8192] = *include_bytes!("../../chr/Alpha.chr");

const BG_PALETTE: [u8; 16] = [
    0x0f, 0x00, 0x10, 0x30, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

unsafe extern "C" {
    fn ppu_off();
    fn ppu_on_all();
    fn pal_bg(palette: *const u8);
    fn vram_adr(addr: u16);
    fn vram_put(c: u8);
}

const fn ntadr_a(col: u16, row: u16) -> u16 {
    0x2000 + 32 * row + col
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    unsafe {
        ppu_off();
        pal_bg(BG_PALETTE.as_ptr());
        vram_adr(ntadr_a(10, 14));
        let msg = b"HELLO RUST!";
        for &c in msg {
            vram_put(c);
        }
        ppu_on_all();
    }
    loop {}
}
