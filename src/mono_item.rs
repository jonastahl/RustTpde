use rustc_codegen_ssa::traits::{PreDefineCodegenMethods};
use rustc_hir::attrs::Linkage;
use rustc_middle::mono::Visibility;
use rustc_middle::ty::Instance;
use rustc_span::def_id::DefId;
use crate::context::CodegenCx;

impl<'tcx> PreDefineCodegenMethods<'tcx> for CodegenCx<'tcx> {
    fn predefine_static(&mut self, def_id: DefId, linkage: Linkage, visibility: Visibility, symbol_name: &str) {
        todo!()
    }

    fn predefine_fn(&mut self, instance: Instance<'tcx>, linkage: Linkage, visibility: Visibility, symbol_name: &str) {
        todo!()
    }
}