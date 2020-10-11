#pragma gnu
struct foo { } S;

/*===
Declaration
    DeclarationSpecifier
        TypeSpecifier
            StructType
                StructKind Struct
                Identifier "foo"
    InitDeclarator
        Declarator
            DeclaratorKind
                Identifier "S"
===*/
