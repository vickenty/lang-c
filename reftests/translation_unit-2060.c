#pragma gnu

typedef int a;
void foo() {
a a;
_Atomic (a) b;
}

/*===
~ERROR
===*/
