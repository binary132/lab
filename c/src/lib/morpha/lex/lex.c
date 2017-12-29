#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "lex.h"

const char* MLX_UNKNOWN_NAME     = "unknown symbol";
const char* MLX_NONE_NAME        = "MLX_NONE";
const char* MLX_ERROR_NAME       = "MLX_ERROR";
const char* MLX_BLOCK_OPEN_NAME  = "MLX_BLOCK_OPEN";
const char* MLX_BLOCK_CLOSE_NAME = "MLX_BLOCK_CLOSE";
const char* MLX_SCALAR_NAME      = "MLX_SCALAR";

MLX_rsp MLX_next(const char* from, char* using, int max_scratch, int max_buf) {
	MLX_kind k = MLX_NONE;
	void*    v = NULL;
	int      s = 0, i = 0;

	// Iterate over input buffer.
	for (i = 0; i < max_buf && k == MLX_NONE; i++) {
		switch (from[i]) {
		case MLX_SYM_BLOCK_OPEN:
			k = MLX_BLOCK_OPEN;
			break;

		case MLX_SYM_BLOCK_CLOSE:
			k = MLX_BLOCK_CLOSE;
			break;

		case MLX_SYM_SCALAR:
			// TODO
			// break;

		default:
			i           = 0;
			char msgv[] = "failed";
			s           = strlen(msgv);

			if (s <= max_scratch) {
				// Only assign lexeme if enough scratch.
				// Otherwise, MLX_rsp.consumed will be
				// greater than max_scratch.
				k = MLX_ERROR;
				memcpy(using, msgv, s);
				v = using;
			}

			goto error;
		}
	}

error:
	return (MLX_rsp){ .offset   = i,
		          .consumed = s,
		          .lex      = { .kind = k, .size = s, .value = v } };
}

const char* MLX_sym_name(MLX_kind k) {
	const char* result;

	switch (k) {
	case MLX_NONE:
		result = MLX_NONE_NAME;
		break;

	case MLX_ERROR:
		result = MLX_ERROR_NAME;
		break;

	case MLX_BLOCK_OPEN:
		result = MLX_BLOCK_OPEN_NAME;
		break;

	case MLX_BLOCK_CLOSE:
		result = MLX_BLOCK_CLOSE_NAME;
		break;

	case MLX_SCALAR:
		result = MLX_SCALAR_NAME;
		break;

	default:
		result = MLX_UNKNOWN_NAME;
		break;
	}

	return result;
}

static bool MLX_kind_is_string(MLX_kind k) { return k == MLX_ERROR; }

static int MLX_write_strn(char* buf, int max, char* source) {
	if (!source) {
		// The source content is empty.
		return 0;
	}

	int len = strlen(source);
	if (len <= max && len > 0) {
		// There is enough buffer space.
		memcpy(buf, source, len);
	}
	return len;
}

int MLX_sym_string(char* buf, int max, MLX_lexeme l) {
	int n = 0;
	if (MLX_kind_is_string(l.kind)) {
		// This lexeme has a char* value, so it doesn't need to be
		// rendered.
		return MLX_write_strn(buf, max, l.value);
	}

	switch (l.kind) {
	case MLX_SCALAR:
		// All Scalars are unsigned long long.  A Scalar must have
		// a non-NULL value.
		n = snprintf(buf, max, "%llu", *(unsigned long long*)l.value);
		break;

	case MLX_BLOCK_OPEN:
		n = snprintf(buf, max, "%c", MLX_SYM_BLOCK_OPEN);
		break;

	case MLX_BLOCK_CLOSE:
		n = snprintf(buf, max, "%c", MLX_SYM_BLOCK_CLOSE);
		break;

	default:
		// We don't know how to render this lexeme kind.
		n = snprintf(buf, max, "unexpected lexeme kind (%s)",
		             MLX_sym_name(l.kind));
		break;
	}

	return n;
}
