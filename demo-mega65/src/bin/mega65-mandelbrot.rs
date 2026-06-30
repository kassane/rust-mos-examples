// Copyright (c) 2026 Matheus C. França
// SPDX-License-Identifier: Apache-2.0
//! MEGA65 Mandelbrot FCM: Full Color Mode 320x200 escape-time fractal.
//! Per-pixel palette via VIC-IV CHR16+FCLRHI; 32x32 hardware math; Enhanced DMA; 40 MHz.

#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]

extern crate demo_mega65 as _;

use core::ptr;

type Fix16 = i16;
const FP_ONE: Fix16 = 256;

const MAX_ITER: u8 = 32;
const CELL_COLS: u8 = 40;
const CELL_ROWS: u8 = 25;
const TILE_PIXELS: u8 = 8;
const TILE_BYTES: u16 = 64;
const TILE_ROW_BYTES: u16 = 2560;
const NUM_CELLS: u16 = 1000;

const GFX_ADDR: u32 = 0x40000;
const TILE_BASE: u16 = 0x1000;

const VIC3_FAST_MASK: u8 = 0x40;
const VIC3_ATTR_MASK: u8 = 0x20;
const VIC3_H640_MASK: u8 = 0x80;
const VIC3_V400_MASK: u8 = 0x08;
const VIC3_PAL_MASK: u8 = 0x04;
const VIC4_CHR16_MASK: u8 = 0x01;
const VIC4_FCLRLO_MASK: u8 = 0x02;
const VIC4_FCLRHI_MASK: u8 = 0x04;
const VIC4_HOTREG_MASK: u8 = 0x80;

const ENABLE_F018B_OPT: u8 = 0x0b;
const SRC_ADDR_BITS_OPT: u8 = 0x80;
const DST_ADDR_BITS_OPT: u8 = 0x81;
const DST_SKIP_RATE_OPT: u8 = 0x85;
const DMA_COPY_CMD: u8 = 0x00;
const DMA_FILL_CMD: u8 = 0x03;

#[repr(C)]
struct DMAListF018B {
    command: u8,
    count: u16,
    source_addr: u16,
    source_bank: u8,
    dest_addr: u16,
    dest_bank: u8,
    command_msb: u8,
    modulo: u16,
}

#[repr(C)]
struct DmaJob {
    opt0: u8,
    opt1: u8,
    opt2: u8,
    opt3: u8,
    opt4: u8,
    opt5: u8,
    opt6: u8,
    end_option: u8,
    dmalist: DMAListF018B,
}

static mut TILE_ROW_BUF: [u8; TILE_ROW_BYTES as usize] = [0; TILE_ROW_BYTES as usize];

const SEGMENT_LEN: u8 = 8;
const MAX_INTENSITY: u8 = 15;

const fn nyb(n: u8) -> u8 {
    (n << 4) | n
}

struct Palette {
    r: [u8; 33],
    g: [u8; 33],
    b: [u8; 33],
}

const PALETTE: Palette = {
    let mut p = Palette {
        r: [0; 33],
        g: [0; 33],
        b: [0; 33],
    };
    let mut i: u8 = 0;
    while i < MAX_ITER {
        let pos = i & (SEGMENT_LEN - 1);
        let v = pos as u16 * MAX_INTENSITY as u16 / (SEGMENT_LEN - 1) as u16;
        let mut rv: u8 = 0;
        let mut gv: u8;
        let mut bv: u8 = 0;
        if i < SEGMENT_LEN {
            gv = v as u8;
            bv = MAX_INTENSITY;
        } else if i < SEGMENT_LEN * 2 {
            gv = MAX_INTENSITY;
            bv = MAX_INTENSITY - v as u8;
        } else if i < SEGMENT_LEN * 3 {
            rv = v as u8;
            gv = MAX_INTENSITY;
        } else {
            rv = MAX_INTENSITY;
            gv = MAX_INTENSITY - v as u8;
        }
        p.r[(i + 1) as usize] = nyb(rv);
        p.g[(i + 1) as usize] = nyb(gv);
        p.b[(i + 1) as usize] = nyb(bv);
        i += 1;
    }
    p
};

