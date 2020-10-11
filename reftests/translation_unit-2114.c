#pragma gnu

typedef int a;
int foo() {
int x = (enum {a, b = (a)1})1;
}

/*===
~ERROR
===*/
