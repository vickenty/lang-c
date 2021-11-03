#pragma gnu
extern __inline __attribute__ ((__always_inline__)) __attribute__
((__artificial__)) __attribute__ ((__warn_unused_result__)) char *
__attribute__ ((__nothrow__ , __leaf__)) realpath (const char *__restrict
__name, char *__restrict __resolved) {}

/*===
TranslationUnit
    ExternalDeclaration
        FunctionDefinition
            DeclarationSpecifier
                StorageClassSpecifier Extern
            DeclarationSpecifier
                FunctionSpecifier Inline
            DeclarationSpecifier
                Extension
                    Attribute "__always_inline__"
            DeclarationSpecifier
                Extension
                    Attribute "__artificial__"
            DeclarationSpecifier
                Extension
                    Attribute "__warn_unused_result__"
            DeclarationSpecifier
                TypeSpecifier Char
            Declarator
                DeclaratorKind
                    Identifier "realpath"
                DerivedDeclarator Pointer
                    PointerQualifier
                        Extension
                            Attribute "__nothrow__"
                        Extension
                            Attribute "__leaf__"
                DerivedDeclarator
                    FunctionDeclarator
                        ParameterDeclaration
                            DeclarationSpecifier
                                TypeQualifier Const
                            DeclarationSpecifier
                                TypeSpecifier Char
                            Declarator
                                DeclaratorKind
                                    Identifier "__name"
                                DerivedDeclarator Pointer
                                    PointerQualifier
                                        TypeQualifier Restrict
                        ParameterDeclaration
                            DeclarationSpecifier
                                TypeSpecifier Char
                            Declarator
                                DeclaratorKind
                                    Identifier "__resolved"
                                DerivedDeclarator Pointer
                                    PointerQualifier
                                        TypeQualifier Restrict
                        Ellipsis None
            Statement Compound
===*/
