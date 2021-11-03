#pragma gnu
typedef enum { FOO, BAR = 1 } * const foobar;

#pragma is_typename foobar
/*===
Declaration
    DeclarationSpecifier
        StorageClassSpecifier Typedef
    DeclarationSpecifier
        TypeSpecifier
            EnumType
                Enumerator
                    Identifier "FOO"
                Enumerator
                    Identifier "BAR"
                    Expression
                        Constant
                            Integer "1"
                                IntegerBase Decimal
                                IntegerSuffix false false
                                    IntegerSize Int
    InitDeclarator
        Declarator
            DeclaratorKind
                Identifier "foobar"
            DerivedDeclarator Pointer
                PointerQualifier
                    TypeQualifier Const
===*/
