#pragma once
#include <cstdint>
#include <tpde/base.hpp>

#include "rustc_codegen_tpde/src/shared.rs.h"

uint32_t compile_ir(ModuleTpde& module);

tpde::u32 size_of_type(Type type);
