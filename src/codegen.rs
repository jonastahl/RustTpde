use rustc_codegen_llvm::LlvmCodegenBackend;
use rustc_codegen_ssa::traits::CodegenBackend;
use rustc_codegen_ssa::{CompiledModules, CrateInfo, TargetConfig};
use rustc_middle::dep_graph::WorkProductMap;
use rustc_middle::mono::MonoItem;
use rustc_middle::ty;
use rustc_middle::ty::{Instance, TyCtxt};
use rustc_middle::util::Providers;
use rustc_session::config::{CrateType, OutputFilenames, PrintRequest};
use rustc_session::Session;
use std::any::Any;

pub struct TpdeCodegenBackend {
    llvm_codegen_backend: Box<dyn CodegenBackend>
}

impl TpdeCodegenBackend {
    pub fn new() -> Box<dyn CodegenBackend> {
        Box::new(TpdeCodegenBackend{
            llvm_codegen_backend: LlvmCodegenBackend::new()
        })
    }
}

fn lower_function_to_tpde<'tcx>(
    tcx: TyCtxt<'tcx>,
    instance: Instance<'tcx>,
    mir_body: &'tcx rustc_middle::mir::Body<'tcx>
) {
    let t = instance.ty(tcx, ty::TypingEnv::fully_monomorphized());
    if t.is_fn() {
        println!("ty: {:?}", t.fn_sig(tcx));
    }

    for (local, local_decl) in mir_body.local_decls.iter_enumerated() {
        println!("local: {:?}, local_decl: {:?}", local, local_decl);
    }

    for (bb_index, bb_data) in mir_body.basic_blocks.iter_enumerated() {
        println!("* bb index: {:?}", bb_index);
        for statement in &bb_data.statements {
            rustc_middle::ty::print::with_no_trimmed_paths!({
                println!("statement: {:?}", statement);
            });
        }

        if let Some(terminator) = &bb_data.terminator {
            rustc_middle::ty::print::with_no_trimmed_paths!({
                println!("terminator: {:?}", terminator);
            });
        } else {
            print!("terminator: None,");
        }
    }
}

impl CodegenBackend for TpdeCodegenBackend {
    fn name(&self) -> &'static str {
        "TPDE"
    }

    fn init(&self, _sess: &Session) {
        println!("init");
        self.llvm_codegen_backend.init(_sess)
    }

    fn codegen_crate<'tcx>(&self, tcx: TyCtxt<'tcx>) -> Box<dyn Any> {
        println!("codegen_crate");

        let cgus = tcx.collect_and_partition_mono_items(());

        for cgu in cgus.codegen_units {
            for (mono_item, _mono_item_data) in cgu.items() {
                match mono_item {
                    MonoItem::Fn(func) => {
                        println!("*************************************\nFunction {:?}", func.def_id());
                        let mir_body = tcx.instance_mir(func.def);

                        lower_function_to_tpde(tcx, *func, mir_body);
                    }
                    MonoItem::Static(def_id) => {
                        println!("Static {def_id:?}");
                        // unimplemented!()
                    }
                    MonoItem::GlobalAsm(item_id) => {
                        println!("GlobalAsm {item_id:?}")
                        // unimplemented!()
                    }
                }
            }
        }

        self.llvm_codegen_backend.codegen_crate(tcx)
    }

    fn join_codegen(&self, ongoing_codegen: Box<dyn Any>, sess: &Session, outputs: &OutputFilenames, crate_info: &CrateInfo) -> (CompiledModules, WorkProductMap) {
        println!("join_codegen");
        self.llvm_codegen_backend.join_codegen(ongoing_codegen, sess, outputs, crate_info)
    }

    // not used by LLVM
    fn link(&self, sess: &Session, compiled_modules: CompiledModules, crate_info: CrateInfo, metadata: rustc_metadata::EncodedMetadata, outputs: &OutputFilenames) {
        println!("link");
        self.llvm_codegen_backend.link(sess, compiled_modules, crate_info, metadata, outputs)
    }

    fn print(&self, _req: &PrintRequest, _out: &mut String, _sess: &Session) {
        self.llvm_codegen_backend.print(_req, _out, _sess)
    }

    fn target_config(&self, _sess: &Session) -> TargetConfig {
        self.llvm_codegen_backend.target_config(_sess)
    }

    fn supported_crate_types(&self, _sess: &Session) -> Vec<CrateType> {
        self.llvm_codegen_backend.supported_crate_types(_sess)
    }

    fn print_passes(&self) {
        self.llvm_codegen_backend.print_passes()
    }

    fn print_version(&self) {
        self.llvm_codegen_backend.print_version()
    }

    fn replaced_intrinsics(&self) -> Vec<rustc_span::symbol::Symbol> {
        self.llvm_codegen_backend.replaced_intrinsics()
    }

    fn fallback_intrinsics(&self) -> Vec<rustc_span::symbol::Symbol> {
        self.llvm_codegen_backend.fallback_intrinsics()
    }

    fn thin_lto_supported(&self) -> bool {
        self.llvm_codegen_backend.thin_lto_supported()
    }

    fn has_zstd(&self) -> bool {
        self.llvm_codegen_backend.has_zstd()
    }

    fn has_mnemonic(&self, _sess: &Session, _mnemonic: &str) -> bool {
        self.llvm_codegen_backend.has_mnemonic(_sess, _mnemonic)
    }

    fn metadata_loader(&self) -> Box<rustc_metadata::creader::MetadataLoaderDyn> {
        self.llvm_codegen_backend.metadata_loader()
    }

    fn provide(&self, _providers: &mut Providers) {
        self.llvm_codegen_backend.provide(_providers)
    }

    fn target_cpu(&self, sess: &Session) -> String {
        self.llvm_codegen_backend.target_cpu(sess)
    }

    fn print_pass_timings(&self) {
        self.llvm_codegen_backend.print_pass_timings()
    }

    fn print_statistics(&self) {
        self.llvm_codegen_backend.print_statistics()
    }

    fn print_statistics_json(&self) -> String {
        self.llvm_codegen_backend.print_statistics_json()
    }
}