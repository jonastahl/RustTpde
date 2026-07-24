mod coverageinfo;
mod intrinsic;

use crate::context::CodegenCx;
use rustc_ast::expand::typetree::FncTree;
use rustc_codegen_ssa::common::{AtomicRmwBinOp, IntPredicate, RealPredicate, SynchronizationScope};
use rustc_codegen_ssa::mir::operand::OperandRef;
use rustc_codegen_ssa::mir::place::PlaceRef;
use rustc_codegen_ssa::traits::{BackendTypes, BuilderMethods, OverflowOp};
use rustc_codegen_ssa::MemFlags;
use rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrs;
use rustc_middle::ty::layout::TyAndLayout;
use rustc_middle::ty::{AtomicOrdering, Instance, Ty};
use rustc_span::Span;
use std::ops::Deref;

pub struct Builder<'a, 'tcx> {
    pub cx: &'a CodegenCx<'tcx>
}

impl<'a, 'tcx> BackendTypes for Builder<'a, 'tcx> {
    type Function = <CodegenCx<'tcx> as BackendTypes>::Function;
    type BasicBlock = <CodegenCx<'tcx> as BackendTypes>::BasicBlock;
    type Funclet = <CodegenCx<'tcx> as BackendTypes>::Funclet;

    type Value = <CodegenCx<'tcx> as BackendTypes>::Value;
    type Type = <CodegenCx<'tcx> as BackendTypes>::Type;
    type FunctionSignature = <CodegenCx<'tcx> as BackendTypes>::FunctionSignature;

    type DIScope = <CodegenCx<'tcx> as BackendTypes>::DIScope;
    type DILocation = <CodegenCx<'tcx> as BackendTypes>::DILocation;
    type DIVariable = <CodegenCx<'tcx> as BackendTypes>::DIVariable;
}

impl<'tcx> BackendTypes for CodegenCx<'tcx> {
    type Function = ();
    type BasicBlock = ();
    type Funclet = ();
    type Value = ();
    type Type = ();
    type FunctionSignature = ();
    type DIScope = ();
    type DILocation = ();
    type DIVariable = ();
}

impl<'tcx> Deref for Builder<'_, 'tcx> {
    type Target = CodegenCx<'tcx>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.cx
    }
}

