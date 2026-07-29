
#include "RustCompiler.h"

namespace tpde_rust {
  
  template<typename Adaptor, typename Derived, typename Config>
  std::unique_ptr<RustCompiler<Adaptor, Derived, Config> > RustCompiler<Adaptor, Derived, Config>::create() {
    return std::make_unique<RustCompiler>();
  }
}
