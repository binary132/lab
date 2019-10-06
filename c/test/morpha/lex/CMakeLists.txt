project(Spartan C)

add_executable(lex_test main_test.h main_test.c lex_test.c)

target_link_libraries(lex_test lex munit)

set_target_properties(lex_test PROPERTIES
  RUNTIME_OUTPUT_DIRECTORY
  ${CMAKE_CURRENT_SOURCE_DIR}/bin
)

add_test(
  NAME lex_test
  WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/bin
  COMMAND ${CMAKE_CURRENT_SOURCE_DIR}/bin/lex_test
)
