// Lex (prefix MLX) is a Morpha library for lexing a given char[].  It
// implements state machines for consuming lexemes from raw memory.

const char MLX_SYM_BLOCK_OPEN;  // = '{';
const char MLX_SYM_BLOCK_CLOSE; // = '}';
const char MLX_SYM_SCALAR;      // = '0'; TODO: Better implementation.

const char* MLX_ERROR_NAME;       // = "MLX_ERROR";
const char* MLX_BLOCK_OPEN_NAME;  // = "MLX_BLOCK_OPEN";
const char* MLX_BLOCK_CLOSE_NAME; // = "MLX_BLOCK_CLOSE";
const char* MLX_SCALAR_NAME;      // = "MLX_SCALAR";

// MLX_kind enumerates the possible token types.
typedef enum {
	MLX_ERROR,       // Something unrecognized
	MLX_BLOCK_OPEN,  // Block opening symbol {
	MLX_BLOCK_CLOSE, // Block closing symbol }
	MLX_SCALAR       // Whole-number sizes, lengths, etc.
} MLX_kind;

// MLX_lexeme represents a single lexeme; the kind dictates the type of
// value, its representation, etc.
typedef struct {
	MLX_kind kind;
	int      size;
	void*    value;
} MLX_lexeme;

// MLX_rsp is an internal type used by the tokenizer to iterate.  offset
// is the amount to step forward in the input buffer; consumed is the
// size of the value buffer ("using" in MLX_next.)
typedef struct {
	MLX_lexeme lex;
	int        offset, consumed;
} MLX_rsp;

// MLX_consume attempts to consume all lexemes from input, placing them
// in "into".  Into should be preallocated with sizeof(into) equal to
// sizeof(input).
//
// If MLX_consume is consuming a lexeme when it reaches the end of input,
// it returns the index of "input" where the current lexeme began.
//
// TODO: Add scratch space / allocator.
// TODO: Move some arguments into "object".
MLX_rsp MLX_consume(MLX_lexeme into[], const char* input, int max);

// MLX_next scans the given memory "from" for lexemes, up to from[max-1].
// If it encounters a partial lexeme, it returns a lexeme having
// kind=ERROR, and allocates an error message with value as const char*.
//
// If the lexeme has a dynamic string value, it will copy it into the
// given buffer.
//
// TODO: Rework this to better support dynamic sizes.
// TODO: Rework to support partial lexemes. (next fn?)
MLX_rsp MLX_next(const char* from, char* using, int max_scratch, int max_buf);

// MLX_sym_name returns a string constant naming the given lexeme kind.
const char* MLX_sym_name(MLX_kind);

// MLX_sym_string writes the string representation of the lexeme into
// the given buffer with given max length.  Return length of string
// written.  If the string would not fit into the buffer, it will return
// the required buffer size.
int MLX_sym_string(char*, int max, MLX_lexeme);
