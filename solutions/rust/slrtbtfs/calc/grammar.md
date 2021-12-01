# Grammars

## Simple Version

```rust
S       -> Expr#
Expr    -> Sum
Sum     -> Mul    | Sum '+' Mul   | Sum '-' Mul
Mul     -> Unary  | Mul '*' Unary | Mul '/' Unary
Unary   -> Power  | '+' Unary     | '-' Unary
Power   -> Atomic | Atomic '^' Power
Atomic  -> Number | Paren
Paren   -> '(' Expr ')'
```

## Without left Recursion

Regular expression syntax is used here, since we can easily do that in a hand rolled parser.

```rust
S       -> Expr#
Expr    -> Sum
Sum     -> Mul (SumOp Mul)*
Mul     -> Unary (MulOp Unary)*
Unary   -> Power | SumOp Unary
Power   -> Atomic | Atomic PowerOp Power
Atomic  -> Number | Paren
Paren   -> '(' Expr ')'

SumOp   -> '+' | '-'
MulOp   -> '*' | '/'
PowerOp -> '^'
```
