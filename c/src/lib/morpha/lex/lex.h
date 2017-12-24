// Lex (prefix MLX) is a Morpha library for lexing a given char[].  It
// implements state machines for consuming lexemes from raw memory.

const char MLX_SYM_BLOCK_OPEN; // = '{';
const char MLX_SYM_BLOCK_CLOSE; // = '}';

const char* MLX_ERROR_NAME; //  = "MLX_ERROR";
const char* MLX_BLOCK_OPEN_NAME; //  = "MLX_BLOCK_OPEN";
const char* MLX_BLOCK_CLOSE_NAME; // = "MLX_BLOCK_CLOSE";

// MLX_kind enumerates the possible token types.
typedef enum {
     MLX_ERROR,
     MLX_BLOCK_OPEN,
     MLX_BLOCK_CLOSE
} MLX_kind;

// MLX_lexeme represents a single lexeme; the kind dictates the type of
// value.
typedef struct {
     MLX_kind kind;
     void* value;
} MLX_lexeme;

// MLX_rsp is an internal type used by the tokenizer to iterate.
typedef struct {
     MLX_lexeme lex;
     int offset;
} MLX_rsp;

// MLX_consume attempts to consume all lexemes from input, placing them
// in "into".  Into should be preallocated with sizeof(into) equal to
// sizeof(input).
//
// If MLX_consume is consuming a lexeme when it reaches the end of input,
// it returns the index of "input" where the current lexeme began.
MLX_rsp MLX_consume(MLX_lexeme into[], const char* input, int max);

// MLX_next scans the given memory for lexemes, up to from[max-1].  If
// it encounters a partial lexeme, it returns a lexeme having
// kind=ERROR, and allocates an error message with value as const char*.
MLX_rsp MLX_next(const char* from, int max);

// MLX_sym_name returns a string constant naming the given lexeme kind.
const char* MLX_sym_name(MLX_kind);
