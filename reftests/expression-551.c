#pragma gnu
a && b && c

/*===
Expression
    BinaryOperatorExpression
        Expression
            BinaryOperatorExpression
                Expression
                    Identifier "a"
                Expression
                    Identifier "b"
                BinaryOperator LogicalAnd
        Expression
            Identifier "c"
        BinaryOperator LogicalAnd
===*/
