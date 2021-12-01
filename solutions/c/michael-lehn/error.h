#ifndef ERROR_H
#define ERROR_H

#include <stdio.h>

#include "lexer.h"

size_t errorCount;
void error(struct TokenPos pos, const char *msg);
void internalError(const char *msg); // calls abort()

#endif // ERROR_H
