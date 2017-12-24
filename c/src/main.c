#include <stdio.h>
#include <stdlib.h>

#include "sds.h"

#include "lex.h"

int main(int argc, char *argv[]) {
  char buf[80];

  if (argc > 1) {
    sds buf = sdsempty();

    int i = 0;
    for (i = 1; i < argc - 1; i++) {
      buf = sdscat(buf, argv[i]);
      buf = sdscat(buf, ", ");
    }

    buf = sdscat(buf, argv[i]);

    printf("args: %s\n", buf);

    sdsfree(buf);
  }

  fgets(buf, 80, stdin);

  MLX_rsp r = MLX_next(buf, 80);
  printf("%s\n", MLX_sym_name(r.lex.kind));

  if (r.lex.value) {
    printf("%s\n", r.lex.value);
    free(r.lex.value);
  }

  return 0;
}
