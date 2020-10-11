#pragma gnu

typedef int a, b;
int x;
void foo() {
if (sizeof(enum {a})) x = sizeof(enum{b});
else x = b;
switch (sizeof(enum {b})) x = b;
a x, y;
b z, w;
}

/*===
TranslationUnit
===*/
