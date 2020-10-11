#pragma gnu

typedef int a;
void foo() {
for (a a;;)
a = a;
while (true) {int a;}
do { int a; } while(true);
_Atomic (a) b;
}

/*===
TranslationUnit
===*/
