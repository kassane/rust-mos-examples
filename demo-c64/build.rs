fn main() {
    let sdk = std::env::var("LLVM_MOS_SDK").unwrap_or_else(|_| {
        let here = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
        here.join("../../llvm-mos")
            .canonicalize()
            .unwrap()
            .display()
            .to_string()
    });
    let out = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());

    let wrapper = format!(
        r#"SEARCH_DIR("{sdk}/mos-platform/c64/lib");
SEARCH_DIR("{sdk}/mos-platform/commodore/lib");
SEARCH_DIR("{sdk}/mos-platform/common/lib");
SEARCH_DIR("{sdk}/mos-platform/common/ldscripts");
__basic_zp_start = 0x0002;
__basic_zp_end = 0x0090;
MEMORY {{ ram (rw) : ORIGIN = 0x0801, LENGTH = 0xC7FF }}
__rc0 = __basic_zp_start;
INCLUDE "imag-regs.ld"
__basic_zp_size = __basic_zp_end - __basic_zp_start;
MEMORY {{ zp : ORIGIN = __rc31 + 1, LENGTH = __basic_zp_end - (__rc31 + 1) }}
REGION_ALIAS("c_readonly", ram)
REGION_ALIAS("c_writeable", ram)
SECTIONS {{ .basic_header : {{ *(.basic_header) }} INCLUDE "c.ld" }}
__stack = 0xD000;
OUTPUT_FORMAT {{ SHORT(ORIGIN(ram)) TRIM(ram) }}"#,
        sdk = sdk
    );
    std::fs::write(out.join("c64-wrapper.ld"), &wrapper).unwrap();
    println!(
        "cargo:rustc-link-arg=-T{}",
        out.join("c64-wrapper.ld").display()
    );
    println!("cargo:rustc-link-arg={sdk}/mos-platform/c64/lib/basic-header.o");
    println!("cargo:rustc-link-arg={sdk}/mos-platform/c64/lib/unmap-basic.o");
    println!("cargo:rustc-link-arg={sdk}/mos-platform/common/lib/crt0.o");
    println!("cargo:rustc-link-lib=c");
    println!("cargo:rustc-link-lib=crt0");
    println!("cargo:rustc-link-lib=crt");

    println!("cargo:rerun-if-env-changed=LLVM_MOS_SDK");
}
