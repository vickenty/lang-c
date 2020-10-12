// #27
#pragma gnu
#pragma typedef test_t
return (test_t) { 1, .x = 2, 3 };

/*===
Statement Return
    Expression
        CompoundLiteral
            TypeName
                SpecifierQualifier
                    TypeSpecifier TypedefName
                        Identifier "test_t"
            InitializerListItem
                Initializer
                    Expression
                        Constant
                            Integer "1"
                                IntegerBase Decimal
                                IntegerSuffix false false
                                    IntegerSize Int
            InitializerListItem
                Designator
                    Identifier "x"
                Initializer
                    Expression
                        Constant
                            Integer "2"
                                IntegerBase Decimal
                                IntegerSuffix false false
                                    IntegerSize Int
            InitializerListItem
                Initializer
                    Expression
                        Constant
                            Integer "3"
                                IntegerBase Decimal
                                IntegerSuffix false false
                                    IntegerSize Int
===*/
