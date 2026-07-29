cmake_minimum_required(VERSION 3.14)
project(tpde_wrapper_project)

set(CMAKE_CXX_STANDARD 23)

add_subdirectory(deps/tpde)

find_program(CXXBRIDGE cxxbridge REQUIRED HINTS $ENV{HOME}/.cargo/bin)

set(CXX_GEN_DIR ${CMAKE_CURRENT_BINARY_DIR}/cxxbridge)
file(MAKE_DIRECTORY ${CXX_GEN_DIR}/rust ${CXX_GEN_DIR}/rustc_codegen_tpde/src)

execute_process(COMMAND ${CXXBRIDGE} --header -o ${CXX_GEN_DIR}/rust/cxx.h)
execute_process(COMMAND ${CXXBRIDGE} ${CMAKE_CURRENT_SOURCE_DIR}/src/shared.rs --header
        -o ${CXX_GEN_DIR}/rustc_codegen_tpde/src/shared.rs.h)

add_library(tpde_cpp STATIC
        tpde_cpp/tpde.cpp
        tpde_cpp/RustAdaptor.cpp
        tpde_cpp/RustCompilerBase.cpp
        tpde_cpp/RustCompiler.cpp
        tpde_cpp/x64/RustCompilerX64.cpp
        tpde_cpp/RustCompiler.cpp
        tpde_cpp/RustCompiler.h
)

target_include_directories(tpde_cpp PRIVATE
        ${CMAKE_CURRENT_SOURCE_DIR}
        ${CXX_GEN_DIR}
        ${CMAKE_CURRENT_SOURCE_DIR}/deps/tpde/tpde/include
)

target_link_libraries(tpde_cpp PRIVATE tpde)

install(TARGETS tpde_cpp ARCHIVE DESTINATION lib)
