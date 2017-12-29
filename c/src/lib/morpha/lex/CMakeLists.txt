project(Spartan C)

set(lex_SRCS
  lex.h
  lex.c
)

add_library(lex STATIC ${lex_SRCS})

find_path(lex_INCLUDE_DIR lex.h ${CMAKE_CURRENT_LIST_DIR})

# Let's try to avoid depending on sds.
# target_include_directories(lex PUBLIC ${sds_INCLUDE_DIR})
# target_link_libraries(lex sds)
