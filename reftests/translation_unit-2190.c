// Test that scope of "if" condition and statement is cleaned up.
#pragma gnu

typedef int a, b;
int x;
void foo() {
if (sizeof(enum {a})) x = sizeof(enum{b});
else x = b;
switch (sizeof(enum {b})) x = b;
a x, y;
b z, w;
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
            InitDeclarator
                Declarator
                    DeclaratorKind
                        Identifier "b"
    ExternalDeclaration
        Declaration
            DeclarationSpecifier
                TypeSpecifier Int
            InitDeclarator
                Declarator
                    DeclaratorKind
                        Identifier "x"
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
                        IfStatement
                            Expression
                                SizeOfTy
                                    TypeName
                                        SpecifierQualifier
                                            TypeSpecifier
                                                EnumType
                                                    Enumerator
                                                        Identifier "a"
                            Statement
                                Expression
                                    BinaryOperatorExpression
                                        Expression
                                            Identifier "x"
                                        Expression
                                            SizeOfTy
                                                TypeName
                                                    SpecifierQualifier
                                                        TypeSpecifier
                                                            EnumType
                                                                Enumerator
                                                                    Identifier "b"
                                        BinaryOperator Assign
                            Statement
                                Expression
                                    BinaryOperatorExpression
                                        Expression
                                            Identifier "x"
                                        Expression
                                            Identifier "b"
                                        BinaryOperator Assign
                BlockItem
                    Statement
                        SwitchStatement
                            Expression
                                SizeOfTy
                                    TypeName
                                        SpecifierQualifier
                                            TypeSpecifier
                                                EnumType
                                                    Enumerator
                                                        Identifier "b"
                            Statement
                                Expression
                                    BinaryOperatorExpression
                                        Expression
                                            Identifier "x"
                                        Expression
                                            Identifier "b"
                                        BinaryOperator Assign
                BlockItem
                    Declaration
                        DeclarationSpecifier
                            TypeSpecifier TypedefName
                                Identifier "a"
                        InitDeclarator
                            Declarator
                                DeclaratorKind
                                    Identifier "x"
                        InitDeclarator
                            Declarator
                                DeclaratorKind
                                    Identifier "y"
                BlockItem
                    Declaration
                        DeclarationSpecifier
                            TypeSpecifier TypedefName
                                Identifier "b"
                        InitDeclarator
                            Declarator
                                DeclaratorKind
                                    Identifier "z"
                        InitDeclarator
                            Declarator
                                DeclaratorKind
                                    Identifier "w"
===*/
