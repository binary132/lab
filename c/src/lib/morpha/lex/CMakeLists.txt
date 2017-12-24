project(Spartan C)

set(lex_SRCS
  lex.h
  lex.c
)

# add_library(lex SHARED ${lex_SRCS}) # Works!
add_library(lex STATIC ${lex_SRCS}) # Doesn't!

find_path(lex_INCLUDE_DIR lex.h ${CMAKE_CURRENT_LIST_DIR})
