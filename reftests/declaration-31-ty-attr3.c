#pragma gnu

struct s {
    int __attribute__((aligned(8))) *i;
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
                        SpecifierQualifier
                            Extension
                                Attribute "aligned"
                                    Expression
                                        Constant
                                            Integer "8"
                                                IntegerBase Decimal
                                                IntegerSuffix false false
                                                    IntegerSize Int
                        StructDeclarator
                            Declarator
                                DeclaratorKind
                                    Identifier "i"
                                DerivedDeclarator Pointer
===*/
