#include <stdio.h>

#include "lex.h"

#include "munit.h"

int main(void) {
     MLX_kind k = MLX_next("{", 0).lex.kind;
     munit_assert_int(k, ==, MLX_BLOCK_OPEN);
     return 0;
}
