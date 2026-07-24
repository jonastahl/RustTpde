use std::cell::RefCell;
use rustc_codegen_ssa::traits::MiscCodegenMethods;
use rustc_data_structures::fx::FxHashMap;
use rustc_middle::mono::CodegenUnit;
use rustc_middle::ty::{ExistentialTraitRef, Instance, Ty, TyCtxt};
use rustc_session::{PointerAuthSchema, Session};
use rustc_span::Symbol;
use crate::shared::ir::ModuleTpde;

pub struct CodegenCx<'tcx> {
    pub tcx: TyCtxt<'tcx>,
    pub codegen_unit: &'tcx CodegenUnit<'tcx>,
    pub tpde_module: ModuleTpde,
}

impl<'tcx> CodegenCx<'tcx> {
    pub(crate) fn new(
        tcx: TyCtxt<'tcx>,
        cgu: &'tcx CodegenUnit<'tcx>,
        tpde_module: ModuleTpde,
    ) -> Self {
        Self {
            tcx,
            codegen_unit: cgu,
            tpde_module,
        }
    }
}

impl<'tcx> MiscCodegenMethods<'tcx> for CodegenCx<'tcx> {
    fn vtables(&self) -> &RefCell<FxHashMap<(Ty<'tcx>, Option<ExistentialTraitRef<'tcx>>), Self::Value>> {
        todo!()
    }

    fn get_fn(&self, instance: Instance<'tcx>) -> Self::Function {
        todo!()
    }

    fn get_fn_addr(&self, instance: Instance<'tcx>, pointer_auth_schema: Option<&PointerAuthSchema>) -> Self::Value {
        todo!()
    }

    fn eh_personality(&self) -> Self::Function {
        todo!()
    }

    fn sess(&self) -> &Session {
        todo!()
    }

    fn set_frame_pointer_type(&self, llfn: Self::Function) {
        todo!()
    }

    fn apply_target_cpu_attr(&self, llfn: Self::Function) {
        todo!()
    }

    fn declare_c_main(&self, fn_type: Self::FunctionSignature) -> Option<Self::Function> {
        todo!()
    }

    fn intrinsic_call_expects_place_always(&self, name: Symbol) -> bool {
        todo!()
    }
}
