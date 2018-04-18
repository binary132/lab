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

	char* in = fgets(buf, 80, stdin);
	if (in == NULL) {
		return 0;
	}

	char tmp[80];
	memset(tmp, '\0', 80);
	MLX_lexeme lexes[80];

	int l = strlen(buf);
	int j = 0; // Alloc buffer consumption
	int n = 0; // Number of lexemes

	for (int i = 0; i < l - 1 && j < 80;) {
		// TODO: Move this into lex.c MLX_consume
		MLX_rsp r = MLX_next(buf + i, tmp + j, 80 - j, 80 - i);
		switch (r.lex.kind) {
		case MLX_ERROR:
			printf("Parse error: %s\n", (char*)(r.lex.value));
			return 1;

		case MLX_NONE:
			break;

		default:;
		}

		lexes[n++] = r.lex;
		i += r.offset;
		j += r.consumed;
	}

	char tmp2[80];

	for (int i = 0; i < n; i++) {
		MLX_lexeme l = lexes[i];
		printf("%s", MLX_sym_name(l.kind));

		if (l.value) {
			MLX_sym_string(tmp2, 80, l);
			printf("(%s)", tmp2);
		}

		printf("\n");
	}

	return 0;
}
