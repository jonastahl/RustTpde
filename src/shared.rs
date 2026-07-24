
#[allow(unused_imports)]
pub use ffi::compile_ir;

pub mod ir {
    pub use super::ffi::{BasicBlock, Function, Instr, ModuleTpde};
}

#[cxx::bridge]
mod ffi {
    pub struct ModuleTpde {
        functions: Vec<Function>,
    }

    pub struct Function {
        name: String,
        basic_blocks: Vec<BasicBlock>,
    }

    pub struct BasicBlock {
        instructions: Vec<Instr>
        // TODO add phis and similar
    }

    pub enum Instr {
        Add,
        Sub,
        Mul,
        Div,
    }

    unsafe extern "C++" {
        include!("tpde_cpp/tpde.h");

        pub fn compile_ir(module: &ModuleTpde) -> u32;
    }
}

mod impls {
    use crate::shared::ir::*;

    impl ModuleTpde {
        pub fn new() -> Self {
            Self {
                functions: vec![],
            }
        }

        pub fn add_function(self: &mut Self, name: String) -> &mut Function {
            self.functions.push_mut(Function {
                name,
                basic_blocks: vec![],
            })
        }
    }

    impl Function {
        pub fn add_basic_block(self: &mut Self) -> &mut BasicBlock {
            self.basic_blocks.push_mut(BasicBlock {
                instructions: vec![],
            })
        }
    }

    impl BasicBlock {
        pub fn add_instruction(self: &mut Self, instr: Instr) -> &mut Instr {
            self.instructions.push_mut(instr)
        }
    }
}