fn fp_mul(a: Fix16, b: Fix16) -> Fix16 {
    unsafe {
        ptr::write_volatile(0xD770 as *mut i32, a as i32);
        ptr::write_volatile(0xD774 as *mut i32, b as i32);
        (ptr::read_volatile(0xD778 as *const u32) >> 8) as Fix16
    }
}

fn mandelbrot(cr: Fix16, ci: Fix16) -> u8 {
    let fp_four: Fix16 = 4 * FP_ONE;
    let mut zr: Fix16 = 0;
    let mut zi: Fix16 = 0;
    let mut i: u8 = 0;
    while i < MAX_ITER {
        let zr2 = fp_mul(zr, zr);
        let zi2 = fp_mul(zi, zi);
        if zr2.wrapping_add(zi2) > fp_four {
            return i;
        }
        zi = fp_mul(zr, zi);
        zi = zi.wrapping_mul(2);
        zi = zi.wrapping_add(ci);
        zr = zr2.wrapping_sub(zi2).wrapping_add(cr);
        i += 1;
    }
    MAX_ITER
}

fn trigger_dma(job: &DmaJob) {
    unsafe {
        let addr = job as *const DmaJob as u16;
        ptr::write_volatile(0xD703 as *mut u8, 1);
        ptr::write_volatile(0xD702 as *mut u8, 0);
        ptr::write_volatile(0xD701 as *mut u8, (addr >> 8) as u8);
        ptr::write_volatile(0xD705 as *mut u8, addr as u8);
    }
}

fn make_dma_fill(dst: u32, value: u8, count: u16) -> DmaJob {
    DmaJob {
        opt0: ENABLE_F018B_OPT,
        opt1: SRC_ADDR_BITS_OPT,
        opt2: 0,
        opt3: DST_ADDR_BITS_OPT,
        opt4: (dst >> 20) as u8,
        opt5: DST_SKIP_RATE_OPT,
        opt6: 1,
        end_option: 0,
        dmalist: DMAListF018B {
            command: DMA_FILL_CMD,
            count,
            source_addr: value as u16,
            source_bank: 0,
            dest_addr: dst as u16,
            dest_bank: (dst >> 16) as u8,
            command_msb: 0,
            modulo: 0,
        },
    }
}

fn make_dma_copy(src: u32, dst: u32, count: u16) -> DmaJob {
    let mut job = make_dma_fill(dst, 0, count);
    job.opt2 = (src >> 20) as u8;
    job.dmalist.command = DMA_COPY_CMD;
    job.dmalist.source_addr = src as u16;
    job.dmalist.source_bank = (src >> 16) as u8;
    job
}

