cmake_minimum_required(VERSION 3.10)

project(system_information)

add_executable(system_information src/main.c)

target_compile_options(system_information PRIVATE -std=c89)
set_target_properties(system_information PROPERTIES LINK_FLAGS "-nostdlib")
set_target_properties(system_information PROPERTIES LINK_FLAGS "-Wl,--entry=_start")