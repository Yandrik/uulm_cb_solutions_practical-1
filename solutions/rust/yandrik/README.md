# OverComplicatedCalculator

A parser project for the Compilerbau Vorlesung in WiSe 2021/2022 at Ulm University

## How to build

Install cargo (e.g. from https://rustup.rs). Then open a terminal in the projects root directory, and
run `cargo build` (or `cargo run` to run instantly)

## Grammar

The grammar implemented in this parser is documented [here](./grammar.md).

## Correctness

The implementation is not quite up to spec, as `5/3*2` is interpreted as `5/(3*2)` instead of the desired `(5/3)*2`. I
personally believe that this is not a problem.
