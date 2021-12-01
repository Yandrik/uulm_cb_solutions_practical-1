#include <stdlib.h>
#include <stdio.h>
#include <math.h>
#include <string.h>

char skipWhitespaceAndGetChar(const char** input) {
    while (**input == ' ' || **input == '\t' || **input == '\n' || **input == '\r')
        (*input)++;
    
    return **input;
}

size_t replaceSpace(const char* src, char* dst) {
    size_t count = 0;
    char c;

    while ((c =*src++) != '\0') {
        if (c == ' ' || c == '\t' || c == '\n' || c == '\r')
            continue;
        
        if (dst != NULL )
            *dst++ = c;
        count++;
    }

    if (dst != NULL)
        *dst = '\0';
    
    return count;
}

// Item automata
double stateS(double accum, const char** input, const char * base);
double stateE(double accum, const char** input, const char * base);
double stateE2(double accum, const char** input, const char * base);
double stateP(double accum, const char** input, const char * base);
double stateP2(double accum, const char** input, const char * base);
double stateT(double accum, const char** input, const char * base);
double stateT2(double accum, const char** input, const char * base);
double stateF(double accum, const char** input, const char * base);
double stateZ(double accum, const char** input, const char * base);
double err(double accum, const char** input, const char * base, const char* expected);

double stateS(double accum, const char** input, const char * base) {
    accum = stateE(accum, input, base);

    switch (skipWhitespaceAndGetChar(input)) {
        case '\0':
            return accum;
        default:
            return err(accum, input, base, "<EOF>");
    }
}

double stateE(double accum, const char** input, const char * base) {
    accum = stateP(accum, input, base);
    accum = stateE2(accum, input, base);
    return accum;
}

double stateE2(double accum, const char** input, const char * base) {
    switch (skipWhitespaceAndGetChar(input)) {
        case '+':
            (*input)++;;
            accum += stateP(accum, input, base);
            return stateE2(accum, input, base);
        case '-':
            (*input)++;;
            accum -= stateP(accum, input, base);
            return stateE2(accum, input, base);
        default:
            return accum;
    }
}

double stateP(double accum, const char** input, const char* base) {
    accum = stateT(accum, input, base);
    accum = stateP2(accum, input, base);
    return accum;
}

double stateP2(double accum, const char** input, const char* base) {
    switch (skipWhitespaceAndGetChar(input)) {
        case '*':
            (*input)++;;
            accum *= stateT(accum, input, base);
            return stateP2(accum, input, base);
        case '/':
            (*input)++;;
            accum /= stateT(accum, input, base);
            return stateP2(accum, input, base);
        default:
            return accum;
    }
}

double stateT(double accum, const char** input, const char* base) {
    switch (skipWhitespaceAndGetChar(input)) {
        case '-':
            (*input)++;;
            return -stateT(accum, input, base);
        default:
            accum = stateF(accum, input, base);
            return stateT2(accum, input, base);
    }
}

double stateT2(double accum, const char** input, const char* base) {
    switch (skipWhitespaceAndGetChar(input)) {
        case '^':
            (*input)++;;
            return pow(accum, stateT(accum, input, base));
        default:
            return accum;
    }
}

double stateF(double accum, const char** input, const char* base) {
    switch (skipWhitespaceAndGetChar(input)) {
        case '(':
            (*input)++;;
            accum = stateE(accum, input, base);

            if (**input != ')')
                return err(accum, input, base, ")");
            else
                (*input)++;;

            return accum;
        default:
            return stateZ(accum, input, base);
    }
}

double stateZ(double accum, const char** input, const char* base) {
    if (**input != '.' && (**input < '1' || **input > '9'))
        return err(accum, input, base, "<NON-ZERO-DIGIT>");
    
    double number = 0;
    double frac = 0;

    while (**input >= '0' && **input <= '9') {
        number = number * 10 + (**input - '0');
        (*input)++;
    }

    if (**input == '.') {
        double div = 10;
        (*input)++;;

        if (**input < '0' || **input > '9')
            return err(accum, input, base, "<DIGIT>");
        
        while (**input >= '0' && **input <= '9') {
            frac = frac + (**input - '0') / div;
            div *= 10;
            (*input)++;;
        }
    }

    return number + frac;
}

double err(double accum, const char** input, const char* base, const char* expected) {
    fprintf(stderr, "%s\n", base);
    long int pos = *input - base;

    for (int i = 0; i < pos; i++) 
        fprintf(stderr, "%c", ' ');
    fprintf(stderr, "%c\n", '^');

    if (**input == '\0') {
        fprintf(stderr, "Error: Missing character at end of input, expected: %s\n", expected);
    } else {
        fprintf(stderr, "Error at column %ld: Got '%c', expected %s\n", pos, **input, expected);
    }

    exit(-1);
    return 0;
}


int main(int argc, const char* argv[]) {
    if (argc < 2) {
        fprintf(stderr, "Missing input");
        return 1;
    } else {
        //size_t size = strlen(argv[1]) - replaceSpace(argv[1], NULL) + 1;
        //char* input = malloc(size * sizeof(char));
        //replaceSpace(argv[1], input);
        //printf("%f\n", stateS(0, (const char**) &input, (const char*) input));

        printf("%f\n", stateS(0, argv+1, argv[1]));
        return 0;
    }
}
