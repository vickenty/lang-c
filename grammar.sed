# Because rust_peg technically doesn't support type parameters, we need to fix them here.

s/struct ParseState < 'input >/struct ParseState<'input, T: Name>/g;
s/< 'input > (/<'input, T: Name>(/g;
s/ParseState < 'input >/ParseState<'input, T>/g;
s/impl < 'input >/impl<'input, T: Name>/g;
#s/ParseState,/ParseState<'_, impl Name>,/g;
s/(input : & str, state : & mut ParseState/<T: Name>(input : \& str, state : \& mut ParseState<T>/g;
