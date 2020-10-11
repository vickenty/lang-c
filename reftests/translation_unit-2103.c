#pragma gnu

typedef int a;
int foo() {
int x = (enum {a})1;
_Atomic(a) b;
}

/*===
~ERROR
===*/
