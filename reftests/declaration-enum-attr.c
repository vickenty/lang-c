#pragma gnu

enum {
    SOME_THING = 0,
    OTHER_THING = 1,
    OLD_THING __attribute__((deprecated)) = 2,
};

/*===
Declaration
    DeclarationSpecifier
        TypeSpecifier
            EnumType
                Enumerator
                    Identifier "SOME_THING"
                    Expression
                        Constant
                            Integer "0"
                                IntegerBase Decimal
                                IntegerSuffix false false
                                    IntegerSize Int
                Enumerator
                    Identifier "OTHER_THING"
                    Expression
                        Constant
                            Integer "1"
                                IntegerBase Decimal
                                IntegerSuffix false false
                                    IntegerSize Int
                Enumerator
                    Identifier "OLD_THING"
                    Expression
                        Constant
                            Integer "2"
                                IntegerBase Decimal
                                IntegerSuffix false false
                                    IntegerSize Int
                    Extension
                        Attribute "deprecated"
===*/
