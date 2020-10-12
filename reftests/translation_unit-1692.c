// Check that a typedef that can be mistaken for a K&R-style argument declaration is correctly
// parsed as an external declaration. What went wrong: until we encounter bar, the thing looks like
// a function definition, where the name is followed by a two declarations K&R-style, similar to:
//
// ```
// int foo(i)
// int i; // <-- __attribute__ and typedef occupy this slot, since both are valid declarations.
// { }
// ```

#pragma gnu

int foo (int) __attribute__ ((__nothrow__));
typedef int named;
int bar (int f) { }


/*===
TranslationUnit
    ExternalDeclaration
        Declaration
            DeclarationSpecifier
                TypeSpecifier Int
            InitDeclarator
                Declarator
                    DeclaratorKind
                        Identifier "foo"
                    DerivedDeclarator
                        FunctionDeclarator
                            ParameterDeclaration
                                DeclarationSpecifier
                                    TypeSpecifier Int
                            Ellipsis None
                    Extension
                        Attribute "__nothrow__"
    ExternalDeclaration
        Declaration
            DeclarationSpecifier
                StorageClassSpecifier Typedef
            DeclarationSpecifier
                TypeSpecifier Int
            InitDeclarator
                Declarator
                    DeclaratorKind
                        Identifier "named"
    ExternalDeclaration
        FunctionDefinition
            DeclarationSpecifier
                TypeSpecifier Int
            Declarator
                DeclaratorKind
                    Identifier "bar"
                DerivedDeclarator
                    FunctionDeclarator
                        ParameterDeclaration
                            DeclarationSpecifier
                                TypeSpecifier Int
                            Declarator
                                DeclaratorKind
                                    Identifier "f"
                        Ellipsis None
            Statement Compound
===*/
