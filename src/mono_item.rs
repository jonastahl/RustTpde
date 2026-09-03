use crate::context::CodegenCx;
use rustc_codegen_ssa::traits::PreDefineCodegenMethods;
use rustc_hir::attrs::Linkage;
use rustc_middle::mono::Visibility;
use rustc_middle::ty;
use rustc_middle::ty::{Instance, Ty};
use rustc_middle::ty::layout::FnAbiOf;
use rustc_span::def_id::DefId;
use rustc_target::callconv::FnAbi;
use crate::shared::ir::{Binding, Function};

impl<'tcx> PreDefineCodegenMethods<'tcx> for CodegenCx<'_, 'tcx> {
    fn predefine_static(
        &mut self,
        def_id: DefId,
        linkage: Linkage,
        visibility: Visibility,
        symbol_name: &str,
    ) {
        let global = self.module.borrow_mut()
            .add_global(symbol_name, linkage);
        self.globals.insert(def_id, global);
    }

    fn predefine_fn(
        &mut self,
        instance: Instance<'tcx>,
        linkage: Linkage,
        visibility: Visibility,
        symbol_name: &str,
    ) {
        self.declare_fn(instance, linkage, visibility, Binding::Definition, symbol_name);
    }
}

impl<'tcx> CodegenCx<'_, 'tcx> {
    pub fn declare_fn(
        &self,
        instance: Instance<'tcx>,
        linkage: Linkage,
        visibility: Visibility,
        binding: Binding,
        symbol_name: &str,
    ) -> Function {
        let fn_abi: &FnAbi<'tcx, Ty<'tcx>> = self.fn_abi_of_instance(instance, ty::List::empty());

        let func = self.module.borrow_mut()
            .add_function(self, symbol_name, &self.create_function_signature(fn_abi), linkage, binding);
        self.functions.borrow_mut().insert(instance, func);
        func
    }
}
