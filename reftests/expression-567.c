#pragma gnu
a || b || c

/*===
Expression
    BinaryOperatorExpression
        Expression
            BinaryOperatorExpression
                Expression
                    Identifier "a"
                Expression
                    Identifier "b"
                BinaryOperator LogicalOr
        Expression
            Identifier "c"
        BinaryOperator LogicalOr
===*/
