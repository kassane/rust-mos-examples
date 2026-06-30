// Copyright (c) 2026 Matheus C. França
// SPDX-License-Identifier: Apache-2.0
//! NES MMC3 pads: gamepad reading demo via NESlib.

#![no_std]
#![no_main]

extern crate demo_nes as _;

unsafe extern "C" {
    fn ppu_off();
    fn set_prg_8000(bank: u8);
    fn set_mirroring(mode: u8);
    fn disable_irq();
    fn vram_adr(addr: u16);
    fn vram_write(src: *const u8, len: u16);
    fn pal_bright(bright: u8);
    fn pal_bg(data: *const u8);
    fn pal_spr(data: *const u8);
    fn bank_spr(n: u8);
    fn ppu_on_all();
    fn ppu_wait_nmi();
    fn pad_poll(pad: u8) -> u8;
    fn check_collision(a: *const u8, b: *const u8) -> u8;
    fn pal_col(index: u8, color: u8);
    fn oam_clear();
    fn oam_meta_spr(x: u8, y: u8, data: *const u8);
}

const MIRROR_VERTICAL: u8 = 0;
const OAM_FLIP_H: u8 = 0x40;

const PAD_LEFT: u8 = 2;
const PAD_RIGHT: u8 = 1;
const PAD_UP: u8 = 8;
const PAD_DOWN: u8 = 4;

const BG_PAL: [u8; 16] = [
    0x00, 0x00, 0x10, 0x30, 0x00, 0x00, 0x10, 0x30, 0x00, 0x00, 0x10, 0x30, 0x00, 0x00, 0x10, 0x30,
];

const SPR_PAL: [u8; 16] = [
    0x0f, 0x0f, 0x0f, 0x28, 0x0f, 0x0f, 0x0f, 0x12, 0x0f, 0x0f, 0x0f, 0x28, 0x0f, 0x0f, 0x0f, 0x28,
];

const CHR_DATA: &[u8; 8192] = include_bytes!("../../chr/Alpha3.chr");

const YELLOW_SPR: [u8; 17] = [
    0, 0, 0x00, 0, 0, 8, 0x10, 0, 8, 0, 0x00, OAM_FLIP_H, 8, 8, 0x10, OAM_FLIP_H, 128,
];

const BLUE_SPR: [u8; 17] = [
    0,
    0,
    0x00,
    1,
    0,
    8,
    0x10,
    1,
    8,
    0,
    0x00,
    1 | OAM_FLIP_H,
    8,
    8,
    0x10,
    1 | OAM_FLIP_H,
    128,
];

#[repr(C)]
struct BoxGuy {
    x: u8,
    y: u8,
    w: u8,
    h: u8,
}

const fn ntadr_a(x: u8, y: u8) -> u16 {
    0x2000 | ((y as u16) << 5) | (x as u16)
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    unsafe {
        ppu_off();
        set_prg_8000(0);
        set_mirroring(MIRROR_VERTICAL);
        disable_irq();
        vram_adr(0x0000);
        vram_write(CHR_DATA.as_ptr(), CHR_DATA.len() as u16);
        pal_bright(4);
        pal_bg(&BG_PAL as *const u8);
        pal_spr(&SPR_PAL as *const u8);
        bank_spr(1);
        vram_adr(ntadr_a(4, 14));
        vram_write(b"MMC3 Collisions\0".as_ptr(), 15);
        ppu_on_all();
    }

    let mut box1 = BoxGuy {
        x: 20,
        y: 20,
        w: 15,
        h: 15,
    };
    let mut box2 = BoxGuy {
        x: 70,
        y: 20,
        w: 15,
        h: 15,
    };

    loop {
        unsafe { ppu_wait_nmi() }

        let pad1 = unsafe { pad_poll(0) };
        let pad2 = unsafe { pad_poll(1) };

        if pad1 & PAD_LEFT != 0 {
            box1.x = box1.x.wrapping_sub(1);
        }
        if pad1 & PAD_RIGHT != 0 {
            box1.x = box1.x.wrapping_add(1);
        }
        if pad1 & PAD_UP != 0 {
            box1.y = box1.y.wrapping_sub(1);
        }
        if pad1 & PAD_DOWN != 0 {
            box1.y = box1.y.wrapping_add(1);
        }

        if pad2 & PAD_LEFT != 0 {
            box2.x = box2.x.wrapping_sub(1);
        }
        if pad2 & PAD_RIGHT != 0 {
            box2.x = box2.x.wrapping_add(1);
        }
        if pad2 & PAD_UP != 0 {
            box2.y = box2.y.wrapping_sub(1);
        }
        if pad2 & PAD_DOWN != 0 {
            box2.y = box2.y.wrapping_add(1);
        }

        let hit = unsafe {
            check_collision(
                &box1 as *const BoxGuy as *const u8,
                &box2 as *const BoxGuy as *const u8,
            )
        };
        unsafe { pal_col(0, if hit != 0 { 0x30 } else { 0x00 }) }

        unsafe {
            oam_clear();
            oam_meta_spr(box1.x, box1.y, &YELLOW_SPR as *const u8);
            oam_meta_spr(box2.x, box2.y, &BLUE_SPR as *const u8);
        }
    }
}
