#pragma gnu
typedef struct {
long long __max_align_ll __attribute__((__aligned__(__alignof__(long long))));
long double __max_align_ld __attribute__((__aligned__(__alignof__(long double))));
} max_align_t;

/*===
Declaration
    DeclarationSpecifier
        StorageClassSpecifier Typedef
    DeclarationSpecifier
        TypeSpecifier
            StructType
                StructKind Struct
                StructDeclaration
                    StructField
                        SpecifierQualifier
                            TypeSpecifier Long
                        SpecifierQualifier
                            TypeSpecifier Long
                        StructDeclarator
                            Declarator
                                DeclaratorKind
                                    Identifier "__max_align_ll"
                                Extension
                                    Attribute "__aligned__"
                                        Expression
                                            AlignOf
                                                TypeName
                                                    SpecifierQualifier
                                                        TypeSpecifier Long
                                                    SpecifierQualifier
                                                        TypeSpecifier Long
                StructDeclaration
                    StructField
                        SpecifierQualifier
                            TypeSpecifier Long
                        SpecifierQualifier
                            TypeSpecifier Double
                        StructDeclarator
                            Declarator
                                DeclaratorKind
                                    Identifier "__max_align_ld"
                                Extension
                                    Attribute "__aligned__"
                                        Expression
                                            AlignOf
                                                TypeName
                                                    SpecifierQualifier
                                                        TypeSpecifier Long
                                                    SpecifierQualifier
                                                        TypeSpecifier Double
    InitDeclarator
        Declarator
            DeclaratorKind
                Identifier "max_align_t"
===*/
