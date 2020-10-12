#pragma gnu
a.b->c[ d[ e ] ] ++

/*===
Expression
    UnaryOperatorExpression
        Expression
            BinaryOperatorExpression
                Expression
                    MemberExpression
                        MemberOperator Indirect
                        Expression
                            MemberExpression
                                MemberOperator Direct
                                Expression
                                    Identifier "a"
                                Identifier "b"
                        Identifier "c"
                Expression
                    BinaryOperatorExpression
                        Expression
                            Identifier "d"
                        Expression
                            Identifier "e"
                        BinaryOperator Index
                BinaryOperator Index
        UnaryOperator PostIncrement
===*/
