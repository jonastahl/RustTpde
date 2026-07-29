#include "tpde.h"

#include "RustCompiler.h"


uint32_t compile_ir(ModuleTpde& module) {
    // TODO move this out, don't want to initialize it every time separately
    const auto compiler = tpde_rust::RustCompiler::create();

    std::vector<uint8_t> buf;
    compiler->compile_to_elf(module, buf);

    // if (obj_out_path.Get() == "-") {
    //     std::cout.write(reinterpret_cast<const char *>(buf.data()), buf.size());
    //     std::cout << std::flush;
    // } else {
    //     std::ofstream out{obj_out_path.Get().c_str(), std::ios::binary};
    //     out.write(reinterpret_cast<const char *>(buf.data()), buf.size());
    // }

    return 0;
}

[[nodiscard]] tpde::u32 size_of_type(Type type) {
    switch (type) {
        case Type::Bool:
        case Type::i8: return 1;
        case Type::i16: return 2;
        case Type::i32: return 4;
        case Type::i64: return 8;
        default:
            throw std::runtime_error("unsupported type");
    }
}