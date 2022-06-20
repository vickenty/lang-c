typedef struct foo {
int x;
} bar;

/*===
Declaration
    DeclarationSpecifier
        StorageClassSpecifier Typedef
    DeclarationSpecifier
        TypeSpecifier
            StructType
                StructKind Struct
                Identifier "foo"
                StructDeclaration
                    StructField
                        SpecifierQualifier
                            TypeSpecifier Int
                        StructDeclarator
                            Declarator
                                DeclaratorKind
                                    Identifier "x"
    InitDeclarator
        Declarator
            DeclaratorKind
                Identifier "bar"
===*/
