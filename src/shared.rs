#[allow(unused_imports)]
pub use ffi::compile_to_file;

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
        slots: Vec<Slot>,

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

        info1: u32,
        info2: u32,
    }

    unsafe extern "C++" {
        include!("tpde_cpp/tpde.h");

        pub fn compile_to_file(module: &mut ModuleTpde, path: &str) -> u32;
    }

    #[derive(Debug, Copy, Clone)]
    pub struct Slot {
        ty: Type
    }

    #[derive(Debug, Copy, Clone)]
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

        // TODO move to branching instructions
        Ret,
    }

    #[derive(Debug)]
    pub struct Instruction {
        kind: InstructionKind,
        ops: Vec<usize>,
        has_result: bool,
        result: usize,
    }
}