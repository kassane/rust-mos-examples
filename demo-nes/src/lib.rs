// Copyright (c) 2026 Matheus C. França
// SPDX-License-Identifier: Apache-2.0
//! NES common: PPU/APU registers and panic handler.

#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[repr(C)]
pub struct Ppu {
    pub ctrl: u8,      // $2000
    pub mask: u8,      // $2001
    pub status: u8,    // $2002
    pub oam_addr: u8,  // $2003
    pub oam_data: u8,  // $2004
    pub scroll: u8,    // $2005
    pub vram_addr: u8, // $2006
    pub vram_data: u8, // $2007
}

pub const PPU: *mut Ppu = 0x2000 as *mut Ppu;

#[repr(C)]
pub struct Apu {
    pub pulse1_ctrl: u8,    // $4000
    pub pulse1_sweep: u8,   // $4001
    pub pulse1_timer_l: u8, // $4002
    pub pulse1_timer_h: u8, // $4003
    pub pulse2_ctrl: u8,    // $4004
    pub pulse2_sweep: u8,   // $4005
    pub pulse2_timer_l: u8, // $4006
    pub pulse2_timer_h: u8, // $4007
    pub tri_ctrl: u8,       // $4008
    _tri_unused: u8,        // $4009
    pub tri_timer_l: u8,    // $400A
    pub tri_timer_h: u8,    // $400B
    pub noise_ctrl: u8,     // $400C
    _noise_unused: u8,      // $400D
    pub noise_period: u8,   // $400E
    pub noise_len: u8,      // $400F
    pub dmc_ctrl: u8,       // $4010
    pub dmc_load: u8,       // $4011
    pub dmc_addr: u8,       // $4012
    pub dmc_len: u8,        // $4013
    pub oam_dma: u8,        // $4014
    pub apu_status: u8,     // $4015
    _pad1: [u8; 2],         // $4016-$4017
}
