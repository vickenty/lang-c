// If parameter list treated "a" as a type specifier instead of identifier, this would succeed.
#pragma gnu

typedef int a;
int foo(int a* b) {}

/*===
~ERROR
===*/
