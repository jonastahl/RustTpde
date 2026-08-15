#pragma once
#include <cstdint>
#include <tpde/base.hpp>

#include "rustc_codegen_tpde/src/shared.rs.h"

uint32_t compile_to_file(ModuleTpde& module, rust::Str path);

tpde::u32 size_of_type(Type type);

namespace operands {
  inline constexpr uint32_t MARKER_IMM = size_t{1} << (std::numeric_limits<std::uint32_t>::digits - 1);
  inline constexpr uint32_t MARKER_PTR = size_t{1} << (std::numeric_limits<std::uint32_t>::digits - 2);
  inline constexpr uint32_t MARKER_RAW = MARKER_IMM | MARKER_PTR;

  inline constexpr uint32_t ERASE = size_t{static_cast<uint32_t>(-1)} >> 2;

  inline bool is_val(uint32_t op) {
    return (op & MARKER_RAW) == 0;
  }

  inline bool is_imm(uint32_t op) {
    return (op & MARKER_RAW) == MARKER_IMM;
  }

  inline bool is_ptr(uint32_t op) {
    return (op & MARKER_RAW) == MARKER_PTR;
  }

  inline bool is_raw(uint32_t op) {
    return (op & MARKER_RAW) == MARKER_RAW;
  }

  inline uint32_t content(size_t op) {
    return op & ERASE;
  }
}
