#pragma gnu

void (****f)(void) __attribute__((noreturn));

/*===
Declaration
    DeclarationSpecifier
        TypeSpecifier Void
    InitDeclarator
        Declarator
            DeclaratorKind
                Declarator
                    DeclaratorKind
                        Identifier "f"
                    PointerDeclarator Pointer
                    PointerDeclarator Pointer
                    PointerDeclarator Pointer
                    PointerDeclarator Pointer
            DerivedDeclarator
                FunctionDeclarator
                    ParameterDeclaration
                        DeclarationSpecifier
                            TypeSpecifier Void
                    Ellipsis None
            Extension
                Attribute "noreturn"
===*/
