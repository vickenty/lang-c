#pragma gnu

struct s {
    union { int i; } __attribute__((aligned(8)));
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
                                    StructKind Union
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
                                Attribute "aligned"
                                    Expression
                                        Constant
                                            Integer "8"
                                                IntegerBase Decimal
                                                IntegerSuffix false false
                                                    IntegerSize Int
===*/
