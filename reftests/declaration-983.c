#pragma gnu
#pragma typedef size_t
extern int strerror_r (int __errnum, char *__buf, size_t __buflen)
__asm__  ("" "__xpg_strerror_r") __attribute__ ((__nothrow__ , __leaf__))
__attribute__ ((__nonnull__ (2)));

/*===
Declaration
    DeclarationSpecifier
        StorageClassSpecifier Extern
    DeclarationSpecifier
        TypeSpecifier Int
    InitDeclarator
        Declarator
            DeclaratorKind
                Identifier "strerror_r"
            DerivedDeclarator
                FunctionDeclarator
                    ParameterDeclaration
                        DeclarationSpecifier
                            TypeSpecifier Int
                        Declarator
                            DeclaratorKind
                                Identifier "__errnum"
                    ParameterDeclaration
                        DeclarationSpecifier
                            TypeSpecifier Char
                        Declarator
                            DeclaratorKind
                                Identifier "__buf"
                            DerivedDeclarator Pointer
                    ParameterDeclaration
                        DeclarationSpecifier
                            TypeSpecifier TypedefName
                                Identifier "size_t"
                        Declarator
                            DeclaratorKind
                                Identifier "__buflen"
                    Ellipsis None
            Extension
                StringLiteral ["\"\"", "\"__xpg_strerror_r\""]
            Extension
                Attribute "__nothrow__"
            Extension
                Attribute "__leaf__"
            Extension
                Attribute "__nonnull__"
                    Expression
                        Constant
                            Integer "2"
                                IntegerBase Decimal
                                IntegerSuffix false false
                                    IntegerSize Int
===*/
