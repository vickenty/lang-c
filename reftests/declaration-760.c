#pragma gnu
struct { int a, b; float c; } S;

/*===
Declaration
    DeclarationSpecifier
        TypeSpecifier
            StructType
                StructKind Struct
                StructDeclaration
                    StructField
                        SpecifierQualifier
                            TypeSpecifier Int
                        StructDeclarator
                            Declarator
                                DeclaratorKind
                                    Identifier "a"
                        StructDeclarator
                            Declarator
                                DeclaratorKind
                                    Identifier "b"
                StructDeclaration
                    StructField
                        SpecifierQualifier
                            TypeSpecifier Float
                        StructDeclarator
                            Declarator
                                DeclaratorKind
                                    Identifier "c"
    InitDeclarator
        Declarator
            DeclaratorKind
                Identifier "S"
===*/
