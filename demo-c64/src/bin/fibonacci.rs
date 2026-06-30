// Copyright (c) 2026 Matheus C. França
// SPDX-License-Identifier: Apache-2.0
//! C64 Fibonacci: computes and displays Fibonacci sequence.

#![no_std]
#![no_main]

extern crate demo_c64 as _;

unsafe extern "C" {
    fn printf(fmt: *const u8, ...) -> i32;
}

const fn fib(n: usize) -> i32 {
    if n <= 1 {
        n as i32
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

const FIB_TABLE: [i32; 10] = [
    fib(0),
    fib(1),
    fib(2),
    fib(3),
    fib(4),
    fib(5),
    fib(6),
    fib(7),
    fib(8),
    fib(9),
];

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    let mut i: i32 = 0;
    loop {
        unsafe {
            printf(b"fib(%d) = \0".as_ptr(), i);
            printf(b"%d\n\0".as_ptr(), FIB_TABLE[i as usize]);
        }
        i += 1;
        if i >= 10 {
            break;
        }
    }
    loop {}
}
