# Taschenrechner

A parser project for the Compilerbau Vorlesung in WiSe 2021/2022 at Ulm University

## How to build

Everything is written in ANSI C, so just use any compiler you like to compile.
Just make sure to link the math library when compiling. E.g.:

```console
gcc -o taschenrechner taschenrechner.c -lm
```

## Executing

Simply execute the compiled program and parse the expression as program argument. E.g.:

```console
./taschenrechner "4*-2^3"
```

## Grammar

See the PDF file for the grammar and item automata.

## Correctness

Having tested various expressions, it seems there are no issues
