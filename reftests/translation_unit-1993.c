#pragma gnu

typedef int a;
void foo() {
unsigned int;
const a;
a x;
unsigned a;
a = 1;
}

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
        FunctionDefinition
            DeclarationSpecifier
                TypeSpecifier Void
            Declarator
                DeclaratorKind
                    Identifier "foo"
                DerivedDeclarator KRFunction
            Statement Compound
                BlockItem
                    Declaration
                        DeclarationSpecifier
                            TypeSpecifier Unsigned
                        DeclarationSpecifier
                            TypeSpecifier Int
                BlockItem
                    Declaration
                        DeclarationSpecifier
                            TypeQualifier Const
                        DeclarationSpecifier
                            TypeSpecifier TypedefName
                                Identifier "a"
                BlockItem
                    Declaration
                        DeclarationSpecifier
                            TypeSpecifier TypedefName
                                Identifier "a"
                        InitDeclarator
                            Declarator
                                DeclaratorKind
                                    Identifier "x"
                BlockItem
                    Declaration
                        DeclarationSpecifier
                            TypeSpecifier Unsigned
                        InitDeclarator
                            Declarator
                                DeclaratorKind
                                    Identifier "a"
                BlockItem
                    Statement
                        Expression
                            BinaryOperatorExpression
                                Expression
                                    Identifier "a"
                                Expression
                                    Constant
                                        Integer "1"
                                            IntegerBase Decimal
                                            IntegerSuffix false false
                                                IntegerSize Int
                                BinaryOperator Assign
===*/
