#include <stdio.h>

#include "lexer.h"

int
main(int argc, const char **argv)
{
    if (argc > 2) {
	fprintf(stderr, "usage: %s [infile]\n", argv[0]);
	return 1;
    } else if (argc == 2) {
	if (! setLexerInputFile(argv[1])) {
	    fprintf(stderr, "can not open %s\n", argv[1]);
	    return 1;
	}
    }

    while (getToken() != EOI) {
	printf("%s:%zu.%zu: %s",
	       getLexerInputFile(),
	       token.pos.line,
	       token.pos.col,
	       tokenKindStr(token.kind));
	if (token.kind == FLOAT_LITERAL) {
	    printf(" %10g", token.value);
	}
	printf("\n");
    }
}
