cmake_minimum_required(VERSION 3.10)

project(system_information)

file(GLOB SRC_FILES "src/*.c")
add_executable(system_information ${SRC_FILES})

target_compile_options(system_information PRIVATE -std=c89)
set_target_properties(system_information PROPERTIES LINK_FLAGS "-nostdlib -nostartfiles")
set_target_properties(system_information PROPERTIES LINK_FLAGS "-Wl,--entry=_start")