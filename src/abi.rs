use crate::builder::Builder;
use crate::context::CodegenCx;
use rustc_abi::{HasDataLayout, Reg, TargetDataLayout};
use rustc_codegen_ssa::mir::place::PlaceRef;
use rustc_codegen_ssa::traits::{AbiBuilderMethods, ArgAbiBuilderMethods, BackendTypes, LayoutTypeCodegenMethods};
use rustc_middle::ty::layout::{FnAbiError, FnAbiOfHelpers, FnAbiRequest, HasTyCtxt, HasTypingEnv, LayoutError, LayoutOfHelpers, MaybeResult, TyAndLayout};
use rustc_middle::ty::{Ty, TyCtxt, TypingEnv};
use rustc_span::Span;
use rustc_target::callconv::{ArgAbi, CastTarget, FnAbi};

impl<'tcx> AbiBuilderMethods for Builder<'_, 'tcx> {
    fn get_param(&mut self, index: usize) -> Self::Value {
        todo!()
    }
}

impl<'tcx> ArgAbiBuilderMethods<'tcx> for Builder<'_, 'tcx> {
    fn store_fn_arg(&mut self, arg_abi: &ArgAbi<'tcx, Ty<'tcx>>, idx: &mut usize, dst: PlaceRef<'tcx, Self::Value>) {
        todo!()
    }

    fn store_arg(&mut self, arg_abi: &ArgAbi<'tcx, Ty<'tcx>>, val: Self::Value, dst: PlaceRef<'tcx, Self::Value>) {
        todo!()
    }
}

impl<'tcx> HasDataLayout for Builder<'_, 'tcx> {
    fn data_layout(&self) -> &TargetDataLayout {
        todo!()
    }
}

impl<'tcx> HasDataLayout for CodegenCx<'tcx> {
    fn data_layout(&self) -> &TargetDataLayout {
        todo!()
    }
}

impl<'tcx> HasTyCtxt<'tcx> for Builder<'_, 'tcx> {
    fn tcx(&self) -> TyCtxt<'tcx> {
        todo!()
    }
}

impl<'tcx> HasTyCtxt<'tcx> for CodegenCx<'tcx> {
    fn tcx(&self) -> TyCtxt<'tcx> {
        todo!()
    }
}

impl<'tcx> HasTypingEnv<'tcx> for Builder<'_, 'tcx> {
    fn typing_env(&self) -> TypingEnv<'tcx> {
        todo!()
    }
}

impl<'tcx> HasTypingEnv<'tcx> for CodegenCx<'tcx> {
    fn typing_env(&self) -> TypingEnv<'tcx> {
        todo!()
    }
}

impl<'tcx> LayoutOfHelpers<'tcx> for Builder<'_, 'tcx> {
    fn handle_layout_err(&self, err: LayoutError<'tcx>, span: Span, ty: Ty<'tcx>) -> <Self::LayoutOfResult as MaybeResult<TyAndLayout<'tcx>>>::Error {
        todo!()
    }
}

impl<'tcx> LayoutOfHelpers<'tcx> for CodegenCx<'tcx> {
    fn handle_layout_err(&self, err: LayoutError<'tcx>, span: Span, ty: Ty<'tcx>) -> <Self::LayoutOfResult as MaybeResult<TyAndLayout<'tcx>>>::Error {
        todo!()
    }
}

impl<'tcx> FnAbiOfHelpers<'tcx> for Builder<'_, 'tcx> {
    fn handle_fn_abi_err(&self, err: FnAbiError<'tcx>, span: Span, fn_abi_request: FnAbiRequest<'tcx>) -> <Self::FnAbiOfResult as MaybeResult<&'tcx FnAbi<'tcx, Ty<'tcx>>>>::Error {
        todo!()
    }
}

impl<'tcx> FnAbiOfHelpers<'tcx> for CodegenCx<'tcx> {
    fn handle_fn_abi_err(&self, err: FnAbiError<'tcx>, span: Span, fn_abi_request: FnAbiRequest<'tcx>) -> <Self::FnAbiOfResult as MaybeResult<&'tcx FnAbi<'tcx, Ty<'tcx>>>>::Error {
        todo!()
    }
}

impl<'tcx> LayoutTypeCodegenMethods<'tcx> for CodegenCx<'tcx> {
    fn backend_type(&self, layout: TyAndLayout<'tcx>) -> Self::Type {
        todo!()
    }

    fn cast_backend_type(&self, ty: &CastTarget) -> Self::Type {
        todo!()
    }

    fn fn_decl_backend_type(&self, fn_abi: &FnAbi<'tcx, Ty<'tcx>>) -> Self::FunctionSignature {
        todo!()
    }

    fn fn_ptr_backend_type(&self, fn_abi: &FnAbi<'tcx, Ty<'tcx>>) -> Self::Type {
        todo!()
    }

    fn reg_backend_type(&self, ty: &Reg) -> Self::Type {
        todo!()
    }

    fn immediate_backend_type(&self, layout: TyAndLayout<'tcx>) -> Self::Type {
        todo!()
    }

    fn scalar_pair_element_backend_type(&self, layout: TyAndLayout<'tcx>, index: usize, immediate: bool) -> Self::Type {
        todo!()
    }
}