impl<'a, 'tpde, 'tcx> BuilderMethods<'a, 'tcx> for Builder<'a, 'tcx> {
    type CodegenCx = CodegenCx<'tcx>;

    fn build(cx: &'a Self::CodegenCx, llbb: Self::BasicBlock) -> Self {
        todo!()
    }

    fn cx(&self) -> &Self::CodegenCx {
        todo!()
    }

    fn llbb(&self) -> Self::BasicBlock {
        todo!()
    }

    fn set_span(&mut self, span: Span) {
        todo!()
    }

    fn append_block(cx: &'a Self::CodegenCx, llfn: Self::Function, name: &str) -> Self::BasicBlock {
        todo!()
    }

    fn append_sibling_block(&mut self, name: &str) -> Self::BasicBlock {
        todo!()
    }

    fn switch_to_block(&mut self, llbb: Self::BasicBlock) {
        todo!()
    }

    fn ret_void(&mut self) {
        todo!()
    }

    fn ret(&mut self, v: Self::Value) {
        todo!()
    }

    fn br(&mut self, dest: Self::BasicBlock) {
        todo!()
    }

    fn cond_br(&mut self, cond: Self::Value, then_llbb: Self::BasicBlock, else_llbb: Self::BasicBlock) {
        todo!()
    }

    fn switch(&mut self, v: Self::Value, else_llbb: Self::BasicBlock, cases: impl ExactSizeIterator<Item=(u128, Self::BasicBlock)>) {
        todo!()
    }

    fn invoke(&mut self, llty: Self::FunctionSignature, fn_attrs: Option<&CodegenFnAttrs>, fn_abi: Option<&rustc_target::callconv::FnAbi<'tcx, Ty<'tcx>>>, llfn: Self::Value, args: &[Self::Value], then: Self::BasicBlock, catch: Self::BasicBlock, funclet: Option<&Self::Funclet>, instance: Option<Instance<'tcx>>) -> Self::Value {
        todo!()
    }

    fn unreachable(&mut self) {
        todo!()
    }

    fn add(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn fadd(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn fadd_fast(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn fadd_algebraic(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn sub(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn fsub(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn fsub_fast(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn fsub_algebraic(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn mul(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn fmul(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn fmul_fast(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn fmul_algebraic(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn udiv(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn exactudiv(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn sdiv(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn exactsdiv(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn fdiv(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn fdiv_fast(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn fdiv_algebraic(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn urem(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn srem(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn frem(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn frem_fast(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn frem_algebraic(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn shl(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn lshr(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn ashr(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn and(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn or(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn xor(&mut self, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn neg(&mut self, v: Self::Value) -> Self::Value {
        todo!()
    }

    fn fneg(&mut self, v: Self::Value) -> Self::Value {
        todo!()
    }

    fn not(&mut self, v: Self::Value) -> Self::Value {
        todo!()
    }

    fn checked_binop(&mut self, oop: OverflowOp, ty: Ty<'tcx>, lhs: Self::Value, rhs: Self::Value) -> (Self::Value, Self::Value) {
        todo!()
    }

    fn from_immediate(&mut self, val: Self::Value) -> Self::Value {
        todo!()
    }

    fn to_immediate_scalar(&mut self, val: Self::Value, scalar: rustc_abi::Scalar) -> Self::Value {
        todo!()
    }

    fn alloca(&mut self, size: rustc_abi::Size, align: rustc_abi::Align) -> Self::Value {
        todo!()
    }

    fn alloca_with_ty(&mut self, layout: TyAndLayout<'tcx>) -> Self::Value {
        todo!()
    }

    fn load(&mut self, ty: Self::Type, ptr: Self::Value, align: rustc_abi::Align) -> Self::Value {
        todo!()
    }

    fn volatile_load(&mut self, ty: Self::Type, ptr: Self::Value, align: rustc_abi::Align) -> Self::Value {
        todo!()
    }

    fn atomic_load(&mut self, ty: Self::Type, ptr: Self::Value, order: AtomicOrdering, size: rustc_abi::Size) -> Self::Value {
        todo!()
    }

    fn load_operand(&mut self, place: PlaceRef<'tcx, Self::Value>) -> OperandRef<'tcx, Self::Value> {
        todo!()
    }

    fn write_operand_repeatedly(&mut self, elem: OperandRef<'tcx, Self::Value>, count: u64, dest: PlaceRef<'tcx, Self::Value>) {
        todo!()
    }

    fn range_metadata(&mut self, load: Self::Value, range: rustc_abi::WrappingRange) {
        todo!()
    }

    fn nonnull_metadata(&mut self, load: Self::Value) {
        todo!()
    }

    fn store(&mut self, val: Self::Value, ptr: Self::Value, align: rustc_abi::Align) -> Self::Value {
        todo!()
    }

    fn store_with_flags(&mut self, val: Self::Value, ptr: Self::Value, align: rustc_abi::Align, flags: MemFlags) -> Self::Value {
        todo!()
    }

    fn atomic_store(&mut self, val: Self::Value, ptr: Self::Value, order: AtomicOrdering, size: rustc_abi::Size) {
        todo!()
    }

    fn gep(&mut self, ty: Self::Type, ptr: Self::Value, indices: &[Self::Value]) -> Self::Value {
        todo!()
    }

    fn inbounds_gep(&mut self, ty: Self::Type, ptr: Self::Value, indices: &[Self::Value]) -> Self::Value {
        todo!()
    }

    fn trunc(&mut self, val: Self::Value, dest_ty: Self::Type) -> Self::Value {
        todo!()
    }

    fn sext(&mut self, val: Self::Value, dest_ty: Self::Type) -> Self::Value {
        todo!()
    }

    fn fptoui_sat(&mut self, val: Self::Value, dest_ty: Self::Type) -> Self::Value {
        todo!()
    }

    fn fptosi_sat(&mut self, val: Self::Value, dest_ty: Self::Type) -> Self::Value {
        todo!()
    }

    fn fptoui(&mut self, val: Self::Value, dest_ty: Self::Type) -> Self::Value {
        todo!()
    }

    fn fptosi(&mut self, val: Self::Value, dest_ty: Self::Type) -> Self::Value {
        todo!()
    }

    fn uitofp(&mut self, val: Self::Value, dest_ty: Self::Type) -> Self::Value {
        todo!()
    }

    fn sitofp(&mut self, val: Self::Value, dest_ty: Self::Type) -> Self::Value {
        todo!()
    }

    fn fptrunc(&mut self, val: Self::Value, dest_ty: Self::Type) -> Self::Value {
        todo!()
    }

    fn fpext(&mut self, val: Self::Value, dest_ty: Self::Type) -> Self::Value {
        todo!()
    }

    fn ptrtoint(&mut self, val: Self::Value, dest_ty: Self::Type) -> Self::Value {
        todo!()
    }

    fn inttoptr(&mut self, val: Self::Value, dest_ty: Self::Type) -> Self::Value {
        todo!()
    }

    fn bitcast(&mut self, val: Self::Value, dest_ty: Self::Type) -> Self::Value {
        todo!()
    }

    fn intcast(&mut self, val: Self::Value, dest_ty: Self::Type, is_signed: bool) -> Self::Value {
        todo!()
    }

    fn pointercast(&mut self, val: Self::Value, dest_ty: Self::Type) -> Self::Value {
        todo!()
    }

    fn icmp(&mut self, op: IntPredicate, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn fcmp(&mut self, op: RealPredicate, lhs: Self::Value, rhs: Self::Value) -> Self::Value {
        todo!()
    }

    fn memcpy(&mut self, dst: Self::Value, dst_align: rustc_abi::Align, src: Self::Value, src_align: rustc_abi::Align, size: Self::Value, flags: MemFlags, tt: Option<FncTree>) {
        todo!()
    }

    fn memmove(&mut self, dst: Self::Value, dst_align: rustc_abi::Align, src: Self::Value, src_align: rustc_abi::Align, size: Self::Value, flags: MemFlags) {
        todo!()
    }

    fn memset(&mut self, ptr: Self::Value, fill_byte: Self::Value, size: Self::Value, align: rustc_abi::Align, flags: MemFlags) {
        todo!()
    }

    fn vscale(&mut self, ty: Self::Type) -> Self::Value {
        todo!()
    }

    fn select(&mut self, cond: Self::Value, then_val: Self::Value, else_val: Self::Value) -> Self::Value {
        todo!()
    }

    fn va_arg(&mut self, list: Self::Value, ty: Self::Type) -> Self::Value {
        todo!()
    }

    fn extract_element(&mut self, vec: Self::Value, idx: Self::Value) -> Self::Value {
        todo!()
    }

    fn vector_splat(&mut self, num_elts: usize, elt: Self::Value) -> Self::Value {
        todo!()
    }

    fn extract_value(&mut self, agg_val: Self::Value, idx: u64) -> Self::Value {
        todo!()
    }

    fn insert_value(&mut self, agg_val: Self::Value, elt: Self::Value, idx: u64) -> Self::Value {
        todo!()
    }

    fn set_personality_fn(&mut self, personality: Self::Function) {
        todo!()
    }

    fn cleanup_landing_pad(&mut self, pers_fn: Self::Function) -> (Self::Value, Self::Value) {
        todo!()
    }

    fn filter_landing_pad(&mut self, pers_fn: Self::Function) {
        todo!()
    }

    fn resume(&mut self, exn0: Self::Value, exn1: Self::Value) {
        todo!()
    }

    fn cleanup_pad(&mut self, parent: Option<Self::Value>, args: &[Self::Value]) -> Self::Funclet {
        todo!()
    }

    fn cleanup_ret(&mut self, funclet: &Self::Funclet, unwind: Option<Self::BasicBlock>) {
        todo!()
    }

    fn catch_pad(&mut self, parent: Self::Value, args: &[Self::Value]) -> Self::Funclet {
        todo!()
    }

    fn catch_switch(&mut self, parent: Option<Self::Value>, unwind: Option<Self::BasicBlock>, handlers: &[Self::BasicBlock]) -> Self::Value {
        todo!()
    }

    fn get_funclet_cleanuppad(&self, funclet: &Self::Funclet) -> Self::Value {
        todo!()
    }

    fn atomic_cmpxchg(&mut self, dst: Self::Value, cmp: Self::Value, src: Self::Value, order: AtomicOrdering, failure_order: AtomicOrdering, weak: bool) -> (Self::Value, Self::Value) {
        todo!()
    }

    fn atomic_rmw(&mut self, op: AtomicRmwBinOp, dst: Self::Value, src: Self::Value, order: AtomicOrdering, ret_ptr: bool) -> Self::Value {
        todo!()
    }

    fn atomic_fence(&mut self, order: AtomicOrdering, scope: SynchronizationScope) {
        todo!()
    }

    fn set_invariant_load(&mut self, load: Self::Value) {
        todo!()
    }

    fn lifetime_start(&mut self, ptr: Self::Value, size: rustc_abi::Size) {
        todo!()
    }

    fn lifetime_end(&mut self, ptr: Self::Value, size: rustc_abi::Size) {
        todo!()
    }

    fn call(&mut self, llty: Self::FunctionSignature, caller_attrs: Option<&CodegenFnAttrs>, fn_abi: Option<&rustc_target::callconv::FnAbi<'tcx, Ty<'tcx>>>, fn_val: Self::Value, args: &[Self::Value], funclet: Option<&Self::Funclet>, callee_instance: Option<Instance<'tcx>>) -> Self::Value {
        todo!()
    }

    fn tail_call(&mut self, llty: Self::FunctionSignature, caller_attrs: Option<&CodegenFnAttrs>, fn_abi: &rustc_target::callconv::FnAbi<'tcx, Ty<'tcx>>, llfn: Self::Value, args: &[Self::Value], funclet: Option<&Self::Funclet>, callee_instance: Option<Instance<'tcx>>) {
        todo!()
    }

    fn zext(&mut self, val: Self::Value, dest_ty: Self::Type) -> Self::Value {
        todo!()
    }

    fn apply_attrs_to_cleanup_callsite(&mut self, llret: Self::Value) {
        todo!()
    }
}