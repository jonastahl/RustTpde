#![feature(rustc_private)]
#![allow(unused_variables, dead_code)]

extern crate rustc_codegen_llvm;
extern crate rustc_codegen_ssa;
extern crate rustc_driver;
extern crate rustc_metadata;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;
extern crate rustc_ast;
extern crate rustc_errors;
extern crate rustc_data_structures;
use rustc_codegen_ssa::traits::CodegenBackend;
use crate::codegen::TpdeCodegenBackend;

mod codegen;
mod ir;

#[cxx::bridge]
pub mod cpp {
    pub struct ModuleTpde {
        functions: Vec<Function>,
        basic_blocks: Vec<BasicBlock>,
        instructions: Vec<Instr>
    }

    pub struct Function {
        from: u32,
        to: u32
    }

    pub struct BasicBlock {
        from: u32,
        to: u32
    }

    pub enum Instr {
        Add,
        Sub,
        Mul,
        Div,
    }

    pub struct Ir {
        instr: Vec<Instr>,
        data: Vec<u32>,
    }

    unsafe extern "C++" {
        include!("tpde_cpp/hello_world.h");

        pub fn hello_world() -> String;

        pub fn compile_ir(ir: &Ir) -> u32;
    }
}

#[unsafe(no_mangle)]
pub fn __rustc_codegen_backend() -> Box<dyn CodegenBackend> {
    Box::new(TpdeCodegenBackend::new())
}

#[cfg(test)]
mod tests {
    use crate::cpp::{Instr, Ir};
    use crate::cpp;

    #[test]
    fn verify_cpp_hello_world() {
        let result = cpp::hello_world();

        assert_eq!(result, "Hello, World!");
    }

    #[test]
    fn compile() {
        let result = cpp::compile_ir(&Ir {
            instr: vec![Instr::Add, Instr::Sub, Instr::Mul, Instr::Div],
            data: vec![1, 2],
        });

        assert_eq!(result, 3);
    }
}