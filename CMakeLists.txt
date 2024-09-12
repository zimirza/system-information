cmake_minimum_required(VERSION 3.10)

project(system_information)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED True)

find_package(CURL REQUIRED)
find_package(X11 REQUIRED)

add_executable(system_information src/main.c)

target_link_libraries(system_information PRIVATE CURL::libcurl ${X11_LIBRARIES})