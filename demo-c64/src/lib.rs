// Copyright (c) 2026 Matheus C. França
// SPDX-License-Identifier: Apache-2.0
//! C64 common: panic handler and KERNAL CHROUT helper.

#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
