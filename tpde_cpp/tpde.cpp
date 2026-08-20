#include "tpde.h"

#include <fstream>

#include "RustCompiler.h"
#include "tpde/RegisterFile.hpp"


uint32_t compile_to_file(ModuleTpde& module, const rust::Str path) {
    // TODO move this out, don't want to initialize it every time separately
    const auto compiler = tpde_rust::RustCompiler::create();

    std::vector<uint8_t> buf;
    compiler->compile_to_elf(module, buf);

    {
        std::string file_path(path.data(), path.size());
        std::ofstream out_file(file_path, std::ios::binary);

        if (!out_file) {
            return 0;
        }

        out_file.write(reinterpret_cast<const char*>(buf.data()), buf.size());
        out_file.close();
    }

    return buf.size();
}

[[nodiscard]] tpde::u32 size_of_type(Type type) {
    switch (type) {
        case Type::Bool:
        case Type::i8: return 1;
        case Type::i16: return 2;
        case Type::i32: return 4;
        case Type::i64: return 8;
        case Type::i128: return 16;
        default:
            throw std::runtime_error("unsupported type");
    }
}

[[nodiscard]] tpde::RegBank reg_bank_of_type(Type type) {
    switch (type) {
        case Type::Bool:
        case Type::i8:
        case Type::i16:
        case Type::i32:
        case Type::i64:
            return tpde::RegBank{0};
        default:
            throw std::runtime_error("unsupported type");
    }
}