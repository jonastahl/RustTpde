use std::time::Instant;
use rustc_codegen_ssa::ModuleCodegen;
use rustc_codegen_ssa::mono_item::MonoItemExt;
use rustc_middle::dep_graph;
use rustc_middle::ty::TyCtxt;
use rustc_span::Symbol;
use crate::builder::Builder;
use crate::context::CodegenCx;
use crate::cpp::ModuleTpde;

pub fn compile_codegen_unit(
    tcx: TyCtxt<'_>,
    cgu_name: Symbol
) -> (ModuleCodegen<ModuleTpde>, u64) {
    let start_time = Instant::now();

    let dep_node = tcx.codegen_unit(cgu_name).codegen_dep_node(tcx);
    let (module, _) = tcx.dep_graph.with_task(
        dep_node,
        tcx,
        || module_codegen(tcx, cgu_name),
        Some(dep_graph::hash_result)
    );
    let time_to_codegen = start_time.elapsed();

    // We assume that the cost to run TPDE on a CGU is proportional to
    // the time we needed for codegenning it.
    let cost = time_to_codegen.as_nanos() as u64;


    fn module_codegen(tcx: TyCtxt<'_>, cgu_name: Symbol) -> ModuleCodegen<ModuleTpde> {
        let cgu = tcx.codegen_unit(cgu_name);

        let tpde_module = ModuleTpde::new();
        {
            let mut cx = CodegenCx::new(tcx, cgu, tpde_module);

            let mono_items = cgu.items_in_deterministic_order(tcx);
            for &(mono_item, data) in &mono_items {
                mono_item.predefine::<Builder<'_, '_>>(
                    &mut cx,
                    cgu_name.as_str(),
                    data.linkage,
                    data.visibility
                    );
            }
        }
        todo!()
    }


    (module, cost)
}