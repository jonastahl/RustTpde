#include "RustCompilerX64.h"

#include "encode_template_x64.hpp"

namespace tpde_rust::x64 {

  struct CompilerConfig : tpde::x64::PlatformConfig {
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

    static GenericValuePart create_addr_for_alloca(tpde::AssignmentPartRef ap);

    void create_helper_call(std::span<IRValueRef> args,
                        ValueRef *result,
                        SymRef sym);

    std::optional<CallBuilder> create_call_builder();
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
}
