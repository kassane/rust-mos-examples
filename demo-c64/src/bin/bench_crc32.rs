#![no_std]
#![no_main]

extern crate demo_c64 as _;

unsafe extern "C" {
    fn printf(fmt: *const u8, ...) -> i32;
}

const KERNAL: *const u8 = 0xE000 as *const u8;
const KERNAL_LEN: usize = 0x2000;
const EXPECTED: u32 = 0xe1fa84c6;

const TABLE: [u32; 256] = {
    let mut t = [0u32; 256];
    let mut i: u32 = 0;
    while i < 256 {
        let mut crc = i;
        let mut j = 0;
        while j < 8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ 0xEDB88320;
            } else {
                crc >>= 1;
            }
            j += 1;
        }
        t[i as usize] = crc;
        i += 1;
    }
    t
};

fn crc32(data: *const u8, len: usize) -> u32 {
    let mut crc: u32 = 0xFFFFFFFF;
    for i in 0..len {
        let idx = ((crc ^ unsafe { *data.add(i) as u32 }) & 0xFF) as usize;
        crc = TABLE[idx] ^ (crc >> 8);
    }
    crc ^ 0xFFFFFFFF
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    unsafe {
        printf(b"crc32.zig\n\0".as_ptr());
        printf(b"Calculates the CRC32 of the C64 Kernal\n\0".as_ptr());
    }
    let result = crc32(KERNAL, KERNAL_LEN);
    unsafe {
        printf(b"CRC32=%08lX\0".as_ptr(), result);
        if result == EXPECTED {
            printf(b" [OK]\n\0".as_ptr());
        } else {
            printf(b" [FAIL] - expected %08lX\n\0".as_ptr(), EXPECTED);
        }
    }
    loop {}
}
