use rustc_codegen_ssa::mir::IntrinsicResult;
use rustc_codegen_ssa::mir::operand::OperandRef;
use rustc_codegen_ssa::mir::place::PlaceValue;
use rustc_codegen_ssa::RetagInfo;
use rustc_codegen_ssa::traits::IntrinsicCallBuilderMethods;
use rustc_middle::ty::Instance;
use rustc_middle::ty::layout::TyAndLayout;
use rustc_span::Span;
use crate::builder::Builder;

impl<'tcx> IntrinsicCallBuilderMethods<'tcx> for Builder<'_, '_, 'tcx> {
    fn codegen_intrinsic_call(&mut self, instance: Instance<'tcx>, args: &[OperandRef<'tcx, Self::Value>], result_layout: TyAndLayout<'tcx>, result_place: Option<PlaceValue<Self::Value>>, span: Span) -> IntrinsicResult<'tcx, Self::Value> {
        todo!()
    }

    fn codegen_llvm_intrinsic_call(&mut self, instance: Instance<'tcx>, args: &[OperandRef<'tcx, Self::Value>], is_cleanup: bool) -> Self::Value {
        todo!()
    }

    fn abort(&mut self) {
        todo!()
    }

    fn assume(&mut self, val: Self::Value) {
        todo!()
    }

    fn expect(&mut self, cond: Self::Value, expected: bool) -> Self::Value {
        todo!()
    }

    fn type_checked_load(&mut self, llvtable: Self::Value, vtable_byte_offset: u64, typeid: &[u8]) -> Self::Value {
        todo!()
    }

    fn va_start(&mut self, val: Self::Value) {
        todo!()
    }

    fn retag_mem(&mut self, place: Self::Value, info: &RetagInfo<Self::Value>) {
        todo!()
    }

    fn retag_reg(&mut self, ptr: Self::Value, info: &RetagInfo<Self::Value>) -> Self::Value {
        todo!()
    }
}