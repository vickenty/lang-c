#pragma gnu

char *__attribute__((aligned(8))) *f;
/*===
Declaration
    DeclarationSpecifier
        TypeSpecifier Char
    InitDeclarator
        Declarator
            DeclaratorKind
                Identifier "f"
            DerivedDeclarator
                PointerQualifier
                    Extension
                        Attribute "aligned"
                            Expression
                                Constant
                                    Integer "8"
                                        IntegerBase Decimal
                                        IntegerSuffix false false
                                            IntegerSize Int
            DerivedDeclarator
===*/
