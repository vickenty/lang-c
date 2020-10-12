#pragma gnu
__typeof__(foo(bar, baz)) ook = foo(bar, baz);

/*===
Declaration
    DeclarationSpecifier
        TypeSpecifier
            TypeOf
                Expression
                    CallExpression
                        Expression
                            Identifier "foo"
                        Expression
                            Identifier "bar"
                        Expression
                            Identifier "baz"
    InitDeclarator
        Declarator
            DeclaratorKind
                Identifier "ook"
        Initializer
            Expression
                CallExpression
                    Expression
                        Identifier "foo"
                    Expression
                        Identifier "bar"
                    Expression
                        Identifier "baz"
===*/
