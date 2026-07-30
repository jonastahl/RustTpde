#pragma once
#include <cstdint>
#include <tpde/base.hpp>

#include "rustc_codegen_tpde/src/shared.rs.h"

uint32_t compile_to_file(ModuleTpde& module, rust::Str path);

tpde::u32 size_of_type(Type type);
