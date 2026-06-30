// Copyright (c) 2026 Matheus C. França
// SPDX-License-Identifier: Apache-2.0
//! MEGA65 vector logo: rotating wireframe "LLVM-MOS" on a 320x200 hires bitmap.
//! VIC-II BMM mode; hardware 32x32 math for fixed-point rotation; Enhanced DMA clear.

#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]

extern crate demo_mega65 as _;

use core::ptr;

type Fix16 = i16;
const FP_ONE: Fix16 = 256;

const SCREEN_W: u16 = 320;
const SCREEN_H: u16 = 200;
const CELL_COLS: u8 = 40;
const CELL_ROWS: u8 = 25;
const SCREEN_RAM_SIZE: u16 = 1000;
const CELL_ROW_BYTES: u16 = 320;
const BITMAP_SIZE: u16 = 8000;
const VIC3_FAST_MASK: u8 = 0x40;
const VIC3_H640_MASK: u8 = 0x80;
const VIC3_V400_MASK: u8 = 0x08;
const VIC4_CHR16_MASK: u8 = 0x01;
const VIC4_FCLRLO_MASK: u8 = 0x02;
const VIC4_FCLRHI_MASK: u8 = 0x04;
const VIC4_HOTREG_MASK: u8 = 0x80;

const ENABLE_F018B_OPT: u8 = 0x0b;
const SRC_ADDR_BITS_OPT: u8 = 0x80;
const DST_ADDR_BITS_OPT: u8 = 0x81;
const DST_SKIP_RATE_OPT: u8 = 0x85;
const DMA_FILL_CMD: u8 = 0x03;

const SIN_TABLE: [Fix16; 256] = [
    0, 6, 13, 19, 25, 31, 38, 44, 50, 56, 62, 68, 74, 80, 86, 92, 98, 104, 109, 115, 121, 126, 132,
    137, 142, 147, 152, 157, 162, 167, 172, 177, 181, 185, 190, 194, 198, 202, 206, 209, 213, 216,
    220, 223, 226, 229, 231, 234, 237, 239, 241, 243, 245, 247, 248, 250, 251, 252, 253, 254, 255,
    255, 256, 256, 256, 256, 256, 255, 255, 254, 253, 252, 251, 250, 248, 247, 245, 243, 241, 239,
    237, 234, 231, 229, 226, 223, 220, 216, 213, 209, 206, 202, 198, 194, 190, 185, 181, 177, 172,
    167, 162, 157, 152, 147, 142, 137, 132, 126, 121, 115, 109, 104, 98, 92, 86, 80, 74, 68, 62,
    56, 50, 44, 38, 31, 25, 19, 13, 6, 0, -5, -12, -18, -24, -30, -37, -43, -49, -55, -61, -67,
    -73, -79, -85, -91, -97, -103, -108, -114, -120, -125, -131, -136, -141, -146, -151, -156,
    -161, -166, -171, -176, -180, -184, -189, -193, -197, -201, -205, -208, -212, -215, -219, -222,
    -225, -228, -230, -233, -236, -238, -240, -242, -244, -246, -247, -249, -250, -251, -252, -253,
    -254, -254, -255, -255, -255, -255, -255, -254, -254, -253, -252, -251, -250, -249, -247, -246,
    -244, -242, -240, -238, -236, -233, -230, -228, -225, -222, -219, -215, -212, -208, -205, -201,
    -197, -193, -189, -184, -180, -176, -171, -166, -161, -156, -151, -146, -141, -136, -131, -125,
    -120, -114, -108, -103, -97, -91, -85, -79, -73, -67, -61, -55, -49, -43, -37, -30, -24, -18,
    -12, -5,
];

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

fn sin8(angle: u8) -> Fix16 {
    SIN_TABLE[angle as usize]
}

fn cos8(angle: u8) -> Fix16 {
    SIN_TABLE[angle.wrapping_add(64) as usize]
}

struct Vertex {
    x: i8,
    y: i8,
}
struct Segment {
    v0: u8,
    v1: u8,
}