fn setup_vic() {
    unsafe {
        core::arch::asm!("sei");
        ptr::write_volatile(0xD02F as *mut u8, 0x47);
        ptr::write_volatile(0xD02F as *mut u8, 0x53);
        ptr::write_volatile(
            0xD05D as *mut u8,
            ptr::read_volatile(0xD05D as *const u8) & !VIC4_HOTREG_MASK,
        );
        ptr::write_volatile(0x0000 as *mut u8, 65);
        let ctrlb = ptr::read_volatile(0xD031 as *const u8);
        ptr::write_volatile(
            0xD031 as *mut u8,
            (ctrlb | VIC3_FAST_MASK | VIC3_ATTR_MASK) & !(VIC3_H640_MASK | VIC3_V400_MASK),
        );
        let ctrlc = ptr::read_volatile(0xD054 as *const u8);
        ptr::write_volatile(
            0xD054 as *mut u8,
            (ctrlc & !VIC4_FCLRLO_MASK) | VIC4_CHR16_MASK | VIC4_FCLRHI_MASK,
        );
        ptr::write_volatile(0xD060 as *mut u8, 0x00);
        ptr::write_volatile(0xD061 as *mut u8, 0x08);
        ptr::write_volatile(0xD062 as *mut u8, 0x00);
        ptr::write_volatile(0xD063 as *mut u8, 0x00);
        ptr::write_volatile(0xD068 as *mut u8, 0x00);
        ptr::write_volatile(0xD069 as *mut u8, 0x00);
        ptr::write_volatile(0xD06A as *mut u8, 0x00);
        ptr::write_volatile(0xD064 as *mut u16, (CELL_COLS as u16) * 2);
        ptr::write_volatile(0xD066 as *mut u8, CELL_COLS);
        ptr::write_volatile(0xD067 as *mut u8, CELL_ROWS);
        let ctrl1 = ptr::read_volatile(0xD000 as *const u8);
        ptr::write_volatile(0xD000 as *mut u8, (ctrl1 & 0xC0) | 0x1B);
        let ctrl2 = ptr::read_volatile(0xD001 as *const u8);
        ptr::write_volatile(0xD001 as *mut u8, (ctrl2 & 0xE0) | 0x08);
        ptr::write_volatile(0xD020 as *mut u8, 0);
        ptr::write_volatile(0xD021 as *mut u8, 0);
        let ctrla = ptr::read_volatile(0xD02A as *const u8);
        ptr::write_volatile(0xD02A as *mut u8, ctrla | VIC3_PAL_MASK);
    }
}

fn setup_screen() {
    unsafe {
        for i in 0..=MAX_ITER as usize {
            ptr::write_volatile((0xD100 + i) as *mut u8, PALETTE.r[i]);
            ptr::write_volatile((0xD200 + i) as *mut u8, PALETTE.g[i]);
            ptr::write_volatile((0xD300 + i) as *mut u8, PALETTE.b[i]);
        }
        for i in 0..NUM_CELLS as usize {
            ptr::write_volatile((0x0800 + i * 2) as *mut u16, TILE_BASE + i as u16);
        }
        for i in 0..NUM_CELLS as usize {
            ptr::write_volatile((0xD800 + i) as *mut u8, 0);
        }
        let fill_job = make_dma_fill(GFX_ADDR, 0, NUM_CELLS * TILE_BYTES);
        trigger_dma(&fill_job);
    }
}

fn render_fractal() {
    let re_min: Fix16 = -2 * FP_ONE;
    let re_max: Fix16 = 153i16;
    let im_min: Fix16 = -FP_ONE;
    let im_max: Fix16 = FP_ONE;
    let re_step: Fix16 = (re_max - re_min) / 320;
    let im_step: Fix16 = (im_max - im_min) / 200;

    for cy in 0..CELL_ROWS {
        for cx in 0..CELL_COLS {
            let tile_off = (cx as u16) * TILE_BYTES;
            for py in 0..TILE_PIXELS {
                let y = (cy as u16 * TILE_PIXELS as u16 + py as u16) as Fix16;
                for px in 0..TILE_PIXELS {
                    let x = (cx as u16 * TILE_PIXELS as u16 + px as u16) as Fix16;
                    let cr = re_min.wrapping_add(x.wrapping_mul(re_step));
                    let ci = im_min.wrapping_add(y.wrapping_mul(im_step));
                    let iter = mandelbrot(cr, ci);
                    unsafe {
                        TILE_ROW_BUF
                            [(tile_off + py as u16 * TILE_PIXELS as u16 + px as u16) as usize] =
                            if iter >= MAX_ITER { 0 } else { iter + 1 };
                    }
                }
            }
        }
        let src: u32 = &raw const TILE_ROW_BUF as u32;
        let dst = GFX_ADDR + (cy as u32) * TILE_ROW_BYTES as u32;
        let copy_job = make_dma_copy(src, dst, TILE_ROW_BYTES);
        trigger_dma(&copy_job);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    setup_vic();
    setup_screen();
    render_fractal();
    loop {}
}
