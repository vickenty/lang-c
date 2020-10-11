#pragma gnu
if (x) do {} while(y); else z();

/*===
Statement
    IfStatement
        Expression
            Identifier "x"
        Statement
            DoWhileStatement
                Statement Compound
                Expression
                    Identifier "y"
        Statement
            Expression
                CallExpression
                    Expression
                        Identifier "z"
===*/
