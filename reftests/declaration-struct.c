struct foo {
int x;
};

/*===
Declaration
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
===*/
