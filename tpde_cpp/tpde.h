#pragma once
#include <cstdint>
#include <tpde/base.hpp>
#include <limits>

#include "rustc_codegen_tpde/src/shared.rs.h"

uint32_t compile_to_file(ModuleTpde& module, rust::Str path);

tpde::u32 size_of_type(Type type);

namespace operands {
  inline constexpr uint32_t MARKER_BLOCK = size_t{7} << (std::numeric_limits<std::uint32_t>::digits - 3);

  inline constexpr uint32_t MARKER_VAL = size_t{0} << (std::numeric_limits<std::uint32_t>::digits - 3);
  inline constexpr uint32_t MARKER_IMM = size_t{1} << (std::numeric_limits<std::uint32_t>::digits - 3);
  inline constexpr uint32_t MARKER_PAIR = size_t{2} << (std::numeric_limits<std::uint32_t>::digits - 3);
  inline constexpr uint32_t MARKER_RAW = size_t{3} << (std::numeric_limits<std::uint32_t>::digits - 3);
  inline constexpr uint32_t MARKER_PTR = size_t{4} << (std::numeric_limits<std::uint32_t>::digits - 3);

  inline bool is(uint32_t op, uint32_t marker) {
    return (op & MARKER_BLOCK) == marker;
  }

  inline bool is_val(uint32_t op) {
    return is(op, MARKER_VAL);
  }

  inline bool is_imm(uint32_t op) {
    return is(op, MARKER_IMM);
  }

  inline bool is_ptr(uint32_t op) {
    return is(op, MARKER_PTR);
  }

  inline bool is_raw(uint32_t op) {
    return is(op, MARKER_RAW);
  }

  inline bool is_pair(uint32_t op) {
    return is(op, MARKER_PAIR);
  }

  inline uint32_t content(size_t op) {
    return op & ~MARKER_BLOCK;
  }
}
