// Copyright (c) 2026 Matheus C. França
// SPDX-License-Identifier: Apache-2.0
//! MEGA65 common: panic handler.

#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
