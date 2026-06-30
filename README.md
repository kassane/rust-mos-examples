# rust-mos-examples

Rust examples targeting MOS 6502 platforms via [llvm-mos-sdk](https://github.com/llvm-mos/llvm-mos-sdk).

## Requirements

- [rust-mos](https://github.com/kassane/rust-mos-examples/releases) — Rust toolchain with LLVM-MOS backend.
- [llvm-mos-sdk](https://github.com/llvm-mos/llvm-mos-sdk/releases) v23.x

> [!IMPORTANT]
> Both toolchains are LLVM 23 based to LTO.

## Building

Cargo build with `-Zbuild-std -Zunstable-options -Zjson-target-spec` (RUSTC_BOOTSTRAP=1 required):

```sh
# NES NROM
RUSTFLAGS="-Clink-arg=-flto -Clink-arg=-Wl,-u,main" cargo build \
  --target targets/mos-nes-nrom-none.json \
  -Zbuild-std -Zunstable-options -Zjson-target-spec \
  --release -p demo-nes --bin nes-nrom-hello

# MEGA65
RUSTFLAGS="-Clink-arg=-flto -Clink-arg=-Wl,-u,main" cargo build \
  --target targets/mos-mega65-none.json \
  -Zbuild-std -Zunstable-options -Zjson-target-spec \
  --release -p demo-mega65
```

## Code Quality

```sh
cargo fmt
cargo clippy --all-targets -- -D warnings
```

## References

- [zig-mos-examples](https://github.com/kassane/zig-mos-examples) - examples based
- [mrk-its/rust-mos](https://github.com/mrk-its/rust-mos) v1.87 [old - no asm-inline support] - this project uses v1.98-dev!