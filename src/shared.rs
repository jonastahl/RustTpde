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
        const_pairs: Vec<PairRef>,
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
        slot_pairs: Vec<PairRef>,

        extern_link: bool,
        only_local: bool,
        weak_link: bool,

        allocas: Vec<Alloca>,
        basic_blocks: Vec<BasicBlock>,
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

    #[derive(Copy, Clone)]
    pub struct PairRef {
        slot_a: u32,
        slot_b: u32,
        offset_b: u8,
    }

    #[derive(Debug, Copy, Clone)]
    pub enum Type {
        Void,
        Bool,
        i8,
        i16,
        i32,
        i64,

        Poison, // Represents that type is not known and cannot be used
    }

    #[derive(Debug)]
    pub enum InstructionKind {
        // Math
        Add,
        Sub,
        Mul,
        Div,

        CMPeq,
        CMPne,
        CMPgt,
        CMPge,
        CMPlt,
        CMPle,

        // Storage operations
        Alloca,
        Store,
        Load,

        // Branching operations
        Ret,
        Br,
        CondBr,

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
            _ => todo!(),
        };
        write!(
            f,
            "[{:#?}] ({} {}) -> {}",
            self.ty, self.data1, self.data2, val
        )
    }
}

impl Debug for ffi::PairRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "(a: {:?}, b: {:?}, offset: {:?})",
            Slot::from_ffi(self.slot_a),
            Slot::from_ffi(self.slot_b),
            self.offset_b
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
