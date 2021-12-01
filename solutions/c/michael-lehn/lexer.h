#ifndef LEXER_H
#define LEXER_H

#include <stdbool.h>
#include <stddef.h>
#include <stdio.h>

enum TokenKind
{
    UNKNOWN,
    EOI, // end of input
    EOL, // end of line
    FLOAT_LITERAL,
    PLUS,     // '+'
    MINUS,    // '-'
    ASTERISK, // '*'
    SLASH,    // '/'
    CARET,    // '^'
    LPAREN,   // '('
    RPAREN,   // ')'
};

struct Token
{
    struct TokenPos
    {
	size_t line, col;
    } pos;
    enum TokenKind kind;
    double value;
};

const char *getLexerInputFile();
bool setLexerInputFile(const char *path);
enum TokenKind getToken();
extern struct Token token;
const char *tokenKindStr(enum TokenKind);
void fprintTokenStr(FILE *out, const struct Token *token);

#endif // LEXER_H