const VERTICES: [Vertex; 31] = [
    Vertex { x: -54, y: -8 },
    Vertex { x: -54, y: 8 },
    Vertex { x: -46, y: 8 },
    Vertex { x: -42, y: -8 },
    Vertex { x: -42, y: 8 },
    Vertex { x: -34, y: 8 },
    Vertex { x: -32, y: -8 },
    Vertex { x: -28, y: 8 },
    Vertex { x: -24, y: -8 },
    Vertex { x: -20, y: 8 },
    Vertex { x: -20, y: -8 },
    Vertex { x: -14, y: 0 },
    Vertex { x: -8, y: -8 },
    Vertex { x: -8, y: 8 },
    Vertex { x: -4, y: 0 },
    Vertex { x: 4, y: 0 },
    Vertex { x: 6, y: 8 },
    Vertex { x: 6, y: -8 },
    Vertex { x: 12, y: 0 },
    Vertex { x: 18, y: -8 },
    Vertex { x: 18, y: 8 },
    Vertex { x: 22, y: -8 },
    Vertex { x: 32, y: -8 },
    Vertex { x: 32, y: 8 },
    Vertex { x: 22, y: 8 },
    Vertex { x: 46, y: -8 },
    Vertex { x: 36, y: -8 },
    Vertex { x: 36, y: 0 },
    Vertex { x: 46, y: 0 },
    Vertex { x: 46, y: 8 },
    Vertex { x: 36, y: 8 },
];

const SEGMENTS: [Segment; 24] = [
    Segment { v0: 0, v1: 1 },
    Segment { v0: 1, v1: 2 },
    Segment { v0: 3, v1: 4 },
    Segment { v0: 4, v1: 5 },
    Segment { v0: 6, v1: 7 },
    Segment { v0: 7, v1: 8 },
    Segment { v0: 9, v1: 10 },
    Segment { v0: 10, v1: 11 },
    Segment { v0: 11, v1: 12 },
    Segment { v0: 12, v1: 13 },
    Segment { v0: 14, v1: 15 },
    Segment { v0: 16, v1: 17 },
    Segment { v0: 17, v1: 18 },
    Segment { v0: 18, v1: 19 },
    Segment { v0: 19, v1: 20 },
    Segment { v0: 21, v1: 22 },
    Segment { v0: 22, v1: 23 },
    Segment { v0: 23, v1: 24 },
    Segment { v0: 24, v1: 21 },
    Segment { v0: 25, v1: 26 },
    Segment { v0: 26, v1: 27 },
    Segment { v0: 27, v1: 28 },
    Segment { v0: 28, v1: 29 },
    Segment { v0: 29, v1: 30 },
];

#[repr(align(8192))]
struct AlignedBitmap([u8; BITMAP_SIZE as usize]);
static mut BITMAP: AlignedBitmap = AlignedBitmap([0; BITMAP_SIZE as usize]);
static mut SCREEN_RAM: [u8; SCREEN_RAM_SIZE as usize] = [0; SCREEN_RAM_SIZE as usize];
static mut SCREEN_X: [i16; 31] = [0; 31];
static mut SCREEN_Y: [i16; 31] = [0; 31];

const ROW_TABLE: [u16; 200] = {
    let mut tbl: [u16; 200] = [0; 200];
    let mut y: u32 = 0;
    while y < 200 {
        tbl[y as usize] = ((y >> 3) * CELL_ROW_BYTES as u32 + (y & 7)) as u16;
        y += 1;
    }
    tbl
};

const BIT_MASK: [u8; 8] = [0x80, 0x40, 0x20, 0x10, 0x08, 0x04, 0x02, 0x01];

fn plot_pixel(x: i16, y: i16) {
    let ux = x as u16;
    let uy = y as u16;
    if ux >= SCREEN_W || uy >= SCREEN_H {
        return;
    }
    unsafe {
        BITMAP.0[ROW_TABLE[uy as usize] as usize + (ux & !7) as usize] |=
            BIT_MASK[(ux & 7) as usize];
    }
}

