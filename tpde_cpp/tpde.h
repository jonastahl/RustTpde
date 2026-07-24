#pragma once
#include <cstdint>

#include "rust/cxx.h"

#include "rustc_codegen_tpde/src/shared.rs.h"

uint32_t compile_ir(const ModuleTpde& module);