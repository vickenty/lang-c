#pragma clang

void (^ const p)(int);

/*===
Declaration
    DeclarationSpecifier
        TypeSpecifier Void
    InitDeclarator
        Declarator
            DeclaratorKind
                Declarator
                    DeclaratorKind
                        Identifier "p"
                    PointerDeclarator Block
                        PointerQualifier
                            TypeQualifier Const
            DerivedDeclarator
                FunctionDeclarator
                    ParameterDeclaration
                        DeclarationSpecifier
                            TypeSpecifier Int
                    Ellipsis None
===*/
