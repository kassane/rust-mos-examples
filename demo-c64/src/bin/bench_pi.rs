#![no_std]
#![no_main]

extern crate demo_c64 as _;

unsafe extern "C" {
    fn printf(fmt: *const u8, ...) -> i32;
}

const SCALE: i32 = 10000;
const ARRINIT: i32 = SCALE / 5;
const NUM_DIG: usize = 560;
const EXPECTED: i32 = 2822;

static mut ARR: [i32; NUM_DIG + 1] = [0; NUM_DIG + 1];
static mut CARRY: i32 = 0;

fn pi_digits(digits: usize) {
    for i in 0..=digits {
        unsafe { ARR[i] = ARRINIT };
    }
    let mut i = digits;
    loop {
        let mut sum: i32 = 0;
        let mut j = i;
        while j > 0 {
            sum = sum.wrapping_mul(j as i32).wrapping_add(SCALE.wrapping_mul(unsafe { ARR[j] }));
            unsafe {
                ARR[j] = sum % ((j * 2 - 1) as i32);
            }
            sum = sum / ((j * 2 - 1) as i32);
            j -= 1;
        }
        unsafe {
            printf(b"%04d\0".as_ptr(), (CARRY.wrapping_add(sum / SCALE)) as i32);
            CARRY = sum % SCALE;
        }
        if i < 14 { break; }
        i -= 14;
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    unsafe {
        printf(b"pi.zig\n\0".as_ptr());
        printf(b"Calculates pi digits\n\0".as_ptr());
    }
    pi_digits(NUM_DIG);
    unsafe {
        printf(b"\ncarry=%ld\0".as_ptr(), CARRY);
        if CARRY == EXPECTED {
            printf(b" [OK]\n\0".as_ptr());
        } else {
            printf(b" [FAIL] - expected %ld\n\0".as_ptr(), EXPECTED);
        }
    }
    loop {}
}
