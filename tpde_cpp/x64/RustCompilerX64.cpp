#include "RustCompilerX64.h"

#include "encode_template_x64.hpp"

namespace tpde_rust::x64 {

  struct CompilerConfig : tpde::x64::PlatformConfig {
    static constexpr bool DEFAULT_VAR_REF_HANDLING = false;
  };

  struct RustCompilerX64 :
    tpde::x64::CompilerX64<
      RustAdaptor,
      RustCompilerX64,
      RustCompilerBase,
      CompilerConfig
    >,
    tpde_encodegen::EncodeCompiler<
      RustAdaptor,
      RustCompilerX64,
      RustCompilerBase,
      CompilerConfig
    >  {
    using Base = tpde::x64::CompilerX64<
      RustAdaptor,
      RustCompilerX64,
      RustCompilerBase,
      CompilerConfig
    >;

    using ScratchReg = Base::ScratchReg;
    using ValuePartRef = Base::ValuePartRef;
    using ValuePart = Base::ValuePart;
    using ValueRef = Base::ValueRef;
    using GenericValuePart = Base::GenericValuePart;

    using AsmReg = Base::AsmReg;

    std::unique_ptr<RustAdaptor> adaptor;

    std::variant<std::monostate, tpde::x64::CCAssignerSysV> cc_assigners;

    static constexpr std::array<AsmReg, 2> LANDING_PAD_RES_REGS = {AsmReg::AX,
                                                                   AsmReg::DX};

    explicit RustCompilerX64(std::unique_ptr<RustAdaptor> &&adaptor)
      : Base{adaptor.get()},
        adaptor(std::move(adaptor)) {
      static_assert(tpde::Compiler<RustCompilerX64, CompilerConfig>);
    }

    void reset() {
      Base::reset();
      EncodeCompiler::reset();
    }

    bool compile_cmp(RustAdaptor::IRInstRef inst, const ValInfo &, u64);
    bool compile_overflow_jump(Instruction&, InstructionKind, bool);

    static GenericValuePart create_addr_for_alloca(tpde::AssignmentPartRef ap);

    void create_helper_call(std::span<IRValueRef> args,
                        ValueRef *result,
                        SymRef sym);

    std::optional<CallBuilder> create_call_builder();

    void load_address_of_var_reference(tpde::x64::AsmReg dst, tpde::AssignmentPartRef ap);

    bool handle_overflow_intrin_128(OverflowOp op,
                                GenericValuePart &&lhs_lo,
                                GenericValuePart &&lhs_hi,
                                GenericValuePart &&rhs_lo,
                                GenericValuePart &&rhs_hi,
                                ValuePart &&res_lo,
                                ValuePart &&res_hi,
                                ValuePart &&res_of);
  };

  std::unique_ptr<RustCompiler> create_compiler() {
    auto adaptor = std::make_unique<RustAdaptor>();
    return std::make_unique<RustCompilerX64>(std::move(adaptor));
  }

