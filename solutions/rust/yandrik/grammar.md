# Grammar

The grammar for the calculator. It is segmented into two parts: 
1. **Regular Grammar** for the Lexer
2. **Context-free Grammar** for the Parser

## Lexer Grammar

```regexp
 ID: \d+
OBR: (
CBR: )
 OP: \*|/|^|-|\+
```

### Lexer DFA
```mermaid
graph LR;
init["s0"] -- "(" --> f0((f0))
init -- ")" --> f1((f1))
init -- "\d" --> f2((f2))
f2 -- "\d" --> f2
init -- "*, /, ^, -, +" --> f3((f3)) 
init -- "[ ]" --> init
init -- "otherwise" --> error[[error]]
```

## Parser Grammar

```
S -> A
A -> M + A | M - A | M
M -> G * M | G / M | G
G -> P ^ G | P
P -> ( A ) | ID
```

Die Grammatik ist eindeutig und nicht linksrekursiv. Außerdem hat sie eine weitere interessante
Eigenschaft: 
Wenn die Grammatik mehrere Potenzen parsed, expandieren diese nach rechts. Beispiel: 
```
S
A
M
G
P  ^ G
ID ^ G
ID ^  P  ^  G
ID ^  ID ^  G
ID ^  ID ^  P  ^  G
ID ^  ID ^  ID ^  ID 
5  ^  4  ^  3  ^  2
```

Dabei sieht der Syntax Tree so aus: 
```
S
|
A
|
M
|
G
| \
P  G
|  | \ 
5  P  G
   |  | \ 
   4  P  G
      |  |
      3  P
         |
         2
```
Wenn also per Recursive Descent immer zuerst das "tiefste" Ergebnis ausgewertet wird, heißt
5^4^3^2 5^(4^(3^(2))), ohne weitere Berechnungen auszuführen.

```mermaid
graph LR;
s1("[S->.A]") --> s2(["[S->A.]"])
```

```mermaid
graph LR;
a1("[A->.M+A] | [A->.M-A] | [A->.M]")
a3(["[A->M.+A] | [A->M.-A] | [A->M.]"])
a4("[A->M+.A]")
a5(["[A->M+A.]"])
a6("[A->M-.A]")
a7(["[A->M-A.]"])

a1 -- M --> a3
a3 -- + --> a4
a4 -- A --> a5
a3 -- - --> a6
a6 -- A --> a7
```

```mermaid
graph LR;
m1("[M->.G*M] | [M->.G/M] | [M->.G]")
m3(["[M->G.*M] | [M->G./M] | [M->G.]"])
m4("[M->G*.M]")
m5(["[M->G*M.]"])
m6("[M->G/.M]")
m7(["[M->G/M.]"])

m1 -- G --> m3
m3 -- * --> m4
m4 -- M --> m5
m3 -- / --> m6
m6 -- M --> m7
```

```mermaid
graph LR;
g1("[G->.P^G] | [G->.P]")
g5(["[G->P.^G] | [G->P.]"])
g6("[G->P^.G]")
g7(["[G->P^G.]"])

g1 -- P --> g5
g5 -- ^ --> g6
g6 -- G --> g7
```

```mermaid
graph LR;
p1("[P->.(A)] | [G->.ID]")
p2("[P->(.A)]")
p3("[P->(A.)]")
p4(["[P->(A).]"])
p5(["[P->ID.]"])
p1 -- "(" --> p2
p2 -- A --> p3
p3 -- ")" --> p4
p1 -- ID --> p5
```
