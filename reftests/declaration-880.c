#pragma gnu
#pragma typedef FILE
#pragma typedef size_t
char *fparseln(FILE *, size_t *, size_t *, const char[3], int);

/*===
Declaration
    DeclarationSpecifier
        TypeSpecifier Char
    InitDeclarator
        Declarator
            DeclaratorKind
                Identifier "fparseln"
            DerivedDeclarator Pointer
            DerivedDeclarator
                FunctionDeclarator
                    ParameterDeclaration
                        DeclarationSpecifier
                            TypeSpecifier TypedefName
                                Identifier "FILE"
                        Declarator
                            DeclaratorKind Abstract
                            DerivedDeclarator Pointer
                    ParameterDeclaration
                        DeclarationSpecifier
                            TypeSpecifier TypedefName
                                Identifier "size_t"
                        Declarator
                            DeclaratorKind Abstract
                            DerivedDeclarator Pointer
                    ParameterDeclaration
                        DeclarationSpecifier
                            TypeSpecifier TypedefName
                                Identifier "size_t"
                        Declarator
                            DeclaratorKind Abstract
                            DerivedDeclarator Pointer
                    ParameterDeclaration
                        DeclarationSpecifier
                            TypeQualifier Const
                        DeclarationSpecifier
                            TypeSpecifier Char
                        Declarator
                            DeclaratorKind Abstract
                            DerivedDeclarator
                                ArrayDeclarator
                                    ArraySize VariableExpression
                                        Expression
                                            Constant
                                                Integer "3"
                                                    IntegerBase Decimal
                                                    IntegerSuffix false false
                                                        IntegerSize Int
                    ParameterDeclaration
                        DeclarationSpecifier
                            TypeSpecifier Int
                    Ellipsis None
===*/
