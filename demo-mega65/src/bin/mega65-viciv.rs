// Copyright (c) 2026 Matheus C. França
// SPDX-License-Identifier: Apache-2.0
//! MEGA65 VIC-IV: full-colour screen demo with cycling border.

#![no_std]
#![no_main]

extern crate demo_mega65 as _;

use core::ptr;

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    unsafe { ptr::write_volatile(0xD021 as *mut u8, 0x1E) } // COLOR_BUBBLEGUM
    loop {
        let border = unsafe { ptr::read_volatile(0xD020 as *const u8) };
        unsafe { ptr::write_volatile(0xD020 as *mut u8, border.wrapping_add(1)) }
    }
}
