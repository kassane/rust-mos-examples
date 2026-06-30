fn main() {
    println!("cargo:rustc-link-arg=-Wl,--gc-sections");
    println!("cargo:rustc-link-arg=-lc");
    println!("cargo:rustc-link-arg=-lcrt");
    println!("cargo:rustc-link-arg=-lcrt0");
}
