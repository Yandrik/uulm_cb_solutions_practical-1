#include <assert.h>
#include <math.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

#include "error.h"
#include "lexer.h"

struct Token token;

static const char *lexerFile = "<stdin>";

const char *
getLexerInputFile(const char *path)
{
    return lexerFile;
}

bool
setLexerInputFile(const char *path)
{
    lexerFile = path;
    return freopen(path, "r", stdin);
}

static struct
{
    int val;
    struct TokenPos pos;
} ch = { ' ' };

static void
nextCh()
{
    static struct TokenPos currPos = { 1, 1 };

    ch.val = getchar();
    ch.pos = currPos;
    if (ch.val == '\n') {
	++currPos.line;
	currPos.col = 1;
    } else {
	++currPos.col;
    }
}

static bool
isSpace(int ch)
{
    return ch == ' ' || ch == '\r' || ch == '\f' || ch == '\v' || ch == '\t';
}

static bool
isDigit(int ch)
{
    return ch >= '0' && ch <= '9';
}

// returns UNKNOWN if float literal is invalid and leaves *value
// unchanged. Othersise returns FLOAT_LITERAL and stores literal in *value
static enum TokenKind
scanFloat(double *value)
{
    if (!value || !(isDigit(ch.val) || ch.val == '.')) {
	internalError("scanFloat(double *)");
	abort();
    }

    double val = 0;
    bool leadingDigits = false, trailingDigits = false;

    while (isDigit(ch.val)) {
	val *= 10;
	val += ch.val - '0';
	nextCh();
	leadingDigits = true;
    }
    if (ch.val == '.') {
	// consume '.'
	nextCh();
	double scale = .1;
	while (isDigit(ch.val)) {
	    val += (ch.val - '0') * scale;
	    scale *= .1;
	    nextCh();
	    trailingDigits = true;
	}
    }
    if (!leadingDigits && !trailingDigits) {
	error(ch.pos, "float literals require a leading or trailing digit\n");
	return UNKNOWN;
    }
    if (ch.val == 'e' || ch.val == 'E') {
	// consume 'e' or 'E'
	nextCh();
	double exponent = 0;
	bool negSign = false;
	// scan (optional) sign of exponent
	while (ch.val == '+' || ch.val == '-') {
	    if (ch.val == '-') {
		negSign = !negSign;
	    }
	    // consume sign
	    nextCh();
	}
	if (!isDigit(ch.val)) {
	    error(ch.pos, "exponent has no digits\n");
	    return UNKNOWN;
	}
	while (isDigit(ch.val)) {
	    exponent *= 10;
	    exponent += ch.val - '0';
	    nextCh();
	}
	if (negSign) {
	    exponent = -exponent;
	}
	val = val * pow(10, exponent);
    }
    *value = val;
    return FLOAT_LITERAL;
}

enum TokenKind
getToken()
{
    // skip whitespaces
    while (isSpace(ch.val)) {
	nextCh();
    }

    token.kind = UNKNOWN;
    token.value = 0;
    token.pos = ch.pos;

    switch (ch.val) {
	case EOF:
	    token.kind = EOI;
	    break;
	case '\n':
	    token.kind = EOL;
	    nextCh();
	    break;
	case '.':
	case '0':
	case '1':
	case '2':
	case '3':
	case '4':
	case '5':
	case '6':
	case '7':
	case '8':
	case '9':
	    token.kind = scanFloat(&token.value);
	    break;
	case '+':
	    token.kind = PLUS;
	    nextCh();
	    break;
	case '-':
	    token.kind = MINUS;
	    nextCh();
	    break;
	case '*':
	    token.kind = ASTERISK;
	    nextCh();
	    break;
	case '/':
	    token.kind = SLASH;
	    nextCh();
	    break;
	case '^':
	    token.kind = CARET;
	    nextCh();
	    break;
	case '(':
	    token.kind = LPAREN;
	    nextCh();
	    break;
	case ')':
	    token.kind = RPAREN;
	    nextCh();
	    break;
	default:
	    // illegal character
	    nextCh();
    }
    return token.kind;
}

const char *
tokenKindStr(enum TokenKind kind)
{
    switch (kind) {
	case UNKNOWN:
	    return "UNKOWN";
	case EOI:
	    return "EOI";
	case EOL:
	    return "EOL";
	case FLOAT_LITERAL:
	    return "FLOAT_LITERAL";
	case PLUS:
	    return "PLUS";
	case MINUS:
	    return "MINUS";
	case ASTERISK:
	    return "ASTERISK";
	case SLASH:
	    return "SLASH";
	case CARET:
	    return "CARET";
	case LPAREN:
	    return "LPAREN";
	case RPAREN:
	    return "RPAREN";
    }
    // just in case (compilers like gcc raise a warning  if switch does not
    // handle all enums):
    internalError("const char *tokenKindStr(enum TokenKind)");
    return 0;
}

void
fprintTokenStr(FILE *out, const struct Token *token)
{
    switch (token->kind) {
	case UNKNOWN:
	    fprintf(out, "UNKNOWN");
	    break;
	case EOI:
	    fprintf(out, "EOI");
	    break;
	case EOL:
	    fprintf(out, "EOL");
	    break;
	case FLOAT_LITERAL:
	    fprintf(out, "%g", token->value);
	    break;
	case PLUS:
	    fprintf(out, "+");
	    break;
	case MINUS:
	    fprintf(out, "-");
	    break;
	case ASTERISK:
	    fprintf(out, "*");
	    break;
	case SLASH:
	    fprintf(out, "/");
	    break;
	case CARET:
	    fprintf(out, "^");
	    break;
	case LPAREN:
	    fprintf(out, "(");
	    break;
	case RPAREN:
	    fprintf(out, ")");
	    break;
	default:
	    // just in case (compilers like gcc raise a warning  if switch does
	    // not handle all enums):
	    internalError("void fprintTokenStr(FILE *, const struct Token *)");
    }
}
