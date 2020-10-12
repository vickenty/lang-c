#pragma gnu

typedef int a;
void foo(int a, _Atomic (a) b) {}

/*===
~ERROR
===*/
