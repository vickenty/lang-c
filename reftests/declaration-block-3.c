#pragma clang
#pragma typedef size_t

void *bsearch_b(const void *__key, const void *__base, size_t __nel,
     size_t __width, int (^ _Nonnull __compar)(const void *, const void *) __attribute__((__noescape__)))
     __attribute__((availability(macosx,introduced=10.6)));

/*===
Declaration
    DeclarationSpecifier
        TypeSpecifier Void
    InitDeclarator
        Declarator
            DeclaratorKind
                Identifier "bsearch_b"
            DerivedDeclarator Pointer
            DerivedDeclarator
                FunctionDeclarator
                    ParameterDeclaration
                        DeclarationSpecifier
                            TypeQualifier Const
                        DeclarationSpecifier
                            TypeSpecifier Void
                        Declarator
                            DeclaratorKind
                                Identifier "__key"
                            DerivedDeclarator Pointer
                    ParameterDeclaration
                        DeclarationSpecifier
                            TypeQualifier Const
                        DeclarationSpecifier
                            TypeSpecifier Void
                        Declarator
                            DeclaratorKind
                                Identifier "__base"
                            DerivedDeclarator Pointer
                    ParameterDeclaration
                        DeclarationSpecifier
                            TypeSpecifier TypedefName
                                Identifier "size_t"
                        Declarator
                            DeclaratorKind
                                Identifier "__nel"
                    ParameterDeclaration
                        DeclarationSpecifier
                            TypeSpecifier TypedefName
                                Identifier "size_t"
                        Declarator
                            DeclaratorKind
                                Identifier "__width"
                    ParameterDeclaration
                        DeclarationSpecifier
                            TypeSpecifier Int
                        Declarator
                            DeclaratorKind
                                Declarator
                                    DeclaratorKind
                                        Identifier "__compar"
                                    DerivedDeclarator Block
                                        PointerQualifier
                                            TypeQualifier Nonnull
                            DerivedDeclarator
                                FunctionDeclarator
                                    ParameterDeclaration
                                        DeclarationSpecifier
                                            TypeQualifier Const
                                        DeclarationSpecifier
                                            TypeSpecifier Void
                                        Declarator
                                            DeclaratorKind Abstract
                                            DerivedDeclarator Pointer
                                    ParameterDeclaration
                                        DeclarationSpecifier
                                            TypeQualifier Const
                                        DeclarationSpecifier
                                            TypeSpecifier Void
                                        Declarator
                                            DeclaratorKind Abstract
                                            DerivedDeclarator Pointer
                                    Ellipsis None
                        Extension
                            Attribute "__noescape__"
                    Ellipsis None
            Extension
                AvailabilityAttribute
===*/
