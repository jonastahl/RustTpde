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
    struct ValRefSpecial {
      uint8_t mode = 4;
      IRValueRef const_data;
    };

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
      if (operands::is_imm(value)) {
        return ValRefSpecial { .const_data = operands::content(value) };
      }

      return std::nullopt;
    }

    ValuePart val_part_ref_special(ValRefSpecial &vrs, u32 part) {
      Value& imm = this->adaptor->mod->immediates[vrs.const_data];

      switch (imm.ty) {
        case Type::Bool:
        case Type::i8:
          return ValuePart(imm.data2, 1, tpde::RegBank{0});
        case Type::i16:
          return ValuePart(imm.data2, 2, tpde::RegBank{0});
        case Type::i32:
          return ValuePart(imm.data2, 4, tpde::RegBank{0});
        case Type::i64:
          return ValuePart(imm.data2, 8, tpde::RegBank{0});

        default:
          throw std::runtime_error("not implemented");
      }
    }

    void prologue_assign_arg(tpde::CCAssigner *cc_assigner,
                             u32 arg_idx,
                             IRValueRef arg) {
      u32 align = size_of_type(Base::adaptor->type_of_ref(arg));
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

    bool compile_inst(RustAdaptor::IRInstRef, InstRange);

    bool compile_unknown(RustAdaptor::IRInstRef inst, const ValInfo &, u64) {
      auto instr = this->adaptor->get_instruction(inst);
      assert(false);
    }

    bool compile_int_binary_op(RustAdaptor::IRInstRef, const ValInfo &, u64);

    bool compile_ret(RustAdaptor::IRInstRef, const ValInfo &, u64);

    bool compile_br(RustAdaptor::IRInstRef, const ValInfo &, u64);

    bool compile_store(RustAdaptor::IRInstRef, const ValInfo &, u64);

    bool compile_store_generic(Instruction&, GenericValuePart &&);

    bool compile_load(RustAdaptor::IRInstRef, const ValInfo &, u64);

    bool compile_load_generic(Instruction&, GenericValuePart &&);

    ValueRef val_ref_local(const size_t local_idx) {
      return this->val_ref(this->adaptor->val_ref_of_slot(local_idx));
    }
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
  bool RustCompilerBase<Adaptor, Derived, Config>::compile_inst(RustAdaptor::IRInstRef instr, InstRange) {
    TPDE_LOG_TRACE("Compiling inst {}", this->adaptor->inst_fmt_ref(i));
    static constexpr auto fns = []() constexpr {
      using CompileFn =
          bool (Derived::*)(RustAdaptor::IRInstRef, const ValInfo &, u64);
      std::array<std::pair<CompileFn, u64>, static_cast<size_t>(InstructionKind::Last)> res{};
      res.fill({&Derived::compile_unknown, 0});

      auto set_fn = [&](InstructionKind kind, CompileFn fn, u64 val = 0) {
        res[static_cast<std::size_t>(kind)] = {fn, val};
      };

      set_fn(InstructionKind::Add, &Derived::compile_int_binary_op, IntBinaryOp::add);
      set_fn(InstructionKind::Sub, &Derived::compile_int_binary_op, IntBinaryOp::sub);
      set_fn(InstructionKind::Mul, &Derived::compile_int_binary_op, IntBinaryOp::mul);
      set_fn(InstructionKind::Div, &Derived::compile_int_binary_op, IntBinaryOp::sdiv);

      set_fn(InstructionKind::Ret, &Derived::compile_ret);

      set_fn(InstructionKind::CMPeq, &Derived::compile_cmp);
      set_fn(InstructionKind::CMPne, &Derived::compile_cmp);
      set_fn(InstructionKind::CMPlt, &Derived::compile_cmp);
      set_fn(InstructionKind::CMPle, &Derived::compile_cmp);
      set_fn(InstructionKind::CMPgt, &Derived::compile_cmp);
      set_fn(InstructionKind::CMPge, &Derived::compile_cmp);

      set_fn(InstructionKind::Store, &Derived::compile_store);
      set_fn(InstructionKind::Load, &Derived::compile_load);

      set_fn(InstructionKind::CondBr, &Derived::compile_unknown);
      set_fn(InstructionKind::Br, &Derived::compile_br);

      return res;
    }();

    Instruction *i = &this->adaptor->get_instruction(instr);
    const ValInfo val_info = this->adaptor->val_info(i);
    assert(static_cast<size_t>(i->kind) < fns.size());
    const auto [compile_fn, arg] = fns[static_cast<std::size_t>(i->kind)];
    return (Base::derived()->*compile_fn)(instr, val_info, arg);
  }

  template<typename Adaptor, typename Derived, typename Config>
  bool RustCompilerBase<Adaptor, Derived, Config>::compile_ret(
    RustAdaptor::IRInstRef instr_ref, const ValInfo &info, u64 op_val) {
    Instruction *instr = &this->adaptor->get_instruction(instr_ref);

    typename Base::RetBuilder rb{*this->derived(), *this->derived()->cur_cc_assigner()};
    if (!instr->ops.empty()) {
      IRValueRef retval = this->adaptor->val_ref_of_slot(instr->ops[0]);
      rb.add(retval);
    }
    rb.ret();
    return true;
  }

  template<typename Adaptor, typename Derived, typename Config>
  bool RustCompilerBase<Adaptor, Derived, Config>::compile_int_binary_op(
    RustAdaptor::IRInstRef instr_ref, const ValInfo &info, u64 op_val) {
    Instruction *instr = &this->adaptor->get_instruction(instr_ref);

    IntBinaryOp op = typename IntBinaryOp::Value(op_val);
    auto parts = this->adaptor->val_parts(info);

    // TODO maybe have extra logic for i128

    using EncodeFnTy =
        bool (Derived::*)(GenericValuePart &&, GenericValuePart &&, ValuePart &);
    static constexpr auto fns = []() constexpr {
      std::array<EncodeFnTy[12], IntBinaryOp::num_ops> res{};
      auto entry = [&res](IntBinaryOp op) { return res[op.index()]; };

#define FN_ENTRY_INT(op, fn)                                                   \
    entry(op)[1] = &Derived::encode_##fn##i##32;                                 \
    entry(op)[2] = &Derived::encode_##fn##i##64;
#define FN_ENTRY_VEC(op, fn, sign)                                             \
    entry(op)[3] = &Derived::encode_##fn##v8##sign##8;                           \
    entry(op)[4] = &Derived::encode_##fn##v4##sign##16;                          \
    entry(op)[5] = &Derived::encode_##fn##v2##sign##32;                          \
    entry(op)[6] = &Derived::encode_##fn##v16##sign##8;                          \
    entry(op)[7] = &Derived::encode_##fn##v8##sign##16;                          \
    entry(op)[8] = &Derived::encode_##fn##v4##sign##32;                          \
    entry(op)[9] = &Derived::encode_##fn##v2##sign##64;
#define FN_ENTRY(op, fn, sign) FN_ENTRY_INT(op, fn) FN_ENTRY_VEC(op, fn, sign)

      FN_ENTRY(IntBinaryOp::add, add, u)
      FN_ENTRY(IntBinaryOp::sub, sub, u)
      FN_ENTRY(IntBinaryOp::mul, mul, u)
      FN_ENTRY_INT(IntBinaryOp::udiv, udiv)
      FN_ENTRY_INT(IntBinaryOp::sdiv, sdiv)
      FN_ENTRY_INT(IntBinaryOp::urem, urem)
      FN_ENTRY_INT(IntBinaryOp::srem, srem)
      FN_ENTRY(IntBinaryOp::land, land, u)
      FN_ENTRY(IntBinaryOp::lxor, lxor, u)
      FN_ENTRY(IntBinaryOp::lor, lor, u)
      FN_ENTRY(IntBinaryOp::shl, shl, u)
      FN_ENTRY(IntBinaryOp::shr, shr, u)
      FN_ENTRY(IntBinaryOp::ashr, ashr, i)
#undef FN_ENTRY
#undef FN_ENTRY_VEC
#undef FN_ENTRY_INT

      // i1 is special.
      entry(IntBinaryOp::add)[10] = &Derived::encode_lxori32;
      entry(IntBinaryOp::add)[11] = &Derived::encode_lxori64;
      entry(IntBinaryOp::sub)[10] = &Derived::encode_lxori32;
      entry(IntBinaryOp::sub)[11] = &Derived::encode_lxori64;
      entry(IntBinaryOp::mul)[10] = &Derived::encode_landi32;
      entry(IntBinaryOp::mul)[11] = &Derived::encode_landi64;
      // udiv: x/1 = x; x/0 = UB => and is equivalent
      entry(IntBinaryOp::udiv)[10] = &Derived::encode_landi32;
      entry(IntBinaryOp::udiv)[11] = &Derived::encode_landi64;
      // sdiv: 0/-1 = 0; -1/-1 = UB; x/0 = UB => and is equivalent
      entry(IntBinaryOp::sdiv)[10] = &Derived::encode_landi32;
      entry(IntBinaryOp::sdiv)[11] = &Derived::encode_landi64;
      // urem/srem are always zero, but we have no encode function to return zero.
      // For now, keep them unassigned.
      entry(IntBinaryOp::land)[10] = &Derived::encode_landi32;
      entry(IntBinaryOp::land)[11] = &Derived::encode_landi64;
      entry(IntBinaryOp::lxor)[10] = &Derived::encode_lxori32;
      entry(IntBinaryOp::lxor)[11] = &Derived::encode_lxori64;
      entry(IntBinaryOp::lor)[10] = &Derived::encode_lori32;
      entry(IntBinaryOp::lor)[11] = &Derived::encode_lori64;
      // shl/lshr/ashr are always poison, so we could use any operation... for
      // now, keep them unassigned.

      return res;
    }();
    auto get_encode_fn =
        [op](Type bvt) -> std::pair<EncodeFnTy, bool> {
      static constexpr auto bvt_lut = []() consteval {
        std::array<u8, unsigned(10)> res{};
        res[unsigned(Type::i8)] = 1;
        res[unsigned(Type::i16)] = 1;
        res[unsigned(Type::i32)] = 1;
        res[unsigned(Type::i64)] = 2;
        return res;
      }();
      unsigned ty_idx = bvt_lut[unsigned(bvt)];
      return {fns[op.index()][ty_idx], ty_idx < 3};
    };

    IRValueRef ir_res = this->adaptor->val_ref_of_slot(instr->result);

    unsigned int_width = size_of_type(Base::adaptor->type_of_ref(ir_res));
    const auto &operands = instr->ops;
    ValueRef lhs = this->val_ref_local(operands[0]);
    ValueRef rhs = this->val_ref_local(operands[1]);
    ValueRef res = this->result_ref(ir_res);

    auto handle_part = [this, int_width, op](EncodeFnTy encode_fn,
                                             bool is_scalar,
                                             ValuePartRef &&lhs_op,
                                             ValuePartRef &&rhs_op,
                                             ValuePartRef &res_op) {
      if (is_scalar) {
        if (op.is_symmetric() && lhs_op.is_const() && !rhs_op.is_const()) {
          // TODO(ts): this is a hack since the encoder can currently not do
          // commutable operations so we reorder immediates manually here
          std::swap(lhs_op, rhs_op);
        }

        // TODO(ts): optimize div/rem by constant to a shift?
        unsigned ext_width = tpde::util::align_up(int_width, 32);
        if (ext_width != int_width) {
          bool sext = op.is_signed();
          if (op.needs_lhs_ext()) {
            lhs_op = std::move(lhs_op).into_extended(sext, int_width, ext_width);
          }
          if (op.needs_rhs_ext()) {
            rhs_op = std::move(rhs_op).into_extended(sext, int_width, ext_width);
          }
        }
      }

      (this->derived()->*encode_fn)(std::move(lhs_op), std::move(rhs_op), res_op);
    };

    for (u32 i = 0, n = parts.count(); i != n; ++i) {
      const Type ty = parts.type(i);
      ValuePartRef res_part = res.part(i);
      if (auto [encode_fn, is_scalar] = get_encode_fn(ty); encode_fn) [[likely]] {
        handle_part(encode_fn, is_scalar, lhs.part(i), rhs.part(i), res_part);
        continue;
      }

      // TODO rn we dont support vectors
      return false;

      // This is a legal vector type for which we don't have an encode function.
      // Extract elements individually and use scalar functions.
      // if (!inst->result->isVectorTy() || int_width == 1) {
      //   return false;
      // }
      // TODO there are no vector types rn that we could not support

      // auto [elem_cnt, elem_ty] = basic_ty_vector_info(ty);
      // auto [encode_fn, is_scalar] = get_encode_fn(elem_ty);
      // assert(is_scalar && "vector element must be a scalar type");
      // if (!encode_fn) {
      //   return false;
      // }
      //
      // tpde::RegBank bank = this->adaptor->basic_ty_part_bank(elem_ty);
      // for (u32 j = 0; j != elem_cnt; ++j) {
      //   u32 elem_idx = i * elem_cnt + j;
      //   ValuePartRef e_res{this, bank};
      //   ValuePartRef e_lhs{this, bank};
      //   ValuePartRef e_rhs{this, bank};
      //   // TODO: we might pass the last element as owned. But this code is
      //   // fallback only, so don't bother optimizing.
      //   ValueRef lhs_unowned = lhs.disowned();
      //   ValueRef rhs_unowned = rhs.disowned();
      //   this->derived()->extract_element(lhs_unowned, elem_idx, elem_ty, e_lhs);
      //   this->derived()->extract_element(rhs_unowned, elem_idx, elem_ty, e_rhs);
      //   handle_part(encode_fn, true, std::move(e_lhs), std::move(e_rhs), e_res);
      //   // insert_element always treats res as unowned.
      //   this->derived()->insert_element(res, elem_idx, elem_ty, std::move(e_res));
      // }
    }
    return true;
  }

  template<typename Adaptor, typename Derived, typename Config>
  bool RustCompilerBase<Adaptor, Derived, Config>::compile_store(
    RustAdaptor::IRInstRef inst, const ValInfo &, u64) {
    Instruction &storei = this->adaptor->get_instruction(inst);

    auto [_, ptr_ref] = this->val_ref_single(storei.ops[1]);
    if (ptr_ref.has_assignment() && ptr_ref.assignment().is_stack_variable()) {
      GenericValuePart addr =
          this->derived()->create_addr_for_alloca(ptr_ref.assignment());

      return compile_store_generic(storei, std::move(addr));
    }
    return compile_store_generic(storei, std::move(ptr_ref));
  }

  template<typename Adaptor, typename Derived, typename Config>
  bool RustCompilerBase<Adaptor, Derived, Config>::compile_store_generic(
    Instruction& storei, GenericValuePart &&ptr_op) {

    const auto op_val = storei.ops[0];
    auto op_ref = this->val_ref(op_val);

    Type ty = this->adaptor->type_of_ref(op_val);

    using EncodeFnTy =
      bool (Derived::*)(GenericValuePart &&, GenericValuePart &&);
    static constexpr auto int_fns = []() consteval {
      std::array<EncodeFnTy, 8> res{};
      res[0] = &Derived::encode_storei8;
      res[1] = &Derived::encode_storei16;
      res[2] = &Derived::encode_storei24;
      res[3] = &Derived::encode_storei32;
      res[4] = &Derived::encode_storei40;
      res[5] = &Derived::encode_storei48;
      res[6] = &Derived::encode_storei56;
      res[7] = &Derived::encode_storei64;
      return res;
    }();

    switch (ty) {
      case Type::Bool:
      case Type::i8:
      case Type::i16:
      case Type::i32:
      case Type::i64: {
        const auto num_bytes = size_of_type(ty);
        EncodeFnTy fn = int_fns[(num_bytes - 1) / 8];
        (this->derived()->*fn)(std::move(ptr_op), op_ref.part(0));
        return true;
      }

      default: return false;
    }
  }

  template<typename Adaptor, typename Derived, typename Config>
  bool RustCompilerBase<Adaptor, Derived, Config>::compile_load(
    RustAdaptor::IRInstRef inst, const ValInfo &, u64) {
    Instruction &loadi = this->adaptor->get_instruction(inst);

    auto [_, ptr_ref] = this->val_ref_single(loadi.ops[0]);
    if (ptr_ref.has_assignment() && ptr_ref.assignment().is_stack_variable()) {
      GenericValuePart addr =
          this->derived()->create_addr_for_alloca(ptr_ref.assignment());

      return compile_load_generic(loadi, std::move(addr));
    }
    return compile_load_generic(loadi, std::move(ptr_ref));
  }

  template <typename Adaptor, typename Derived, typename Config>
  bool RustCompilerBase<Adaptor, Derived, Config>::compile_load_generic(
  Instruction& loadi, GenericValuePart &&ptr_op) {
    Type ty = this->adaptor->type_of_ref(loadi.result);

    using EncodeFnTy = bool (Derived::*)(GenericValuePart &&, ValuePart &&);
    static constexpr auto int_fns = []() consteval {
      std::array<EncodeFnTy[2], 8> res{};
      res[0][0] = &Derived::encode_loadi8_zext;
      res[0][1] = &Derived::encode_loadi8_sext;
      res[1][0] = &Derived::encode_loadi16_zext;
      res[1][1] = &Derived::encode_loadi16_sext;
      res[2][0] = &Derived::encode_loadi24;
      res[3][0] = &Derived::encode_loadi32_zext;
      res[3][1] = &Derived::encode_loadi32_sext;
      res[4][0] = &Derived::encode_loadi40;
      res[5][0] = &Derived::encode_loadi48;
      res[6][0] = &Derived::encode_loadi56;
      res[7][0] = &Derived::encode_loadi64;
      return res;
    }();

    bool sext = false;
    switch (ty) {
      case Type::Bool:
      case Type::i8:
      case Type::i16:
      case Type::i32:
      case Type::i64: {
        const auto num_bytes = size_of_type(ty);
        EncodeFnTy fn = int_fns[(num_bytes - 1) / 8][sext];

        (this->derived()->*fn)(std::move(ptr_op), this->result_ref(loadi.result).part(0));
        return true;
      }

      default: return false;
    }
  }

  template<typename Adaptor, typename Derived, typename Config>
  bool RustCompilerBase<Adaptor, Derived, Config>::compile_br(RustAdaptor::IRInstRef instr, const ValInfo &, u64) {
    Instruction &bri = this->adaptor->get_instruction(instr);

    assert(operands::is_raw(bri.ops[0]));
    Base::generate_uncond_branch(operands::content(bri.ops[0]));

    return true;
  }
}
