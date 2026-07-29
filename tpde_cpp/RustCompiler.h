#pragma once
#include "RustAdaptor.h"
#include "../deps/tpde/tpde/include/tpde/CompilerBase.hpp"

namespace tpde_rust {
  template<typename Adaptor, typename Derived, typename Config>
  class RustCompiler : tpde::CompilerBase<Adaptor, Derived, Config> {

  protected:
    RustCompiler() {
      static_assert(tpde::Compiler<Derived, Config>);
      static_assert(std::is_same_v<Adaptor, RustAdaptor>);
    }

  public:
    virtual ~RustCompiler();

    RustCompiler(const RustCompiler &) = delete;
    RustCompiler &operator=(const RustCompiler &) = delete;

    static std::unique_ptr<RustCompiler> create();

    virtual bool compile_to_elf(ModuleTpde &mod, std::vector<uint8_t> &buf) = 0;
  };
}
