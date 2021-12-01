#include <stdio.h>

#include "lexer.h"
#include "parser.h"

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

    parser();
}
