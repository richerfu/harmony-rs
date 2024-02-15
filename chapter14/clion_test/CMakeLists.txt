cmake_minimum_required(VERSION 3.4.1)
project(clion_test)

include($ENV{HOME}/Library/Huawei/Sdk/openharmony/9/native/build/cmake/ohos.toolchain.cmake)

set(CMAKE_CXX_STANDARD 17)

set(CMAKE_VERBOSE_MAKEFILE on)

include_directories(${NATIVERENDER_ROOT_PATH}
        ${NATIVERENDER_ROOT_PATH}/include)

add_library(entry SHARED hello.cpp)
target_link_libraries(entry PUBLIC ace_napi.z)
