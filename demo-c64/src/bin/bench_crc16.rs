#![no_std]
#![no_main]

extern crate demo_c64 as _;

unsafe extern "C" {
    fn printf(fmt: *const u8, ...) -> i32;
}

const KERNAL: *const u8 = 0xE000 as *const u8;
const KERNAL_LEN: usize = 0x2000;
const EXPECTED: u16 = 0xffd0;

fn crc16(data: *const u8, len: usize) -> u16 {
    let mut crc: u16 = 0;
    for i in 0..len {
        crc ^= (unsafe { *data.add(i) } as u16) << 8;
        for _ in 0..8 {
            if crc & 0x8000 != 0 {
                crc = (crc << 1) ^ 0x1021;
            } else {
                crc <<= 1;
            }
        }
    }
    crc
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    unsafe {
        printf(b"crc16.zig\n\0".as_ptr());
        printf(b"Calculates the CRC16 of the C64 Kernal\n\0".as_ptr());
    }
    let result = crc16(KERNAL, KERNAL_LEN);
    unsafe {
        printf(b"CRC16=%04X\0".as_ptr(), result as u32);
        if result == EXPECTED {
            printf(b" [OK]\n\0".as_ptr());
        } else {
            printf(b" [FAIL] - expected %04X\n\0".as_ptr(), EXPECTED as u32);
        }
    }
    loop {}
}
