#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "lex.h"

const char MLX_SYM_BLOCK_OPEN  = '{';
const char MLX_SYM_BLOCK_CLOSE = '}';

const char* MLX_ERROR_NAME       = "MLX_ERROR";
const char* MLX_BLOCK_OPEN_NAME  = "MLX_BLOCK_OPEN";
const char* MLX_BLOCK_CLOSE_NAME = "MLX_BLOCK_CLOSE";

MLX_rsp MLX_next(const char* from, int max) {
     MLX_kind k;
     void* v;

     switch (from[0]) {
     case MLX_SYM_BLOCK_OPEN:
	  k = MLX_BLOCK_OPEN;
	  break;

     case MLX_SYM_BLOCK_CLOSE:
	  k = MLX_BLOCK_CLOSE;
	  break;

     default:
	  k = MLX_ERROR;
	  const char msgv[] = "failed";
	  int l = strlen(msgv);
	  char* msg = malloc(l*sizeof(char));
	  memcpy(msg, msgv, l);
	  v = msg;
	  break;
     }

     return (MLX_rsp){ .lex = { .kind = k, .value = v } };
}

const char* MLX_sym_name(MLX_kind k) {
     const char* result;

     switch (k) {
     case MLX_BLOCK_CLOSE:
	  result = MLX_BLOCK_CLOSE_NAME;
	  break;

     case MLX_BLOCK_OPEN:
	  result = MLX_BLOCK_OPEN_NAME;
	  break;

     default:
	  result = MLX_ERROR_NAME;
	  break;
     }

     return result;
}
