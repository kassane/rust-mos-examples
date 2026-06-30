// Copyright (c) 2026 Matheus C. França
// SPDX-License-Identifier: Apache-2.0
//! C64 CRC8 benchmark.

#![no_std]
#![no_main]

extern crate demo_c64 as _;

unsafe extern "C" {
    fn printf(fmt: *const u8, ...) -> i32;
}

const KERNAL: *const u8 = 0xE000 as *const u8;
const KERNAL_LEN: usize = 0x2000;
const EXPECTED: u8 = 0xa2;

fn crc8(data: *const u8, len: usize) -> u8 {
    let mut crc: u8 = 0;
    for i in 0..len {
        crc ^= unsafe { *data.add(i) };
        for _ in 0..8 {
            crc = if crc & 0x80 != 0 {
                (crc << 1) ^ 0x1D
            } else {
                crc << 1
            };
        }
    }
    crc
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    unsafe {
        printf(b"crc8.zig\n\0".as_ptr());
        printf(b"Calculates the CRC8 of the C64 Kernal\n\0".as_ptr());
    }
    let result = crc8(KERNAL, KERNAL_LEN);
    unsafe {
        printf(b"CRC8=%02X\0".as_ptr(), result as u32);
        if result == EXPECTED {
            printf(b" [OK]\n\0".as_ptr());
        } else {
            printf(b" [FAIL] - expected %02X\n\0".as_ptr(), EXPECTED as u32);
        }
    }
    loop {}
}
