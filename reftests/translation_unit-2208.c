#pragma gnu

typedef int a;
void foo() {
int x;
if (sizeof(enum {a})) x = (_Atomic(a))1;
}

/*===
~ERROR
===*/
