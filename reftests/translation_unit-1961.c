#pragma gnu

typedef int a;
int foo() {
int a;
}

/*===
TranslationUnit
    ExternalDeclaration
        Declaration
            DeclarationSpecifier
                StorageClassSpecifier Typedef
            DeclarationSpecifier
                TypeSpecifier Int
            InitDeclarator
                Declarator
                    DeclaratorKind
                        Identifier "a"
    ExternalDeclaration
        FunctionDefinition
            DeclarationSpecifier
                TypeSpecifier Int
            Declarator
                DeclaratorKind
                    Identifier "foo"
                DerivedDeclarator KRFunction
            Statement Compound
                BlockItem
                    Declaration
                        DeclarationSpecifier
                            TypeSpecifier Int
                        InitDeclarator
                            Declarator
                                DeclaratorKind
                                    Identifier "a"
===*/
