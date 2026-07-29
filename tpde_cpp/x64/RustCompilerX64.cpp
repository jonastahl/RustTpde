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
  };

  std::unique_ptr<RustCompiler> create_compiler() {
    auto adaptor = std::make_unique<RustAdaptor>();
    return std::make_unique<RustCompilerX64>(std::move(adaptor));
  }
}
