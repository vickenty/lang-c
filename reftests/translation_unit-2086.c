#pragma gnu

typedef int a;
int foo() {
int a = sizeof(_Atomic(a));
}

/*===
~ERROR
===*/
