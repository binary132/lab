#include <stdbool.h>

#include "lex.h"

#include "munit.h"

#include "main_test.h"

MunitResult test_sym_name(const MunitParameter params[],
                          void*                user_data_or_fixture) {
	struct {
		MLX_kind    given;
		const char* expect;
		bool        last;
	} tests[] = { {
		        .given  = -1,
		        .expect = MLX_UNKNOWN_NAME,
		      },
		      {
		        .given  = MLX_ERROR,
		        .expect = MLX_ERROR_NAME,
		      },
		      {
		        .given  = MLX_BLOCK_OPEN,
		        .expect = MLX_BLOCK_OPEN_NAME,
		      },
		      {
		        .given  = MLX_BLOCK_CLOSE,
		        .expect = MLX_BLOCK_CLOSE_NAME,
		      },
		      {
		        .given  = MLX_SCALAR,
		        .expect = MLX_SCALAR_NAME,
		      },
		      {
		        .given  = MLX_NONE,
		        .expect = MLX_NONE_NAME,
		      },
		      {
		        .given  = MLX_SCALAR,
		        .expect = MLX_SCALAR_NAME,
		      },
		      { .last = true } };

	for (int i = 0; !(tests[i].last); i++) {
		MLX_kind    ek = tests[i].given;
		const char* es = tests[i].expect;

		munit_logf(MUNIT_LOG_INFO, "test %d: given %d expect \"%s\"", i,
		           ek, es);

		const char* result = MLX_sym_name(tests[i].given);
		int         gotlen = strlen(result);

		munit_logf(MUNIT_LOG_INFO, "  got \"%s\"[%d]", result, gotlen);
		munit_assert_int(gotlen, ==, strlen(tests[i].expect));
		munit_assert_memory_equal(gotlen, result, tests[i].expect);
	}

	return MUNIT_OK;
}

MunitResult test_sym_string(const MunitParameter params[],
                            void*                user_data_or_fixture) {
	unsigned long long v = 12345;

	struct {
		char*       buf;
		int         max;
		MLX_lexeme  given;
		const char* expect;
		int         expect_len;
		bool        last;
	} tests[] = {
		{
		  .max        = 1,
		  .given      = (MLX_lexeme){ .kind = -1 },
		  .expect_len = 39,
		  .expect     = "",
		},
		{
		  .max        = 80,
		  .given      = (MLX_lexeme){ .kind = -1 },
		  .expect_len = 39,
		  .expect     = "unexpected lexeme kind (unknown symbol)",
		},
		{
		  .max        = 80,
		  .given      = (MLX_lexeme){ .kind = MLX_ERROR },
		  .expect_len = 0,
		  .expect     = "",
		},
		{
		  .max   = 4,
		  .given = (MLX_lexeme){ .kind = MLX_ERROR, .value = "oops" },
		  .expect_len = 4,
		  .expect     = "oops",
		},
		{
		  .max   = 80,
		  .given = (MLX_lexeme){ .kind = MLX_ERROR, .value = "oops" },
		  .expect_len = 4,
		  .expect     = "oops",
		},
		{
		  .max        = 80,
		  .given      = (MLX_lexeme){ .kind = MLX_BLOCK_OPEN },
		  .expect_len = 1,
		  .expect     = "{",
		},
		{
		  .max        = 80,
		  .given      = (MLX_lexeme){ .kind = MLX_BLOCK_OPEN },
		  .expect_len = 1,
		  .expect     = "{",
		},
		{
		  .max        = 80,
		  .given      = (MLX_lexeme){ .kind = MLX_BLOCK_CLOSE },
		  .expect_len = 1,
		  .expect     = "}",
		},
		{
		  .max        = 80,
		  .given      = (MLX_lexeme){ .kind = MLX_SCALAR, .value = &v },
		  .expect_len = 5,
		  .expect     = "12345",
		},
		{ .last = true }
	};

	for (int i = 0; !(tests[i].last); i++) {
		MLX_kind    ek = tests[i].given.kind;
		const char* es = tests[i].expect;
		char        tb[80];
		int         tmx = tests[i].max;
		int         el  = tests[i].expect_len;

		memset(tb, '\0', 80);
		munit_logf(MUNIT_LOG_INFO, "test %d: given %s(%s)", i,
		           MLX_sym_name(ek), (char*)tests[i].given.value);
		munit_logf(MUNIT_LOG_INFO, "  expect \"%s\"[%d]", es, el);

		int gotlen = MLX_sym_string(tb, tmx, tests[i].given);

		munit_logf(MUNIT_LOG_INFO, "  got \"%s\"[%d]", tb, gotlen);
		munit_assert_int(gotlen, ==, tests[i].expect_len);
		munit_assert_memory_equal(strlen(tests[i].expect), tb,
		                          tests[i].expect);
	}

	return MUNIT_OK;
}

