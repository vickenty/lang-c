#pragma gnu
#pragma clang
int f __attribute__((availability(p1,introduced=1.2.3))) __attribute__((availability(p2,unavailable,replacement="f2")));

/*===
Declaration
    DeclarationSpecifier
        TypeSpecifier Int
    InitDeclarator
        Declarator
            DeclaratorKind
                Identifier "f"
            Extension
                AvailabilityAttribute
            Extension
                AvailabilityAttribute
===*/
