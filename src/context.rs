use crate::builder::Builder;
use crate::shared::ir::{Function, ModuleTpde};
use rustc_codegen_ssa::traits::MiscCodegenMethods;
use rustc_data_structures::fx::FxHashMap;
use rustc_middle::mono::CodegenUnit;
use rustc_middle::ty::layout::HasTyCtxt;
use rustc_middle::ty::{ExistentialTraitRef, Instance, Ty, TyCtxt};
use rustc_session::{PointerAuthSchema, Session};
use rustc_span::Symbol;
use std::cell::RefCell;
use rustc_abi::TargetDataLayout;

pub struct CodegenCx<'tpde, 'tcx> {
    pub tcx: TyCtxt<'tcx>,
    pub codegen_unit: &'tcx CodegenUnit<'tcx>,

    pub tpde_module: &'tpde RefCell<ModuleTpde>,
    pub functions: FxHashMap<Instance<'tcx>, Function>,

    pub data_layout: TargetDataLayout,
}

impl<'tpde, 'tcx> CodegenCx<'tpde, 'tcx> {
    pub(crate) fn new(
        tcx: TyCtxt<'tcx>,
        cgu: &'tcx CodegenUnit<'tcx>,
        tpde_module: &'tpde RefCell<ModuleTpde>,
    ) -> Self {
        let sess = tcx.sess;

        let data_layout = sess.target.parse_data_layout().unwrap_or_else(|err| {
            sess.dcx().emit_fatal(err);
        });

        Self {
            tcx,
            codegen_unit: cgu,
            tpde_module,
            functions: FxHashMap::default(),
            data_layout,
        }
    }
}

impl<'tcx> MiscCodegenMethods<'tcx> for CodegenCx<'_, 'tcx> {
    fn vtables(&self) -> &RefCell<FxHashMap<(Ty<'tcx>, Option<ExistentialTraitRef<'tcx>>), Self::Value>> {
        todo!()
    }

    fn get_fn(&self, instance: Instance<'tcx>) -> Self::Function {
        if let Some(&i) = self.functions.get(&instance) {
            return i;
        };

        todo!()
    }

    fn get_fn_addr(&self, instance: Instance<'tcx>, pointer_auth_schema: Option<&PointerAuthSchema>) -> Self::Value {
        todo!()
    }

    fn eh_personality(&self) -> Self::Function {
        todo!()
    }

    fn sess(&self) -> &Session {
        self.tcx.sess
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

impl<'tcx> HasTyCtxt<'tcx> for Builder<'_, '_, 'tcx> {
    fn tcx(&self) -> TyCtxt<'tcx> {
        self.tcx
    }
}

impl<'tcx> HasTyCtxt<'tcx> for CodegenCx<'_, 'tcx> {
    fn tcx(&self) -> TyCtxt<'tcx> {
        self.tcx
    }
}
