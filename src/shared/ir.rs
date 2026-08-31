use super::ffi;
pub use super::ffi::ModuleTpde;
use crate::context::CodegenCx;
use core::fmt::{Debug, Formatter};
use rustc_hir::attrs::Linkage;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Function(usize);
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Global(usize);
#[derive(Debug, Copy, Clone)]
pub struct BasicBlock {
    function: Function,
    index: usize,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Slot {
    Value(Function, u32),
    Pair(Function, u32),
    Const(u32),
    CPair(u32),
    Raw(u32),
    Alloc(u32),
    Func(Function),
    Global(Global)
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum FullType {
    Single(Type),
    Pair(Type, Type, u8),
    Memory{sized: bool}
}

pub struct FunctionSignatureRef(usize);
pub struct FunctionSignature {
    pub args: Vec<Type>,
    pub ret: Option<FullType>,
}

pub fn size_of_type(ty: Type) -> u32 {
    match ty {
        Type::Void => 0,
        Type::Bool | Type::i8 => 1,
        Type::i16 => 2,
        Type::i32 => 4,
        Type::i64 => 8,
        Type {
            repr: 6_u8..=u8::MAX,
        } => todo!(),
    }
}

pub use super::ffi::InstructionKind;
pub use super::ffi::Type;

impl ModuleTpde {
    pub fn new() -> Self {
        Self {
            functions: vec![],
            consts: vec![],
            const_pairs: vec![],
            globals: vec![],
            relocations: vec![],
        }
    }

    pub fn add_function<'tpde, 'tcx>(
        self: &mut ModuleTpde,
        cx: &CodegenCx<'tpde, 'tcx>,
        name: &str,
        fn_sign: &FunctionSignature,
        linkage: Linkage,
    ) -> Function {
        self.functions.push(ffi::Function {
            name: name.to_string(),
            n_args: fn_sign.args.len(),
            has_ret: fn_sign.ret.is_some(),
            slots: fn_sign.args.iter().map(|ty| ffi::Slot { ty: *ty }).collect(),
            slot_pairs: vec![],
            flags: Self::create_linker_flags(linkage),
            allocas: vec![],
            basic_blocks: vec![],
        });
        Function(self.functions.len() - 1)
    }

    pub fn add_global(
        self: &mut ModuleTpde,
        name: &str,
        linkage: Linkage
    ) -> Global {
        self.globals.push(ffi::Global {
            name: name.to_string(),
            size: 0,
            align: 0,
            flags: Self::create_linker_flags(linkage),
            chunks: vec![],
            data: vec![],
        });
        Global(self.globals.len() - 1)
    }

    pub fn global_set_align(
        self: &mut ModuleTpde,
        global: Global,
        align: u32
    ) {
        self.globals[global.0].align = align;
    }

    pub fn global_add_unit_chunk(
        &mut self,
        global: Global,
        size: u32
    ) {
        self.globals[global.0].chunks.push(ffi::Chunk {
            type_: ffi::ChunkType::UnInit,
            data: size
        })
    }

    pub fn global_add_init_chunk(
        &mut self,
        global: Global,
        chunk: &[u8]
    ) {
        let global = &mut self.globals[global.0];
        global.data.push(ffi::ChunkData { data: chunk.to_vec() });
        global.chunks.push(ffi::Chunk {
            type_: ffi::ChunkType::Init,
            data: global.data.len() as u32 - 1
        })
    }

    pub fn global_add_reloc_chunk(
        &mut self,
        global: Global,
        address_space: u32,
    ) {
        self.relocations.push(ffi::Relocation {
            address_space
        });
        self.globals[global.0].chunks.push(ffi::Chunk {
            type_: ffi::ChunkType::Reloc,
            data: self.relocations.len() as u32 - 1
        })
    }

    fn create_linker_flags(
        linkage: Linkage
    ) -> ffi::LinkerFlags {
        ffi::LinkerFlags {
            extern_link: linkage == Linkage::AvailableExternally,
            only_local: linkage == Linkage::Internal,
            weak_link: linkage == Linkage::WeakODR
                || linkage == Linkage::WeakAny
                || linkage == Linkage::ExternalWeak,
        }
    }

    pub fn get_slot(&self, func: Function, index: u32) -> Slot {
        Slot::new_val(func, index)
    }

    pub fn type_of_slot(&self, slot: Slot) -> FullType {
        match slot {
            Slot::Value(func, ind) => {
                FullType::Single(self.functions[func.0].slots[ind as usize].ty)
            }
            Slot::Pair(func, ind) => {
                let func = &self.functions[func.0];
                let pair = &func.slot_pairs[ind as usize];
                let FullType::Single(slot_a) = self.type_of_slot(Slot::from_ffi(pair.slot_a))
                else {
                    unreachable!()
                };
                let FullType::Single(slot_b) = self.type_of_slot(Slot::from_ffi(pair.slot_b))
                else {
                    unreachable!()
                };
                FullType::Pair(slot_a, slot_b, pair.offset_b)
            }
            Slot::Const(ind) => FullType::Single(self.consts[ind as usize].ty),
            Slot::CPair(ind) => {
                let pair = &self.const_pairs[ind as usize];
                let FullType::Single(slot_a) = self.type_of_slot(Slot::from_ffi(pair.slot_a))
                else {
                    unreachable!()
                };
                let FullType::Single(slot_b) = self.type_of_slot(Slot::from_ffi(pair.slot_b))
                else {
                    unreachable!()
                };
                FullType::Pair(slot_a, slot_b, pair.offset_b)
            }
            Slot::Alloc(_) => todo!(),
            Slot::Raw(_) => unreachable!(),
            Slot::Func(func) => todo!(),
            Slot::Global(_) => todo!(),
        }
    }

    fn get_function_mut(self: &mut ModuleTpde, func: &Function) -> &mut ffi::Function {
        self.functions
            .get_mut(func.0)
            .unwrap_or_else(|| panic!("Function not found"))
    }

    fn get_function(self: &ModuleTpde, func: &Function) -> &ffi::Function {
        self.functions
            .get(func.0)
            .unwrap_or_else(|| panic!("Function not found"))
    }

    pub fn add_basic_block(self: &mut Self, func: &Function, name: &str) -> BasicBlock {
        let function = self.get_function_mut(func);
        function.basic_blocks.push(ffi::BasicBlock {
            name: name.to_string(),
            instructions: vec![],
            info1: 0,
            info2: 0,
        });
        BasicBlock::new(*func, function.basic_blocks.len() - 1)
    }

    fn get_basic_block_mut_helper(
        func: &mut ffi::Function,
        bb: BasicBlock,
    ) -> &mut ffi::BasicBlock {
        func.basic_blocks
            .get_mut(bb.index())
            .unwrap_or_else(|| panic!("Basic block not found"))
    }

    fn get_basic_block_mut(self: &mut Self, bb: BasicBlock) -> &mut ffi::BasicBlock {
        let function = self.get_function_mut(&bb.function());
        ModuleTpde::get_basic_block_mut_helper(function, bb)
    }

    fn get_basic_block(self: &Self, bb: BasicBlock) -> &ffi::BasicBlock {
        let function = self.get_function(&bb.function());
        function
            .basic_blocks
            .get(bb.index())
            .unwrap_or_else(|| panic!("Basic block not found"))
    }

    #[inline]
    pub fn add_instruction(&mut self, bb: BasicBlock, instr: InstructionKind, ops: Vec<Slot>) {
        self.add_instruction_raw_internal(bb, instr, ops.as_slice(), None);
    }

    #[inline]
    pub fn add_instruction_ret(
        &mut self,
        bb: BasicBlock,
        instr: InstructionKind,
        ops: Vec<Slot>,
        ret: Type,
    ) -> Slot {
        self.add_instruction_raw_internal(bb, instr, ops.as_slice(), Some(FullType::Single(ret))).unwrap()
    }

    #[inline]
    pub fn add_instruction_ret_first(
        &mut self,
        bb: BasicBlock,
        instr: InstructionKind,
        ops: Vec<Slot>,
    ) -> Slot {
        assert!(ops.len() >= 1);

        let func = self.get_function_mut(&bb.function());
        let op = *ops.get(0).unwrap();

        // create new slot with type of first arg
        let ret = match op {
            Slot::Value(f, v) => {
                assert_eq!(bb.function(), f);
                func.slots.get(v as usize).unwrap().ty
            }
            Slot::Const(v) => {
                self.consts.get(v as usize).unwrap().ty
            }
            _ => panic!("First operand of return instruction must be a value slot"),
        };
        self.add_instruction_raw_internal(bb, instr, ops.as_slice(), Some(FullType::Single(ret))).unwrap()
    }

    #[inline]
    fn add_instruction_raw_internal(
        &mut self,
        bb: BasicBlock,
        instr: InstructionKind,
        ops: &[Slot],
        ret: Option<FullType>,
    ) -> Option<Slot> {
        enum ReturnType {
            None,
            Single(Slot),
            Pair(Slot, Slot, Slot),
        }

        impl ReturnType {
            fn num_ret(&self) -> u8 {
                match self {
                    ReturnType::None => 0,
                    ReturnType::Single(_) => 1,
                    ReturnType::Pair(_, _, _) => 2,
                }
            }

            fn result_a(&self) -> u32 {
                match self {
                    ReturnType::None => 0,
                    ReturnType::Single(slot) => slot.to_ffi(),
                    ReturnType::Pair(_, a, _) => a.to_ffi(),
                }
            }

            fn result_b(&self) -> u32 {
                match self {
                    ReturnType::None => 0,
                    ReturnType::Single(_) => 0,
                    ReturnType::Pair(_, _, b) => b.to_ffi(),
                }
            }

            fn result(&self) -> Option<Slot> {
                match self {
                    ReturnType::None => None,
                    ReturnType::Single(slot) => Some(*slot),
                    ReturnType::Pair(slot, _, _) => Some(*slot),
                }
            }
        }

        let ret: ReturnType =
            match ret {
                None => ReturnType::None,
                Some(FullType::Single(ty)) => ReturnType::Single(self.add_slot(bb.function, ty)),
                Some(FullType::Pair(ty_a, ty_b, offset_b)) => {
                    let slot_a = self.add_slot(bb.function, ty_a);
                    let slot_b = self.add_slot(bb.function, ty_b);
                    let pair = self.add_pair(bb.function, slot_a, slot_b, offset_b);
                    ReturnType::Pair(pair, slot_a, slot_b)
                }
                Some(FullType::Memory { .. }) => todo!(),
            };

        let basic_block = self.get_basic_block_mut(bb);

        basic_block.instructions.push(ffi::Instruction {
            kind: instr,
            ops: ops.iter().map(|s| s.to_ffi()).collect(),
            has_result: ret.num_ret() >= 1,
            result: ret.result_a(),
        });
        if ret.num_ret() >= 2 {
            basic_block.instructions.push(ffi::Instruction {
                kind: InstructionKind::AddRet,
                ops: vec![],
                has_result: true,
                result: ret.result_b(),
            });
        }

        ret.result()
    }

    pub fn add_call(
        &mut self,
        bb: BasicBlock,
        func_ref: Slot,
        func_sign: &FunctionSignature,
        args: &[Slot]) -> Option<Slot> {
        let mut ops = Vec::with_capacity(1 + args.len());
        ops.push(func_ref);
        ops.extend_from_slice(args);
        self.add_instruction_raw_internal(
            bb,
            InstructionKind::Call,
            ops.as_slice(),
            func_sign.ret
        )
    }

    pub fn add_alloca(&mut self, func: Function, size: usize, align: usize) -> Slot {
        let func = self.get_function_mut(&func);

        func.allocas.push(ffi::Alloca { size, align });
        Slot::new_alloc((func.allocas.len() - 1) as u32)
    }

    pub fn add_const(&mut self, ty: Type, data: u128) -> Slot {
        let consts = &mut self.consts;
        consts.push(ffi::Value::new(ty, data));
        Slot::new_const((consts.len() - 1) as u32)
    }

    pub fn add_const_pair(
        &mut self,
        slot_a: Slot,
        slot_b: Slot,
        offset_b: u8
    ) -> Slot {
        let const_pairs = &mut self.const_pairs;
        const_pairs.push(ffi::PairRef {
            slot_a: slot_a.to_ffi(),
            slot_b: slot_b.to_ffi(),
            offset_b,
        });
        Slot::new_cpair((const_pairs.len() - 1) as u32)
    }

    pub fn add_const_pair_values(
        &mut self,
        ty_a: Type,
        ty_b: Type,
        offset_b: u8,
        data_a: u128,
        data_b: u128,
    ) -> Slot {
        let slot_a = self.add_const(ty_a, data_a);
        let slot_b = self.add_const(ty_b, data_b);
        self.add_const_pair(slot_a, slot_b, offset_b)
    }

    pub fn extract_vals(&self, pair: Slot) -> (Slot, Slot, u8) {
        let v = match pair {
            Slot::CPair(ind) => self.const_pairs[ind as usize],
            Slot::Pair(func, ind) => self.functions[func.0].slot_pairs[ind as usize],
            _ => panic!("agg_val has to be a pair"),
        };
        (
            Slot::from_ffi(v.slot_a),
            Slot::from_ffi(v.slot_b),
            v.offset_b,
        )
    }

    #[inline]
    pub fn add_slot(&mut self, func: Function, ty: Type) -> Slot {
        let slots = &mut self.functions[func.0].slots;
        slots.push(ffi::Slot { ty });
        Slot::new_val(func, slots.len() as u32 - 1)
    }

    pub fn add_pair(&mut self, func: Function, slot_a: Slot, slot_b: Slot, offset_b: u8) -> Slot {
        let slot_pairs = &mut self.functions[func.0].slot_pairs;
        slot_pairs.push(ffi::PairRef {
            slot_a: slot_a.to_ffi(),
            slot_b: slot_b.to_ffi(),
            offset_b,
        });
        Slot::new_pair(func, (slot_pairs.len() - 1) as u32)
    }

    pub fn add_br(&mut self, bb: BasicBlock, to: BasicBlock) {
        self.add_instruction(
            bb,
            InstructionKind::Br,
            vec![Slot::new_raw(to.index as u32)],
        );
    }

    pub fn add_cond_br(
        &mut self,
        bb: BasicBlock,
        cond: Slot,
        thenbb: BasicBlock,
        elsebb: BasicBlock,
    ) {
        self.add_instruction(
            bb,
            InstructionKind::CondBr,
            vec![
                cond,
                Slot::new_raw(thenbb.index as u32),
                Slot::new_raw(elsebb.index as u32),
            ],
        );
    }
}

type Marker = u32;
pub const MARKER_BLOCK: Marker = 7_u32 << (u32::BITS - 3);

pub const MARKER_VAL: Marker = 0_u32 << (u32::BITS - 3);
pub const MARKER_CONST: Marker = 1_u32 << (u32::BITS - 3);
pub const MARKER_RAW: Marker = 2_u32 << (u32::BITS - 3);
pub const MARKER_PTR: Marker = 3_u32 << (u32::BITS - 3);
pub const MARKER_FUNC: Marker = 4_u32 << (u32::BITS - 3);
pub const MARKER_GLOBAL: Marker = 5_u32 << (u32::BITS - 3);
// pub const MARKER_PAIR: Marker = 2_u32 << (u32::BITS - 3);
// pub const MARKER_CPAIR: Marker = 3_u32 << (u32::BITS - 3);
impl Slot {
    fn new_val(func: Function, index: u32) -> Self {
        Self::Value(func, index)
    }

    fn new_pair(func: Function, index: u32) -> Self {
        Self::Pair(func, index)
    }

    fn new_const(index: u32) -> Self {
        Self::Const(index)
    }

    fn new_cpair(index: u32) -> Self {
        Self::CPair(index)
    }

    fn new_alloc(index: u32) -> Self {
        Self::Alloc(index)
    }

    pub fn new_raw(u: u32) -> Self {
        Self::Raw(u)
    }

    pub fn new_func(func: Function) -> Self {
        Self::Func(func)
    }

    pub fn new_global(global: Global) -> Self {
        Self::Global(global)
    }

    pub fn to_ffi(&self) -> u32 {
        match self {
            Self::Value(_, v) => *v,
            Self::Const(i) => *i | MARKER_CONST,
            Self::Alloc(p) => *p | MARKER_PTR,
            Self::Raw(r) => *r | MARKER_RAW,
            Self::Func(f) => (f.0 as u32) | MARKER_FUNC,
            Self::Global(g) => g.0 as u32 | MARKER_GLOBAL,
            Self::Pair(_, p) =>
                unreachable!("Only used for tracking during generation"),
                // *p | MARKER_PAIR,
            Self::CPair(p) =>
                unreachable!("Only used for tracking during generation"),
                // *p | MARKER_CPAIR,
        }
    }

    #[inline]
    fn is(ffi: u32, marker: Marker) -> Option<u32> {
        if (ffi & MARKER_BLOCK) == marker {
            return Some(ffi & !MARKER_BLOCK);
        }
        None
    }

    pub fn from_ffi(ffi: u32) -> Self {
        if let Some(u) = Self::is(ffi, MARKER_VAL) {
            return Self::Value(Function(0), u);
        }
        if let Some(u) = Self::is(ffi, MARKER_CONST) {
            return Self::Const(u);
        }
        if let Some(u) = Self::is(ffi, MARKER_PTR) {
            return Self::Alloc(u);
        }
        if let Some(u) = Self::is(ffi, MARKER_RAW) {
            return Self::Raw(u);
        }
        if let Some(f) = Self::is(ffi, MARKER_FUNC) {
            return Self::Func(Function(f as usize));
        }
        if let Some(g) = Self::is(ffi, MARKER_GLOBAL) {
            return Self::Global(Global(g as usize));
        }
        unreachable!()
        // if let Some(u) = Self::is(ffi, MARKER_CPAIR) {
        //     return Self::CPair(u);
        // }
        // if let Some(u) = Self::is(ffi, MARKER_PAIR) {
        //     return Self::Pair(Function(0), u);
        // }
    }

    pub fn get_func(&self) -> Option<Function> {
        match self {
            Slot::Value(func, _) => Some(*func),
            Slot::Pair(func, _) => Some(*func),
            _ => None,
        }
    }
}

impl Debug for Slot {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Value(func, v) => write!(f, "[val: {}]", v),
            Self::Alloc(v) => write!(f, "[alloc: {}]", v),
            Self::Raw(v) => write!(f, "[raw: {}]", v),
            Self::Func(v) => write!(f, "[func: {}]", v.0),
            Self::Global(g) => write!(f, "[global: {}]", g.0),
            Self::Const(v) => write!(f, "[const: {}]", v),
            Self::Pair(func, v) =>
                unreachable!("Only used for tracking during generation"),
                // write!(f, "[pair: {}]", v),
            Self::CPair(v) =>
                unreachable!("Only used for tracking during generation"),
                // write!(f, "[cpair: {}]", v),
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

impl Slot {
    pub fn pair_slots(&self, module: &ModuleTpde) -> Option<(Slot, Slot)> {
        let pair = match self {
            Slot::Pair(func, ind) => module.functions[func.0].slot_pairs[*ind as usize],
            Slot::CPair(ind) => module.const_pairs[*ind as usize],
            _ => return None,
        };
        Some((Slot::from_ffi(pair.slot_a), Slot::from_ffi(pair.slot_b)))
    }
}

impl ffi::Value {
    fn new(ty: Type, v: u128) -> Self {
        ffi::Value {
            ty,
            data1: (v >> 64) as u64,
            data2: v as u64,
        }
    }

    pub fn data(&self) -> u128 {
        (self.data1 as u128) << 64 + (self.data2 as u128)
    }
}