#pragma gnu
int foo(int a __attribute__((unused)), int b __attribute__((unused))) {}

/*===
TranslationUnit
    ExternalDeclaration
        FunctionDefinition
            DeclarationSpecifier
                TypeSpecifier Int
            Declarator
                DeclaratorKind
                    Identifier "foo"
                DerivedDeclarator
                    FunctionDeclarator
                        ParameterDeclaration
                            DeclarationSpecifier
                                TypeSpecifier Int
                            Declarator
                                DeclaratorKind
                                    Identifier "a"
                            Extension
                                Attribute "unused"
                        ParameterDeclaration
                            DeclarationSpecifier
                                TypeSpecifier Int
                            Declarator
                                DeclaratorKind
                                    Identifier "b"
                            Extension
                                Attribute "unused"
                        Ellipsis None
            Statement Compound
===*/
