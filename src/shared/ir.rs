use core::fmt::{Debug, Formatter};
use crate::context::CodegenCx;
use rustc_hir::attrs::Linkage;
use rustc_middle::ty::Ty;
use rustc_target::callconv::{FnAbi, PassMode};
pub use super::ffi::ModuleTpde;
use super::ffi;

#[derive(Debug, Copy, Clone)]
pub struct Function(usize);
#[derive(Debug, Copy, Clone)]
pub struct BasicBlock {
    function: Function,
    index: usize,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Slot {
    Value(u32),
    Ptr(u32),
    Immediate(u32),
    Raw(u32)
}

pub use super::ffi::Type;
pub use super::ffi::InstructionKind;

impl ModuleTpde {

    pub fn new() -> Self {
        Self {
            functions: vec![],
            immediates: vec![]
        }
    }

    pub fn add_function<'tpde, 'tcx>(self: &mut ModuleTpde, cx: &CodegenCx<'tpde, 'tcx>, name: &str, fn_abi: &FnAbi<'tcx, Ty<'tcx>>, linkage: Linkage) -> Function {
        let n_args = fn_abi.args.len();
        let has_ret = !fn_abi.ret.is_ignore();

        // we can ignore variadic arguments
        let args =
            if fn_abi.c_variadic { &fn_abi.args[..fn_abi.fixed_count as usize] } else { &fn_abi.args };
        let slots: Vec<ffi::Slot> = args.iter().map(|arg| {
            let ty = match &fn_abi.ret.mode {
                PassMode::Ignore => Type::Void,
                PassMode::Direct(_) => cx.tpde_type(arg.layout),
                PassMode::Pair(..) => todo!(),
                PassMode::Cast { cast, pad_i32: _ } => todo!(),
                PassMode::Indirect { .. } => {
                    todo!()
                }
            };
            ffi::Slot { ty }
        }).collect();

        {
            let return_ty: Type = match &fn_abi.ret.mode {
                PassMode::Ignore => Type::Void,
                PassMode::Direct(_) => cx.tpde_type(fn_abi.ret.layout),
                PassMode::Pair(..) => todo!(),
                PassMode::Cast { cast, pad_i32: _ } => todo!(),
                PassMode::Indirect { .. } => {
                    todo!()
                }
            };
        }

        self.functions.push(ffi::Function {
            name: name.to_string(),
            n_args,
            has_ret,
            slots,
            extern_link: linkage == Linkage::AvailableExternally,
            only_local: linkage == Linkage::Internal,
            weak_link: linkage == Linkage::WeakODR
                || linkage == Linkage::WeakAny
                || linkage == Linkage::ExternalWeak,
            allocas: vec![],
            basic_blocks: vec![],
        });
        Function(self.functions.len() - 1)
    }

    pub fn get_slot(&self, index: u32) -> Slot {
        Slot::new_val(index)
    }

    fn get_function_mut(self: &mut ModuleTpde, func: &Function) -> &mut ffi::Function {
        self.functions.get_mut(func.0)
            .unwrap_or_else(|| panic!("Function not found"))
    }

    fn get_function(self: &ModuleTpde, func: &Function) -> &ffi::Function {
        self.functions.get(func.0)
            .unwrap_or_else(|| panic!("Function not found"))
    }

    pub fn add_basic_block(self: &mut Self, func: &Function, name: &str) -> BasicBlock {
        let function = self.get_function_mut(func);
        function
            .basic_blocks.push(ffi::BasicBlock {
            name: name.to_string(),
            instructions: vec![],
            info1: 0,
            info2: 0,
        });
        BasicBlock::new(*func, function.basic_blocks.len() - 1)
    }

