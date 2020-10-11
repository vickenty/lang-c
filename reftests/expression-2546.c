#pragma gnu
(struct test_struct) { 1, .x = 2, 3 }

/*===
Expression
    CompoundLiteral
        TypeName
            SpecifierQualifier
                TypeSpecifier
                    StructType
                        StructKind Struct
                        Identifier "test_struct"
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
