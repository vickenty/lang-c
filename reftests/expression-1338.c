#pragma gnu
({ int p = 0; p; })

/*===
Expression
    Statement Compound
        BlockItem
            Declaration
                DeclarationSpecifier
                    TypeSpecifier Int
                InitDeclarator
                    Declarator
                        DeclaratorKind
                            Identifier "p"
                    Initializer
                        Expression
                            Constant
                                Integer "0"
                                    IntegerBase Decimal
                                    IntegerSuffix false false
                                        IntegerSize Int
        BlockItem
            Statement
                Expression
                    Identifier "p"
===*/
