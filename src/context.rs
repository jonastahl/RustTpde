use crate::builder::Builder;
use crate::shared::ir::{Binding, Function, FunctionSignature, Global, Module, Slot};
use rustc_abi::TargetDataLayout;
use rustc_codegen_ssa::traits::MiscCodegenMethods;
use rustc_data_structures::fx::FxHashMap;
use rustc_hir::attrs::Linkage;
use rustc_middle::mono::{CodegenUnit, Visibility};
use rustc_middle::ty::layout::HasTyCtxt;
use rustc_middle::ty::{ExistentialTraitRef, Instance, Ty, TyCtxt};
use rustc_session::{PointerAuthSchema, Session};
use rustc_span::Symbol;
use rustc_span::def_id::DefId;
use std::cell::{Cell, RefCell};

pub struct CodegenCx<'tpde, 'tcx> {
    pub tcx: TyCtxt<'tcx>,
    pub codegen_unit: &'tcx CodegenUnit<'tcx>,

    pub module: &'tpde RefCell<Module>,

    pub functions: RefCell<FxHashMap<Instance<'tcx>, Function>>,
    pub function_signatures: RefCell<Vec<FunctionSignature>>,

    pub globals: FxHashMap<DefId, Global>,

    pub data_layout: TargetDataLayout,

    pub global_gen_sym_counter: Cell<usize>,
}

impl<'tpde, 'tcx> CodegenCx<'tpde, 'tcx> {
    pub fn new(
        tcx: TyCtxt<'tcx>,
        cgu: &'tcx CodegenUnit<'tcx>,
        ir: &'tpde RefCell<Module>,
    ) -> Self {
        let sess = tcx.sess;

        let data_layout = sess.target.parse_data_layout().unwrap_or_else(|err| {
            sess.dcx().emit_fatal(err);
        });

        Self {
            tcx,
            codegen_unit: cgu,
            module: ir,
            functions: RefCell::new(FxHashMap::default()),
            function_signatures: RefCell::new(vec![]),
            globals: FxHashMap::default(),
            data_layout,
            global_gen_sym_counter: Cell::new(0),
        }
    }
}

impl<'tcx> MiscCodegenMethods<'tcx> for CodegenCx<'_, 'tcx> {
    fn vtables(&self) -> &RefCell<FxHashMap<(Ty<'tcx>, Option<ExistentialTraitRef<'tcx>>), Self::Value>> {
        todo!()
    }

    fn get_fn(&self, instance: Instance<'tcx>) -> Self::Function {
        if let Some(&i) = self.functions.borrow().get(&instance) {
            return i;
        };

        let name = self.tcx.symbol_name(instance).name;
        self.declare_fn(
            instance,
            Linkage::External,
            Visibility::Hidden, // TODO find exact visibility
            Binding::Declaration,
            name,
        )
    }

    fn get_fn_addr(&self, instance: Instance<'tcx>, pointer_auth_schema: Option<&PointerAuthSchema>) -> Self::Value {
        Slot::new_func(self.get_fn(instance))
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
