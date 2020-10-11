#pragma gnu
__asm ("pmovmskb %1, %0" : "=r" (__m) : "x" (__x));

/*===
Statement
    AsmStatement
        GnuExtendedAsmStatement
            StringLiteral ["\"pmovmskb %1, %0\""]
            GnuAsmOperand
                StringLiteral ["\"=r\""]
                Expression
                    Identifier "__m"
            GnuAsmOperand
                StringLiteral ["\"x\""]
                Expression
                    Identifier "__x"
===*/
