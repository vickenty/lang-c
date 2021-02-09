#pragma gnu

struct s {
    int i;;
};


/*===
Declaration
    DeclarationSpecifier
        TypeSpecifier
            StructType
                StructKind Struct
                Identifier "s"
                StructDeclaration
                    StructField
                        SpecifierQualifier
                            TypeSpecifier Int
                        StructDeclarator
                            Declarator
                                DeclaratorKind
                                    Identifier "i"
===*/