fn draw_line(x0: i16, y0: i16, x1: i16, y1: i16) {
    let mut ax = x0;
    let mut ay = y0;
    let mut dx = x1 - x0;
    let mut dy = y1 - y0;
    let mut sx: i16 = 1;
    let mut sy: i16 = 1;

    if dx < 0 {
        dx = -dx;
        sx = -1;
    }
    if dy < 0 {
        dy = -dy;
        sy = -1;
    }

    if dx >= dy {
        let mut err = dx >> 1;
        let mut i = 0;
        while i <= dx {
            plot_pixel(ax, ay);
            err -= dy;
            if err < 0 {
                ay += sy;
                err += dx;
            }
            ax += sx;
            i += 1;
        }
    } else {
        let mut err = dy >> 1;
        let mut i = 0;
        while i <= dy {
            plot_pixel(ax, ay);
            err -= dx;
            if err < 0 {
                ax += sx;
                err += dy;
            }
            ay += sy;
            i += 1;
        }
    }
}

fn fp_set_a(v: Fix16) {
    unsafe {
        ptr::write_volatile(0xD770 as *mut i32, v as i32);
    }
}

fn fp_set_b(v: Fix16) {
    unsafe {
        ptr::write_volatile(0xD774 as *mut i32, v as i32);
    }
}

fn fp_result() -> Fix16 {
    unsafe { (ptr::read_volatile(0xD778 as *const u32) >> 8) as Fix16 }
}

fn fp_mul(a: Fix16, b: Fix16) -> Fix16 {
    fp_set_a(a);
    fp_set_b(b);
    fp_result()
}

fn transform_vertices(spin: u8, tilt: u8, yaw: u8, scale: Fix16) {
    let ss = sin8(spin);
    let cs = cos8(spin);
    let ct = cos8(tilt);
    let cy = cos8(yaw);

    for (i, v) in VERTICES.iter().enumerate() {
        let vx = (v.x as Fix16) * FP_ONE;
        let vy = (v.y as Fix16) * FP_ONE;

        fp_set_a(vy);
        fp_set_b(ct);
        let vy_t = fp_result();

        fp_set_a(vy_t);
        fp_set_b(cs);
        let vy_t_cs = fp_result();
        fp_set_b(ss);
        let vy_t_ss = fp_result();

        fp_set_a(vx);
        fp_set_b(cy);
        let vx_y = fp_result();

        fp_set_a(vx_y);
        fp_set_b(ss);
        let vx_y_ss = fp_result();
        fp_set_b(cs);
        let vx_y_cs = fp_result();

        let rx = vx_y_cs.wrapping_sub(vy_t_ss);
        let ry = vx_y_ss.wrapping_add(vy_t_cs);

        fp_set_a(rx);
        fp_set_b(scale);
        unsafe {
            SCREEN_X[i] = (fp_result() >> 8) + (SCREEN_W / 2) as i16;
        }

        fp_set_a(ry);
        unsafe {
            SCREEN_Y[i] = (fp_result() >> 8) + (SCREEN_H / 2) as i16;
        }
    }
}

fn draw_segments() {
    for seg in &SEGMENTS {
        unsafe {
            draw_line(
                SCREEN_X[seg.v0 as usize],
                SCREEN_Y[seg.v0 as usize],
                SCREEN_X[seg.v1 as usize],
                SCREEN_Y[seg.v1 as usize],
            );
        }
    }
}

fn clear_bitmap() {
    let dst = &raw mut BITMAP as u32;
    let fill_job = make_dma_fill(dst, 0, BITMAP_SIZE);
    trigger_dma(&fill_job);
}