  bool RustCompilerX64::compile_cmp(const RustAdaptor::IRInstRef inst, const ValInfo &, u64) {
    Instruction& cmpi = adaptor->get_instruction(inst);

    // check if we can fuse it
    if (adaptor->get_basic_block(inst.block).instructions.size() > inst.inst) {
      const auto jmpi = adaptor->get_instruction(inst.next());
      if (jmpi.kind == InstructionKind::CondBr && cmpi.result == jmpi.ops[0]) {
        const IRValueRef left = cmpi.ops[0];
        const IRValueRef right = cmpi.ops[1];

        assert(operands::is_val(left));

        auto lhs = this->val_ref_local(left);
        auto lhs_op = lhs.part(0);

        const Type tyl = Base::adaptor->type_of_ref(left);
        const Type tyr = Base::adaptor->type_of_ref(right);
        assert(tyl == tyr);

        const auto lhs_reg = lhs_op.has_reg() ? lhs_op.cur_reg() : lhs_op.load_to_reg();

        if (operands::is_val(right)) {
          auto rhs = this->val_ref_local(operands::content(right));
          auto rhs_op = rhs.part(0);
          const auto rhs_reg = rhs_op.has_reg() ? rhs_op.cur_reg() : rhs_op.load_to_reg();

          switch (tyl) {
            case Type::i8: ASM(CMP8rr, lhs_reg, rhs_reg); break;
            case Type::i16: ASM(CMP16rr, lhs_reg, rhs_reg); break;
            case Type::i32: ASM(CMP32rr, lhs_reg, rhs_reg); break;
            case Type::i64: ASM(CMP64rr, lhs_reg, rhs_reg); break;
            default:
              TPDE_UNREACHABLE("Invalid type");
          }
        } else if (operands::is_const(right)) {
          // TODO support has_assignment mi operations
          uint64_t imm_h;
          uint64_t imm_l;
          {
            const auto imm_ref = operands::content(right);

            const auto& imm_info = this->adaptor->mod->consts[imm_ref];
            imm_h = imm_info.data1;
            imm_l = imm_info.data2;
          }

          switch (tyl) {
            case Type::i8: ASM(CMP8ri, lhs_reg, static_cast<i8>(imm_l)); break;
            case Type::i16: ASM(CMP16ri, lhs_reg, static_cast<i16>(imm_l)); break;
            case Type::i32: ASM(CMP32ri, lhs_reg, static_cast<i32>(imm_l)); break;
            case Type::i64: {
              u64 val = static_cast<i64>(imm_l);
              if (i32(val) == val) {
                ASM(CMP64ri, lhs_reg, imm_l);
              } else {
                ScratchReg scratch{this};
                AsmReg tmp_reg = scratch.alloc_gp();
                materialize_constant(&imm_l, tpde::RegBank{0}, 8, tmp_reg);
                ASM(CMP64rr, lhs_reg, tmp_reg);
              }
              break;
            }
            default:
              TPDE_UNREACHABLE("Invalid type");
          }
        } else {
          TPDE_UNREACHABLE("Invalid rhs");
        }

        Jump jump;
        switch (cmpi.kind) {
          case InstructionKind::CMPeq: jump = Jump::je; break;
          case InstructionKind::CMPne: jump = Jump::jne; break;
          case InstructionKind::CMPugt: jump = Jump::ja; break;
          case InstructionKind::CMPuge: jump = Jump::jae; break;
          case InstructionKind::CMPult: jump = Jump::jb; break;
          case InstructionKind::CMPule: jump = Jump::jbe; break;
          case InstructionKind::CMPsgt: jump = Jump::jg; break;
          case InstructionKind::CMPsge: jump = Jump::jge; break;
          case InstructionKind::CMPslt: jump = Jump::jl; break;
          case InstructionKind::CMPsle: jump = Jump::jle; break;
            // TODO add for all the unsigned things
          default: TPDE_UNREACHABLE("invalid icmp predicate");
        }
        generate_cond_branch(jump, operands::content(jmpi.ops[1]), operands::content(jmpi.ops[2]));

        return true;
      }
    }

    // Only support fusing cmp and condbr by now
    return false;
  }

  bool RustCompilerX64::compile_overflow_jump(Instruction& jmpi, InstructionKind kind, bool is_signed) {
    Jump jump;
    switch (kind) {
      case InstructionKind::Add:
      case InstructionKind::Sub:
        jump = is_signed ? Jump::jo : Jump::jb;
        break;
      case InstructionKind::Mul:
        jump = Jump::jo;
        break;
      default: TPDE_UNREACHABLE("Invalid op for overflow");
    }
    generate_cond_branch(jump, operands::content(jmpi.ops[1]), operands::content(jmpi.ops[2]));
    return true;
  }

  RustCompilerX64::GenericValuePart
    RustCompilerX64::create_addr_for_alloca(tpde::AssignmentPartRef ap) {
    return GenericValuePart::Expr{AsmReg::BP, ap.variable_stack_off()};
  }

