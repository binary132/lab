#include "munit.h"

#include "main_test.h"

MunitTest tests[] = { {
	                "/test-sym-name",       /* name */
	                test_sym_name,          /* test */
	                NULL,                   /* setup */
	                NULL,                   /* tear_down */
	                MUNIT_TEST_OPTION_NONE, /* options */
	                NULL,                   /* parameters */
	              },
	              {
	                "/test-sym-string",     /* name */
	                test_sym_string,        /* test */
	                NULL,                   /* setup */
	                NULL,                   /* tear_down */
	                MUNIT_TEST_OPTION_NONE, /* options */
	                NULL,                   /* parameters */
	              },
	              {
	                "/test-kind",           /* name */
	                test_kind,              /* test */
	                NULL,                   /* setup */
	                NULL,                   /* tear_down */
	                MUNIT_TEST_OPTION_NONE, /* options */
	                NULL,                   /* parameters */
	              },
	              /* Mark the end of the array with an entry where the test
	               * function is NULL */
	              { NULL, NULL, NULL, NULL, MUNIT_TEST_OPTION_NONE,
	                NULL } };

static const MunitSuite suite = {
	"/lex-tests",            /* name */
	tests,                   /* tests */
	NULL,                    /* suites */
	1,                       /* iterations */
	MUNIT_SUITE_OPTION_NONE, /* options */
};

int main(int argc, const char* argv[]) {
	return munit_suite_main(&suite, NULL, argc, (char* const*)argv);
}
