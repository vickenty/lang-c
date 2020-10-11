// If struct field treated "a" as a type specifier instead of identifier, this would succeed.

#pragma gnu

typedef int a;
struct a { a a, b; };

/*===
TranslationUnit
===*/
