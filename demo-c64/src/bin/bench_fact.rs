#![no_std]
#![no_main]

extern crate demo_c64 as _;

unsafe extern "C" {
    fn printf(fmt: *const u8, ...) -> i32;
}

const SIZE: usize = 16;
const N_ITER: i32 = 1000;
const EXPECTED: i32 = 188806544;

const FACT_TABLE: [i32; SIZE] = {
    let mut t = [0i32; SIZE];
    t[0] = 1;
    let mut i = 1;
    while i < SIZE {
        t[i] = t[i - 1].wrapping_mul(i as i32);
        i += 1;
    }
    t
};

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    unsafe {
        printf(b"fact.zig\n\0".as_ptr());
        printf(b"Calculates factorials (1000 iterations)\n\0".as_ptr());
    }
    let mut array = [0i32; SIZE];
    let mut res: i32 = 0;
    let mut i: i32 = 0;
    while i < N_ITER {
        for j in 0..SIZE {
            array[j] = array[j].wrapping_add(FACT_TABLE[j]);
        }
        i += 1;
    }
    for j in 0..SIZE {
        res = res.wrapping_add(array[j]);
    }
    unsafe {
        printf(b"res=%ld\0".as_ptr(), res);
        if res == EXPECTED {
            printf(b" [OK]\n\0".as_ptr());
        } else {
            printf(b" [FAIL] - expected %ld\n\0".as_ptr(), EXPECTED);
        }
    }
    loop {}
}
