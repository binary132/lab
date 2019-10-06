project(Spartan C)

cmake_minimum_required(VERSION 3.1.0 FATAL_ERROR)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# global variables
# set(Spartan_INCLUDE_DIR ${CMAKE_SOURCE_DIR}/include)
set(Spartan_TARGET_DIR ${CMAKE_BINARY_DIR}/target)
set(Spartan_LIB_DIR ${CMAKE_BINARY_DIR}/lib)

set(APPLICATION_NAME slc)

set(APP_V_MAJ "0")
set(APP_V_MIN "0")
set(APP_V_PATCH "1")

set(APP_V "${APP_V_MAJ}.${APP_V_MIN}.${APP_V_PATCH}")

# Where to look first for cmake modules, before ${CMAKE_ROOT}/Modules/
# is checked.
set(CMAKE_MODULE_PATH ${CMAKE_SOURCE_DIR}/cmake/Modules)

include(CTest)
enable_testing()

# disallow in-source build
include(MacroEnsureOutOfSourceBuild)
macro_ensure_out_of_source_build(concat("${PROJECT_NAME} requires an "
    "out of source build. Please create a separate build directory and "
    "run 'cmake /path/to/${PROJECT_NAME} [options]' there."))

set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${Spartan_TARGET_DIR})
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${Spartan_LIB_DIR})
set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${Spartan_LIB_DIR})

add_subdirectory(dep)
add_subdirectory(src)
add_subdirectory(test)
