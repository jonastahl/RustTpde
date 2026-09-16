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
use rustc_span::{sym, Symbol};
use rustc_span::def_id::DefId;
use std::cell::{Cell, RefCell};
use rustc_data_structures::base_n::{ToBaseN, ALPHANUMERIC_ONLY};
use rustc_middle::ty;

pub struct CodegenCx<'tpde, 'tcx> {
    pub tcx: TyCtxt<'tcx>,
    pub codegen_unit: &'tcx CodegenUnit<'tcx>,

    pub module: &'tpde RefCell<Module>,

    pub functions: RefCell<FxHashMap<Instance<'tcx>, Function>>,
    pub function_signatures: RefCell<Vec<FunctionSignature>>,

    pub globals: FxHashMap<DefId, Global>,
    pub vtables: RefCell<FxHashMap<(Ty<'tcx>, Option<ty::ExistentialTraitRef<'tcx>>), Slot>>,

    pub data_layout: TargetDataLayout,

    pub global_gen_sym_counter: Cell<usize>,
    pub local_gen_sym_counter: Cell<usize>,

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
            local_gen_sym_counter: Cell::new(0),
            vtables: RefCell::new(FxHashMap::default()),
        }
    }
}

impl<'tcx> MiscCodegenMethods<'tcx> for CodegenCx<'_, 'tcx> {
    fn vtables(
        &self,
    ) -> &RefCell<FxHashMap<(Ty<'tcx>, Option<ExistentialTraitRef<'tcx>>), Self::Value>> {
        &self.vtables
    }

    fn get_fn(&self, instance: Instance<'tcx>) -> Self::Function {
        if let Some(&i) = self.functions.borrow().get(&instance) {
            return i;
        };

        let name = self.tcx.symbol_name(instance).name;
        self.declare_fn(
            instance,
            name,
            Linkage::External,
            Visibility::Hidden, // TODO find exact visibility
            Binding::Declaration,
        )
    }

    fn get_fn_addr(
        &self,
        instance: Instance<'tcx>,
        pointer_auth_schema: Option<&PointerAuthSchema>,
    ) -> Self::Value {
        Slot::new_func(self.get_fn(instance))
    }

    fn eh_personality(&self) -> Self::Function {
        let def_id = match self.tcx.lang_items().eh_personality() {
            Some(id) => id,
            None => {
                panic!("eh_personality is required but not defined in lang_items");
            }
        };

        let instance = Instance::mono(self.tcx, def_id);
        let name = self.tcx.symbol_name(instance).name;

        self.declare_fn(
            instance,
            name,
            Linkage::External,
            Visibility::Hidden,
            Binding::Declaration,
        )
    }

    fn sess(&self) -> &Session {
        self.tcx.sess
    }

    fn set_frame_pointer_type(&self, llfn: Self::Function) {
        // We just don't use it
    }

    fn apply_target_cpu_attr(&self, llfn: Self::Function) {
        // We are not that specialized
    }

    fn declare_c_main(&self, fn_type: Self::FunctionSignature) -> Option<Self::Function> {
        let entry_name = self.sess().target.entry_name.as_ref();

        let sign = self.function_signatures.borrow()[fn_type].clone();
        let func = self.module.borrow_mut()
            .add_function(self, entry_name, sign, Linkage::External, Binding::Definition);
        Some(func)
    }

    fn intrinsic_call_expects_place_always(&self, name: Symbol) -> bool {
        matches!(name, sym::black_box)
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

impl CodegenCx<'_, '_> {
    /// Generates a new symbol name with the given prefix. This symbol name must
    /// only be used for definitions with `internal` or `private` linkage.
    pub(crate) fn generate_local_symbol_name(&self, prefix: &str) -> String {
        let idx = self.local_gen_sym_counter.get();
        self.local_gen_sym_counter.set(idx + 1);
        // Include a '.' character, so there can be no accidental conflicts with
        // user defined names
        let mut name = String::with_capacity(prefix.len() + 6);
        name.push_str(prefix);
        name.push('.');
        name.push_str(&(idx as u64).to_base(ALPHANUMERIC_ONLY));
        name
    }

    /// Generates a new global symbol name with the given prefix.
    pub(crate) fn generate_global_symbol_name(&self) -> String {
        let idx = self.global_gen_sym_counter.get();
        self.global_gen_sym_counter.set(idx + 1);

        let sym = self.codegen_unit.symbol_name();
        let prefix = sym.as_str();
        let mut name = String::with_capacity(prefix.len() + 6);
        name.push_str(prefix);
        name.push('.');
        name.push_str(&(idx as u64).to_base(ALPHANUMERIC_ONLY));
        name
    }
}
