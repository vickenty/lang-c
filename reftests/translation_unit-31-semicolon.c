#pragma gnu

void f(void) {
};


/*===
TranslationUnit
    ExternalDeclaration
        FunctionDefinition
            DeclarationSpecifier
                TypeSpecifier Void
            Declarator
                DeclaratorKind
                    Identifier "f"
                DerivedDeclarator
                    FunctionDeclarator
                        ParameterDeclaration
                            DeclarationSpecifier
                                TypeSpecifier Void
                        Ellipsis None
            Statement Compound
===*/
