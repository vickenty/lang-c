// enum {a} defines a new variable "a" into the current scope immediately after its declaration.
#pragma gnu

typedef int a;
int foo() {
int x = (enum {a, b = (a)1})1;
}

/*===
~ERROR
===*/
