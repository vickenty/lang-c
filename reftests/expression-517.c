#pragma gnu
a-- * ++b / c

/*===
Expression
    BinaryOperatorExpression
        Expression
            BinaryOperatorExpression
                Expression
                    UnaryOperatorExpression
                        Expression
                            Identifier "a"
                        UnaryOperator PostDecrement
                Expression
                    UnaryOperatorExpression
                        UnaryOperator PreIncrement
                        Expression
                            Identifier "b"
                BinaryOperator Multiply
        Expression
            Identifier "c"
        BinaryOperator Divide
===*/
