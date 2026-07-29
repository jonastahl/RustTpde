#include "tpde.h"

#include<print>


uint32_t compile_ir(const ModuleTpde& module) {
    std::println("compiling ir");

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