MunitResult test_kind(const MunitParameter params[],
                      void*                user_data_or_fixture) {
	struct {
		char*   given;
		int     max, bufmax;
		MLX_rsp expect;
		bool    last;
	} tests[] = {
		{
		  .given = "{",
		  .max   = 1,
		  .expect =
		    (MLX_rsp){ .offset = 1, .lex = { .kind = MLX_BLOCK_OPEN } },
		},
		{
		  .given = "{",
		  .max   = 1,
		  .expect =
		    (MLX_rsp){ .offset = 1, .lex = { .kind = MLX_BLOCK_OPEN } },
		},
		{
		  .given  = "}",
		  .max    = 1,
		  .expect = (MLX_rsp){ .offset = 1,
		                       .lex    = { .kind = MLX_BLOCK_CLOSE } },
		},
		{
		  .given  = "x",
		  .max    = 1,
		  .bufmax = 4,
		  .expect = (MLX_rsp){ .consumed = 6,
		                       .lex =
		                         {
		                           .kind  = MLX_NONE,
		                           .value = "",
		                           .size  = 6,
		                         } },
		},
		{
		  .given  = "x",
		  .max    = 1,
		  .bufmax = 6,
		  .expect = (MLX_rsp){ .consumed = 6,
		                       .lex =
		                         {
		                           .kind  = MLX_ERROR,
		                           .value = "failed",
		                           .size  = 6,
		                         } },
		},
		{ .last = true }
	};

	for (int i = 0; !(tests[i].last); i++) {
		char*    given = tests[i].given;
		int      max   = tests[i].max;
		MLX_rsp  exp   = tests[i].expect;
		MLX_kind ek    = exp.lex.kind;
		void*    ev    = exp.lex.value;
		int      es    = exp.lex.size;
		int      bmax  = tests[i].bufmax;

		int eoff  = exp.offset;
		int econs = exp.consumed;

		munit_logf(MUNIT_LOG_INFO, "test %d: given %s, expect %s(%s)",
		           i, given, MLX_sym_name(ek), (ev ? ev : ""));

		char tmp[80];

		MLX_rsp got = MLX_next(given, tmp, bmax, max);

		MLX_kind gk = got.lex.kind;
		int      gs = got.lex.size;
		void*    gv = got.lex.value;

		int goff  = got.offset;
		int gcons = got.consumed;

		munit_logf(MUNIT_LOG_INFO, "  got %s[%d]", MLX_sym_name(gk),
		           gs);
		munit_assert_int(gk, ==, ek);
		munit_assert_int(gs, ==, es);
		munit_assert_int(goff, ==, eoff);
		munit_assert_int(gcons, ==, econs);

		if (gv || es) {
			// We got a 'value', or expected one.
			char buf[80];
			memset(buf, '\0', 80);
			int n = MLX_sym_string(buf, 80, got.lex);
			munit_assert_int(n, <=, 80);
			munit_assert_int(n, >, 0);
			munit_logf(MUNIT_LOG_INFO, "  with value \"%s\"[%d]",
			           buf, gs);

			munit_assert_int(es, ==, gs);
			munit_assert_memory_equal(strlen(ev), gv, ev);
		}
	}

	return MUNIT_OK;
}
