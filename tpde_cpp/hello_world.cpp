#include "hello_world.h"

#include<print>

rust::String hello_world() {
    return "Hello, World!";
}

uint32_t compile_ir(const Ir& ir) {
    for (auto i : ir.instr) {
        switch (i) {
            case Instr::Add:
                std::println("Add");
                break;
            case Instr::Sub:
                std::println("Sub");
                break;
            case Instr::Mul:
                std::println("Mul");
                break;
            case Instr::Div:
                std::println("Div");
                break;
        }
    }

    uint32_t sum = 0;
    for (auto i : ir.data) {
        sum += i;
    }
    return sum;
}