#include "RustCompilerBase.h"

namespace tpde_rust {
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

      // clang-format off

      // Terminators
      res[InstructionKind::Add] = {&Derived::compile_int_binary_op, IntBinaryOp::add};
      res[InstructionKind::Sub] = {&Derived::compile_int_binary_op, IntBinaryOp::sub};
      res[InstructionKind::Mul] = {&Derived::compile_int_binary_op, IntBinaryOp::mul};
      res[InstructionKind::Div] = {&Derived::compile_int_binary_op, IntBinaryOp::sdiv};

      res[InstructionKind::Ret] = {&Derived::compile_ret, 0};
      res[InstructionKind::RetVoid] = {&Derived::compile_ret_void, 0};
      // clang-format on
      return res;
    }();

    const ValInfo &val_info = this->adaptor->val_info(i);
    assert(i->kind < fns.size());
    const auto [compile_fn, arg] = fns[i->kind];
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
