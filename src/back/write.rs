use crate::shared;
use crate::shared::ir::Module;
use rustc_codegen_ssa::back::link::ensure_removed;
use rustc_codegen_ssa::back::write::{BitcodeSection, CodegenContext, EmitObj, ModuleConfig};
use rustc_codegen_ssa::{CompiledModule, ModuleCodegen};
use rustc_data_structures::profiling::SelfProfilerRef;
use rustc_errors::DiagCtxtHandle;
use rustc_fs_util::link_or_copy;
use rustc_session::config::OutputType;

pub(crate) fn codegen(
    cgcx: &CodegenContext,
    prof: &SelfProfilerRef,
    dcx: DiagCtxtHandle<'_>,
    mut module: ModuleCodegen<Module>,
    config: &ModuleConfig,
) -> CompiledModule {
    let bc_out = cgcx.output_filenames.temp_path_for_cgu(OutputType::Bitcode, &module.name);
    let obj_out = cgcx.output_filenames.temp_path_for_cgu(OutputType::Object, &module.name);

    if config.bitcode_needed() {
        if config.emit_bc || config.emit_obj == EmitObj::Bitcode {
            todo!()
        }
    }

    if config.emit_ir {
        let out =
            cgcx.output_filenames.temp_path_for_cgu(OutputType::LlvmAssembly, &module.name);
        let content = format!("{:#?}", module.module_llvm.tpde());
        std::fs::write(out, content).expect("write file");
    }

    if config.emit_asm {
        todo!()
    }

    match config.emit_obj {
        EmitObj::ObjectCode(_) => {
            println!("Compiling to obj file: {}", obj_out.to_str().expect("path to str"));
            shared::compile_to_file(module.module_llvm.tpde_mut(), obj_out.to_str().expect("path to str"));
        }
        EmitObj::Bitcode => {
            println!("Copying bitcode file to obj file: {}", obj_out.to_str().expect("path to str"));
            if let Err(err) = link_or_copy(&bc_out, &obj_out) {
                todo!()
            }

            if !config.emit_bc {
                ensure_removed(dcx, &bc_out);
            }
        }
        EmitObj::None => {}
    }

    module.into_compiled_module(
        config.emit_obj == EmitObj::ObjectCode(BitcodeSection::Full),
        false,
        false,
        false,
        config.emit_ir,
        &cgcx.output_filenames
    )
}