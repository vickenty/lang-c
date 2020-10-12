#pragma gnu
__extension__ union { long l; };

/*===
TranslationUnit
    ExternalDeclaration
        Declaration
            DeclarationSpecifier
                TypeSpecifier
                    StructType
                        StructKind Union
                        StructDeclaration
                            StructField
                                SpecifierQualifier
                                    TypeSpecifier Long
                                StructDeclarator
                                    Declarator
                                        DeclaratorKind
                                            Identifier "l"
===*/
