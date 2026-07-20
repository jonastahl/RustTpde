
# TPDE Backend for Rust

Build the library:
```bash
cargo build --release
```

Use in rustc
```bash
rustc -Zcodegen-backend=target/release/librustc_codegen_tpde.so <files>
```
Use in cargo
```bash
WORKDIR=$(pwd) # replace by this directory
RUSTFLAGS="-Zcodegen-backend=$WORKDIR/target/release/librustc_codegen_tpde.so" cargo build
```
