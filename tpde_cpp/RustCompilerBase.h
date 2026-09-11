#pragma once
#include "RustAdaptor.h"
#include "RustCompiler.h"
#include "../deps/tpde/tpde/include/tpde/CompilerBase.hpp"

namespace tpde_rust {
  enum class LibFunc {
    divti3,
    udivti3,
    modti3,
    umodti3,
    fmod,
    fmodf,
    fmodf16,
    floorf,
    floor,
    ceilf,
    ceil,
    roundf,
    round,
    nearbyintf,
    nearbyint,
    rintf,
    rint,
    lround,
    lroundf,
    memcpy,
    memset,
    memmove,
    resume,
    powisf2,
    powidf2,
    trunc,
    truncf,
    fma,
    fmaf,
    pow,
    powf,
    sin,
    sinf,
    cos,
    cosf,
    tan,
    tanf,
    asin,
    asinf,
    acos,
    acosf,
    atan,
    atanf,
    atan2,
    atan2f,
    sinh,
    sinhf,
    cosh,
    coshf,
    tanh,
    tanhf,
    log,
    logf,
    logl,
    logf128,
    log2,
    log2f,
    log10,
    log10f,
    exp,
    expf,
    exp2,
    exp2f,
    modf,
    modff,
    frexp,
    frexpf,
    trunctfsf2,
    trunctfdf2,
    extendsftf2,
    extenddftf2,
    eqtf2,
    netf2,
    gttf2,
    getf2,
    lttf2,
    letf2,
    unordtf2,
    floatsitf,
    floatditf,
    floatunditf,
    floatunsitf,
    fixtfdi,
    fixunstfdi,
    addtf3,
    subtf3,
    multf3,
    divtf3,
    MAX
  };

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

    std::vector<SymRef> global_symbols;

    using ValInfo = Adaptor::ValInfo;

    struct ValRefSpecial {
      enum MODE : uint8_t {
        CONST = 4,
      };

      uint8_t mode;
      IRValueRef data;

    private:
      ValRefSpecial(uint8_t mode, IRValueRef ref) : mode(mode), data{ref} {
      }

    public:
      static ValRefSpecial make_const(IRValueRef data) {
        return {CONST, data};
      }
    };

    tpde::util::BumpAllocator<> const_allocator;

    explicit RustCompilerBase(RustAdaptor *adaptor) : Base{adaptor} {
      static_assert(tpde::Compiler<Derived, Config>);
      static_assert(std::is_same_v<Adaptor, RustAdaptor>);
    }

    Derived *derived() { return static_cast<Derived *>(this); }

    const Derived *derived() const { return static_cast<Derived *>(this); }

    bool compile_to_elf(ModuleTpde &mod, std::vector<uint8_t> &buf) override;

    static bool cur_func_may_emit_calls() { return true; }

    SymRef cur_personality_func() const;

    static bool try_force_fixed_assignment(IRValueRef) { return false; }

    void setup_var_ref_assignments() {
    }

    RustAdaptor::ValueParts val_parts(IRValueRef val) const {
      return this->adaptor->val_parts(val);
    }

    std::optional<ValRefSpecial> val_ref_special(IRValueRef value) {
      if (operands::is_const(value) || operands::is_global(value)
          || operands::is_global_ptr(value)) {
        return ValRefSpecial::make_const(value);
      }
      return std::nullopt;
    }

    ValuePart val_part_ref_special(ValRefSpecial &vrs, u32 part) {
      if (operands::is_const(vrs.data)) {
        Value &imm = this->adaptor->mod->consts[operands::content(vrs.data)];

        switch (imm.ty) {
          using enum Type;
          case Bool:
          case i8:
            return ValuePart(imm.data2, 1, tpde::RegBank{0});
          case i16:
            return ValuePart(imm.data2, 2, tpde::RegBank{0});
          case i32:
            return ValuePart(imm.data2, 4, tpde::RegBank{0});
          case i64:
            return ValuePart(imm.data2, 8, tpde::RegBank{0});
          case i128:
            switch (part) {
              case 0:
                return ValuePart(imm.data2, 8, tpde::RegBank{0});
              case 1:
                return ValuePart(imm.data1, 8, tpde::RegBank{0});
              default:
                throw std::runtime_error("invalid part");
            }
          case f32:
              return ValuePart(imm.data2, 4, tpde::RegBank{1});
          case f64:
            return ValuePart(imm.data2, 8, tpde::RegBank{1});

          default:
            throw std::runtime_error("not implemented");
        }
      } {
        uint32_t glob_start = this->adaptor->cur_func->allocas.size()
                              + this->adaptor->cur_func->slots.size();
        uint32_t glob_ptr_start = glob_start
                                  + this->adaptor->mod->globals.size();

        u32 gv_id = operands::content(vrs.data);
        u32 loc_id;
        if (operands::is_global(vrs.data)) {
          loc_id = glob_start + gv_id;
        } else if (operands::is_global_ptr(vrs.data)) {
          loc_id = glob_ptr_start + gv_id;
        } else {
          throw std::runtime_error("unknown special mode");
        }
        tpde::ValLocalIdx local_idx{loc_id};

        auto *assignment = this->val_assignment(local_idx);
        if (!assignment) {
          this->init_variable_ref(local_idx, loc_id - glob_start);
          assignment = this->val_assignment(local_idx);
        }
        return ValuePart{local_idx, assignment, 0, /*owned=*/false};
      }
    }

    void prologue_assign_arg(tpde::CCAssigner *cc_assigner,
                             u32 arg_idx,
                             IRValueRef arg) {
      u32 align = size_of_type(Base::adaptor->type_of_ref(arg)) / 8;
      bool allow_split = true; // TODO
      Base::prologue_assign_arg(cc_assigner, arg_idx, arg, align, allow_split);
    }

    void define_func_idx(IRFuncRef func, const u32 idx) {
      // As they are worked through in the same order as in the IR they should be equal
      assert(func - this->adaptor->mod->functions.data() == idx);
    }

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

    std::array<SymRef, static_cast<size_t>(LibFunc::MAX)> libfunc_syms;

    bool compile(ModuleTpde &mod);

    bool compile_inst(RustAdaptor::IRInstRef, InstRange);

    bool compile_unknown(RustAdaptor::IRInstRef inst, const ValInfo &, u64) {
      auto instr = this->adaptor->get_instruction(inst);
      assert(false);
    }

    bool compile_int_binary_op(RustAdaptor::IRInstRef, const ValInfo &, u64);

    bool compile_float_binary_op(RustAdaptor::IRInstRef, const ValInfo &, u64);

    bool compile_overflowable(RustAdaptor::IRInstRef, const ValInfo &, u64);

    bool compile_overflow(Instruction &, Instruction &);

    bool compile_ret(RustAdaptor::IRInstRef, const ValInfo &, u64);

    bool compile_br(RustAdaptor::IRInstRef, const ValInfo &, u64);

    bool compile_gep(RustAdaptor::IRInstRef, const ValInfo &, u64);

    bool compile_store(RustAdaptor::IRInstRef, const ValInfo &, u64);

    bool compile_store_generic(Instruction &, GenericValuePart &&);

    bool compile_load(RustAdaptor::IRInstRef, const ValInfo &, u64);

