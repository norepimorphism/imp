# Architecture

IMP follows a frontend-backend design. The frontend is an interactive shell program that forwards textual input to the backend, which lexes, parses, and evaulates its input as a mathematical expression before returning results back to the frontend in either textual or graphical form.

![](high-level-architecture.dot.svg)

## Frontend

The frontend crate provided in this workspace is a reference implementation.

## Backend

The backend follows a traditional interpreter model:

1. The lexer tokenizes source code, stripping comments and whitespace in the process.
2. The parser assigns meaning to these tokens by grouping them into expressions.
3. These expressions are transformed across multiple intermediate passes.
3. The interpreter evaluates each expression.
