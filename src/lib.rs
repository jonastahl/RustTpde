#![feature(rustc_private)]

extern crate rustc_driver;

extern crate rustc_codegen_ssa;
extern crate rustc_middle;
extern crate rustc_session;

use std::any::Any;
use rustc_codegen_ssa::{CompiledModules, CrateInfo};
use rustc_codegen_ssa::traits::CodegenBackend;
use rustc_middle::dep_graph::WorkProductMap;
use rustc_middle::ty::TyCtxt;

struct TpdeCodegenBackend {}

impl CodegenBackend for TpdeCodegenBackend {
    fn name(&self) -> &'static str {
        todo!()
    }

    fn target_cpu(&self, sess: &rustc_session::Session) -> String {
        todo!()
    }

    fn codegen_crate<'tcx>(&self, tcx: TyCtxt<'tcx>) -> Box<dyn Any> {
        todo!()
    }

    fn join_codegen(&self, ongoing_codegen: Box<dyn Any>, sess: &rustc_session::Session, outputs: &rustc_session::config::OutputFilenames, crate_info: &CrateInfo) -> (CompiledModules, WorkProductMap) {
        todo!()
    }
}