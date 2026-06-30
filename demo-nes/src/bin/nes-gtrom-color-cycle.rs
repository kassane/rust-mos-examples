// Copyright (c) 2026 Matheus C. França
// SPDX-License-Identifier: Apache-2.0
//! NES GTROM color cycle: animated palette demo.

#![no_std]
#![no_main]

extern crate demo_nes as _;

const PAL: [u8; 16] = [0x0f; 16];

unsafe extern "C" {
    fn ppu_off();
    fn set_prg_bank(bank: i8) -> i8;
    fn set_chr_bank(bank: i8);
    fn set_nt_bank(bank: i8);
    fn set_mapper_green_led(on: bool) -> i8;
    fn set_mapper_red_led(on: bool) -> i8;
    fn pal_bg(data: *const u8);
    fn ppu_on_bg();
    fn ppu_wait_nmi();
    fn pal_col(index: i8, color: i8);
    fn pad_poll(pad: i8) -> i8;
    fn get_pad_new(pad: i8) -> i8;
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    unsafe {
        ppu_off();
        set_prg_bank(0);
        set_chr_bank(0);
        set_nt_bank(0);
        set_mapper_green_led(true);
        set_mapper_red_led(false);
        pal_bg(&PAL as *const u8);
        ppu_on_bg();
    }

    let mut color: u8 = 0;
    let mut green_on = true;
    let mut red_on = false;

    loop {
        for _ in 0..30 {
            unsafe { ppu_wait_nmi() }
        }
        color = (color + 1) & 0x3f;
        unsafe { pal_col(0, color as i8) }

        unsafe { pad_poll(0) };
        let pad_new = unsafe { get_pad_new(0) as u8 };
        if pad_new & 0x10 != 0 {
            green_on = !green_on;
            unsafe {
                set_mapper_green_led(green_on);
            }
        }
        if pad_new & 0x20 != 0 {
            red_on = !red_on;
            unsafe {
                set_mapper_red_led(red_on);
            }
        }
    }
}
