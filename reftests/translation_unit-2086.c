// Technically, "a" is defined as a symbol before the "= .." part of the initializer is parsed.

#pragma gnu

typedef int a;
int foo() {
int a = sizeof(_Atomic(a));
}

/*===
~ERROR
===*/
