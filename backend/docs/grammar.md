# Grammar

```abnf
program = 1*expression
expression = *"(" operation 1*operand *")"
operation = 1*ALPHA / operator
operator = "+" / "-" / "*" / "/"
operand = expression / number
number = 1*DIGIT
```
