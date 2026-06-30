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
        r#"SEARCH_DIR("{sdk}/mos-platform/sim/lib");
SEARCH_DIR("{sdk}/mos-platform/common/lib");
SEARCH_DIR("{sdk}/mos-platform/common/ldscripts");
INCLUDE "{sdk}/mos-platform/sim/lib/link.ld""#,
        sdk = sdk
    );
    std::fs::write(out.join("wrapper.ld"), &wrapper).unwrap();
    println!(
        "cargo:rustc-link-arg=-T{}",
        out.join("wrapper.ld").display()
    );
    println!("cargo:rustc-link-arg={sdk}/mos-platform/common/lib/crt0.o");
    println!("cargo:rustc-link-lib=c");
    println!("cargo:rustc-link-lib=crt0");
    println!("cargo:rustc-link-lib=crt");

    println!("cargo:rerun-if-env-changed=LLVM_MOS_SDK");
}
