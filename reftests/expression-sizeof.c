#pragma typedef a
sizeof(a) + sizeof b + sizeof(c*10)
/*===
Expression
    BinaryOperatorExpression
        Expression
            BinaryOperatorExpression
                Expression
                    SizeOfTy
                        TypeName
                            SpecifierQualifier
                                TypeSpecifier TypedefName
                                    Identifier "a"
                Expression
                    SizeOfVal
                        Expression
                            Identifier "b"
                BinaryOperator Plus
        Expression
            SizeOfVal
                Expression
                    BinaryOperatorExpression
                        Expression
                            Identifier "c"
                        Expression
                            Constant
                                Integer "10"
                                    IntegerBase Decimal
                                    IntegerSuffix false false
                                        IntegerSize Int
                        BinaryOperator Multiply
        BinaryOperator Plus
===*/
