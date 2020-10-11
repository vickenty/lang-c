// #27 - make sure expressions starting with a type name in parens still work
#pragma gnu
#pragma typedef test_t
return (test_t) + 1;

/*===
Statement Return
    Expression
        CastExpression
            TypeName
                SpecifierQualifier
                    TypeSpecifier TypedefName
                        Identifier "test_t"
            Expression
                UnaryOperatorExpression
                    UnaryOperator Plus
                    Expression
                        Constant
                            Integer "1"
                                IntegerBase Decimal
                                IntegerSuffix false false
                                    IntegerSize Int
===*/
