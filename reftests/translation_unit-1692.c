#pragma gnu

int foo (int) __attribute__ ((__nothrow__));
typedef int named;
int bar (int f) { }


/*===
TranslationUnit
    ExternalDeclaration
        Declaration
            DeclarationSpecifier
                TypeSpecifier Int
            InitDeclarator
                Declarator
                    DeclaratorKind
                        Identifier "foo"
                    DerivedDeclarator
                        FunctionDeclarator
                            ParameterDeclaration
                                DeclarationSpecifier
                                    TypeSpecifier Int
                            Ellipsis None
                    Extension
                        Attribute "__nothrow__"
    ExternalDeclaration
        Declaration
            DeclarationSpecifier
                StorageClassSpecifier Typedef
            DeclarationSpecifier
                TypeSpecifier Int
            InitDeclarator
                Declarator
                    DeclaratorKind
                        Identifier "named"
    ExternalDeclaration
        FunctionDefinition
            DeclarationSpecifier
                TypeSpecifier Int
            Declarator
                DeclaratorKind
                    Identifier "bar"
                DerivedDeclarator
                    FunctionDeclarator
                        ParameterDeclaration
                            DeclarationSpecifier
                                TypeSpecifier Int
                            Declarator
                                DeclaratorKind
                                    Identifier "f"
                        Ellipsis None
            Statement Compound
===*/
