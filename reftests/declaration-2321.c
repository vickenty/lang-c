// This is the first Clang-specific declaration you'll encounter in macOS
// if you #include <stdio.h>.

#pragma gnu
#pragma clang
int (* _Nullable _close)(void *);

/*===
Declaration
    DeclarationSpecifier
        TypeSpecifier Int
    InitDeclarator
        Declarator
            DeclaratorKind
                Declarator
                    DeclaratorKind
                        Identifier "_close"
                    DerivedDeclarator Pointer
                        PointerQualifier
                            TypeQualifier Nullable
            DerivedDeclarator
                FunctionDeclarator
                    ParameterDeclaration
                        DeclarationSpecifier
                            TypeSpecifier Void
                        Declarator
                            DeclaratorKind Abstract
                            DerivedDeclarator Pointer
                    Ellipsis None
===*/