    fn get_basic_block_mut_helper<'a>(func: &'a mut ffi::Function, bb: BasicBlock) -> &'a mut ffi::BasicBlock {
        func
            .basic_blocks.get_mut(bb.index())
            .unwrap_or_else(|| panic!("Basic block not found"))
    }

    fn get_basic_block_mut(self: &mut Self, bb: BasicBlock) -> &mut ffi::BasicBlock {
        let function = self.get_function_mut(&bb.function());
        ModuleTpde::get_basic_block_mut_helper(function, bb)
    }

    fn get_basic_block(self: &Self, bb: BasicBlock) -> &ffi::BasicBlock {
        let function = self.get_function(&bb.function());
        function
            .basic_blocks.get(bb.index())
            .unwrap_or_else(|| panic!("Basic block not found"))
    }

    pub fn add_instruction_ret(&mut self, bb: BasicBlock, instr: InstructionKind, ops: Vec<Slot>) -> Slot {
        assert!(ops.len() >= 1);

        let func = self.get_function_mut(&bb.function());
        let op = ops.get(0).unwrap();

        // create new slot with type of first arg
        if let &Slot::Value(v) = op {
            let ret = func.slots.get(v as usize).unwrap().ty;
            self.add_instruction_raw(bb, instr, ops, Some(ret))
        } else {
            panic!("First operand of return instruction must be a value slot");
        }
    }

    pub fn add_instruction(&mut self, bb: BasicBlock, instr: InstructionKind, ops: Vec<Slot>) {
        self.add_instruction_raw(bb, instr, ops, None);
    }

    #[inline]
    pub fn add_instruction_raw(&mut self, bb: BasicBlock, instr: InstructionKind, ops: Vec<Slot>, ret: Option<Type>) -> Slot {
        let func = self.get_function_mut(&bb.function());

        if let Some(ty) = ret {
            func.slots.push(ffi::Slot { ty });
        }
        let result = (func.slots.len() - 1) as u32;

        let basic_block = ModuleTpde::get_basic_block_mut_helper(func, bb);

        basic_block.instructions.push(ffi::Instruction {
            kind: instr,
            ops: ops.iter().map(|s| s.to_ffi()).collect(),
            has_result: ret.is_some(),
            result
        });

        Slot::new_val(result as u32)
    }

    pub fn add_alloca(&mut self, func: Function, size: usize, align: usize) -> Slot {
        let func = self.get_function_mut(&func);

        func.allocas.push(ffi::Alloca{size, align});
        Slot::new_ptr(func.allocas.len() as u32)
    }

    pub fn add_immediate(&mut self, ty: Type, data: u128) -> Slot {
        self.immediates.push(ffi::Value::new(ty, data));
        Slot::new_imm((self.immediates.len() - 1) as u32)
    }

    pub fn add_br(&mut self, bb: BasicBlock, to: BasicBlock) {
        self.add_instruction_raw(
            bb,
            InstructionKind::Br,
            vec![Slot::new_raw(to.index as u32)],
            None
        );
    }

    pub fn add_cond_br(&mut self, bb: BasicBlock, cond: Slot, thenbb: BasicBlock, elsebb: BasicBlock) {
        self.add_instruction_raw(
            bb,
            InstructionKind::CondBr,
            vec![cond, Slot::new_raw(thenbb.index as u32), Slot::new_raw(elsebb.index as u32)],
            None
        );
    }
}

impl Slot {
    fn new_val(index: u32) -> Slot {
        Slot::Value(index)
    }

    fn new_ptr(index: u32) -> Slot {
        Slot::Ptr(index)
    }

    fn new_imm(index: u32) -> Slot {
        Slot::Immediate(index)
    }

    pub fn new_raw(u: u32) -> Slot { Slot::Raw(u) }

    pub const MARKER_IMM: u32 = 1_u32 << (u32::BITS - 1);
    pub const MARKER_PTR: u32 = 1_u32 << (u32::BITS - 2);
    pub const MARKER_RAW: u32 = Self::MARKER_IMM | Self::MARKER_PTR;

    pub fn to_ffi(&self) -> u32 {
        match self {
            Slot::Value(v) => *v,
            Slot::Immediate(i) => *i | Self::MARKER_IMM,
            Slot::Ptr(p) => *p | Self::MARKER_PTR,
            Slot::Raw(r) => *r | Self::MARKER_RAW
        }
    }

    pub fn from_ffi(ffi: u32) -> Slot {
        if ffi & Self::MARKER_RAW != 0 {
            if ffi & Self::MARKER_PTR == 0 {
                return Slot::Immediate(ffi & !Self::MARKER_IMM);
            }
            if ffi & Self::MARKER_IMM == 0 {
                return Slot::Ptr(ffi & !Self::MARKER_PTR);
            }

            return Slot::Raw(ffi & !Self::MARKER_RAW);
        }
        Slot::Value(ffi)
    }
}

impl Debug for Slot {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Slot::Value(v) => write!(f, "[val: {}]", v),
            Slot::Ptr(v) => write!(f, "[ptr: {}]", v),
            Slot::Immediate(v) => write!(f, "[imm: {}]", v),
            Slot::Raw(v) => write!(f, "[raw: {}]", v),
        }
    }
}

impl BasicBlock {
    fn new(function: Function, index: usize) -> BasicBlock {
        BasicBlock { function, index }
    }

    pub fn function(&self) -> Function {
        self.function
    }

    fn index(&self) -> usize {
        self.index
    }
}

impl ffi::Value {
    fn new(ty: Type, v: u128) -> Self {
        ffi::Value {
            ty, data1: (v >> 64) as u64, data2: v as u64
        }
    }

    pub fn data(&self) -> u128 {
        (self.data1 as u128) << 64 + (self.data2 as u128)
    }
}