  void RustCompilerX64::create_helper_call(std::span<IRValueRef> args, ValueRef *result, SymRef sym) {
    tpde::util::SmallVector<CallArg, 8> arg_vec{};
    for (auto arg : args) {
      arg_vec.push_back(CallArg{arg});
    }

    generate_call(sym, arg_vec, result);
  }

  std::optional<tpde::x64::CompilerX64<RustAdaptor, RustCompilerX64, RustCompilerBase, CompilerConfig>::CallBuilder>
  RustCompilerX64::create_call_builder() {
    cc_assigners = tpde::x64::CCAssignerSysV(false);
    return CallBuilder{*this, std::get<tpde::x64::CCAssignerSysV>(cc_assigners)};
  }

  void RustCompilerX64::load_address_of_var_reference(tpde::x64::AsmReg dst, tpde::AssignmentPartRef ap) {
    const uint32_t glob_ptr_start = this->adaptor->mod->globals.size();

    uint32_t glob_id = ap.variable_ref_data();
    uint32_t offset = 0;
    if (glob_id >= glob_ptr_start) {
      auto &[id, off] = this->adaptor->mod->global_ptrs[glob_id - glob_ptr_start];
      glob_id = id;
      offset = off;
    }
    assert(glob_id < this->adaptor->mod->globals.size());
    assert(glob_id < this->global_symbols.size());
    const Global& global = this->adaptor->mod->globals[glob_id];
    const auto sym = this->global_symbols[glob_id];
    assert(sym.valid());

    if (global.flags.extern_link) {
      // mov the ptr from the GOT
      ASM(MOV64rm, dst, FE_MEM(FE_IP, 0, FE_NOREG, -1));
      reloc_text(sym, tpde::elf::R_X86_64_GOTPCREL, text_writer.offset() - 4, - 4);
      if (offset != 0) {
        ASM(LEA64rm, dst, FE_MEM(dst, 0, FE_NOREG, static_cast<int32_t>(offset)));
      }
    } else {
      // emit lea with relocation
      ASM(LEA64rm, dst, FE_MEM(FE_IP, 0, FE_NOREG, -1));
      reloc_text(sym, tpde::elf::R_X86_64_PC32, text_writer.offset() - 4, static_cast<int64_t>(offset) - 4);
    }
  }

  bool RustCompilerX64::handle_overflow_intrin_128(OverflowOp op, GenericValuePart &&lhs_lo, GenericValuePart &&lhs_hi,
    GenericValuePart &&rhs_lo, GenericValuePart &&rhs_hi, ValuePart &&res_lo, ValuePart &&res_hi, ValuePart &&res_of) {

    using EncodeFnTy = bool (RustCompilerX64::*)(GenericValuePart &&,
                                                 GenericValuePart &&,
                                                 GenericValuePart &&,
                                                 GenericValuePart &&,
                                                 ValuePart &,
                                                 ValuePart &,
                                                 ValuePart &);
    EncodeFnTy encode_fn = nullptr;
    switch (op) {
      case OverflowOp::uadd:
        encode_fn = &RustCompilerX64::encode_of_add_u128;
        break;
      case OverflowOp::sadd:
        encode_fn = &RustCompilerX64::encode_of_add_i128;
        break;
      case OverflowOp::usub:
        encode_fn = &RustCompilerX64::encode_of_sub_u128;
        break;
      case OverflowOp::ssub:
        encode_fn = &RustCompilerX64::encode_of_sub_i128;
        break;
      case OverflowOp::umul:
        encode_fn = &RustCompilerX64::encode_of_mul_u128;
        break;
      case OverflowOp::smul:
        encode_fn = &RustCompilerX64::encode_of_mul_i128;
        break;
      default: TPDE_UNREACHABLE("invalid operation");
    }

    return (this->*encode_fn)(std::move(lhs_lo),
                              std::move(lhs_hi),
                              std::move(rhs_lo),
                              std::move(rhs_hi),
                              res_lo,
                              res_hi,
                              res_of);
  }
}
