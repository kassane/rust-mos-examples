// Copyright (c) 2026 Matheus C. França
// SPDX-License-Identifier: Apache-2.0
//! MEGA65 hello: writes greeting to screen RAM and sets border colour.

#![no_std]
#![no_main]

extern crate demo_mega65 as _;

use core::ptr;

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    // 80-column screen RAM at $0800; row 14 (0-indexed) = 14 * 80
    let screen = 0x0800 as *mut u8;
    let msg = [
        b'H' - 0x40,
        b'E' - 0x40,
        b'L' - 0x40,
        b'L' - 0x40,
        b'O' - 0x40,
        b',' - 0x00,
        0x20,
        b'R' - 0x40,
        b'U' - 0x40,
        b'S' - 0x40,
        b'T' - 0x40,
        b'!' - 0x00,
        0x20,
        b'W' - 0x40,
        b'E' - 0x40,
        b'L' - 0x40,
        b'C' - 0x40,
        b'O' - 0x40,
        b'M' - 0x40,
        b'E' - 0x40,
        0x20,
        b'T' - 0x40,
        b'O' - 0x40,
        0x20,
        b'M' - 0x40,
        b'E' - 0x40,
        b'G' - 0x40,
        b'A' - 0x40,
        b'6' - 0x00,
        b'5' - 0x00,
        b'.' - 0x00,
    ];
    for (i, &c) in msg.iter().enumerate() {
        unsafe { ptr::write_volatile(screen.add(14 * 80 + i), c) }
    }
    unsafe { ptr::write_volatile(0xD020 as *mut u8, 5) } // green border
    loop {}
}
