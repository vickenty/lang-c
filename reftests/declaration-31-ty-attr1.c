#pragma gnu

// #31

struct s {
    struct t {
        int i;
    } __attribute((packed)) v;
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
                            TypeSpecifier
                                StructType
                                    StructKind Struct
                                    Identifier "t"
                                    StructDeclaration
                                        StructField
                                            SpecifierQualifier
                                                TypeSpecifier Int
                                            StructDeclarator
                                                Declarator
                                                    DeclaratorKind
                                                        Identifier "i"
                        SpecifierQualifier
                            Extension
                                Attribute "packed"
                        StructDeclarator
                            Declarator
                                DeclaratorKind
                                    Identifier "v"
===*/
