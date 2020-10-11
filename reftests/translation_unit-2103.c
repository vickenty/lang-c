// enum {a} defines a new variable "a" into the current scope. So the next _Atomic(a) must fail.

#pragma gnu

typedef int a;
int foo() {
int x = (enum {a})1;
_Atomic(a) b;
}

/*===
~ERROR
===*/
