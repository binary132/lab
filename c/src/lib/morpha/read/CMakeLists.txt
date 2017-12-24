project(Spartan C)

set(read_SRCS
  read.h
  read.c
)

add_library(read STATIC ${read_SRCS})

find_path(read_INCLUDE_DIR read.h ${CMAKE_CURRENT_LIST_DIR})
