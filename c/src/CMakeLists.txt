project(Spartan C)

add_subdirectory(lib)

set(main_SRCS main.c)

add_executable(${APPLICATION_NAME} ${main_SRCS})

target_include_directories(${APPLICATION_NAME} PUBLIC
  ${lex_INCLUDE_DIR}
  ${sds_INCLUDE_DIR}
)

target_link_libraries(${APPLICATION_NAME}
  lex
  sds
)
