fn main() {
    let target = std::env::var("TARGET").unwrap_or_default();

    // Detect mapper from target triple (e.g. "mos-nes-unrom-none")
    let (prg, chr, chr_ram, mapper) = if target.contains("nes-unrom-512") {
        (512, 0, 32, 30)
    } else if target.contains("nes-unrom") {
        (128, 0, 8, 2)
    } else if target.contains("nes-mmc1") {
        (128, 0, 8, 1)
    } else if target.contains("nes-mmc3") {
        (256, 0, 8, 4)
    } else if target.contains("nes-cnrom") {
        (32, 8, 0, 3)
    } else if target.contains("nes-gtrom") {
        (512, 0, 8, 77)
    } else if target.contains("nes-action53") {
        (32, 0, 8, 28)
    } else {
        // NROM / default (CHR-ROM with font)
        (32, 8, 0, 0)
    };

    println!("cargo:rustc-link-arg=-Wl,--defsym,__prg_rom_size={}", prg);
    println!("cargo:rustc-link-arg=-Wl,--defsym,__chr_rom_size={}", chr);
    println!(
        "cargo:rustc-link-arg=-Wl,--defsym,__chr_ram_size={}",
        chr_ram
    );
    println!("cargo:rustc-link-arg=-Wl,--defsym,__chr_nvram_size=0");
    println!("cargo:rustc-link-arg=-Wl,--defsym,__prg_ram_size=0");
    println!("cargo:rustc-link-arg=-Wl,--defsym,__prg_nvram_size=0");
    println!("cargo:rustc-link-arg=-Wl,--defsym,__mapper={}", mapper);
    println!("cargo:rustc-link-lib=neslib");
    println!("cargo:rustc-link-lib=nesdoug");
    println!("cargo:rustc-link-lib=c");
    println!("cargo:rustc-link-lib=crt");
    println!("cargo:rustc-link-lib=crt0");
    println!("cargo:rerun-if-env-changed=TARGET");
    println!("cargo:rerun-if-env-changed=LLVM_MOS_SDK");
}
