#pragma once
#include "RustAdaptor.h"
#include "RustCompiler.h"
#include "../deps/tpde/tpde/include/tpde/CompilerBase.hpp"

namespace tpde_rust {
  template<typename Adaptor, typename Derived, typename Config>
  struct RustCompilerBase : RustCompiler, tpde::CompilerBase<Adaptor, Derived, Config> {
    using Base = tpde::CompilerBase<Adaptor, Derived, Config>;

    using IRValueRef = Base::IRValueRef;
    using IRBlockRef = Base::IRBlockRef;
    using IRFuncRef = Base::IRFuncRef;
    using ScratchReg = Base::ScratchReg;
    using ValuePartRef = Base::ValuePartRef;
    using ValuePart = Base::ValuePart;
    using ValueRef = Base::ValueRef;
    using GenericValuePart = Base::GenericValuePart;
    using InstRange = Base::InstRange;

    using SecRef = tpde::SecRef;
    using SymRef = tpde::SymRef;

    using AsmReg = Base::AsmReg;

    using ValInfo = Adaptor::ValInfo;
    using ValRefSpecial = Base::ValRefSpecial;

    tpde::util::BumpAllocator<> const_allocator;

    explicit RustCompilerBase(RustAdaptor *adaptor) : Base{adaptor} {
      static_assert(tpde::Compiler<Derived, Config>);
      static_assert(std::is_same_v<Adaptor, RustAdaptor>);
    }

    bool compile_to_elf(ModuleTpde &mod, std::vector<uint8_t> &buf) override;

    static bool cur_func_may_emit_calls() { return true; }

    SymRef cur_personality_func() const;

    static bool try_force_fixed_assignment(IRValueRef) { return false; }

    RustAdaptor::ValueParts val_parts(IRValueRef val) const {
      return this->adaptor->val_parts(val);
    }

    std::optional<ValRefSpecial> val_ref_special(IRValueRef value) {
      // TODO we don't support constants or globals so far
      throw std::runtime_error("not implemented");
    }

    ValuePart val_part_ref_special(ValRefSpecial &vrs, u32 part) {
      // TODO we don't support constants or globals so far
      throw std::runtime_error("not implemented");
    }

    void prologue_assign_arg(tpde::CCAssigner *cc_assigner,
                             u32 arg_idx,
                             IRValueRef arg) {
      u32 align = size_of_type(arg->ty);
      bool allow_split = true; // TODO
      Base::prologue_assign_arg(cc_assigner, arg_idx, arg, align, allow_split);
    }

    void define_func_idx(IRFuncRef func, const u32 idx) {
      // TODO we could save the numbering of functions here?
    }

    // TODO can we use default var ref handling?
    // void setup_var_ref_assignments() {}
    // void load_address_of_var_reference(AsmReg dst, tpde::AssignmentPartRef ap);

    struct IntBinaryOp {
    private:
      static constexpr u32 index_mask = (1 << 4) - 1;
      static constexpr u32 bit_symm = 1 << 4;
      static constexpr u32 bit_signed = 1 << 5;
      static constexpr u32 bit_ext_lhs = 1 << 6;
      static constexpr u32 bit_ext_rhs = 1 << 7;
      static constexpr u32 bit_div = 1 << 8;
      static constexpr u32 bit_rem = 1 << 9;
      static constexpr u32 bit_shift = 1 << 10;

    public:
      enum Value : u32 {
        add = 0 | bit_symm,
        sub = 1,
        mul = 2 | bit_symm,
        udiv = 3 | bit_ext_lhs | bit_ext_rhs | bit_div,
        sdiv = 4 | bit_signed | bit_ext_lhs | bit_ext_rhs | bit_div,
        urem = 5 | bit_ext_lhs | bit_ext_rhs | bit_rem,
        srem = 6 | bit_signed | bit_ext_lhs | bit_ext_rhs | bit_rem,
        land = 7 | bit_symm,
        lor = 8 | bit_symm,
        lxor = 9 | bit_symm,
        shl = 10 | bit_shift,
        shr = 11 | bit_ext_lhs | bit_shift,
        ashr = 12 | bit_signed | bit_ext_lhs | bit_shift,
        num_ops = 13
      };

      Value op;

      constexpr IntBinaryOp(Value op) : op(op) {
      }

