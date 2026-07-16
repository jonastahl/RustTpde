
fn main() {
    let dst = cmake::Config::new(".").build();

    println!("cargo:rustc-link-search=native={}", dst.join("lib").display());
    println!("cargo:rustc-link-search=native={}", dst.join("lib64").display());

    println!("cargo:rustc-link-lib=static=tpde_cpp");
    println!("cargo:rustc-link-lib=static=tpde");

    cxx_build::bridge("src/lib.rs")
        .include(".")
        .include("deps/tpde/tpde/include")
        .flag_if_supported("-std=c++23")
        .compile("rustc_codegen_tpde");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=tpde_cpp");
    println!("cargo:rerun-if-changed=CMakeLists.txt");
}