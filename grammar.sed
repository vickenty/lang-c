s/struct ParseState < 'input >/struct ParseState<'input, T: Name>/g;
s/< 'input > (/<'input, T: Name>(/g;
s/ParseState < 'input >/ParseState<'input, T>/g;
s/impl < 'input >/impl<'input, T: Name>/g;
s/ParseState,/ParseState<'_, impl Name>,/g;