#include <math.h>
#include <stdbool.h>
#include <stddef.h>

#include "error.h"
#include "lexer.h"
#include "parser.h"

// for parsing expressio
static void parseExpressionSequence();
static double parseExpression();
static double parseAdditiveExpression();
static double parseMultiplicativeExpression();
static double parsePowerExpression();
static double parseUnaryExpression();
static double parsePrimaryExpression();

// for error handling
static bool expected(enum TokenKind kind);
static void skipRestOfLine();

void
parser()
{
    getToken();
    if (token.kind != EOI) {
	parseExpressionSequence();
    }
    if (!expected(EOI)) {
	error(token.pos, "expected end of input\n");
	error(token.pos, "got '");
	fprintTokenStr(stderr, &token);
	fprintf(stderr, "'\n");
	skipRestOfLine();
    }
    return;
}

void
parseExpressionSequence()
{
    do {
	// ignore previous errors (give user a chance to do it right now)
	errorCount = 0;
	double val = parseExpression();
	if (!errorCount) {
	    printf(">> %g\n", val);
	}
    } while (token.kind != EOI);
}

static double
parseExpression()
{
    double val = 0;

    if (token.kind != EOL) {
	val = parseAdditiveExpression();
    }
    if (!expected(EOL)) {
	error(token.pos, "expected newline after expression\n");
	error(token.pos, "got '");
	fprintTokenStr(stderr, &token);
	fprintf(stderr, "'\n");
	skipRestOfLine();
    }
    getToken();
    return val;
}

static double
parseAdditiveExpression()
{
    double val = parseMultiplicativeExpression();
    while (token.kind == PLUS || token.kind == MINUS) {
	struct Token op = token;
	getToken();
	double operand = parseMultiplicativeExpression();
	if (op.kind == PLUS) {
	    val += operand;
	} else if (op.kind == MINUS) {
	    val -= operand;
	} else {
	    internalError("double parseAdditiveExpression()");
	}
    }
    return val;
}

static double
parseMultiplicativeExpression()
{
    double val = parsePowerExpression();
    while (token.kind == ASTERISK || token.kind == SLASH) {
	struct Token op = token;
	getToken();
	double operand = parsePowerExpression();
	if (op.kind == ASTERISK) {
	    val *= operand;
	} else if (op.kind == SLASH) {
	    val /= operand;
	} else {
	    internalError("double parseMultiplicativeExpression()");
	}
    }
    return val;
}

static double
parsePowerExpression()
{
    double val = parseUnaryExpression();
    if (token.kind != CARET) {
	return val;
    }
    getToken();
    val = pow(val, parsePowerExpression());
    return val;
}

static double
parseUnaryExpression()
{
    double val = 0;
    switch (token.kind) {
	case PLUS:
	    getToken();
	    val = parseUnaryExpression();
	    break;
	case MINUS:
	    getToken();
	    val = -parseUnaryExpression();
	    break;
	default:
	    val = parsePrimaryExpression();
    }
    return val;
}


static double
parsePrimaryExpression()
{
    double val = 0;
    switch (token.kind) {
	case FLOAT_LITERAL:
	    val = token.value;
	    getToken();
	    return val;
	case LPAREN:
	    getToken();
	    val = parseAdditiveExpression();
	    if (!expected(RPAREN)) {
		error(token.pos, "expected ')'\n");
		skipRestOfLine();
		return 0;
	    }
	    getToken();
	    return val;
	default:
	    error(token.pos, "primary expression expected\n");
	    skipRestOfLine();
	    return 0;
    }
}

static bool
expected(enum TokenKind kind)
{
    return token.kind == kind;
}

static void
skipRestOfLine()
{
    while (token.kind != EOL && token.kind != EOI) {
	getToken();
    }
}


