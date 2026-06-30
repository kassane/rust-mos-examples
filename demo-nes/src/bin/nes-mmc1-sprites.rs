// Copyright (c) 2026 Matheus C. França
// SPDX-License-Identifier: Apache-2.0
//! NES MMC1 sprites: sprite animation demo via NESlib.

#![no_std]
#![no_main]

extern crate demo_nes as _;

unsafe extern "C" {
    fn ppu_off();
    fn set_mmc1_ctrl(value: u8);
    fn set_prg_bank(bank: u8);
    fn vram_adr(addr: u16);
    fn vram_write(src: *const u8, len: u16);
    fn pal_bright(bright: u8);
    fn pal_bg(data: *const u8);
    fn pal_spr(data: *const u8);
    fn bank_spr(n: u8);
    fn ppu_on_all();
    fn ppu_wait_nmi();
    fn oam_clear();
    fn oam_spr(x: u8, y: u8, chrnum: u8, attr: u8);
    fn oam_meta_spr(x: u8, y: u8, data: *const u8);
}

const OAM_FLIP_H: u8 = 0x40;

const BG_PAL: [u8; 16] = [
    0x0f, 0x00, 0x10, 0x30, 0x0f, 0x00, 0x10, 0x30, 0x0f, 0x00, 0x10, 0x30, 0x0f, 0x00, 0x10, 0x30,
];

const SPR_PAL: [u8; 16] = [
    0x0f, 0x0f, 0x0f, 0x28, 0x0f, 0x0f, 0x0f, 0x28, 0x0f, 0x0f, 0x0f, 0x28, 0x0f, 0x0f, 0x0f, 0x28,
];

const CHR_DATA: &[u8; 8192] = include_bytes!("../../chr/Alpha2.chr");

const METASPRITE: [u8; 17] = [
    0, 0, 0x01, 0x00, 0, 8, 0x11, 0x00, 8, 0, 0x01, OAM_FLIP_H, 8, 8, 0x11, OAM_FLIP_H, 128,
];

const METASPRITE2: [u8; 29] = [
    8, 0, 0x03, 0x00, 0, 8, 0x12, 0x00, 8, 8, 0x13, 0x00, 16, 8, 0x12, OAM_FLIP_H, 0, 16, 0x22,
    0x00, 8, 16, 0x23, 0x00, 16, 16, 0x22, OAM_FLIP_H, 128,
];

const fn ntadr_a(x: u8, y: u8) -> u16 {
    0x2000 | ((y as u16) << 5) | (x as u16)
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    unsafe {
        ppu_off();
        set_mmc1_ctrl(0x0E);
        set_prg_bank(0);
        vram_adr(0x0000);
        vram_write(CHR_DATA.as_ptr(), CHR_DATA.len() as u16);
        pal_bright(4);
        pal_bg(&BG_PAL as *const u8);
        pal_spr(&SPR_PAL as *const u8);
        bank_spr(1);
        vram_adr(ntadr_a(5, 14));
        vram_write(b"MMC1 Sprites\0".as_ptr(), 12);
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
