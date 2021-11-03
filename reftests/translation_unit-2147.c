#pragma gnu

void foo() {
typedef int a;
{
a a;
}
_Atomic (a) b;
}

/*===
TranslationUnit
    ExternalDeclaration
        FunctionDefinition
            DeclarationSpecifier
                TypeSpecifier Void
            Declarator
                DeclaratorKind
                    Identifier "foo"
                DerivedDeclarator KRFunction
            Statement Compound
                BlockItem
                    Declaration
                        DeclarationSpecifier
                            StorageClassSpecifier Typedef
                        DeclarationSpecifier
                            TypeSpecifier Int
                        InitDeclarator
                            Declarator
                                DeclaratorKind
                                    Identifier "a"
                BlockItem
                    Statement Compound
                        BlockItem
                            Declaration
                                DeclarationSpecifier
                                    TypeSpecifier TypedefName
                                        Identifier "a"
                                InitDeclarator
                                    Declarator
                                        DeclaratorKind
                                            Identifier "a"
                BlockItem
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
