# Because rust_peg technically doesn't support type parameters, we need to fix them here.

s/struct ParseState < 'input >/struct ParseState<'input, T: Interner>/g;
s/< 'input > (/<'input, T: Interner>(/g;
s/ParseState < 'input >/ParseState<'input, T>/g;
s/impl < 'input >/impl<'input, T: Interner>/g;
s/(input : & str, state : & mut ParseState/<T: Interner>(input : \& str, state : \& mut ParseState<T>/g;
