#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "sds.h"

#include "lex.h"

int main(int argc, char* argv[]) {
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

	char tmp[80];
	memset(tmp, '\0', 80);
	MLX_rsp r = MLX_next(buf, tmp, 80, 80);
	printf("%s\n", MLX_sym_name(r.lex.kind));

	if (r.lex.value) {
	     char tmp2[80];
	     MLX_sym_string(tmp2, 80, r.lex);
	     printf("%s\n", tmp2);
	}

	return 0;
}
