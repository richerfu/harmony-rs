cmake_minimum_required(VERSION 3.4.1)
project(clion_test)

# 将 SDK 路径配置为OHOS_NDK_HOME 的环境变量
include($ENV{OHOS_NDK_HOME}/native/build/cmake/ohos.toolchain.cmake)

set(CMAKE_CXX_STANDARD 17)

set(CMAKE_VERBOSE_MAKEFILE on)

include_directories(${NATIVERENDER_ROOT_PATH}
        ${NATIVERENDER_ROOT_PATH}/include)

add_library(entry SHARED hello.cpp)
target_link_libraries(entry PUBLIC libace_napi.z.so)
