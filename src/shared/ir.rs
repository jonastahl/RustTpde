use rustc_hir::attrs::Linkage;
use rustc_middle::ty::Ty;
use rustc_target::callconv::{FnAbi, PassMode};
use crate::context::CodegenCx;
pub use super::ffi::ModuleTpde;
use super::ffi;

#[derive(Debug, Copy, Clone)]
pub struct Function(usize);
#[derive(Debug, Copy, Clone)]
pub struct BasicBlock(pub Function, pub usize);
#[derive(Debug, Copy, Clone)]
pub struct Slot(usize);

pub use super::ffi::Type;
pub use super::ffi::InstructionKind;

impl ModuleTpde {

    pub fn new() -> Self {
        Self {
            functions: vec![],
        }
    }

    pub fn add_function<'tpde, 'tcx>(self: &mut ModuleTpde, cx: &CodegenCx<'tpde, 'tcx>, name: &str, fn_abi: &FnAbi<'tcx, Ty<'tcx>>, linkage: Linkage) -> Function {
        let n_args = fn_abi.args.len();
        let has_ret = !fn_abi.ret.is_ignore();

        // we can ignore variadic arguments
        let args =
            if fn_abi.c_variadic { &fn_abi.args[..fn_abi.fixed_count as usize] } else { &fn_abi.args };
        let mut slots: Vec<Type> = args.iter().map(|arg| {
            match &fn_abi.ret.mode {
                PassMode::Ignore => Type::Void,
                PassMode::Direct(_) => cx.tpde_type(arg.layout),
                PassMode::Pair(..) => todo!(),
                PassMode::Cast { cast, pad_i32: _ } => todo!(),
                PassMode::Indirect { .. } => {
                    todo!()
                }
            }
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
            basic_blocks: vec![],
        });
        Function(self.functions.len() - 1)
    }

    pub fn get_slot(&self, index: usize) -> Slot {
        Slot(index)
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
            instructions: vec![]
        });
        BasicBlock(*func, function.basic_blocks.len() - 1)
    }

    fn get_basic_block_mut_helper<'a>(func: &'a mut ffi::Function, bb: BasicBlock) -> &'a mut ffi::BasicBlock {
        func
            .basic_blocks.get_mut(bb.1)
            .unwrap_or_else(|| panic!("Basic block not found"))
    }

    fn get_basic_block_mut(self: &mut Self, bb: BasicBlock) -> &mut ffi::BasicBlock {
        let function = self.get_function_mut(&bb.0);
        ModuleTpde::get_basic_block_mut_helper(function, bb)
    }

    fn get_basic_block(self: &Self, bb: BasicBlock) -> &ffi::BasicBlock {
        let function = self.get_function(&bb.0);
        function
            .basic_blocks.get(bb.1)
            .unwrap_or_else(|| panic!("Basic block not found"))
    }

    pub fn add_instruction_op_binary(&mut self, bb: BasicBlock, instr: InstructionKind, lhs: Slot, rhs: Slot) -> Slot {
        let func = self.get_function_mut(&bb.0);

        // create new slot with type of lhs
        func.slots.push(*func.slots.get(lhs.0).unwrap());
        let slot = func.slots.len() - 1;

        let basic_block = ModuleTpde::get_basic_block_mut_helper(func, bb);

        basic_block.instructions.push(ffi::Instruction {
            kind: instr,
            slot,
            lhs: lhs.0,
            rhs: rhs.0
        });

        Slot(slot)
    }

    pub fn add_instruction_statement(&mut self, bb: BasicBlock, instr: InstructionKind, slot: Slot) {
        let basic_block = self.get_basic_block_mut(bb);

        basic_block.instructions.push(ffi::Instruction {
            kind: instr,
            slot: 67,
            lhs: slot.0,
            rhs: 67
        });
    }
}


impl PartialEq for Slot {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}