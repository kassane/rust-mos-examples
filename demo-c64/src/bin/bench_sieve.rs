#![no_std]
#![no_main]

extern crate demo_c64 as _;

unsafe extern "C" {
    fn printf(fmt: *const u8, ...) -> i32;
}

const SIZE: usize = 8191;
const N_ITER: i32 = 10;
const EXPECTED: u32 = 1900;

static mut FLAGS: [bool; SIZE] = [false; SIZE];

fn sieve(n: usize) -> u32 {
    let mut count: u32 = 1;
    for k in 0..n {
        unsafe { FLAGS[k] = true };
    }
    for i in 0..n {
        if unsafe { FLAGS[i] } {
            let prime = i + i + 3;
            let mut k = i + prime;
            while k < n {
                unsafe { FLAGS[k] = false };
                k += prime;
            }
            count = count.wrapping_add(1);
        }
    }
    count
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    unsafe {
        printf(b"sieve.zig\n\0".as_ptr());
        printf(b"Calculates the primes from 1 to 16382 (10 iterations)\n\0".as_ptr());
    }
    let mut prime_count: u32 = 0;
    let mut i: i32 = 0;
    while i < N_ITER {
        prime_count = sieve(SIZE);
        i += 1;
    }
    unsafe {
        printf(b"count=%u\0".as_ptr(), prime_count);
        if prime_count == EXPECTED {
            printf(b" [OK]\n\0".as_ptr());
        } else {
            printf(b" [FAIL] - expected %u\n\0".as_ptr(), EXPECTED);
        }
    }
    loop {}
}
