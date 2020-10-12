#pragma gnu
a << b << c

/*===
Expression
    BinaryOperatorExpression
        Expression
            BinaryOperatorExpression
                Expression
                    Identifier "a"
                Expression
                    Identifier "b"
                BinaryOperator ShiftLeft
        Expression
            Identifier "c"
        BinaryOperator ShiftLeft
===*/
