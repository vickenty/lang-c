#pragma gnu

typedef int a;
int foo(a a) {}
int bar(int a);
_Atomic (a) b;


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
                DerivedDeclarator
                    FunctionDeclarator
                        ParameterDeclaration
                            DeclarationSpecifier
                                TypeSpecifier TypedefName
                                    Identifier "a"
                            Declarator
                                DeclaratorKind
                                    Identifier "a"
                        Ellipsis None
            Statement Compound
    ExternalDeclaration
        Declaration
            DeclarationSpecifier
                TypeSpecifier Int
            InitDeclarator
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
                                        Identifier "a"
                            Ellipsis None
    ExternalDeclaration
        Declaration
            DeclarationSpecifier
                TypeSpecifier Atomic
                    TypeName
                        SpecifierQualifier
                            TypeSpecifier TypedefName
                                Identifier "a"
            InitDeclarator
                Declarator
                    DeclaratorKind
                        Identifier "b"
===*/
