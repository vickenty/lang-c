#pragma gnu
int main(argc, argv) int argc; char **argv; { }

/*===
TranslationUnit
    ExternalDeclaration
        FunctionDefinition
            DeclarationSpecifier
                TypeSpecifier Int
            Declarator
                DeclaratorKind
                    Identifier "main"
                DerivedDeclarator
                    Identifier "argc"
                    Identifier "argv"
            Declaration
                DeclarationSpecifier
                    TypeSpecifier Int
                InitDeclarator
                    Declarator
                        DeclaratorKind
                            Identifier "argc"
            Declaration
                DeclarationSpecifier
                    TypeSpecifier Char
                InitDeclarator
                    Declarator
                        DeclaratorKind
                            Identifier "argv"
                        DerivedDeclarator
                        DerivedDeclarator
            Statement Compound
===*/
