#pragma gnu

void (__attribute__((noreturn)) ****f) (void);

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
                    DerivedDeclarator Pointer
                    DerivedDeclarator Pointer
                    DerivedDeclarator Pointer
                    DerivedDeclarator Pointer
                    Extension
                        Attribute "noreturn"
            DerivedDeclarator
                FunctionDeclarator
                    ParameterDeclaration
                        DeclarationSpecifier
                            TypeSpecifier Void
                    Ellipsis None
===*/
