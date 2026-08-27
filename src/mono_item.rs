use crate::context::CodegenCx;
use rustc_codegen_ssa::traits::PreDefineCodegenMethods;
use rustc_hir::attrs::Linkage;
use rustc_middle::mono::Visibility;
use rustc_middle::ty;
use rustc_middle::ty::{Instance, Ty};
use rustc_middle::ty::layout::FnAbiOf;
use rustc_span::def_id::DefId;
use rustc_target::callconv::FnAbi;

impl<'tcx> PreDefineCodegenMethods<'tcx> for CodegenCx<'_, 'tcx> {
    fn predefine_static(
        &mut self,
        def_id: DefId,
        linkage: Linkage,
        visibility: Visibility,
        symbol_name: &str,
    ) {
        todo!()
    }

    fn predefine_fn(
        &mut self,
        instance: Instance<'tcx>,
        linkage: Linkage,
        visibility: Visibility,
        symbol_name: &str,
    ) {
        let fn_abi: &FnAbi<'tcx, Ty<'tcx>> = self.fn_abi_of_instance(instance, ty::List::empty());

        let func = self.tpde_module.borrow_mut()
            .add_function(self, symbol_name, &self.create_function_signature(fn_abi), linkage);
        self.functions.insert(instance, func);
    }
}
