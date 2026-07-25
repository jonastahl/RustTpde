use rustc_codegen_ssa::traits::{StaticBuilderMethods, StaticCodegenMethods};
use rustc_middle::mir::interpret::ConstAllocation;
use rustc_span::def_id::DefId;
use crate::builder::Builder;
use crate::context::CodegenCx;

impl<'tcx> StaticBuilderMethods for Builder<'_, '_, 'tcx> {
    fn get_static(&mut self, def_id: DefId) -> Self::Value {
        todo!()
    }
}

impl<'tcx> StaticCodegenMethods for CodegenCx<'_, 'tcx> {
    fn static_addr_of(&self, alloc: ConstAllocation<'_>, kind: Option<&str>) -> Self::Value {
        todo!()
    }

    fn codegen_static(&mut self, def_id: DefId) {
        todo!()
    }
}