    bool compile_load_generic(Instruction &, GenericValuePart &&);

    bool compile_memcpy(RustAdaptor::IRInstRef, const ValInfo &, u64);

    bool compile_call(RustAdaptor::IRInstRef, const ValInfo &, u64);

    bool compile_cast(RustAdaptor::IRInstRef, const ValInfo &, u64);

    bool compile_int_ext(RustAdaptor::IRInstRef, const ValInfo &, u64);
    bool compile_int_trunc(RustAdaptor::IRInstRef, const ValInfo &, u64);
    bool compile_float_ext_trunc(RustAdaptor::IRInstRef, const ValInfo &, u64);
    bool compile_float_to_int(RustAdaptor::IRInstRef, const ValInfo &, u64);
    bool compile_int_to_float(RustAdaptor::IRInstRef, const ValInfo &, u64);

    bool compile_neg(RustAdaptor::IRInstRef, const ValInfo &, u64);
    bool compile_fneg(RustAdaptor::IRInstRef, const ValInfo &, u64);

    bool compile_not(RustAdaptor::IRInstRef, const ValInfo &, u64);

    bool compile_fcmp(RustAdaptor::IRInstRef, const ValInfo &, u64);

    SymRef get_libfunc_sym(LibFunc func);

    bool hook_post_func_sym_init();
  };

  #define DEFINE_U64_ENUM(Name, ...) \
    struct Name { \
        enum Value : u64 { __VA_ARGS__ }; \
        Value v = static_cast<Value>(0); \
        constexpr Name() = default; \
        constexpr Name(Value val) : v(val) {} \
        constexpr operator Value() const { return v; } \
    };

  DEFINE_U64_ENUM(FloatBinaryOp, add, sub, mul, div, rem)
  DEFINE_U64_ENUM(FloatCmpOp, OEQ, OGT, OGE, OLT, OLE, ONE, ORD, UNO, UEQ, UGT, UGE, ULT, ULE, UNE)
  DEFINE_U64_ENUM(OverflowOp, uadd, sadd, usub, ssub, umul, smul)

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

      // Int math
      set_fn(InstructionKind::Add, &Derived::compile_overflowable, IntBinaryOp::add);
      set_fn(InstructionKind::Sub, &Derived::compile_overflowable, IntBinaryOp::sub);
      set_fn(InstructionKind::Mul, &Derived::compile_overflowable, IntBinaryOp::mul);
      set_fn(InstructionKind::uDiv, &Derived::compile_int_binary_op, IntBinaryOp::udiv);
      set_fn(InstructionKind::sDiv, &Derived::compile_int_binary_op, IntBinaryOp::sdiv);
      set_fn(InstructionKind::uRem, &Derived::compile_int_binary_op, IntBinaryOp::urem);
      set_fn(InstructionKind::sRem, &Derived::compile_int_binary_op, IntBinaryOp::srem);
      set_fn(InstructionKind::Neg, &Derived::compile_neg);

      // Float math
      set_fn(InstructionKind::fAdd, &Derived::compile_float_binary_op, FloatBinaryOp::add);
      set_fn(InstructionKind::fSub, &Derived::compile_float_binary_op, FloatBinaryOp::sub);
      set_fn(InstructionKind::fMul, &Derived::compile_float_binary_op, FloatBinaryOp::mul);
      set_fn(InstructionKind::fDiv, &Derived::compile_float_binary_op, FloatBinaryOp::div);
      set_fn(InstructionKind::fRem, &Derived::compile_float_binary_op, FloatBinaryOp::rem);
      set_fn(InstructionKind::fNeg, &Derived::compile_fneg);

      // Logic
      set_fn(InstructionKind::And, &Derived::compile_int_binary_op, IntBinaryOp::land);
      set_fn(InstructionKind::Or, &Derived::compile_int_binary_op, IntBinaryOp::lor);
      set_fn(InstructionKind::Shl, &Derived::compile_int_binary_op, IntBinaryOp::shl);
      set_fn(InstructionKind::lShr, &Derived::compile_int_binary_op, IntBinaryOp::shr);
      set_fn(InstructionKind::aShr, &Derived::compile_int_binary_op, IntBinaryOp::ashr);
      set_fn(InstructionKind::Xor, &Derived::compile_int_binary_op, IntBinaryOp::lxor);
      set_fn(InstructionKind::Not, &Derived::compile_not);

      set_fn(InstructionKind::Ret, &Derived::compile_ret);


      // Int cmp
      set_fn(InstructionKind::CMPeq, &Derived::compile_cmp);
      set_fn(InstructionKind::CMPne, &Derived::compile_cmp);
      set_fn(InstructionKind::CMPult, &Derived::compile_cmp);
      set_fn(InstructionKind::CMPule, &Derived::compile_cmp);
      set_fn(InstructionKind::CMPugt, &Derived::compile_cmp);
      set_fn(InstructionKind::CMPuge, &Derived::compile_cmp);
      set_fn(InstructionKind::CMPslt, &Derived::compile_cmp);
      set_fn(InstructionKind::CMPsle, &Derived::compile_cmp);
      set_fn(InstructionKind::CMPsgt, &Derived::compile_cmp);
      set_fn(InstructionKind::CMPsge, &Derived::compile_cmp);

      // Float cmp
      set_fn(InstructionKind::RealOEQ, &Derived::compile_fcmp, FloatCmpOp::OEQ);
      set_fn(InstructionKind::RealOGT, &Derived::compile_fcmp, FloatCmpOp::OGT);
      set_fn(InstructionKind::RealOGE, &Derived::compile_fcmp, FloatCmpOp::OGE);
      set_fn(InstructionKind::RealOLT, &Derived::compile_fcmp, FloatCmpOp::OLT);
      set_fn(InstructionKind::RealOLE, &Derived::compile_fcmp, FloatCmpOp::OLE);
      set_fn(InstructionKind::RealONE, &Derived::compile_fcmp, FloatCmpOp::ONE);
      set_fn(InstructionKind::RealORD, &Derived::compile_fcmp, FloatCmpOp::ORD);
      set_fn(InstructionKind::RealUNO, &Derived::compile_fcmp, FloatCmpOp::UNO);
      set_fn(InstructionKind::RealUEQ, &Derived::compile_fcmp, FloatCmpOp::UEQ);
      set_fn(InstructionKind::RealUGT, &Derived::compile_fcmp, FloatCmpOp::UGT);
      set_fn(InstructionKind::RealUGE, &Derived::compile_fcmp, FloatCmpOp::UGE);
      set_fn(InstructionKind::RealULT, &Derived::compile_fcmp, FloatCmpOp::ULT);
      set_fn(InstructionKind::RealULE, &Derived::compile_fcmp, FloatCmpOp::ULE);
      set_fn(InstructionKind::RealUNE, &Derived::compile_fcmp, FloatCmpOp::UNE);

      set_fn(InstructionKind::GEP, &Derived::compile_gep);
      set_fn(InstructionKind::Store, &Derived::compile_store);
      set_fn(InstructionKind::Load, &Derived::compile_load);
      set_fn(InstructionKind::MemCpy, &Derived::compile_memcpy);
      set_fn(InstructionKind::Call, &Derived::compile_call);

      set_fn(InstructionKind::CondBr, &Derived::compile_condbr);
      set_fn(InstructionKind::Br, &Derived::compile_br);

      set_fn(InstructionKind::Cast, &Derived::compile_cast);
      set_fn(InstructionKind::Trunc, &Derived::compile_int_trunc);
      set_fn(InstructionKind::zExt, &Derived::compile_int_ext, /*sign=*/false);
      set_fn(InstructionKind::sExt, &Derived::compile_int_ext, /*sign=*/true);
      set_fn(InstructionKind::fTrunc, &Derived::compile_float_ext_trunc);
      set_fn(InstructionKind::fExt, &Derived::compile_float_ext_trunc);
      set_fn(InstructionKind::fTou, &Derived::compile_float_to_int, /*flags=sign,!sat*/0);
      set_fn(InstructionKind::fTos, &Derived::compile_float_to_int, /*flags=!sign,!sat*/1);
      set_fn(InstructionKind::fTou_sat, &Derived::compile_float_to_int, /*flags=!sign|sat*/0b10);
      set_fn(InstructionKind::fTos_sat, &Derived::compile_float_to_int, /*flags=sign|sat*/0b11);
      set_fn(InstructionKind::uTof, &Derived::compile_int_to_float, /*sign=*/false);
      set_fn(InstructionKind::sTof, &Derived::compile_int_to_float, /*sign=*/true);

      return res;
    }();

    Instruction *i = &this->adaptor->get_instruction(instr);
    const ValInfo val_info = this->adaptor->val_info(i);
    assert(static_cast<size_t>(i->kind) < fns.size());
    const auto [compile_fn, arg] = fns[static_cast<std::size_t>(i->kind)];
    return (derived()->*compile_fn)(instr, val_info, arg);
  }

  template<typename Adaptor, typename Derived, typename Config>
  bool RustCompilerBase<Adaptor, Derived, Config>::compile_ret(
    RustAdaptor::IRInstRef instr_ref, const ValInfo &, u64) {
    Instruction *instr = &this->adaptor->get_instruction(instr_ref);

    typename Base::RetBuilder rb{*derived(), *derived()->cur_cc_assigner()};
    if (!instr->ops.empty()) {
      for (auto op: instr->ops) {
        rb.add(op);
      }
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
        using enum Type;
        std::array<u8, static_cast<unsigned>(Last)> res{};
        res[unsigned(Bool)] = 1;
        res[unsigned(i8)] = 1;
        res[unsigned(i16)] = 1;
        res[unsigned(i32)] = 1;
        res[unsigned(i64)] = 2;
        return res;
      }();
      unsigned ty_idx = bvt_lut[unsigned(bvt)];
      return {fns[op.index()][ty_idx], ty_idx < 3};
    };

    unsigned int_width = size_of_type(Base::adaptor->type_of_ref(instr->result));
    const auto &operands = instr->ops;
    ValueRef lhs = this->val_ref(operands[0]);
    ValueRef rhs = this->val_ref(operands[1]);
    ValueRef res = this->result_ref(instr->result);

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

      (derived()->*encode_fn)(std::move(lhs_op), std::move(rhs_op), res_op);
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
      //   derived()->extract_element(lhs_unowned, elem_idx, elem_ty, e_lhs);
      //   derived()->extract_element(rhs_unowned, elem_idx, elem_ty, e_rhs);
      //   handle_part(encode_fn, true, std::move(e_lhs), std::move(e_rhs), e_res);
      //   // insert_element always treats res as unowned.
      //   derived()->insert_element(res, elem_idx, elem_ty, std::move(e_res));
      // }
    }
    return true;
  }

  template<typename Adaptor, typename Derived, typename Config>
  bool RustCompilerBase<Adaptor, Derived, Config>::compile_float_binary_op(
    RustAdaptor::IRInstRef inst, const ValInfo &val_info, u64 op) {
    Instruction &finstr = this->adaptor->get_instruction(inst);

    auto lhs = this->val_ref(finstr.ops[0]);
    auto rhs = this->val_ref(finstr.ops[1]);
    ValueRef res = this->result_ref(finstr.result);

    if (op == FloatBinaryOp::rem) {
      LibFunc lf;
      switch (val_info.type) {
        using enum Type;
        case f32: lf = LibFunc::fmodf;
          break;
        case f64: lf = LibFunc::fmod;
          break;
        default: return false;
      }

      auto cb = derived()->create_call_builder();
      cb->add_arg(lhs.part(0), tpde::CCAssignment{});
      cb->add_arg(rhs.part(0), tpde::CCAssignment{});
      cb->call(get_libfunc_sym(lf));
      cb->add_ret(res);
      return true;
    }

    using EncodeFnTy =
        bool (Derived::*)(GenericValuePart &&, GenericValuePart &&, ValuePart &&);
    EncodeFnTy encode_fn = nullptr;

    switch (val_info.type) {
      using enum Type;
      case f32:
        switch (op) {
          using enum FloatBinaryOp::Value;
          case add: encode_fn = &Derived::encode_addf32;
            break;
          case sub: encode_fn = &Derived::encode_subf32;
            break;
          case mul: encode_fn = &Derived::encode_mulf32;
            break;
          case div: encode_fn = &Derived::encode_divf32;
            break;
          default: TPDE_UNREACHABLE("invalid FloatBinaryOp");
        }
        break;
      case f64:
        switch (op) {
          using enum FloatBinaryOp::Value;
          case add: encode_fn = &Derived::encode_addf64;
            break;
          case sub: encode_fn = &Derived::encode_subf64;
            break;
          case mul: encode_fn = &Derived::encode_mulf64;
            break;
          case div: encode_fn = &Derived::encode_divf64;
            break;
          default: TPDE_UNREACHABLE("invalid FloatBinaryOp");
        }
        break;
      default: return false;
    }

    return (derived()->*encode_fn)(lhs.part(0), rhs.part(0), res.part(0));
  }

  template<typename Adaptor, typename Derived, typename Config>
  bool RustCompilerBase<Adaptor, Derived, Config>::compile_overflowable(RustAdaptor::IRInstRef instr,
                                                                        const ValInfo &info, u64 op) {
    auto instr_size = this->adaptor->get_basic_block(instr.block).instructions.size();
    if (instr_size > instr.inst) {
      Instruction &pot_overflow = this->adaptor->get_instruction(instr.next());
      if (pot_overflow.kind == InstructionKind::OverflowCheck) {
        Instruction &inst = this->adaptor->get_instruction(instr);
        if (!compile_overflow(inst, pot_overflow)) {
          return false;
        }

        if (instr_size > instr.next().inst) {
          Instruction &pot_condbr = this->adaptor->get_instruction(instr.next().next());
          if (pot_condbr.kind == InstructionKind::CondBr && pot_overflow.result == pot_condbr.ops[0]) {
            assert(this->analyzer.liveness_info(this->adaptor->val_local_idx(pot_overflow.result)).ref_count == 2);
            // We can drop the register used for the overflow check
            this->val_ref(pot_overflow.result).reset();

            bool is_signed = operands::content(pot_overflow.ops[0]);
            return derived()->compile_overflow_jump(pot_condbr, inst.kind, is_signed);
          }
        }
        return true;
      }
    }

    return this->compile_int_binary_op(instr, info, op);
  }

  template<typename Adaptor, typename Derived, typename Config>
  bool RustCompilerBase<Adaptor, Derived, Config>::compile_overflow(Instruction &op_instr, Instruction &of_instr) {
    ValueRef lhs = this->val_ref(op_instr.ops[0]);
    ValueRef rhs = this->val_ref(op_instr.ops[1]);
    ValueRef res = this->result_ref(op_instr.result);
    ValueRef of_res = this->result_ref(of_instr.result);

    OverflowOp op;
    bool is_signed = operands::content(of_instr.ops[0]);
    switch (op_instr.kind) {
      case InstructionKind::Add:
        op = is_signed ? OverflowOp::sadd : OverflowOp::uadd;
        break;
      case InstructionKind::Sub:
        op = is_signed ? OverflowOp::ssub : OverflowOp::usub;
        break;
      case InstructionKind::Mul:
        op = is_signed ? OverflowOp::smul : OverflowOp::umul;
        break;
      default:
        assert(false && "Only support integer types");
    }

    const Type ty = this->adaptor->type_of_ref(op_instr.ops[0]);
    switch (ty) {
      using enum Type;
      case i8:
      case i16:
      case i32:
      case i64:
      case i128:
        break;
      default:
        assert(false && "Only support integer types");
    }
    const auto width = size_of_type(ty);

    if (width == 128) {
      if (!derived()->handle_overflow_intrin_128(op,
                                                       lhs.part(0),
                                                       lhs.part(1),
                                                       rhs.part(0),
                                                       rhs.part(1),
                                                       res.part(0),
                                                       res.part(1),
                                                       of_res.part(0))) {
        return false;
      }
      return true;
    }

    u32 width_idx = 0;
    switch (width) {
      case 8: width_idx = 0;
        break;
      case 16: width_idx = 1;
        break;
      case 32: width_idx = 2;
        break;
      case 64: width_idx = 3;
        break;
      default: return false;
    }

    using EncodeFnTy = bool (Derived::*)(
      GenericValuePart &&, GenericValuePart &&, ValuePart &&, ValuePart &&);
    std::array<std::array<EncodeFnTy, 4>, 6> encode_fns = {
      {
        {
          &Derived::encode_of_add_u8,
          &Derived::encode_of_add_u16,
          &Derived::encode_of_add_u32,
          &Derived::encode_of_add_u64
        },
        {
          &Derived::encode_of_add_i8,
          &Derived::encode_of_add_i16,
          &Derived::encode_of_add_i32,
          &Derived::encode_of_add_i64
        },
        {
          &Derived::encode_of_sub_u8,
          &Derived::encode_of_sub_u16,
          &Derived::encode_of_sub_u32,
          &Derived::encode_of_sub_u64
        },
        {
          &Derived::encode_of_sub_i8,
          &Derived::encode_of_sub_i16,
          &Derived::encode_of_sub_i32,
          &Derived::encode_of_sub_i64
        },
        {
          &Derived::encode_of_mul_u8,
          &Derived::encode_of_mul_u16,
          &Derived::encode_of_mul_u32,
          &Derived::encode_of_mul_u64
        },
        {
          &Derived::encode_of_mul_i8,
          &Derived::encode_of_mul_i16,
          &Derived::encode_of_mul_i32,
          &Derived::encode_of_mul_i64
        },
      }
    };

    EncodeFnTy encode_fn = encode_fns[op][width_idx];
    (derived()->*encode_fn)(lhs.part(0), rhs.part(0), res.part(0), of_res.part(0));
    return true;
  }

  template<typename Adaptor, typename Derived, typename Config>
  bool RustCompilerBase<Adaptor, Derived, Config>::compile_gep(
    RustAdaptor::IRInstRef inst, const ValInfo &, u64) {
    // operands:
    // base - ptr
    // scale - raw
    // index - val or const

    RustAdaptor::IRInstRef gep_ref = inst;
    Instruction *gep = &this->adaptor->get_instruction(inst);

    GenericValuePart addr = typename GenericValuePart::Expr{}; {
      ValueRef index_vr{this};
      ValuePartRef index_vp{this};
      auto &expr = std::get<typename GenericValuePart::Expr>(addr.state);

      // Kept separate from expr.disp, we don't want to fold the displacement
      // whenever we add an index. Indices are sign-extended, but we must use an
      // unsigned integer here to correctly handle overflows.
      u64 displacement = 0;
      // If set, the base is actually a stack variable reference and expr.base is
      // still uninitialized.
      bool base_is_stack_var = false;

      auto [ptr_ref, base] = this->val_ref_single(gep->ops[0]);
      if (base.has_assignment() && base.assignment().is_stack_variable()) {
        base_is_stack_var = true;
      } else {
        expr.base = base.load_to_reg();
        if (base.can_salvage()) {
          expr.base = ScratchReg{this};
          std::get<ScratchReg>(expr.base).alloc_specific(base.salvage());
        }
      }

      // The instruction following the last fused GEP, if it might be fusable.
      Instruction *next_val = nullptr;
      RustAdaptor::IRInstRef next_ref{};
      do {
        const u64 scale = operands::content(gep->ops[1]);

        const IRValueRef idx = gep->ops[2];
        if (operands::is_const(idx)) {
          // Constant index: fold into the displacement.
          const Value &imm = this->adaptor->mod->consts[operands::content(idx)];
          displacement += static_cast<u64>(scale) * imm.data2;
        } else if (scale == 0) {
          // The index doesn't contribute anything, but we still have to
          // reference it for the reference counting to stay correct.
          (void) this->val_ref(idx);
        } else {
          if (base_is_stack_var) {
            addr = derived()->create_addr_for_alloca(base.assignment());
            assert(addr.is_expr());
            displacement += expr.disp;
            expr.disp = 0;
            base_is_stack_var = false;
          }

          if (expr.scale) {
            // We already have an index; materialize the current address
            // expression into a register and use it as the new base.
            derived()->gval_expr_as_reg(addr);
            index_vp.reset();
            index_vr.reset();
            base.reset();
            ptr_ref.reset();

            ScratchReg new_base = std::move(std::get<ScratchReg>(addr.state));
            addr = typename GenericValuePart::Expr{};
            expr.base = std::move(new_base);
          }

          const unsigned idx_width = size_of_type(this->adaptor->type_of_ref(idx));
          index_vr = this->val_ref(idx);
          if (idx_width != 64) {
            index_vp = index_vr.part(0).into_extended(true, idx_width, 64);
          } else {
            index_vp = index_vr.part(0);
          }
          if (index_vp.can_salvage()) {
            expr.index = ScratchReg{this};
            std::get<ScratchReg>(expr.index).alloc_specific(index_vp.salvage());
          } else {
            expr.index = index_vp.load_to_reg();
          }

          expr.scale = scale;
        }

        // Try to fuse the following instruction. This is only possible if it is
        // the sole user of this GEP and directly follows it.
        if (!gep->has_result) {
          break;
        }
        // The definition itself counts as one reference.
        const auto local_idx = this->adaptor->val_local_idx(gep->result);
        if (this->analyzer.liveness_info(local_idx).ref_count > 2) {
          break;
        }

        next_ref = gep_ref.next();
        const auto &insts = this->adaptor->get_basic_block(gep_ref.block).instructions;
        if (next_ref.inst >= insts.size()) {
          next_ref = {};
          break;
        }
        next_val = &this->adaptor->get_instruction(next_ref);

        if (true || // we don't merge multiple GEPs for now
            next_val->kind != InstructionKind::GEP ||
            next_val->ops[0] != gep->result) {
          break;
        }

        // Chain of GEPs: fold the next one into this address computation.
        gep_ref = next_ref;
        gep = next_val;
        next_val = nullptr;
      } while (true);

      if (base_is_stack_var) {
        if (!next_val) {
          // Create a new stack variable reference to avoid materializing this
          // simple addition.
          (void) this->result_ref_stack_slot(
            gep->result, base.assignment(), displacement);
          return true;
        }

        addr = derived()->create_addr_for_alloca(base.assignment());
        expr.disp += displacement;
      } else {
        expr.disp = displacement;
      }

      if (next_val) {
        if (next_val->kind == InstructionKind::Store &&
            next_val->ops[1] == gep->result) {
          return compile_store_generic(*next_val, std::move(addr));
        }
        if (next_val->kind == InstructionKind::Load &&
            next_val->ops[0] == gep->result) {
          return compile_load_generic(*next_val, std::move(addr));
        }
      }
    }

    auto [res_vr, res_ref] = this->result_ref_single(gep->result);

    AsmReg res_reg = derived()->gval_expr_as_reg(addr);
    if (auto *op_reg = std::get_if<ScratchReg>(&addr.state)) {
      res_ref.set_value(std::move(*op_reg));
    } else {
      derived()->mov(res_ref.alloc_reg(), res_reg, 8);
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
          derived()->create_addr_for_alloca(ptr_ref.assignment());

      return compile_store_generic(storei, std::move(addr));
    }
    return compile_store_generic(storei, std::move(ptr_ref));
  }

  template<typename Adaptor, typename Derived, typename Config>
  bool RustCompilerBase<Adaptor, Derived, Config>::compile_store_generic(
    Instruction &storei, GenericValuePart &&ptr_op) {
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
      using enum Type;
      case Bool:
      case i8:
      case i16:
      case i32:
      case i64: {
        const auto num_bytes = size_of_type(ty) / 8;
        EncodeFnTy fn = int_fns[num_bytes - 1];
        (derived()->*fn)(std::move(ptr_op), op_ref.part(0));
        return true;
      }
      case i128: {
        derived()->encode_storei128(std::move(ptr_op), op_ref.part(0), op_ref.part(1));
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
          derived()->create_addr_for_alloca(ptr_ref.assignment());

      return compile_load_generic(loadi, std::move(addr));
    }
    return compile_load_generic(loadi, std::move(ptr_ref));
  }

  template<typename Adaptor, typename Derived, typename Config>
  bool RustCompilerBase<Adaptor, Derived, Config>::compile_load_generic(
    Instruction &loadi, GenericValuePart &&ptr_op) {
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
      using enum Type;
      case Bool:
      case i8:
      case i16:
      case i32:
      case i64:
      case ptr: {
        const auto num_bytes = size_of_type(ty) / 8;
        EncodeFnTy fn = int_fns[num_bytes - 1][sext];

        (derived()->*fn)(std::move(ptr_op), this->result_ref(loadi.result).part(0));
        return true;
      }

      default: throw std::runtime_error("Unsupported type for loadi");
    }
  }

  template<typename Adaptor, typename Derived, typename Config>
  bool RustCompilerBase<Adaptor, Derived, Config>::compile_memcpy(RustAdaptor::IRInstRef inst, const ValInfo &, u64) {
    Instruction &memcpy = this->adaptor->get_instruction(inst);

    const auto dst = memcpy.ops[0];
    const auto src = memcpy.ops[2];
    const auto len = memcpy.ops[4];

    std::array<IRValueRef, 3> args{dst, src, len};

    derived()->create_helper_call(args, nullptr, get_libfunc_sym(LibFunc::memcpy));
    return true;
  }

  template<typename Adaptor, typename Derived, typename Config>
  bool RustCompilerBase<Adaptor, Derived, Config>::compile_call(RustAdaptor::IRInstRef instr, const ValInfo &, u64) {
    auto cb = derived()->create_call_builder();
    if (!cb) {
      return false;
    }

    Instruction &calli = this->adaptor->get_instruction(instr);
    for (auto &op: calli.ops | std::ranges::views::drop(1)) {
      using CallArg = typename Derived::CallArg;

      CallArg arg{op};
      // TODO need to set flags for arg

      cb->add_arg(arg);
    } {
      const auto func = calli.ops[0];
      assert(operands::is_func(func));
      SymRef sym = this->func_syms[operands::content(func)];
      cb->call(sym);
    }

    if (calli.has_result) {
      tpde::CCAssignment cca; {
        auto res = calli.result;
        assert(operands::is_val(res));
        ValueRef ref = this->result_ref(res);
        ValuePart part = ref.part(0);
        cb->add_ret(part, cca);
      }

      if (this->adaptor->get_basic_block(instr.block).instructions.size() > instr.inst) {
        Instruction &pot_addret = this->adaptor->get_instruction(instr.next());
        if (pot_addret.kind == InstructionKind::AddRet) {
          auto res = pot_addret.result;
          assert(operands::is_val(res));
          ValueRef ref = this->result_ref(res);
          ValuePart part = ref.part(0);
          cb->add_ret(part, cca);
        }
      }
    }

    return true;
  }

  template<typename Adaptor, typename Derived, typename Config>
  bool RustCompilerBase<Adaptor, Derived, Config>::compile_cast(RustAdaptor::IRInstRef instr, const ValInfo &val_info,
                                                                u64) {
    Instruction &casti = this->adaptor->get_instruction(instr);
    assert(operands::is_val(casti.ops[0]));
    assert(operands::is_val(casti.result));

    IRValueRef src_ref = operands::content(casti.ops[0]);
    IRValueRef res_ref = operands::content(casti.result);

    const Type src_ty = this->adaptor->type_of_ref(src_ref);
    const Type res_ty = this->adaptor->type_of_ref(res_ref);
    assert(size_of_type(src_ty) == size_of_type(res_ty));

    ValueRef src = this->val_ref(src_ref);
    ValueRef res = this->result_ref(res_ref);

    auto part_count = this->adaptor->val_parts(val_info).count();
    for (u32 i = 0; i != part_count; ++i) {
      res.part(i).set_value(src.part(i));
    }
    return true;
  }

  template<typename Adaptor, typename Derived, typename Config>
  bool RustCompilerBase<Adaptor, Derived, Config>::compile_int_ext(RustAdaptor::IRInstRef instr, const ValInfo &,
                                                                   u64 sign) {
    Instruction &exti = this->adaptor->get_instruction(instr);
    const Type dst_ty = this->adaptor->type_of_ref(exti.result);

    if (!is_integer(dst_ty))
      return false;

    auto src_val = exti.ops[0];
    const Type src_ty = this->adaptor->type_of_ref(src_val);

    unsigned src_width = size_of_type(src_ty);
    unsigned dst_width = size_of_type(dst_ty);
    assert(dst_width > src_width);

    auto src_ref = this->val_ref(src_val);
    auto res = this->result_ref(exti.result);

    if (src_width <= 64) {
      ValuePartRef low = src_ref.part(0);
      if (src_width < 64) {
        unsigned ext_width = dst_width <= 64 ? dst_width : 64;
        low = std::move(low).into_extended(sign, src_width, ext_width);
      }
      if (dst_width > 64) {
        auto res_ref_high = res.part(1);

        if (sign) {
          if (!low.has_reg()) {
            low.load_to_reg();
          }
          derived()->encode_fill_with_sign64(low.get_unowned_ref(), res_ref_high);
        } else {
          res_ref_high.set_value(ValuePart{u64{0}, 8, res_ref_high.bank()});
        }
      }

      res.part(0).set_value(std::move(low));
      return true;
    }

    if (src_width < 128 && dst_width <= 128) {
      res.part(0).set_value(src_ref.part(0));
      res.part(1).set_value(
        src_ref.part(1).into_extended(sign, src_width - 64, 64));
      return true;
    }

    return false;
  }

  template<typename Adaptor, typename Derived, typename Config>
  bool RustCompilerBase<Adaptor, Derived, Config>::compile_int_trunc(RustAdaptor::IRInstRef inst, const ValInfo &, u64) {
    const Instruction& trunci = this->adaptor->get_instruction(inst);
    auto val = trunci.ops[0];

    ValueRef src_vr = this->val_ref(val);
    ValueRef res_vr = this->result_ref(trunci.result);

    switch (this->adaptor->type_of_ref(val)) {
      using enum Type;
      case i8:
      case i16:
      case i32:
      case i64:
        // no-op, users will extend anyways. When truncating an i128, the first part
        // contains the lowest bits.
        res_vr.part(0).set_value(src_vr.part(0));
        return true;
      case i128:
        res_vr.part(0).set_value(src_vr.part(0));
        res_vr.part(1).set_value(src_vr.part(1));
        return true;
      default: return false;
    }
  }

  template<typename Adaptor, typename Derived, typename Config>
  bool RustCompilerBase<Adaptor, Derived, Config>::
  compile_float_ext_trunc(RustAdaptor::IRInstRef instref, const ValInfo &, u64) {
    const Instruction& inst = this->adaptor->get_instruction(instref);

    auto src_val = inst.ops[0];
    const Type src_ty = this->adaptor->type_of_ref(src_val);
    const Type dst_ty = this->adaptor->type_of_ref(inst.result);

    auto res_vr = this->result_ref(inst.result);

    if (src_ty == Type::f64 && dst_ty == Type::f32) {
      auto src_ref = this->val_ref(src_val);
      derived()->encode_f64tof32(src_ref.part(0), res_vr.part(0));
    } else if (src_ty == Type::f32 && dst_ty == Type::f64) {
      auto src_ref = this->val_ref(src_val);
      derived()->encode_f32tof64(src_ref.part(0), res_vr.part(0));
    } else {
      return false;
    }

    return true;
  }

  template<typename Adaptor, typename Derived, typename Config>
  bool RustCompilerBase<Adaptor, Derived, Config>::compile_float_to_int(RustAdaptor::IRInstRef instref, const ValInfo &, u64 flags) {
    const Instruction& inst = this->adaptor->get_instruction(instref);

    bool sign = flags & 0b01;
    bool saturate = flags & 0b10;

    auto src_val = inst.ops[0];
    const Type src_ty = this->adaptor->type_of_ref(src_val);

    const auto bit_width = size_of_type(this->adaptor->type_of_ref(inst.result));

    if (bit_width > 64) {
      return false;
    }

    unsigned ty_idx;
    switch (src_ty) {
      using enum Type;
      case f32: ty_idx = 0; break;
      case f64: ty_idx = 1; break;
      default: return false;
    }

    using EncodeFnTy = bool (Derived::*)(GenericValuePart &&, ValuePart &&);
    static constexpr auto fns = []() {
      // fns[is_double][dst64][sign][sat]
      std::array<EncodeFnTy[2][2][2], 2> fns{};
      fns[0][0][0][0] = &Derived::encode_f32tou32;
      fns[0][0][0][1] = &Derived::encode_f32tou32_sat;
      fns[0][0][1][0] = &Derived::encode_f32toi32;
      fns[0][0][1][1] = &Derived::encode_f32toi32_sat;
      fns[0][1][0][0] = &Derived::encode_f32tou64;
      fns[0][1][0][1] = &Derived::encode_f32tou64_sat;
      fns[0][1][1][0] = &Derived::encode_f32toi64;
      fns[0][1][1][1] = &Derived::encode_f32toi64_sat;
      fns[1][0][0][0] = &Derived::encode_f64tou32;
      fns[1][0][0][1] = &Derived::encode_f64tou32_sat;
      fns[1][0][1][0] = &Derived::encode_f64toi32;
      fns[1][0][1][1] = &Derived::encode_f64toi32_sat;
      fns[1][1][0][0] = &Derived::encode_f64tou64;
      fns[1][1][0][1] = &Derived::encode_f64tou64_sat;
      fns[1][1][1][0] = &Derived::encode_f64toi64;
      fns[1][1][1][1] = &Derived::encode_f64toi64_sat;
      return fns;
    }();
    EncodeFnTy fn = fns[ty_idx][bit_width > 32][sign][saturate];

    if (saturate && bit_width % 32 != 0) {
      // TODO: clamp result to smaller integer bounds
      return false;
    }

    auto src_ref = this->val_ref(src_val);
    auto res_ref = this->result_ref(inst.result);
    return (derived()->*fn)(src_ref.part(0), res_ref.part(0));
  }

  template<typename Adaptor, typename Derived, typename Config>
  bool RustCompilerBase<Adaptor, Derived, Config>::compile_int_to_float(RustAdaptor::IRInstRef instref, const ValInfo &val_info, u64 sign) {
    const Instruction& inst = this->adaptor->get_instruction(instref);
    const auto src_val = inst.ops[0];
    const Type dst_ty = this->adaptor->type_of_ref(inst.result);

    auto bit_width = size_of_type(this->adaptor->type_of_ref(src_val));
    if (bit_width > 64) {
      return false;
    }

    ValueRef src_ref = this->val_ref(src_val);
    ValuePartRef src_op = src_ref.part(0);
    ValueRef res = this->result_ref(inst.result);

    if (bit_width != 32 && bit_width != 64) {
      unsigned ext = tpde::util::align_up(bit_width, 32);
      src_op = std::move(src_op).into_extended(sign, bit_width, ext);
    }

    unsigned ty_idx;
    switch (val_info.type) {
      using enum Type;
      case f32: ty_idx = 0; break;
      case f64: ty_idx = 1; break;
      default: return false;
    }

    using EncodeFnTy = bool (Derived::*)(GenericValuePart &&, ValuePart &&);
    static constexpr auto encode_fns = []() consteval {
      std::array<EncodeFnTy[2][2], 2> res;
      res[0][0][0] = &Derived::encode_i32tof32;
      res[0][0][1] = &Derived::encode_i32tof64;
      res[0][1][0] = &Derived::encode_i64tof32;
      res[0][1][1] = &Derived::encode_i64tof64;
      res[1][0][0] = &Derived::encode_u32tof32;
      res[1][0][1] = &Derived::encode_u32tof64;
      res[1][1][0] = &Derived::encode_u64tof32;
      res[1][1][1] = &Derived::encode_u64tof64;
      return res;
    }();
    EncodeFnTy fn = encode_fns[!sign][bit_width > 32][ty_idx];
    (derived()->*fn)(std::move(src_op), res.part(0));
    return true;
  }

  template<typename Adaptor, typename Derived, typename Config>
  bool RustCompilerBase<Adaptor, Derived, Config>::compile_neg(RustAdaptor::IRInstRef instr, const ValInfo &, u64) {
    Instruction &negi = this->adaptor->get_instruction(instr);

    ValueRef src = this->val_ref(negi.ops[0]);
    ValueRef res = this->result_ref(negi.result);

    const Type type = this->adaptor->type_of_ref(negi.ops[0]);
    switch (type) {
      using enum Type;
      case Bool:
      case i8:
      case i16:
      case i32: derived()->encode_negi32(src.part(0), res.part(0));
        break;
      case i64: derived()->encode_negi64(src.part(0), res.part(0));
        break;
      case i128: derived()->encode_negi128(src.part(0), src.part(1), res.part(0), res.part(1));
        break;
      default: return false;
    }
    return true;
  }

  template<typename Adaptor, typename Derived, typename Config>
  bool RustCompilerBase<Adaptor, Derived, Config>::compile_fneg(RustAdaptor::IRInstRef inst, const ValInfo & val_info, u64) {
    Instruction& fnegi = this->adaptor->get_instruction(inst);
    ValueRef src = this->val_ref(fnegi.ops[0]);
    ValueRef res = this->result_ref(fnegi.result);
    switch (val_info.type) {
      using enum Type;
      case f32: derived()->encode_fnegf32(src.part(0), res.part(0)); break;
      case f64: derived()->encode_fnegf64(src.part(0), res.part(0)); break;
      default: return false;
    }
    return true;
  }

  template<typename Adaptor, typename Derived, typename Config>
  bool RustCompilerBase<Adaptor, Derived, Config>::compile_not(RustAdaptor::IRInstRef instr, const ValInfo &, u64) {
    Instruction &noti = this->adaptor->get_instruction(instr);

    ValueRef src = this->val_ref(noti.ops[0]);
    ValueRef res = this->result_ref(noti.result);

    const Type type = this->adaptor->type_of_ref(noti.ops[0]);
    switch (type) {
      using enum Type;
      case Bool: derived()->encode_notbool(src.part(0), res.part(0));
        break;
      case i8:
      case i16:
      case i32: derived()->encode_not32(src.part(0), res.part(0));
        break;
      case i64: derived()->encode_not64(src.part(0), res.part(0));
        break;
      case i128: derived()->encode_not128(src.part(0), src.part(1), res.part(0), res.part(1));
        break;
      default: return false;
    }
    return true;
  }

  template<typename Adaptor, typename Derived, typename Config>
  bool RustCompilerBase<Adaptor, Derived, Config>::compile_fcmp(RustAdaptor::IRInstRef instr, const ValInfo &, u64 op) {
    Instruction& fcmpi = this->adaptor->get_instruction(instr);
    Type type = this->adaptor->type_of_ref(fcmpi.ops[0]);

    ValueRef lhs = this->val_ref(fcmpi.ops[0]);
    ValueRef rhs = this->val_ref(fcmpi.ops[1]);
    ValueRef res = this->result_ref(fcmpi.result);

    using EncodeFnTy =
        bool (Derived::*)(GenericValuePart &&, GenericValuePart &&, ValuePart &&);
    EncodeFnTy fn = nullptr;

    switch (type) {
      using enum Type;
      using enum FloatCmpOp::Value;
      case f32:
        switch (op) {
          case OEQ: fn = &Derived::encode_fcmp_oeq_float; break;
          case OGT: fn = &Derived::encode_fcmp_ogt_float; break;
          case OGE: fn = &Derived::encode_fcmp_oge_float; break;
          case OLT: fn = &Derived::encode_fcmp_olt_float; break;
          case OLE: fn = &Derived::encode_fcmp_ole_float; break;
          case ONE: fn = &Derived::encode_fcmp_one_float; break;
          case ORD: fn = &Derived::encode_fcmp_ord_float; break;
          case UEQ: fn = &Derived::encode_fcmp_ueq_float; break;
          case UGT: fn = &Derived::encode_fcmp_ugt_float; break;
          case UGE: fn = &Derived::encode_fcmp_uge_float; break;
          case ULT: fn = &Derived::encode_fcmp_ult_float; break;
          case ULE: fn = &Derived::encode_fcmp_ule_float; break;
          case UNE: fn = &Derived::encode_fcmp_une_float; break;
          case UNO: fn = &Derived::encode_fcmp_uno_float; break;
          default: TPDE_UNREACHABLE("invalid fcmp predicate");
        }
        break;
      case f64:
        switch (op) {
          case OEQ: fn = &Derived::encode_fcmp_oeq_double; break;
          case OGT: fn = &Derived::encode_fcmp_ogt_double; break;
          case OGE: fn = &Derived::encode_fcmp_oge_double; break;
          case OLT: fn = &Derived::encode_fcmp_olt_double; break;
          case OLE: fn = &Derived::encode_fcmp_ole_double; break;
          case ONE: fn = &Derived::encode_fcmp_one_double; break;
          case ORD: fn = &Derived::encode_fcmp_ord_double; break;
          case UEQ: fn = &Derived::encode_fcmp_ueq_double; break;
          case UGT: fn = &Derived::encode_fcmp_ugt_double; break;
          case UGE: fn = &Derived::encode_fcmp_uge_double; break;
          case ULT: fn = &Derived::encode_fcmp_ult_double; break;
          case ULE: fn = &Derived::encode_fcmp_ule_double; break;
          case UNE: fn = &Derived::encode_fcmp_une_double; break;
          case UNO: fn = &Derived::encode_fcmp_uno_double; break;
          default: TPDE_UNREACHABLE("invalid fcmp predicate");
        }
        break;
      default: TPDE_UNREACHABLE("invalid fcmp type");
    }

    return (derived()->*fn)(lhs.part(0), rhs.part(0), res.part(0));
  }

  template<typename Adaptor, typename Derived, typename Config>
  RustCompilerBase<Adaptor, Derived, Config>::SymRef
  RustCompilerBase<Adaptor, Derived, Config>::get_libfunc_sym(LibFunc func) {
    assert(func < LibFunc::MAX);
    SymRef &sym = libfunc_syms[static_cast<size_t>(func)];
    if (sym.valid()) [[likely]] {
      return sym;
    }

    std::string_view name = "???";
    switch (func) {
      using enum LibFunc;
      case divti3: name = "__divti3";
        break;
      case udivti3: name = "__udivti3";
        break;
      case modti3: name = "__modti3";
        break;
      case umodti3: name = "__umodti3";
        break;
      case fmod: name = "fmod";
        break;
      case fmodf: name = "fmodf";
        break;
      case fmodf16: name = "fmodf16";
        break;
      case floorf: name = "floorf";
        break;
      case floor: name = "floor";
        break;
      case ceilf: name = "ceilf";
        break;
      case ceil: name = "ceil";
        break;
      case roundf: name = "roundf";
        break;
      case round: name = "round";
        break;
      case nearbyintf: name = "nearbyintf";
        break;
      case nearbyint: name = "nearbyint";
        break;
      case rintf: name = "rintf";
        break;
      case rint: name = "rint";
        break;
      case lround: name = "lround";
        break;
      case lroundf: name = "lroundf";
        break;
      case memcpy: name = "memcpy";
        break;
      case memset: name = "memset";
        break;
      case memmove: name = "memmove";
        break;
      case resume: name = "_Unwind_Resume";
        break;
      case powisf2: name = "__powisf2";
        break;
      case powidf2: name = "__powidf2";
        break;
      case trunc: name = "trunc";
        break;
      case truncf: name = "truncf";
        break;
      case fma: name = "fma";
        break;
      case fmaf: name = "fmaf";
        break;
      case pow: name = "pow";
        break;
      case powf: name = "powf";
        break;
      case sin: name = "sin";
        break;
      case sinf: name = "sinf";
        break;
      case cos: name = "cos";
        break;
      case cosf: name = "cosf";
        break;
      case tan: name = "tan";
        break;
      case tanf: name = "tanf";
        break;
      case asin: name = "asin";
        break;
      case asinf: name = "asinf";
        break;
      case acos: name = "acos";
        break;
      case acosf: name = "acosf";
        break;
      case atan: name = "atan";
        break;
      case atanf: name = "atanf";
        break;
      case atan2: name = "atan2";
        break;
      case atan2f: name = "atan2f";
        break;
      case sinh: name = "sinh";
        break;
      case sinhf: name = "sinhf";
        break;
      case cosh: name = "cosh";
        break;
      case coshf: name = "coshf";
        break;
      case tanh: name = "tanh";
        break;
      case tanhf: name = "tanhf";
        break;
      case log: name = "log";
        break;
      case logf: name = "logf";
        break;
      case logl: name = "logl";
        break;
      case logf128: name = "logf128";
        break;
      case log2: name = "log2";
        break;
      case log2f: name = "log2f";
        break;
      case log10: name = "log10";
        break;
      case log10f: name = "log10f";
        break;
      case exp: name = "exp";
        break;
      case expf: name = "expf";
        break;
      case exp2: name = "exp2";
        break;
      case exp2f: name = "exp2f";
        break;
      case modf: name = "modf";
        break;
      case modff: name = "modff";
        break;
      case frexp: name = "frexp";
        break;
      case frexpf: name = "frexpf";
        break;
      case trunctfsf2: name = "__trunctfsf2";
        break;
      case trunctfdf2: name = "__trunctfdf2";
        break;
      case extendsftf2: name = "__extendsftf2";
        break;
      case extenddftf2: name = "__extenddftf2";
        break;
      case eqtf2: name = "__eqtf2";
        break;
      case netf2: name = "__netf2";
        break;
      case gttf2: name = "__gttf2";
        break;
      case getf2: name = "__getf2";
        break;
      case lttf2: name = "__lttf2";
        break;
      case letf2: name = "__letf2";
        break;
      case unordtf2: name = "__unordtf2";
        break;
      case floatsitf: name = "__floatsitf";
        break;
      case floatditf: name = "__floatditf";
        break;
      case floatunsitf: name = "__floatunsitf";
        break;
      case floatunditf: name = "__floatunditf";
        break;
      case fixtfdi: name = "__fixtfdi";
        break;
      case fixunstfdi: name = "__fixunstfdi";
        break;
      case addtf3: name = "__addtf3";
        break;
      case subtf3: name = "__subtf3";
        break;
      case multf3: name = "__multf3";
        break;
      case divtf3: name = "__divtf3";
        break;
      default: TPDE_UNREACHABLE("invalid libfunc");
    }

    sym =
        this->assembler.sym_add_undef(name, tpde::Assembler::SymBinding::GLOBAL);
    return sym;
  }

  static tpde::Assembler::SymBinding convert_linkage(const Global &global) {
    if (global.flags.only_local)
      return tpde::Assembler::SymBinding::LOCAL;
    if (global.flags.weak_link)
      return tpde::Assembler::SymBinding::WEAK;
    return tpde::Assembler::SymBinding::GLOBAL;
  }

  template<typename Adaptor, typename Derived, typename Config>
  bool RustCompilerBase<Adaptor, Derived, Config>::hook_post_func_sym_init() {
    global_symbols.clear();

    global_symbols.reserve(this->adaptor->mod->globals.size());
    for (const Global &global: this->adaptor->mod->globals) {
      std::string_view name(global.name.data(), global.name.size());

      auto binding = convert_linkage(global);
      SymRef ref;
      if (global.thread_loc) {
        ref = this->assembler.sym_predef_tls(name, binding);
      } else if (global.flags.extern_link) {
        ref = this->assembler.sym_predef_data(name, binding);
      } else {
        ref = this->assembler.sym_add_undef(name, binding);
      }
      global_symbols.push_back(ref);

      // TODO declaration for linker
      // TODO visibility
    }

    size_t i = 0;
    for (Global global: this->adaptor->mod->globals) {
      SymRef &sym = global_symbols[i];

      tpde::SectionKind kind; {
        bool needs_relocs = !global.relocations.empty();
        bool init_zero = !global.init;
        bool read_only = global.read_only;
        if (global.thread_loc) {
          kind = init_zero ? tpde::SectionKind::ThreadBSS : tpde::SectionKind::ThreadData;
        } else if (!read_only && init_zero) {
          assert(!needs_relocs && "BSS section must not have relocations");
          kind = tpde::SectionKind::BSS;
        } else if (read_only) {
          kind = needs_relocs ? tpde::SectionKind::DataRelRO : tpde::SectionKind::ReadOnly;
        } else {
          kind = tpde::SectionKind::Data;
        }
      }
      SecRef sec = this->assembler.create_section(kind);

      // TODO find sym

      if (global.init) {
        u32 off;
        this->assembler.sym_def_predef_data(sec, sym, global.data, global.align, &off);
        for (Relocation &reloc: global.relocations) {
          assert(operands::is_global(reloc.slot));
          SymRef &target = global_symbols[operands::content(reloc.slot)];

          this->assembler.reloc_abs(sec, target, off + reloc.offset, 0);

          // would be the code for absolut relocation
          //   this->assembler.reloc_pc32(sec, target, off + inner_off, addend);
        }
      } else {
        this->assembler.sym_def_predef_zero(sec, sym, global.size, global.align);
      }
      ++i;
    }

    return true;
  }

  template<typename Adaptor, typename Derived, typename Config>
  bool RustCompilerBase<Adaptor, Derived, Config>::compile_br(RustAdaptor::IRInstRef instr, const ValInfo &, u64) {
    Instruction &bri = this->adaptor->get_instruction(instr);

    assert(operands::is_raw(bri.ops[0]));
    Base::generate_uncond_branch(operands::content(bri.ops[0]));

    return true;
  }
}
