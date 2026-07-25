#[allow(unused_imports)]
pub use ffi::compile_ir;

pub mod ir;

#[cxx::bridge]
mod ffi {
    #[derive(Debug)]
    pub struct ModuleTpde {
        functions: Vec<Function>,
    }

    enum Linkage {
        External
    }

    #[derive(Debug)]
    pub struct Function {
        name: String,
        n_args: usize,
        has_ret: bool,
        slots: Vec<Type>,

        extern_link: bool,
        only_local: bool,
        weak_link: bool,

        basic_blocks: Vec<BasicBlock>,
    }

    #[derive(Debug)]
    pub struct BasicBlock {
        name: String,
        instructions: Vec<Instruction>,
        // TODO add phis and similar
    }

    unsafe extern "C++" {
        include!("tpde_cpp/tpde.h");

        pub fn compile_ir(module: &ModuleTpde) -> u32;
    }

    #[derive(Debug)]
    pub enum Type {
        Void,
        Bool,
        i8,
        i16,
        i32,
        i64,
    }

    #[derive(Debug)]
    pub enum InstructionKind {
        Add,
        Sub,
        Mul,
        Div,

        Ret,
        RetVoid,
    }

    #[derive(Debug)]
    pub struct Instruction {
        kind: InstructionKind,
        slot: usize,
        lhs: usize,
        rhs: usize,
    }
}