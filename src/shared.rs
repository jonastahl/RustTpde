use crate::shared::ffi::Type;
use crate::shared::ir::Slot;
use core::fmt::{Debug, Formatter};
#[allow(unused_imports)]
pub use ffi::compile_to_file;

pub mod ir;

#[cxx::bridge]
mod ffi {

    #[derive(Debug)]
    pub struct ModuleTpde {
        functions: Vec<Function>,
        consts: Vec<Value>,
        globals: Vec<Global>,
        relocations: Vec<Relocation>,
    }

    enum Linkage {
        External,
    }

    #[derive(Debug)]
    pub struct Function {
        name: String,
        n_args: usize,
        has_ret: bool,
        slots: Vec<Slot>,

        flags: LinkerFlags,

        allocas: Vec<Alloca>,
        basic_blocks: Vec<BasicBlock>,
    }

    #[derive(Debug)]
    pub struct LinkerFlags {
        extern_link: bool,
        only_local: bool,
        weak_link: bool,
    }

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

    #[derive(Copy, Clone)]
    pub struct Slot {
        ty: Type,
    }

    #[derive(Debug, Copy, Clone)]
    pub enum Type {
        Void,
        Bool,
        i8,
        i16,
        i32,
        i64,
        i128,
        ptr
    }

    #[derive(Debug)]
    pub enum InstructionKind {
        // Math
        Add,
        Sub,
        Mul,
        Div,

        // Bitwise
        And,
        Or,
        Shl,

        CMPeq,
        CMPne,
        CMPsgt,
        CMPsge,
        CMPslt,
        CMPsle,
        CMPugt,
        CMPuge,
        CMPult,
        CMPule,

        // Storage operations
        Alloca,
        Store,
        Load,
        GEP,
        MemCpy,

        // Branching operations
        Ret,
        Br,
        CondBr,

        // Calls
        Call,

        // Casts
        Cast,
        Zext,

        // Pair return type
        AddRet,

        Last,
    }

    pub struct Instruction {
        kind: InstructionKind,
        ops: Vec<u32>,
        has_result: bool,
        result: u32,
    }

    pub struct Alloca {
        size: usize,
        align: usize,
    }

    pub struct Value {
        ty: Type,
        data1: u64,
        data2: u64,
    }

    #[derive(Debug)]
    pub struct Global {
        name: String,

        align: u32,
        // mutable: bool,

        flags: LinkerFlags,

        size: u32,
        chunks: Vec<Chunk>,
        data: Vec<ChunkData>,
    }

    #[derive(Debug)]
    enum ChunkType {
        Init,
        UnInit,
        Reloc,
    }

    #[derive(Debug)]
    pub struct Chunk {
        type_: ChunkType,
        // position for Init and Reloc, size for UnInit
        data: u32,
    }

    #[derive(Debug)]
    pub struct ChunkData {
        data: Vec<u8>,
    }

    #[derive(Debug)]
    pub struct Relocation {
        address_space: u32,
    }
}

impl Debug for ffi::BasicBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}: ", self.name)?;
        f.debug_list().entries(&self.instructions).finish()
    }
}

impl Debug for ffi::Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "<{:?}> (", self.kind)?;
        let mut first = true;
        for &op in &self.ops {
            if !first {
                write!(f, ", ")?;
            }
            first = false;
            write!(f, "{:?}", Slot::from_ffi(op))?;
        }
        write!(f, ")")?;

        if self.has_result {
            write!(f, " -> {:?}", Slot::from_ffi(self.result))?;
        }

        Ok(())
    }
}

impl Debug for ffi::Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let val = match self.ty {
            Type::i8 => format!("{}", self.data1 as i8),
            Type::i16 => format!("{}", self.data1 as i16),
            Type::i32 => format!("{}", self.data1 as i32),
            Type::i64 => format!("{}", self.data1 as i64),
            Type::i128 => format!("{}", (self.data1 as i128) << 64 | (self.data1 as i128)),
            Type::ptr => format!("[ptr: {}]", self.data1),
            _ => todo!(),
        };
        write!(
            f,
            "[{:#?}] ({} {}) -> {}",
            self.ty, self.data1, self.data2, val
        )
    }
}

impl Debug for ffi::Slot {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self.ty)
    }
}

impl Debug for ffi::Alloca {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "size: {:?}, align: {:?}", self.size, self.align)
    }
}