fn make_dma_fill(dst_within_first_64k: u32, value: u8, count: u16) -> DmaJob {
    DmaJob {
        opt0: ENABLE_F018B_OPT,
        opt1: SRC_ADDR_BITS_OPT,
        opt2: 0,
        opt3: DST_ADDR_BITS_OPT,
        opt4: (dst_within_first_64k >> 20) as u8,
        opt5: DST_SKIP_RATE_OPT,
        opt6: 1,
        end_option: 0,
        dmalist: DMAListF018B {
            command: DMA_FILL_CMD,
            count,
            source_addr: value as u16,
            source_bank: 0,
            dest_addr: dst_within_first_64k as u16,
            dest_bank: (dst_within_first_64k >> 16) as u8,
            command_msb: 0,
            modulo: 0,
        },
    }
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

fn wait_vblank() {
    unsafe {
        while ptr::read_volatile(0xD000 as *const u8) & 0x80 != 0 {}
        while ptr::read_volatile(0xD000 as *const u8) & 0x80 == 0 {}
    }
}

fn setup_vic() {
    unsafe {
        let cell_color: u8 = 0x86;

        core::arch::asm!("sei");

        ptr::write_volatile(0xD02F as *mut u8, 0x47);
        ptr::write_volatile(0xD02F as *mut u8, 0x53);

        ptr::write_volatile(
            0xD05D as *mut u8,
            ptr::read_volatile(0xD05D as *const u8) & !VIC4_HOTREG_MASK,
        );

        let cpu_ddr = ptr::read_volatile(0x0000 as *const u8);
        ptr::write_volatile(0x0000 as *mut u8, cpu_ddr & !1);

        let ctrlb = ptr::read_volatile(0xD031 as *const u8);
        ptr::write_volatile(
            0xD031 as *mut u8,
            (ctrlb | VIC3_FAST_MASK) & !(VIC3_H640_MASK | VIC3_V400_MASK),
        );

        let scrn = &raw mut SCREEN_RAM as u16;
        ptr::write_volatile(0xD060 as *mut u8, scrn as u8);
        ptr::write_volatile(0xD061 as *mut u8, (scrn >> 8) as u8);
        ptr::write_volatile(0xD062 as *mut u8, 0x00);
        ptr::write_volatile(0xD063 as *mut u8, 0x00);

        let bm = &raw mut BITMAP as u16;
        ptr::write_volatile(0xD068 as *mut u8, bm as u8);
        ptr::write_volatile(0xD069 as *mut u8, (bm >> 8) as u8);
        ptr::write_volatile(0xD06A as *mut u8, 0x00);

        ptr::write_volatile(0xD064 as *mut u16, CELL_COLS as u16);
        ptr::write_volatile(0xD066 as *mut u8, CELL_COLS);
        ptr::write_volatile(0xD067 as *mut u8, CELL_ROWS);

        ptr::write_volatile(0xD000 as *mut u8, 0x3B);
        ptr::write_volatile(0xD001 as *mut u8, 0x08);

        let ctrlc = ptr::read_volatile(0xD054 as *const u8);
        ptr::write_volatile(
            0xD054 as *mut u8,
            ctrlc & !(VIC4_CHR16_MASK | VIC4_FCLRHI_MASK | VIC4_FCLRLO_MASK),
        );

        ptr::write_volatile(0xD020 as *mut u8, 6);
        ptr::write_volatile(0xD021 as *mut u8, 6);

        for i in 0..SCREEN_RAM_SIZE as usize {
            ptr::write_volatile((0x0800 + i) as *mut u8, cell_color);
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    setup_vic();

    let scale: Fix16 = 540;
    let breath_amp: Fix16 = 70;

    let mut spin_acc: u16 = 0;
    let mut tilt_acc: u16 = 0;
    let mut yaw_acc: u16 = 0;
    let mut breath_acc: u16 = 0;

    loop {
        wait_vblank();
        clear_bitmap();

        let spin = (spin_acc >> 8) as u8;
        let tilt = (tilt_acc >> 8) as u8;
        let yaw = (yaw_acc >> 8) as u8;
        let breath = (breath_acc >> 8) as u8;

        let s = scale.wrapping_add(fp_mul(sin8(breath), breath_amp));

        transform_vertices(spin, tilt, yaw, s);
        draw_segments();

        spin_acc = spin_acc.wrapping_add(139);
        tilt_acc = tilt_acc.wrapping_add(107);
        yaw_acc = yaw_acc.wrapping_add(181);
        breath_acc = breath_acc.wrapping_add(79);
    }
}
