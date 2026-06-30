// Copyright (c) 2026 Matheus C. França
// SPDX-License-Identifier: Apache-2.0
//! NES CNROM sprites: sprite animation demo via NESlib.

#![no_std]
#![no_main]

extern crate demo_nes as _;

#[used]
#[unsafe(link_section = ".chr_rom")]
static CHR_ROM: [u8; 8192] = *include_bytes!("../../chr/Alpha2.chr");

unsafe extern "C" {
    fn ppu_off();
    fn set_chr_bank(bank: u8);
    fn pal_bg(data: *const u8);
    fn pal_spr(data: *const u8);
    fn bank_spr(n: u8);
    fn vram_adr(adr: u16);
    fn vram_write(src: *const u8, size: u16);
    fn ppu_on_all();
    fn ppu_wait_nmi();
    fn oam_clear();
    fn oam_spr(x: u8, y: u8, chrnum: u8, attr: u8);
    fn oam_meta_spr(x: u8, y: u8, data: *const u8);
}

fn ntadr_a(col: u8, row: u8) -> u16 {
    0x2000 | ((row as u16) << 5) | (col as u16)
}

const PALETTE_BG: [u8; 16] = [
    0x0f, 0x00, 0x10, 0x30, 0x0f, 0x00, 0x10, 0x30, 0x0f, 0x00, 0x10, 0x30, 0x0f, 0x00, 0x10, 0x30,
];
const PALETTE_SP: [u8; 16] = [
    0x0f, 0x0f, 0x0f, 0x28, 0x0f, 0x0f, 0x0f, 0x28, 0x0f, 0x0f, 0x0f, 0x28, 0x0f, 0x0f, 0x0f, 0x28,
];

const METASPRITE: [u8; 17] = [
    0, 0, 0x01, 0x00, 0, 8, 0x11, 0x00, 8, 0, 0x01, 0x40, 8, 8, 0x11, 0x40, 128,
];

const METASPRITE2: [u8; 29] = [
    8, 0, 0x03, 0x00, 0, 8, 0x12, 0x00, 8, 8, 0x13, 0x00, 16, 8, 0x12, 0x40, 0, 16, 0x22, 0x00, 8,
    16, 0x23, 0x00, 16, 16, 0x22, 0x40, 128,
];

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    unsafe {
        ppu_off();
        set_chr_bank(0);
        pal_bg(&PALETTE_BG as *const u8);
        pal_spr(&PALETTE_SP as *const u8);
        bank_spr(1);
        vram_adr(ntadr_a(5, 14));
        vram_write(b"CNROM Sprites".as_ptr(), 13);
        ppu_on_all();
    }
    let mut y_pos: u8 = 0x40;
    let x_pos: u8 = 0x88;
    let x_pos2: u8 = 0xa0;
    let x_pos3: u8 = 0xc0;
    loop {
        unsafe { ppu_wait_nmi() }
        unsafe {
            oam_clear();
            oam_spr(x_pos, y_pos, 0, 0);
            oam_meta_spr(x_pos2, y_pos, &METASPRITE as *const u8);
            oam_meta_spr(x_pos3, y_pos, &METASPRITE2 as *const u8);
        }
        y_pos = y_pos.wrapping_add(1);
    }
}
