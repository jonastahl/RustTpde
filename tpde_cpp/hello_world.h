#pragma once
#include <cstdint>

#include "rust/cxx.h"

#include "rustc_codegen_tpde/src/lib.rs.h"

rust::String hello_world();

uint32_t compile_ir(const Ir& ir);