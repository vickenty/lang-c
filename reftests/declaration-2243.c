#pragma gnu
_Float64 foo = 1.5;

/*===
Declaration
    DeclarationSpecifier
        TypeSpecifier
            TS18661FloatType 64
                TS18661FloatFormat BinaryInterchange
    InitDeclarator
        Declarator
            DeclaratorKind
                Identifier "foo"
        Initializer
            Expression
                Constant
                    Float "1.5"
                        FloatBase Decimal
                        FloatSuffix false
                            FloatFormat Double
===*/
