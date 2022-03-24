# Architecture

Oracle follows a frontend-backend design. The frontend accepts textual input, which the backend translates into mathematics and returns the result back to the frontend, where the textual and/or graphical output is displayed.

![](./backend-model.svg)


## Frontend

## Backend

The backend follows a traditional compiler model:

1. The lexer tokenizes source code, stripping comments and whitespace in the process.
2. The parser assigns meaning to the tokens by grouping them into expressions.
3. The interpreter evaluates each expression.
