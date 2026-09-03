use crate::builder::Builder;
use crate::context::CodegenCx;
use rustc_abi::{HasDataLayout, TargetDataLayout};
use rustc_codegen_ssa::mir::place::PlaceRef;
use rustc_codegen_ssa::traits::{AbiBuilderMethods, ArgAbiBuilderMethods};
use rustc_middle::ty::layout::{FnAbiError, FnAbiOfHelpers, FnAbiRequest, HasTypingEnv, LayoutError, LayoutOfHelpers, MaybeResult, TyAndLayout};
use rustc_middle::ty::{Ty, TypingEnv};
use rustc_span::Span;
use rustc_target::callconv::{ArgAbi, FnAbi};

impl<'tpde, 'tcx> AbiBuilderMethods for Builder<'_, 'tpde, 'tcx> {
    fn get_param(&mut self, index: usize) -> Self::Value {
        self.module.borrow().get_slot(self.basic_block.function(), index as u32)
    }
}

impl<'tpde, 'tcx> ArgAbiBuilderMethods<'tcx> for Builder<'_, 'tpde, 'tcx> {
    fn store_fn_arg(&mut self, arg_abi: &ArgAbi<'tcx, Ty<'tcx>>, idx: &mut usize, dst: PlaceRef<'tcx, Self::Value>) {
        todo!()
    }

    fn store_arg(&mut self, arg_abi: &ArgAbi<'tcx, Ty<'tcx>>, val: Self::Value, dst: PlaceRef<'tcx, Self::Value>) {
        todo!()
    }
}

impl<'tcx> HasDataLayout for Builder<'_, '_, 'tcx> {
    fn data_layout(&self) -> &TargetDataLayout {
        self.tcx.data_layout()
    }
}

impl<'tcx> HasDataLayout for CodegenCx<'_, 'tcx> {
    fn data_layout(&self) -> &TargetDataLayout {
        &self.data_layout
    }
}

impl<'tcx> HasTypingEnv<'tcx> for Builder<'_, '_, 'tcx> {
    fn typing_env(&self) -> TypingEnv<'tcx> {
        self.cx.typing_env()
    }
}

impl<'tcx> HasTypingEnv<'tcx> for CodegenCx<'_, 'tcx> {
    fn typing_env(&self) -> TypingEnv<'tcx> {
        TypingEnv::fully_monomorphized()
    }
}

impl<'tcx> LayoutOfHelpers<'tcx> for Builder<'_, '_, 'tcx> {
    fn handle_layout_err(&self, err: LayoutError<'tcx>, span: Span, ty: Ty<'tcx>) -> <Self::LayoutOfResult as MaybeResult<TyAndLayout<'tcx>>>::Error {
        todo!()
    }
}

impl<'tcx> LayoutOfHelpers<'tcx> for CodegenCx<'_, 'tcx> {
    fn handle_layout_err(&self, err: LayoutError<'tcx>, span: Span, ty: Ty<'tcx>) -> <Self::LayoutOfResult as MaybeResult<TyAndLayout<'tcx>>>::Error {
        todo!()
    }
}

impl<'tcx> FnAbiOfHelpers<'tcx> for Builder<'_, '_, 'tcx> {
    fn handle_fn_abi_err(&self, err: FnAbiError<'tcx>, span: Span, fn_abi_request: FnAbiRequest<'tcx>) -> <Self::FnAbiOfResult as MaybeResult<&'tcx FnAbi<'tcx, Ty<'tcx>>>>::Error {
        todo!()
    }
}

impl<'tcx> FnAbiOfHelpers<'tcx> for CodegenCx<'_, 'tcx> {
    fn handle_fn_abi_err(&self, err: FnAbiError<'tcx>, span: Span, fn_abi_request: FnAbiRequest<'tcx>) -> <Self::FnAbiOfResult as MaybeResult<&'tcx FnAbi<'tcx, Ty<'tcx>>>>::Error {
        todo!()
    }
}