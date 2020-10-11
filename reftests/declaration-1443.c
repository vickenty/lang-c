#pragma gnu
union { long double __l; int __i[3]; } __u = { __l: __x };

/*===
Declaration
    DeclarationSpecifier
        TypeSpecifier
            StructType
                StructKind Union
                StructDeclaration
                    StructField
                        SpecifierQualifier
                            TypeSpecifier Long
                        SpecifierQualifier
                            TypeSpecifier Double
                        StructDeclarator
                            Declarator
                                DeclaratorKind
                                    Identifier "__l"
                StructDeclaration
                    StructField
                        SpecifierQualifier
                            TypeSpecifier Int
                        StructDeclarator
                            Declarator
                                DeclaratorKind
                                    Identifier "__i"
                                DerivedDeclarator
                                    ArrayDeclarator
                                        ArraySize VariableExpression
                                            Expression
                                                Constant
                                                    Integer "3"
                                                        IntegerBase Decimal
                                                        IntegerSuffix false false
                                                            IntegerSize Int
    InitDeclarator
        Declarator
            DeclaratorKind
                Identifier "__u"
        Initializer
            InitializerListItem
                Designator
                    Identifier "__l"
                Initializer
                    Expression
                        Identifier "__x"
===*/
