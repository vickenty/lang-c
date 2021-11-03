#pragma gnu

typedef int a;
void foo() {
for (a a;;)
a = a;
while (true) {int a;}
do { int a; } while(true);
_Atomic (a) b;
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
                    Statement
                        ForStatement
                            ForInitializer
                                Declaration
                                    DeclarationSpecifier
                                        TypeSpecifier TypedefName
                                            Identifier "a"
                                    InitDeclarator
                                        Declarator
                                            DeclaratorKind
                                                Identifier "a"
                            Statement
                                Expression
                                    BinaryOperatorExpression
                                        Expression
                                            Identifier "a"
                                        Expression
                                            Identifier "a"
                                        BinaryOperator Assign
                BlockItem
                    Statement
                        WhileStatement
                            Expression
                                Identifier "true"
                            Statement Compound
                                BlockItem
                                    Declaration
                                        DeclarationSpecifier
                                            TypeSpecifier Int
                                        InitDeclarator
                                            Declarator
                                                DeclaratorKind
                                                    Identifier "a"
                BlockItem
                    Statement
                        DoWhileStatement
                            Statement Compound
                                BlockItem
                                    Declaration
                                        DeclarationSpecifier
                                            TypeSpecifier Int
                                        InitDeclarator
                                            Declarator
                                                DeclaratorKind
                                                    Identifier "a"
                            Expression
                                Identifier "true"
                BlockItem
                    Declaration
                        DeclarationSpecifier
                            TypeSpecifier Atomic
                                TypeName
                                    SpecifierQualifier
                                        TypeSpecifier TypedefName
                                            Identifier "a"
                        InitDeclarator
                            Declarator
                                DeclaratorKind
                                    Identifier "b"
===*/