      /// Whether the operation is symmetric.
      constexpr bool is_symmetric() const { return op & bit_symm; }
      /// Whether the operation is signed and therefore needs sign-extension.
      constexpr bool is_signed() const { return op & bit_signed; }
      /// Whether the operation needs the first operand extended.
      constexpr bool needs_lhs_ext() const { return op & bit_ext_lhs; }
      /// Whether the operation needs the second operand extended.
      constexpr bool needs_rhs_ext() const { return op & bit_ext_rhs; }
      /// Whether the operation is a div
      constexpr bool is_div() const { return op & bit_div; }
      /// Whether the operation is a rem
      constexpr bool is_rem() const { return op & bit_rem; }
      /// Whether the operation is a shift
      constexpr bool is_shift() const { return op & bit_shift; }

      constexpr unsigned index() const { return op & index_mask; }

      bool operator==(const IntBinaryOp &o) const { return op == o.op; }
    };

    struct FloatBinaryOp {
      enum {
        add,
        sub,
        mul,
        div,
        rem
      };
    };

    enum class OverflowOp {
      uadd,
      sadd,
      usub,
      ssub,
      umul,
      smul
    };

    bool compile(ModuleTpde &mod);

    bool compile_inst(const Instruction *, InstRange);

    bool compile_unknown(const Instruction *, const ValInfo &, u64) {
      return false;
    }

    bool compile_int_binary_op(const Instruction *, const ValInfo &, u64);

    bool compile_ret(const Instruction *, const ValInfo &, u64);

    bool compile_ret_void(const Instruction *, const ValInfo &, u64);
  };

  template<typename Adaptor, typename Derived, typename Config>
  bool RustCompilerBase<Adaptor, Derived, Config>::compile_to_elf(
    ModuleTpde &mod, std::vector<uint8_t> &buf) {
    if (this->adaptor->mod) {
      Base::derived()->reset();
    }
    if (!compile(mod)) {
      return false;
    }

    buf = this->assembler.build_object_file();
    return true;
  }

  template<typename Adaptor, typename Derived, typename Config>
  RustCompilerBase<Adaptor, Derived, Config>::SymRef
  RustCompilerBase<Adaptor, Derived, Config>::cur_personality_func() const {
    // TODO
    return {};
  }

  template<typename Adaptor, typename Derived, typename Config>
  bool RustCompilerBase<Adaptor, Derived, Config>::compile(ModuleTpde &mod) {
    if (!this->adaptor->switch_module(mod)) {
      return false;
    }

    if (!Base::compile()) {
      return false;
    }

    return true;
  }

  template<typename Adaptor, typename Derived, typename Config>
  bool RustCompilerBase<Adaptor, Derived, Config>::compile_inst(const Instruction *i, InstRange) {
    TPDE_LOG_TRACE("Compiling inst {}", this->adaptor->inst_fmt_ref(i));
    static constexpr auto fns = []() constexpr {
      using CompileFn =
          bool (Derived::*)(const Instruction *, const ValInfo &, u64);
      std::array<std::pair<CompileFn, u64>, 20> res{};
      res.fill({&Derived::compile_unknown, 0});

      auto set_fn = [&](InstructionKind kind, CompileFn fn, u64 val = 0) {
        res[static_cast<std::size_t>(kind)] = {fn, val};
      };

      set_fn(InstructionKind::Add, &Derived::compile_int_binary_op, IntBinaryOp::add);
      set_fn(InstructionKind::Sub, &Derived::compile_int_binary_op, IntBinaryOp::sub);
      set_fn(InstructionKind::Mul, &Derived::compile_int_binary_op, IntBinaryOp::mul);
      set_fn(InstructionKind::Div, &Derived::compile_int_binary_op, IntBinaryOp::sdiv);

      set_fn(InstructionKind::Ret, &Derived::compile_ret);
      set_fn(InstructionKind::RetVoid, &Derived::compile_ret_void);

      return res;
    }();

    const ValInfo val_info = this->adaptor->val_info(i);
    assert(static_cast<size_t>(i->kind) < fns.size());
    const auto [compile_fn, arg] = fns[static_cast<std::size_t>(i->kind)];
    return (Base::derived()->*compile_fn)(i, val_info, arg);
  }

  template<typename Adaptor, typename Derived, typename Config>
  bool RustCompilerBase<Adaptor, Derived, Config>::compile_ret(
    const Instruction *inst, const ValInfo &info, u64 op_val) {
    // TODO
    return false;
  }

  template<typename Adaptor, typename Derived, typename Config>
  bool RustCompilerBase<Adaptor, Derived, Config>::compile_ret_void(
    const Instruction *inst, const ValInfo &info, u64 op_val) {
    // TODO
    return false;
  }

  template<typename Adaptor, typename Derived, typename Config>
  bool RustCompilerBase<Adaptor, Derived, Config>::compile_int_binary_op(
    const Instruction *inst, const ValInfo &info, u64 op_val) {
    // TODO
    return false;
  }
}
