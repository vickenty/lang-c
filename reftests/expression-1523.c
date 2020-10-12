#pragma gnu
__builtin_offsetof(struct { struct { int b; } a[2]; }, a->b)

/*===
Expression
    OffsetOfExpression
        TypeName
            SpecifierQualifier
                TypeSpecifier
                    StructType
                        StructKind Struct
                        StructDeclaration
                            StructField
                                SpecifierQualifier
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
                                                                Identifier "b"
                                StructDeclarator
                                    Declarator
                                        DeclaratorKind
                                            Identifier "a"
                                        DerivedDeclarator
                                            ArrayDeclarator
                                                ArraySize VariableExpression
                                                    Expression
                                                        Constant
                                                            Integer "2"
                                                                IntegerBase Decimal
                                                                IntegerSuffix false false
                                                                    IntegerSize Int
        OffsetDesignator
            Identifier "a"
            OffsetMember IndirectMember
                Identifier "b"
===*/
