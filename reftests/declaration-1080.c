#pragma gnu
__attribute__((noreturn)) void d0 (void),
__attribute__((format(printf, 1, 2))) d1 (const char *, ...),
d2 (void);

/*===
Declaration
    DeclarationSpecifier
        Extension
            Attribute "noreturn"
    DeclarationSpecifier
        TypeSpecifier Void
    InitDeclarator
        Declarator
            DeclaratorKind
                Identifier "d0"
            DerivedDeclarator
                FunctionDeclarator
                    ParameterDeclaration
                        DeclarationSpecifier
                            TypeSpecifier Void
                    Ellipsis None
    InitDeclarator
        Declarator
            DeclaratorKind
                Identifier "d1"
            DerivedDeclarator
                FunctionDeclarator
                    ParameterDeclaration
                        DeclarationSpecifier
                            TypeQualifier Const
                        DeclarationSpecifier
                            TypeSpecifier Char
                        Declarator
                            DeclaratorKind Abstract
                            DerivedDeclarator Pointer
                    Ellipsis Some
            Extension
                Attribute "format"
                    Expression
                        Identifier "printf"
                    Expression
                        Constant
                            Integer "1"
                                IntegerBase Decimal
                                IntegerSuffix false false
                                    IntegerSize Int
                    Expression
                        Constant
                            Integer "2"
                                IntegerBase Decimal
                                IntegerSuffix false false
                                    IntegerSize Int
    InitDeclarator
        Declarator
            DeclaratorKind
                Identifier "d2"
            DerivedDeclarator
                FunctionDeclarator
                    ParameterDeclaration
                        DeclarationSpecifier
                            TypeSpecifier Void
                    Ellipsis None
===*/
