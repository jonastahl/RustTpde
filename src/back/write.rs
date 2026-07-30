use crate::shared;
use crate::shared::ir::ModuleTpde;
use rustc_codegen_ssa::back::write::{BitcodeSection, CodegenContext, EmitObj, ModuleConfig};
use rustc_codegen_ssa::{CompiledModule, ModuleCodegen};
use rustc_codegen_ssa::back::link::ensure_removed;
use rustc_data_structures::profiling::SelfProfilerRef;
use rustc_errors::DiagCtxtHandle;
use rustc_session::config::OutputType;
use rustc_fs_util::link_or_copy;

pub(crate) fn codegen(
    cgcx: &CodegenContext,
    prof: &SelfProfilerRef,
    dcx: DiagCtxtHandle<'_>,
    mut module: ModuleCodegen<ModuleTpde>,
    config: &ModuleConfig,
) -> CompiledModule {
    let bc_out = cgcx.output_filenames.temp_path_for_cgu(OutputType::Bitcode, &module.name);
    let obj_out = cgcx.output_filenames.temp_path_for_cgu(OutputType::Object, &module.name);

    if config.bitcode_needed() {
        if config.emit_bc || config.emit_obj == EmitObj::Bitcode {
            todo!()
        }

        if config.emit_obj == EmitObj::ObjectCode(BitcodeSection::Full) {
            println!("Compiling to file: {}", bc_out.to_str().expect("path to str"));
            shared::compile_to_file(&mut module.module_llvm, bc_out.to_str().expect("path to str"));
        }
    }

    if config.emit_ir {
        let out =
            cgcx.output_filenames.temp_path_for_cgu(OutputType::LlvmAssembly, &module.name);
        std::fs::write(out, format!("{:#?}", module.module_llvm)).expect("write file");
    }

    if config.emit_asm {
        todo!()
    }

    match config.emit_obj {
        EmitObj::ObjectCode(_) => {
            println!("Compiling to file: {}", obj_out.to_str().expect("path to str"));
            shared::compile_to_file(&mut module.module_llvm, obj_out.to_str().expect("path to str"));
        }
        EmitObj::Bitcode => {
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