#pragma gnu
switch(num) {
    for(;;) {
	case 1 ... 10:
	return;
	break;
    }
}
/*===
Statement
    SwitchStatement
        Expression
            Identifier "num"
        Statement Compound
            BlockItem
                Statement
                    ForStatement
                        ForInitializer Empty
                        Statement Compound
                            BlockItem
                                Statement
                                    LabeledStatement
                                        Label
                                            CaseRange
                                                Expression
                                                    Constant
                                                        Integer "1"
                                                            IntegerBase Decimal
                                                            IntegerSuffix false false
                                                                IntegerSize Int
                                                Expression
                                                    Constant
                                                        Integer "10"
                                                            IntegerBase Decimal
                                                            IntegerSuffix false false
                                                                IntegerSize Int
                                        Statement Return
                            BlockItem
                                Statement Break
===*/
