# Grammar

```abnf
program = *expression
expression = *"(" operation *operand *")"
operation = 1*ALPHA / operator
operator = "+" / "-" / "*" / "/"
operand = expression / number
number = 1*DIGIT
```
