#![feature(rustc_private)]
#![allow(unused_variables, dead_code)]

extern crate rustc_codegen_llvm;
extern crate rustc_codegen_ssa;
extern crate rustc_driver;
extern crate rustc_metadata;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;
use rustc_codegen_ssa::traits::CodegenBackend;
use crate::codegen::TpdeCodegenBackend;

mod codegen;

#[unsafe(no_mangle)]
pub fn __rustc_codegen_backend() -> Box<dyn CodegenBackend> {
    TpdeCodegenBackend::new()
}

#[cxx::bridge]
pub mod ffi {
    enum Instr {
        Add,
        Sub,
        Mul,
        Div,
    }

    struct Ir {
        instr: Vec<Instr>,
        data: Vec<u32>,
    }

    unsafe extern  "C++" {
        include!("tpde_cpp/hello_world.h");

        pub fn hello_world() -> String;

        pub fn compile_ir(ir: &Ir) -> u32;
    }
}

#[cfg(test)]
mod tests {
    // Import the ffi module from the parent scope (lib.rs)
    use super::ffi;

    #[test]
    fn verify_cpp_hello_world() {
        // Call the C++ function
        let result = ffi::hello_world();

        assert_eq!(result, "Hello, World!");
    }
}