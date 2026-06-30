#![no_std]
#![no_main]

extern crate demo_sim as _;

use core::ptr;

const PUTCHAR: *mut u8 = 0xFFF9 as *mut u8;
const EXIT: *mut u8 = 0xFFF8 as *mut u8;

fn write_char(c: u8) {
    unsafe { ptr::write_volatile(PUTCHAR, c) }
}

fn write_str(s: &[u8]) {
    for &c in s {
        write_char(c);
    }
}

fn write_u16_padded(v: u16, width: u8) {
    let powers = [10000u16, 1000, 100, 10, 1];
    let digits = if v >= 10000 {
        5
    } else if v >= 1000 {
        4
    } else if v >= 100 {
        3
    } else if v >= 10 {
        2
    } else {
        1
    };
    for _ in digits..width {
        write_char(b' ');
    }
    let mut n = v;
    let mut printed = false;
    for &p in &powers {
        let mut d: u8 = 0;
        while n >= p {
            n -= p;
            d += 1;
        }
        if d != 0 || printed || p == 1 {
            write_char(b'0' + d);
            printed = true;
        }
    }
}

fn write_u16(v: u16) {
    write_u16_padded(v, 0);
}

fn reset_clock() {
    unsafe { ptr::write_volatile(0xFFF0 as *mut u8, 0) }
}

fn read_clock() -> u16 {
    unsafe {
        let lo = ptr::read_volatile(0xFFF0 as *const u8) as u16;
        let hi = ptr::read_volatile(0xFFF1 as *const u8) as u16;
        lo | (hi << 8)
    }
}

fn fib(n: u8) -> u16 {
    let mut a: u16 = 0;
    let mut b: u16 = 1;
    for _ in 0..n {
        let t = a.wrapping_add(b);
        a = b;
        b = t;
    }
    a
}

#[unsafe(no_mangle)]
pub static mut FIB_N: u8 = 20;
#[unsafe(no_mangle)]
pub static mut FIB_N10: u8 = 10;

fn count_primes() -> u8 {
    let mut sieve = [0u8; 128];
    sieve[0] = 1;
    sieve[1] = 1;
    let mut p: u8 = 2;
    while p < 128 {
        if sieve[p as usize] == 0 {
            let mut mul: u16 = (p as u16) * (p as u16);
            while mul < 128 {
                sieve[mul as usize] = 1;
                mul += p as u16;
            }
        }
        p += 1;
    }
    let mut count: u8 = 0;
    for i in 0..128u8 {
        if sieve[i as usize] == 0 {
            count += 1;
        }
    }
    count
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    write_str(b"mos-sim benchmarks\n");
    write_str(b"==================\n");

    reset_clock();
    let r10 = fib(unsafe { FIB_N10 });
    let c10 = read_clock();
    write_str(b"fib(10) = ");
    write_u16_padded(r10, 6);
    write_str(b"  (");
    write_u16_padded(c10, 4);
    write_str(b" cycles)\n");

    reset_clock();
    let r20 = fib(unsafe { FIB_N });
    let c20 = read_clock();
    write_str(b"fib(20) = ");
    write_u16_padded(r20, 6);
    write_str(b"  (");
    write_u16_padded(c20, 4);
    write_str(b" cycles)\n");

    reset_clock();
    let primes = count_primes();
    let cs = read_clock();
    write_str(b"sieve<127>: ");
    write_u16(primes as u16);
    write_str(b" primes  (");
    write_u16_padded(cs, 4);
    write_str(b" cycles)\n");

    unsafe { ptr::write_volatile(EXIT, 0) }
    loop {}
}
