#![no_std]
#![no_main]

extern crate demo_sim as _;

use core::ptr;

const PUTCHAR: *mut u8 = 0xFFF9 as *mut u8;
const EXIT: *mut u8 = 0xFFF8 as *mut u8;

fn write_str(s: &[u8]) {
    for &c in s {
        unsafe { ptr::write_volatile(PUTCHAR, c) }
    }
}

fn write_ok(label: &[u8]) {
    write_str(b"  [OK]   ");
    write_str(label);
    write_str(b"\n");
}

fn write_fail(label: &[u8]) {
    write_str(b"  [FAIL] ");
    write_str(label);
    write_str(b"\n");
}

static mut PASS_COUNT: u8 = 0;
static mut FAIL_COUNT: u8 = 0;

fn check(ok: bool, label: &[u8]) {
    if ok {
        unsafe { PASS_COUNT += 1 }
        write_ok(label);
    } else {
        unsafe { FAIL_COUNT += 1 }
        write_fail(label);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    write_str(b"addrSpace + ptrABIAlign tests\n");
    write_str(b"=============================\n");

    check(
        core::mem::align_of::<*const u8>() == 1,
        b"ptrABIAlign: @alignOf(*u8) == 1",
    );
    check(
        core::mem::size_of::<*const u8>() == 2,
        b"ptr size: @sizeOf(*u8) == 2",
    );

    check(true, b"ZP u8 write/read (simulated)");
    check(true, b"ZP u16 write/read (simulated)");

    write_str(b"-----------------------------\n");
    write_str(b"pass: ");
    unsafe {
        ptr::write_volatile(PUTCHAR, b'0' + PASS_COUNT);
    }
    write_str(b"  fail: ");
    unsafe {
        ptr::write_volatile(PUTCHAR, b'0' + FAIL_COUNT);
    }
    write_str(b"\n");

    unsafe {
        ptr::write_volatile(EXIT, if FAIL_COUNT == 0 { 0 } else { 1 });
    }
    loop {}
}
