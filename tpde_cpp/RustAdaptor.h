#pragma once

#include "tpde.h"

#include <deps/tpde/tpde-llvm/src/base.hpp>
#include <tpde/IRAdaptor.hpp>
#include <tpde/RegisterFile.hpp>

namespace tpde_rust {
  // doing this with a lambda throws magical errors
  struct PointifyFunctor {
    template <typename T>
    auto operator()(T& item) const {
      return &item;
    }
  };
  constexpr auto view_pointify = std::views::transform(PointifyFunctor{});

  struct RustAdaptor {
    using IRValueRef = const Slot *;
    using IRInstRef = const Instruction *;
    using IRBlockRef = BasicBlock *;
    using IRFuncRef = Function *;

    static constexpr IRValueRef INVALID_VALUE_REF = nullptr;
    static constexpr IRBlockRef INVALID_BLOCK_REF = nullptr;
    static constexpr IRFuncRef INVALID_FUNC_REF = nullptr;

    static constexpr bool TPDE_PROVIDES_HIGHEST_VAL_IDX = true;
    static constexpr bool TPDE_LIVENESS_VISIT_ARGS = true;

    ModuleTpde *mod = nullptr;
    Function *cur_func = nullptr;

    struct ValInfo {
      Type type;
    };

    [[nodiscard]] u32 func_count() const { return mod->functions.size(); }

    [[nodiscard]] auto funcs() const {
      return mod->functions | std::views::transform([](Function &fn) { return &fn; });
    }

    [[nodiscard]] auto funcs_to_compile() const { return funcs(); }

    [[nodiscard]] static std::string_view func_link_name(const IRFuncRef func) {
      return func->name.data();
    }

    [[nodiscard]] static bool func_extern(const IRFuncRef func) {
      return func->extern_link;
    }

    [[nodiscard]] static bool func_only_local(const IRFuncRef func) {
      return func->only_local;
    }

    [[nodiscard]] static bool func_has_weak_linkage(const IRFuncRef func) {
      return func->weak_link;
    }

    [[nodiscard]] static bool cur_needs_unwind_info() {
      // TODO cur_func need unwind?
      return true;
    }

    [[nodiscard]] static bool cur_is_vararg() {
      // TODO cur_func is vararg?
      return false;
    }

    [[nodiscard]] u32 cur_highest_val_idx() const {
      return cur_func->slots.size();
    }

    [[nodiscard]] auto cur_args() const {
      return cur_func->slots
             | std::views::take(cur_func->n_args)
             | view_pointify;
    }

    [[nodiscard]] static bool cur_arg_is_byval(const u32 idx) {
      // TODO so far only byval supported
      return true;
    }

    [[nodiscard]] u32 cur_arg_byval_size(const u32 idx) const {
      return size_of_type(cur_func->slots[idx].ty);
    }

    [[nodiscard]] u32 cur_arg_byval_align(const u32 idx) const {
      return size_of_type(cur_func->slots[idx].ty);
    }

    [[nodiscard]] static u32 cur_arg_is_sret(const u32 idx) {
      // TODO so far only byval supported
      return false;
    }

    [[nodiscard]] static const auto &cur_static_allocas() {
      return std::views::empty<IRValueRef>;
    }

    [[nodiscard]] static auto cur_has_dynamic_alloca() {
      // TODO
      return false;
    }

    [[nodiscard]] IRBlockRef cur_entry_block() const {
      return &cur_func->basic_blocks.front();
    }

    [[nodiscard]] auto cur_blocks() const {
      return cur_func->basic_blocks
             | view_pointify;
    }

    [[nodiscard]] static auto block_succs(const IRBlockRef bb) {
      // TODO
      return std::views::empty<IRBlockRef>;
    }

    [[nodiscard]] static auto block_insts(const IRBlockRef bb) {
      return bb->instructions
             | view_pointify;
    }

    [[nodiscard]] static auto block_phis(const IRBlockRef bb) {
      // TODO
      return std::views::empty<IRValueRef>;
    }

    [[nodiscard]] static u32 block_info(const IRBlockRef bb) {
      return bb->info1;
    }

    void block_set_info(IRBlockRef bb, const u32 info) {
      bb->info1 = info;
    }

    [[nodiscard]] static u32 block_info2(const IRBlockRef bb) {
      return bb->info2;
    }

    void block_set_info2(IRBlockRef bb, const u32 info) {
      bb->info2 = info;
    }

    [[nodiscard]] std::string block_fmt_ref(const IRBlockRef block) const {
      // TODO
      return std::string(block->name);
    }

    [[nodiscard]] tpde::ValLocalIdx val_local_idx(const IRValueRef ir_value) const {
      return static_cast<tpde::ValLocalIdx>(std::ranges::distance(ir_value, cur_func->slots.data()));
    }

    [[nodiscard]] bool val_ignore_in_liveness_analysis(const IRValueRef value) const {
      // TODO
      return false;
    }

    [[nodiscard]] bool val_is_phi(const IRValueRef value) const {
      // TODO needed when phis nodes are added
      return false;
    }

    [[nodiscard]] auto val_as_phi(const IRValueRef value) const {
      struct PHIRef {

        [[nodiscard]] u32 incoming_count() const {
          return 0;
        }

        [[nodiscard]] IRValueRef incoming_val_for_slot(const u32 slot) const {
          return 0;
        }

        [[nodiscard]] IRBlockRef incoming_block_for_slot(const u32 slot) const {
          return 0;
        }

        [[nodiscard]] IRValueRef incoming_val_for_block(const IRBlockRef block) const {
          return 0;
        }
      };

      return PHIRef {};
    }

    [[nodiscard]] u32 val_alloca_size(IRValueRef val) const {
      return size_of_type(val->ty);
    }

    [[nodiscard]] u32 val_alloca_align(IRValueRef val) const {
      return size_of_type(val->ty);
    }

    [[nodiscard]] std::string value_fmt_ref(const IRValueRef val) const {
      // TODO
      return "value";
    }

    [[nodiscard]] auto inst_operands(const IRInstRef inst) const {
      return inst->ops
        | std::views::transform([this](const std::size_t i) { return &cur_func->slots[i]; });
    }

    [[nodiscard]] auto inst_results(const IRInstRef inst) const {
      return std::views::single(&cur_func->slots[inst->result]);
    }

    [[nodiscard]] bool inst_fused(const IRInstRef inst) const {
      return false;
    }

    ValInfo val_info(const Instruction *inst) const {
      return ValInfo{cur_func->slots[inst->result].ty};
    }

    [[nodiscard]] std::string inst_fmt_ref(const IRInstRef inst) const {
      return "Instance";
    }

    static void start_compile() {}

    static void end_compile() {}

    [[nodiscard]] bool switch_func(IRFuncRef func) {
      cur_func = func;
      return true;
    }

    bool switch_module(ModuleTpde &mod) {
      this->mod = &mod;
      return true;
    }

    void reset() {
      cur_func = INVALID_FUNC_REF;
    }

    // things for compiler

    struct ValueParts {
      Type ty;

      static u32 count() {
        return 1;
      }

      [[nodiscard]] Type type(u32 n) const {
        return ty;
      }

      [[nodiscard]] u32 size_bytes(u32 n) const {
        return size_of_type(ty);
      }

      static tpde::RegBank reg_bank(u32 n) {
        // TODO everything in basic register bank so far
        return tpde::RegBank{0};
      }
    };

    static ValueParts val_parts(const IRValueRef value) {
      return ValueParts{value->ty};
    }
  };

  static_assert(tpde::IRAdaptor<RustAdaptor>);
}
