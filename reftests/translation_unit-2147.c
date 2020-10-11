#pragma gnu

void foo() {
typedef int a;
{
a a;
}
_Atomic (a) b;
}

/*===
TranslationUnit
===*/
