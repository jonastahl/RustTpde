#include "RustCompilerX64.h"

namespace tpde_rust::x64 {

  struct CompilerConfig : tpde::x64::PlatformConfig {
  };

  class RustCompilerX64;
  using Base = tpde::x64::CompilerX64<
    RustAdaptor,
    RustCompilerX64,
    RustCompiler,
    CompilerConfig
  >;

  class RustCompilerX64
    // : public Base
  {

    // using ScratchReg = typename Base::ScratchReg;
    // using ValuePartRef = typename Base::ValuePartRef;
    // using ValuePart = typename Base::ValuePart;
    // using ValueRef = typename Base::ValueRef;
    // using GenericValuePart = typename Base::GenericValuePart;
    //
    // using AsmReg = typename Base::AsmReg;
    //
    // std::unique_ptr<RustAdaptor> adaptor;
    //
    // std::variant<std::monostate, tpde::x64::CCAssignerSysV> cc_assigners;
    //
    // static constexpr std::array<AsmReg, 2> LANDING_PAD_RES_REGS = {AsmReg::AX,
    //                                                                AsmReg::DX};

    // explicit RustCompilerX64(std::unique_ptr<RustAdaptor> &&adaptor)
    //     : Base{adaptor.get()}, adaptor(std::move(adaptor)) {
    //   static_assert(tpde::Compiler<RustCompilerX64, tpde::x64::PlatformConfig>);
    // }
  };
}
