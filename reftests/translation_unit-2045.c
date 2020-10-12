// Struct fields maintain a separate

#pragma gnu

typedef int a;
struct a { a a; a b; };

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
        Declaration
            DeclarationSpecifier
                TypeSpecifier
                    StructType
                        StructKind Struct
                        Identifier "a"
                        StructDeclaration
                            StructField
                                SpecifierQualifier
                                    TypeSpecifier TypedefName
                                        Identifier "a"
                                StructDeclarator
                                    Declarator
                                        DeclaratorKind
                                            Identifier "a"
                        StructDeclaration
                            StructField
                                SpecifierQualifier
                                    TypeSpecifier TypedefName
                                        Identifier "a"
                                StructDeclarator
                                    Declarator
                                        DeclaratorKind
                                            Identifier "b"
===*/
