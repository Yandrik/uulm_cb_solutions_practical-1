#include <stdlib.h>
#include <stdio.h>

#include "error.h"

void
error(struct TokenPos pos, const char *msg)
{
    ++errorCount;
    fprintf(stderr,
	    "%s:%zu.%zu: error: %s",
	    getLexerInputFile(),
	    pos.line,
	    pos.col,
	    msg);
}

void
internalError(const char *msg)
{
    fprintf(stderr, "internal error in '%s'\n", msg);
    abort();
}

