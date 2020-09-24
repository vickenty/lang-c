use ast::*;
use env::Env;
use span::{Node, Span};

fn ident<T: From<Identifier>>(i: &str) -> T {
    Identifier {
        name: i.to_string(),
    }
    .into()
}

impl<T> From<T> for Node<T> {
    fn from(t: T) -> Node<T> {
        Node::new(t, Span::none())
    }
}

impl<T> From<T> for Box<Node<T>> {
    fn from(t: T) -> Box<Node<T>> {
        Box::new(t.into())
    }
}

impl<'a> From<&'a str> for Node<String> {
    fn from(t: &'a str) -> Node<String> {
        t.to_owned().into()
    }
}

macro_rules! mk_from_inner {
    ( $( $i:ident => $p:ident :: $v:ident ; )* ) => (
        $(
            impl From<$i> for $p {
                fn from(i: $i) -> $p {
                    $p::$v(i.into())
                }
            }

            impl From<$i> for Node<$p> {
                fn from(i: $i) -> Node<$p> {
                    $p::$v(i.into()).into()
                }
            }

            impl From<$i> for Box<Node<$p>> {
                fn from(i: $i) -> Box<Node<$p>> {
                    $p::$v(i.into()).into()
                }
            }
        )*
    );
}

mk_from_inner! {
    ArrayDeclarator => DerivedDeclarator::Array;
    AsmStatement => Statement::Asm;
    Attribute => Extension::Attribute;
    AvailabilityAttribute => Extension::AvailabilityAttribute;
    CallExpression => Expression::Call;
    CallExpression => Initializer::Expression;
    CallExpression => TypeOf::Expression;
    CastExpression => Expression::Cast;
    CompoundLiteral => Expression::CompoundLiteral;
    Constant => Expression::Constant;
    Constant => Initializer::Expression;
    Declaration => BlockItem::Declaration;
    Declaration => ExternalDeclaration::Declaration;
    Declarator => DeclaratorKind::Declarator;
    DoWhileStatement => Statement::DoWhile;
    EnumType => DeclarationSpecifier::TypeSpecifier;
    EnumType => TypeSpecifier::Enum;
    Expression => Initializer::Expression;
    FunctionDeclarator => DerivedDeclarator::Function;
    FunctionDefinition => ExternalDeclaration::FunctionDefinition;
    FunctionSpecifier => DeclarationSpecifier::Function;
    GnuExtendedAsmStatement => AsmStatement::GnuExtended;
    GnuExtendedAsmStatement => Statement::Asm;
    Identifier => DeclaratorKind::Identifier;
    Identifier => Expression::Identifier;
    IfStatement => Statement::If;
    Statement => BlockItem::Statement;
    Statement => Expression::Statement;
    StaticAssert => ExternalDeclaration::StaticAssert;
    StorageClassSpecifier => DeclarationSpecifier::StorageClass;
    StructField => StructDeclaration::Field;
    StructType => DeclarationSpecifier::TypeSpecifier;
    StructType => SpecifierQualifier::TypeSpecifier;
    StructType => TypeSpecifier::Struct;
    TS18661FloatType => DeclarationSpecifier::TypeSpecifier;
    TS18661FloatType => TypeSpecifier::TS18661Float;
    TypeQualifier => DeclarationSpecifier::TypeQualifier;
    TypeQualifier => PointerQualifier::TypeQualifier;
    TypeSpecifier => DeclarationSpecifier::TypeSpecifier;
    TypeSpecifier => SpecifierQualifier::TypeSpecifier;
}

mod expr {
    use ast::*;
    use span::Node;

    pub fn string<T: From<Expression>>(i: &str) -> T {
        Expression::StringLiteral(vec![i.to_string()].into()).into()
    }

    pub fn unop<T: From<Expression>>(op: UnaryOperator, e: Expression) -> T {
        Expression::UnaryOperator(
            UnaryOperatorExpression {
                operator: op.into(),
                operand: e.into(),
            }
            .into(),
        )
        .into()
    }

    pub fn binop<T: From<Expression>>(op: BinaryOperator, a: Expression, b: Expression) -> T {
        Expression::BinaryOperator(
            BinaryOperatorExpression {
                operator: op.into(),
                lhs: a.into(),
                rhs: b.into(),
            }
            .into(),
        )
        .into()
    }

    pub fn member<T: From<Expression>>(
        op: MemberOperator,
        e: Expression,
        i: Node<Identifier>,
    ) -> T {
        Expression::Member(
            MemberExpression {
                operator: op.into(),
                expression: Box::new(e.into()),
                identifier: i,
            }
            .into(),
        )
        .into()
    }
}

mod int {
    use ast::*;

    pub fn num<T: From<Constant>>(base: IntegerBase, number: &str, suffix: IntegerSuffix) -> T {
        Constant::Integer(Integer {
            base: base,
            number: number.to_string().into_boxed_str(),
            suffix: suffix,
        })
        .into()
    }

    pub const NONE: IntegerSuffix = IntegerSuffix {
        size: IntegerSize::Int,
        unsigned: false,
        imaginary: false,
    };

    pub const UL: IntegerSuffix = IntegerSuffix {
        size: IntegerSize::Long,
        unsigned: true,
        imaginary: false,
    };

    pub fn zero<T: From<Constant>>() -> T {
        num(IntegerBase::Decimal, "0", NONE.clone())
    }

    pub fn dec<T: From<Constant>>(n: &str) -> T {
        num(IntegerBase::Decimal, n, NONE.clone())
    }
}

mod float {
    use ast::*;

    pub fn num<T: From<Constant>>(base: FloatBase, number: &str, suffix: FloatSuffix) -> T {
        Constant::Float(Float {
            base: base,
            number: number.to_string().into_boxed_str(),
            suffix: suffix,
        })
        .into()
    }

    pub const NONE: FloatSuffix = FloatSuffix {
        format: FloatFormat::Double,
        imaginary: false,
    };

    pub fn dec<T: From<Constant>>(n: &str) -> T {
        num(FloatBase::Decimal, n, NONE.clone())
    }
}

fn cchar(i: &str) -> Constant {
    Constant::Character(i.to_string())
}

fn cstr<T: From<StringLiteral>>(i: &[&str]) -> T {
    i.into_iter()
        .map(|s| String::from(*s))
        .collect::<Vec<String>>()
        .into()
}

#[test]
fn test_integer() {
    use self::int::{num, NONE, UL};
    use ast::IntegerBase::*;
    use parser::constant;

    let env = &mut Env::new();

    assert_eq!(constant("0", env), Ok(num(Decimal, "0", NONE.clone())));
    assert_eq!(constant("1", env), Ok(num(Decimal, "1", NONE.clone())));
    assert_eq!(
        constant("1234567890", env),
        Ok(num(Decimal, "1234567890", NONE.clone()))
    );
    assert_eq!(
        constant("01234567", env),
        Ok(num(Octal, "1234567", NONE.clone()))
    );
    assert_eq!(
        constant("0x1234567890abdefABCDEF", env),
        Ok(num(Hexadecimal, "1234567890abdefABCDEF", NONE.clone()))
    );
    assert_eq!(
        constant("0b0001001000110100", env),
        Ok(num(Binary, "0001001000110100", NONE.clone()))
    );
    assert_eq!(constant("042lu", env), Ok(num(Octal, "42", UL.clone())));
    assert_eq!(constant("042ul", env), Ok(num(Octal, "42", UL.clone())));
    assert_eq!(constant("042uL", env), Ok(num(Octal, "42", UL.clone())));

    assert!(constant("1a", env).is_err());
    assert!(constant("08", env).is_err());
    assert!(constant("0xX", env).is_err());
    assert!(constant("1lul", env).is_err());
    assert!(constant("2lL", env).is_err());
    assert!(constant("0b2", env).is_err());
}

#[test]
fn test_floating() {
    use self::float::*;
    use ast::FloatBase::*;
    use parser::constant;

    let env = &mut Env::new();

    const F: FloatSuffix = FloatSuffix {
        format: FloatFormat::Float,
        imaginary: false,
    };

    const L: FloatSuffix = FloatSuffix {
        format: FloatFormat::LongDouble,
        imaginary: false,
    };

    assert_eq!(constant("2.", env), Ok(num(Decimal, "2.", NONE.clone())));
    assert_eq!(
        constant("2.e2", env),
        Ok(num(Decimal, "2.e2", NONE.clone()))
    );
    assert_eq!(constant(".2", env), Ok(num(Decimal, ".2", NONE.clone())));
    assert_eq!(
        constant(".2e2", env),
        Ok(num(Decimal, ".2e2", NONE.clone()))
    );
    assert_eq!(constant("2.0", env), Ok(num(Decimal, "2.0", NONE.clone())));
    assert_eq!(constant("2.0f", env), Ok(num(Decimal, "2.0", F.clone())));

    assert_eq!(
        constant("24.01e100", env),
        Ok(num(Decimal, "24.01e100", NONE.clone()))
    );
    assert_eq!(
        constant("24.01e+100", env),
        Ok(num(Decimal, "24.01e+100", NONE.clone()))
    );
    assert_eq!(
        constant("24.01e-100", env),
        Ok(num(Decimal, "24.01e-100", NONE.clone()))
    );
    assert_eq!(
        constant("24.01e100f", env),
        Ok(num(Decimal, "24.01e100", F.clone()))
    );

    assert_eq!(
        constant("0x2Ap19L", env),
        Ok(num(Hexadecimal, "2Ap19", L.clone()))
    );
    assert_eq!(
        constant("0x2A.p19L", env),
        Ok(num(Hexadecimal, "2A.p19", L.clone()))
    );
    assert_eq!(
        constant("0x.DEp19L", env),
        Ok(num(Hexadecimal, ".DEp19", L.clone()))
    );
    assert_eq!(
        constant("0x2A.DEp19L", env),
        Ok(num(Hexadecimal, "2A.DEp19", L.clone()))
    );
}

#[test]
fn ts18661_literal() {
    use self::float::*;
    use ast::FloatBase::*;
    use parser::constant;

    let env = &mut Env::new();

    const F16: FloatSuffix = FloatSuffix {
        format: FloatFormat::TS18661Format(TS18661FloatType {
            format: TS18661FloatFormat::BinaryInterchange,
            width: 16,
        }),
        imaginary: false,
    };

    const F64: FloatSuffix = FloatSuffix {
        format: FloatFormat::TS18661Format(TS18661FloatType {
            format: TS18661FloatFormat::BinaryInterchange,
            width: 64,
        }),
        imaginary: false,
    };

    assert_eq!(
        constant("1.0f64", env),
        Ok(num(Decimal, "1.0", F64.clone()))
    );
    assert_eq!(
        constant("0xAp1f16", env),
        Ok(num(Hexadecimal, "Ap1", F16.clone()))
    );
}

#[test]
fn test_character() {
    use parser::constant;

    let env = &mut Env::new();

    assert_eq!(constant("'a'", env), Ok(cchar("'a'")));
    assert_eq!(constant(r"'\n'", env), Ok(cchar(r"'\n'")));
    assert_eq!(constant(r"'\\'", env), Ok(cchar(r"'\\'")));
    assert_eq!(constant(r"'\''", env), Ok(cchar(r"'\''")));
    assert_eq!(constant(r"'\1'", env), Ok(cchar(r"'\1'")));
    assert_eq!(constant(r"'\02'", env), Ok(cchar(r"'\02'")));
    assert_eq!(constant(r"'\027'", env), Ok(cchar(r"'\027'")));
    assert_eq!(constant(r"'\xde'", env), Ok(cchar(r"'\xde'")));
}

#[test]
fn test_string() {
    use self::expr::*;
    use parser::expression;

    let env = &mut Env::new();

    assert_eq!(expression(r#""foo""#, env), Ok(string(r#""foo""#)));
    assert_eq!(expression(r#""foo\n""#, env), Ok(string(r#""foo\n""#)));
    assert_eq!(expression(r#""\'\"""#, env), Ok(string(r#""\'\"""#)));
    assert_eq!(expression(r#""\xaf""#, env), Ok(string(r#""\xaf""#)));
}

#[test]
fn test_postfix() {
    use self::expr::*;
    use ast::BinaryOperator::Index;
    use ast::MemberOperator::{Direct, Indirect};
    use ast::UnaryOperator::PostIncrement;
    use parser::expression;

    let env = &mut Env::new();

    assert_eq!(
        expression("a  ++", env),
        Ok(unop(PostIncrement, ident("a")))
    );
    assert_eq!(
        expression("a.b->c[ d[ e ] ] ++", env),
        Ok(unop(
            PostIncrement,
            binop(
                Index,
                member(Indirect, member(Direct, ident("a"), ident("b")), ident("c")),
                binop(Index, ident("d"), ident("e")),
            ),
        ))
    );
}

#[test]
fn test_multiplicative() {
    use self::expr::*;
    use ast::BinaryOperator::{Divide, Multiply};
    use ast::UnaryOperator::{PostDecrement, PreIncrement};
    use parser::expression;

    let env = &mut Env::new();

    assert_eq!(
        expression("a-- * ++b / c", env),
        Ok(binop(
            Divide,
            binop(
                Multiply,
                unop(PostDecrement, ident("a")),
                unop(PreIncrement, ident("b")),
            ),
            ident("c"),
        ))
    );
}

#[test]
fn test_logical_and() {
    use self::expr::*;
    use ast::BinaryOperator::LogicalAnd;
    use parser::expression;
    let env = &mut Env::new();

    assert_eq!(
        expression("a && b", env),
        Ok(binop(LogicalAnd, ident("a"), ident("b")))
    );
}

#[test]
fn test_chained_and() {
    use self::expr::*;
    use ast::BinaryOperator::LogicalAnd;
    use parser::expression;
    let env = &mut Env::new();

    assert_eq!(
        expression("a && b && c", env),
        Ok(binop(
            LogicalAnd,
            binop(LogicalAnd, ident("a"), ident("b")),
            ident("c"),
        ))
    );
}

#[test]
fn test_chained_or() {
    use self::expr::*;
    use ast::BinaryOperator::LogicalOr;
    use parser::expression;
    let env = &mut Env::new();
    assert_eq!(
        expression("a || b || c", env),
        Ok(binop(
            LogicalOr,
            binop(LogicalOr, ident("a"), ident("b")),
            ident("c"),
        ))
    );
}

#[test]
fn test_chained_shl() {
    use self::expr::*;
    use ast::BinaryOperator::ShiftLeft;
    use parser::expression;
    let env = &mut Env::new();
    assert_eq!(
        expression("a << b << c", env),
        Ok(binop(
            ShiftLeft,
            binop(ShiftLeft, ident("a"), ident("b")),
            ident("c"),
        ))
    );
}

#[test]
fn test_chained_shr() {
    use self::expr::*;
    use ast::BinaryOperator::ShiftRight;
    use parser::expression;
    let env = &mut Env::new();
    assert_eq!(
        expression("a >> b >> c", env),
        Ok(binop(
            ShiftRight,
            binop(ShiftRight, ident("a"), ident("b")),
            ident("c"),
        ))
    );
}

#[test]
fn test_comma() {
    use ast::Expression::Comma;
    use parser::expression;

    let env = &mut Env::new();

    assert_eq!(expression("a", env), Ok(ident("a")));
    assert_eq!(
        expression("a, a, a,a\n,a", env),
        Ok(Comma(Box::new(vec![ident("a"); 5].into())).into())
    );
}

#[test]
fn test_cast() {
    use ast::TypeName;
    use ast::TypeSpecifier::Int;
    use env::Env;
    use parser::expression;

    let env = &mut Env::new();

    assert_eq!(
        expression("(int) 1", env),
        Ok(CastExpression {
            type_name: TypeName {
                specifiers: vec![Int.into()],
                declarator: None,
            }
            .into(),
            expression: int::dec("1"),
        }
        .into())
    );

    assert!(expression("(foo) 1", env).is_err());
}

#[test]
fn test_declaration1() {
    use ast::ArraySize::{StaticExpression, VariableUnknown};
    use ast::DerivedDeclarator::Pointer;
    use ast::StorageClassSpecifier::Typedef;
    use ast::TypeQualifier::Const;
    use ast::TypeSpecifier::Int;
    use parser::declaration;

    let env = &mut Env::new();

    assert_eq!(
        declaration("int typedef * foo, baz[static 10][const *];", env),
        Ok(Declaration {
            specifiers: vec![Int.into(), Typedef.into()],
            declarators: vec![
                InitDeclarator {
                    declarator: Declarator {
                        kind: ident("foo"),
                        derived: vec![Pointer(vec![]).into()],
                        extensions: vec![],
                    }
                    .into(),
                    initializer: None,
                }
                .into(),
                InitDeclarator {
                    declarator: Declarator {
                        kind: ident("baz"),
                        derived: vec![
                            ArrayDeclarator {
                                qualifiers: vec![],
                                size: StaticExpression(int::dec("10")),
                            }
                            .into(),
                            ArrayDeclarator {
                                qualifiers: vec![Const.into()],
                                size: VariableUnknown,
                            }
                            .into(),
                        ],
                        extensions: vec![],
                    }
                    .into(),
                    initializer: None,
                }
                .into(),
            ],
        }
        .into())
    );

    assert!(env.is_typename("foo"));
    assert!(env.is_typename("baz"));
}

#[test]
fn test_declaration2() {
    use ast::DerivedDeclarator::Pointer;
    use ast::Enumerator;
    use ast::StorageClassSpecifier::Typedef;
    use ast::TypeQualifier::Const;
    use parser::declaration;

    let env = &mut Env::new();

    assert_eq!(
        declaration("typedef enum { FOO, BAR = 1 } * const foobar;", env),
        Ok(Declaration {
            specifiers: vec![
                Typedef.into(),
                EnumType {
                    identifier: None,
                    enumerators: vec![
                        Enumerator {
                            identifier: ident("FOO"),
                            expression: None,
                        }
                        .into(),
                        Enumerator {
                            identifier: ident("BAR"),
                            expression: Some(int::dec("1")),
                        }
                        .into(),
                    ],
                }
                .into(),
            ],
            declarators: vec![InitDeclarator {
                declarator: Declarator {
                    kind: ident("foobar"),
                    derived: vec![Pointer(vec![Const.into()]).into()],
                    extensions: vec![],
                }
                .into(),
                initializer: None,
            }
            .into()],
        }
        .into())
    );

    assert!(env.is_typename("foobar"));
}

#[test]
fn test_declaration3() {
    use ast::TypeSpecifier::{Float, Int};
    use parser::declaration;

    let env = &mut Env::new();

    assert_eq!(
        declaration("struct { int a, b; float c; } S;", env).unwrap(),
        Declaration {
            specifiers: vec![StructType {
                kind: StructKind::Struct.into(),
                identifier: None,
                declarations: Some(vec![
                    StructField {
                        specifiers: vec![Int.into()],
                        declarators: vec![
                            StructDeclarator {
                                declarator: Some(
                                    Declarator {
                                        kind: ident("a"),
                                        derived: vec![],
                                        extensions: vec![],
                                    }
                                    .into(),
                                ),
                                bit_width: None,
                            }
                            .into(),
                            StructDeclarator {
                                declarator: Some(
                                    Declarator {
                                        kind: ident("b"),
                                        derived: vec![],
                                        extensions: vec![],
                                    }
                                    .into(),
                                ),
                                bit_width: None,
                            }
                            .into(),
                        ],
                    }
                    .into(),
                    StructField {
                        specifiers: vec![Float.into()],
                        declarators: vec![StructDeclarator {
                            declarator: Some(
                                Declarator {
                                    kind: ident("c"),
                                    derived: vec![],
                                    extensions: vec![],
                                }
                                .into(),
                            ),
                            bit_width: None,
                        }
                        .into()],
                    }
                    .into(),
                ]),
            }
            .into()],
            declarators: vec![InitDeclarator {
                declarator: Declarator {
                    kind: ident("S"),
                    derived: vec![],
                    extensions: vec![],
                }
                .into(),
                initializer: None,
            }
            .into()],
        }
        .into()
    );
}

#[test]
fn test_declaration4() {
    use ast::TypeQualifier::Restrict;
    use ast::TypeSpecifier::Int;
    use parser::declaration;

    assert_eq!(
        declaration("int __restrict__;", &mut Env::with_core()),
        Ok(Declaration {
            specifiers: vec![Int.into()],
            declarators: vec![InitDeclarator {
                declarator: Declarator {
                    kind: ident("__restrict__"),
                    derived: vec![],
                    extensions: vec![],
                }
                .into(),
                initializer: None,
            }
            .into()],
        }
        .into())
    );

    assert_eq!(
        declaration("int __restrict__;", &mut Env::with_gnu()),
        Ok(Declaration {
            specifiers: vec![Int.into(), Restrict.into()],
            declarators: vec![],
        }
        .into())
    );
}

#[test]
fn test_declaration5() {
    use self::int::dec;
    use ast::ArraySize::VariableExpression;
    use ast::DeclaratorKind::Abstract;
    use ast::DerivedDeclarator::Pointer;
    use ast::TypeQualifier::Const;
    use ast::TypeSpecifier::{Char, Int, TypedefName};
    use parser::declaration;

    let env = &mut Env::new();

    env.add_typename("FILE");
    env.add_typename("size_t");

    assert_eq!(
        declaration(
            "char *fparseln(FILE *, size_t *, size_t *, const char[3], int);",
            env
        ),
        Ok(Declaration {
            specifiers: vec![Char.into()],
            declarators: vec![InitDeclarator {
                declarator: Declarator {
                    kind: ident("fparseln"),
                    derived: vec![
                        Pointer(vec![]).into(),
                        FunctionDeclarator {
                            parameters: vec![
                                ParameterDeclaration {
                                    specifiers: vec![TypedefName(ident("FILE")).into()],
                                    declarator: Some(
                                        Declarator {
                                            kind: Abstract.into(),
                                            derived: vec![Pointer(vec![]).into()],
                                            extensions: vec![],
                                        }
                                        .into(),
                                    ),
                                    extensions: vec![],
                                }
                                .into(),
                                ParameterDeclaration {
                                    specifiers: vec![TypedefName(ident("size_t")).into()],
                                    declarator: Some(
                                        Declarator {
                                            kind: Abstract.into(),
                                            derived: vec![Pointer(vec![]).into()],
                                            extensions: vec![],
                                        }
                                        .into(),
                                    ),
                                    extensions: vec![],
                                }
                                .into(),
                                ParameterDeclaration {
                                    specifiers: vec![TypedefName(ident("size_t")).into()],
                                    declarator: Some(
                                        Declarator {
                                            kind: Abstract.into(),
                                            derived: vec![Pointer(vec![]).into()],
                                            extensions: vec![],
                                        }
                                        .into(),
                                    ),
                                    extensions: vec![],
                                }
                                .into(),
                                ParameterDeclaration {
                                    specifiers: vec![Const.into(), Char.into()],
                                    declarator: Some(
                                        Declarator {
                                            kind: Abstract.into(),
                                            derived: vec![ArrayDeclarator {
                                                qualifiers: vec![],
                                                size: VariableExpression(dec("3")),
                                            }
                                            .into()],
                                            extensions: vec![],
                                        }
                                        .into(),
                                    ),
                                    extensions: vec![],
                                }
                                .into(),
                                ParameterDeclaration {
                                    specifiers: vec![Int.into()],
                                    declarator: None,
                                    extensions: vec![],
                                }
                                .into(),
                            ],
                            ellipsis: Ellipsis::None,
                        }
                        .into(),
                    ],
                    extensions: vec![],
                }
                .into(),
                initializer: None,
            }
            .into()],
        }
        .into())
    );
}

#[test]
fn test_attribute() {
    use ast::DerivedDeclarator::Pointer;
    use ast::Extension::AsmLabel;
    use ast::StorageClassSpecifier::Extern;
    use ast::TypeSpecifier::{Char, Int, TypedefName};
    use parser::declaration;

    let env = &mut Env::new();
    env.add_typename("size_t");

    assert_eq!(
        declaration(
            concat!(
                "extern int strerror_r (int __errnum, char *__buf, size_t __buflen)\n",
                "__asm__  (\"\" \"__xpg_strerror_r\") __attribute__ ((__nothrow__ , __leaf__))\n",
                "__attribute__ ((__nonnull__ (2)));",
            ),
            env,
        ),
        Ok(Declaration {
            specifiers: vec![Extern.into(), Int.into()],
            declarators: vec![InitDeclarator {
                declarator: Declarator {
                    kind: ident("strerror_r"),
                    derived: vec![FunctionDeclarator {
                        parameters: vec![
                            ParameterDeclaration {
                                specifiers: vec![Int.into()],
                                declarator: Some(
                                    Declarator {
                                        kind: ident("__errnum"),
                                        derived: vec![],
                                        extensions: vec![],
                                    }
                                    .into(),
                                ),
                                extensions: vec![],
                            }
                            .into(),
                            ParameterDeclaration {
                                specifiers: vec![Char.into()],
                                declarator: Some(
                                    Declarator {
                                        kind: ident("__buf"),
                                        derived: vec![Pointer(vec![]).into()],
                                        extensions: vec![],
                                    }
                                    .into(),
                                ),
                                extensions: vec![],
                            }
                            .into(),
                            ParameterDeclaration {
                                specifiers: vec![TypedefName(ident("size_t")).into()],
                                declarator: Some(
                                    Declarator {
                                        kind: ident("__buflen"),
                                        derived: vec![],
                                        extensions: vec![],
                                    }
                                    .into(),
                                ),
                                extensions: vec![],
                            }
                            .into(),
                        ],
                        ellipsis: Ellipsis::None,
                    }
                    .into()],
                    extensions: vec![
                        AsmLabel(cstr(&[r#""""#, r#""__xpg_strerror_r""#])).into(),
                        Attribute {
                            name: "__nothrow__".into(),
                            arguments: vec![],
                        }
                        .into(),
                        Attribute {
                            name: "__leaf__".into(),
                            arguments: vec![],
                        }
                        .into(),
                        Attribute {
                            name: "__nonnull__".into(),
                            arguments: vec![int::dec("2")],
                        }
                        .into(),
                    ],
                }
                .into(),
                initializer: None,
            }
            .into()],
        }
        .into())
    );
}

#[test]
fn test_attribute2() {
    use self::int::dec;
    use ast::DeclarationSpecifier::Extension;
    use ast::DeclaratorKind::Abstract;
    use ast::DerivedDeclarator::Pointer;
    use ast::TypeQualifier::Const;
    use ast::TypeSpecifier::{Char, Void};
    use parser::declaration;

    assert_eq!(
        declaration(
            r#"__attribute__((noreturn)) void d0 (void),
                __attribute__((format(printf, 1, 2))) d1 (const char *, ...),
                 d2 (void);"#,
            &mut Env::new()
        ),
        Ok(Declaration {
            specifiers: vec![
                Extension(vec![Attribute {
                    name: "noreturn".into(),
                    arguments: vec![],
                }
                .into()])
                .into(),
                Void.into(),
            ],
            declarators: vec![
                InitDeclarator {
                    declarator: Declarator {
                        kind: ident("d0"),
                        derived: vec![FunctionDeclarator {
                            parameters: vec![ParameterDeclaration {
                                specifiers: vec![Void.into()],
                                declarator: None,
                                extensions: vec![],
                            }
                            .into()],
                            ellipsis: Ellipsis::None,
                        }
                        .into()],
                        extensions: vec![],
                    }
                    .into(),
                    initializer: None,
                }
                .into(),
                InitDeclarator {
                    declarator: Declarator {
                        kind: ident("d1"),
                        derived: vec![FunctionDeclarator {
                            parameters: vec![ParameterDeclaration {
                                specifiers: vec![Const.into(), Char.into()],
                                declarator: Some(
                                    Declarator {
                                        kind: Abstract.into(),
                                        derived: vec![Pointer(vec![]).into()],
                                        extensions: vec![],
                                    }
                                    .into(),
                                ),
                                extensions: vec![],
                            }
                            .into()],
                            ellipsis: Ellipsis::Some,
                        }
                        .into()],
                        extensions: vec![Attribute {
                            name: "format".into(),
                            arguments: vec![ident("printf"), dec("1"), dec("2")],
                        }
                        .into()],
                    }
                    .into(),
                    initializer: None,
                }
                .into(),
                InitDeclarator {
                    declarator: Declarator {
                        kind: ident("d2"),
                        derived: vec![FunctionDeclarator {
                            parameters: vec![ParameterDeclaration {
                                specifiers: vec![Void.into()],
                                declarator: None,
                                extensions: vec![],
                            }
                            .into()],
                            ellipsis: Ellipsis::None,
                        }
                        .into()],
                        extensions: vec![],
                    }
                    .into(),
                    initializer: None,
                }
                .into(),
            ],
        }
        .into())
    );
}

#[test]
fn test_attribute3() {
    use ast::DeclarationSpecifier::Extension;
    use ast::DerivedDeclarator::Pointer;
    use ast::FunctionSpecifier::Inline;
    use ast::Statement::Compound;
    use ast::StorageClassSpecifier::Extern;
    use ast::TypeQualifier::{Const, Restrict};
    use ast::TypeSpecifier::Char;
    use parser::translation_unit;

    assert_eq!(
        translation_unit(
            concat!(
                "extern __inline __attribute__ ((__always_inline__)) __attribute__ \n",
                "((__artificial__)) __attribute__ ((__warn_unused_result__)) char *\n",
                "__attribute__ ((__nothrow__ , __leaf__)) realpath (const char *__restrict\n",
                "__name, char *__restrict __resolved) {}"
            ),
            &mut Env::new()
        ),
        Ok(TranslationUnit(vec![
            ExternalDeclaration::FunctionDefinition(
                FunctionDefinition {
                    specifiers: vec![
                        Extern.into(),
                        Inline.into(),
                        Extension(vec![Attribute {
                            name: "__always_inline__".into(),
                            arguments: vec![],
                        }
                        .into()])
                        .into(),
                        Extension(vec![Attribute {
                            name: "__artificial__".into(),
                            arguments: vec![],
                        }
                        .into()])
                        .into(),
                        Extension(vec![Attribute {
                            name: "__warn_unused_result__".into(),
                            arguments: vec![],
                        }
                        .into()])
                        .into(),
                        Char.into(),
                    ],
                    declarator: Declarator {
                        kind: ident("realpath"),
                        derived: vec![
                            Pointer(vec![PointerQualifier::Extension(vec![
                                Attribute {
                                    name: "__nothrow__".into(),
                                    arguments: vec![],
                                }
                                .into(),
                                Attribute {
                                    name: "__leaf__".into(),
                                    arguments: vec![],
                                }
                                .into(),
                            ])
                            .into()])
                            .into(),
                            FunctionDeclarator {
                                parameters: vec![
                                    ParameterDeclaration {
                                        specifiers: vec![Const.into(), Char.into()],
                                        declarator:
                                            Some(
                                                Declarator {
                                                    kind: ident("__name"),
                                                    derived: vec![
                                                        Pointer(vec![Restrict.into()]).into()
                                                    ],
                                                    extensions: vec![],
                                                }
                                                .into(),
                                            ),
                                        extensions: vec![],
                                    }
                                    .into(),
                                    ParameterDeclaration {
                                        specifiers: vec![Char.into()],
                                        declarator:
                                            Some(
                                                Declarator {
                                                    kind: ident("__resolved"),
                                                    derived: vec![
                                                        Pointer(vec![Restrict.into()]).into()
                                                    ],
                                                    extensions: vec![],
                                                }
                                                .into(),
                                            ),
                                        extensions: vec![],
                                    }
                                    .into(),
                                ],
                                ellipsis: Ellipsis::None,
                            }
                            .into(),
                        ],
                        extensions: vec![],
                    }
                    .into(),
                    declarations: vec![],
                    statement: Compound(vec![]).into(),
                }
                .into(),
            )
            .into()
        ]))
        .into()
    );
}

#[test]
fn test_alignof() {
    use ast::Expression::AlignOf;
    use ast::TypeSpecifier::Long;
    use parser::expression;

    assert_eq!(
        expression("_Alignof(long long)", &mut Env::new()),
        Ok(AlignOf(
            TypeName {
                specifiers: vec![Long.into(), Long.into()],
                declarator: None,
            }
            .into(),
        )
        .into())
    );

    assert_eq!(
        expression("__alignof(long long)", &mut Env::new()),
        Ok(AlignOf(
            TypeName {
                specifiers: vec![Long.into(), Long.into()],
                declarator: None,
            }
            .into(),
        )
        .into())
    );

    assert_eq!(
        expression("__alignof__(long long)", &mut Env::new()),
        Ok(AlignOf(
            TypeName {
                specifiers: vec![Long.into(), Long.into()],
                declarator: None,
            }
            .into(),
        )
        .into())
    );
}

#[test]
fn test_stmt_expr() {
    use ast::Statement::{Compound, Expression};
    use ast::TypeSpecifier::Int;
    use parser::expression;

    assert_eq!(
        expression("({ int p = 0; p; })", &mut Env::new()),
        Ok(Compound(vec![
            Declaration {
                specifiers: vec![Int.into()],
                declarators: vec![InitDeclarator {
                    declarator: Declarator {
                        kind: ident("p"),
                        derived: vec![],
                        extensions: vec![],
                    }
                    .into(),
                    initializer: Some(int::zero()),
                }
                .into()],
            }
            .into(),
            Expression(Some(ident("p"))).into(),
        ])
        .into())
    );
}

#[test]
fn test_expr_cast() {
    use ast::TypeName;
    use ast::TypeSpecifier::TypedefName;
    use parser::expression;

    let env = &mut Env::new();
    env.add_typename("U64");

    assert_eq!(
        expression("(U64)foo", env),
        Ok(CastExpression {
            type_name: TypeName {
                specifiers: vec![TypedefName(ident("U64")).into()],
                declarator: None,
            }
            .into(),
            expression: ident("foo"),
        }
        .into())
    );
}

#[test]
fn test_directives() {
    use parser::translation_unit;

    assert_eq!(
        translation_unit(
            r#"# 1 "<stdin>"
# 1 "<built-in>"
# 1 "<command-line>"
# 31 "<command-line>"
# 1 "/usr/include/stdc-predef.h" 1 3 4
# 32 "<command-line>" 2
# 1 "<stdin>"
"#,
            &mut Env::new()
        ),
        Ok(TranslationUnit(vec![]))
    );
}

#[test]
fn test_gnu_asm() {
    use parser::statement;

    assert_eq!(
        statement(
            r#"__asm ("pmovmskb %1, %0" : "=r" (__m) : "x" (__x));"#,
            &mut Env::new()
        ),
        Ok(GnuExtendedAsmStatement {
            qualifier: None,
            template: cstr(&[r#""pmovmskb %1, %0""#]),
            outputs: vec![GnuAsmOperand {
                symbolic_name: None,
                constraints: cstr(&[r#""=r""#]),
                variable_name: ident("__m"),
            }
            .into()],
            inputs: vec![GnuAsmOperand {
                symbolic_name: None,
                constraints: cstr(&[r#""x""#]),
                variable_name: ident("__x"),
            }
            .into()],
            clobbers: vec![],
        }
        .into())
    );
}

#[test]
fn test_union() {
    use self::int::dec;
    use ast::ArraySize::VariableExpression;
    use ast::Designator::Member;
    use ast::Initializer::{Expression, List};
    use ast::TypeSpecifier::{Double, Int, Long};
    use parser::declaration;

    assert_eq!(
        declaration(
            "union { long double __l; int __i[3]; } __u = { __l: __x };",
            &mut Env::new()
        ),
        Ok(Declaration {
            specifiers: vec![StructType {
                kind: StructKind::Union.into(),
                identifier: None,
                declarations: Some(vec![
                    StructField {
                        specifiers: vec![Long.into(), Double.into()],
                        declarators: vec![StructDeclarator {
                            declarator: Some(
                                Declarator {
                                    kind: ident("__l"),
                                    derived: vec![],
                                    extensions: vec![],
                                }
                                .into(),
                            ),
                            bit_width: None,
                        }
                        .into()],
                    }
                    .into(),
                    StructField {
                        specifiers: vec![Int.into()],
                        declarators: vec![StructDeclarator {
                            declarator: Some(
                                Declarator {
                                    kind: ident("__i"),
                                    derived: vec![ArrayDeclarator {
                                        qualifiers: vec![],
                                        size: VariableExpression(dec("3")),
                                    }
                                    .into()],
                                    extensions: vec![],
                                }
                                .into(),
                            ),
                            bit_width: None,
                        }
                        .into()],
                    }
                    .into(),
                ]),
            }
            .into()],
            declarators: vec![InitDeclarator {
                declarator: Declarator {
                    kind: ident("__u"),
                    derived: vec![],
                    extensions: vec![],
                }
                .into(),
                initializer: Some(
                    List(vec![InitializerListItem {
                        designation: vec![Member(ident("__l")).into()],
                        initializer: Expression(ident("__x")).into(),
                    }
                    .into()])
                    .into(),
                ),
            }
            .into()],
        }
        .into())
    );
}

#[test]
fn test_offsetof() {
    use self::int::dec;
    use ast::ArraySize::VariableExpression;
    use ast::Expression::OffsetOf;
    use ast::OffsetMember::IndirectMember;
    use ast::TypeSpecifier::Int;
    use parser::expression;

    assert_eq!(
        expression(
            "__builtin_offsetof(struct { struct { int b; } a[2]; }, a->b)",
            &mut Env::new()
        ),
        Ok(OffsetOf(
            OffsetOfExpression {
                type_name: TypeName {
                    specifiers: vec![StructType {
                        kind: StructKind::Struct.into(),
                        identifier: None,
                        declarations: Some(vec![StructField {
                            specifiers: vec![StructType {
                                kind: StructKind::Struct.into(),
                                identifier: None,
                                declarations: Some(vec![StructField {
                                    specifiers: vec![Int.into()],
                                    declarators: vec![StructDeclarator {
                                        declarator: Some(
                                            Declarator {
                                                kind: ident("b"),
                                                derived: vec![],
                                                extensions: vec![],
                                            }
                                            .into(),
                                        ),
                                        bit_width: None,
                                    }
                                    .into()],
                                }
                                .into()]),
                            }
                            .into()],
                            declarators: vec![StructDeclarator {
                                declarator: Some(
                                    Declarator {
                                        kind: ident("a"),
                                        derived: vec![ArrayDeclarator {
                                            qualifiers: vec![],
                                            size: VariableExpression(dec("2")),
                                        }
                                        .into()],
                                        extensions: vec![],
                                    }
                                    .into(),
                                ),
                                bit_width: None,
                            }
                            .into()],
                        }
                        .into()]),
                    }
                    .into()],
                    declarator: None,
                }
                .into(),
                designator: OffsetDesignator {
                    base: ident("a"),
                    members: vec![IndirectMember(ident("b")).into()],
                }
                .into(),
            }
            .into()
        )
        .into())
    );
}

#[test]
fn test_call() {
    use parser::expression;

    assert_eq!(
        expression("foo(bar, baz)", &mut Env::new()),
        Ok(CallExpression {
            callee: ident("foo"),
            arguments: vec![ident("bar"), ident("baz")],
        }
        .into())
    );
}

#[test]
fn test_typeof() {
    use ast::TypeSpecifier::TypeOf;
    use parser::declaration;

    assert_eq!(
        declaration(
            "__typeof__(foo(bar, baz)) ook = foo(bar, baz);",
            &mut Env::new()
        ),
        Ok(Declaration {
            specifiers: vec![TypeOf(
                CallExpression {
                    callee: ident("foo"),
                    arguments: vec![ident("bar"), ident("baz")],
                }
                .into(),
            )
            .into()],
            declarators: vec![InitDeclarator {
                declarator: Declarator {
                    kind: ident("ook"),
                    derived: vec![],
                    extensions: vec![],
                }
                .into(),
                initializer: Some(
                    CallExpression {
                        callee: ident("foo"),
                        arguments: vec![ident("bar"), ident("baz")],
                    }
                    .into(),
                ),
            }
            .into()],
        }
        .into())
    );
}

#[test]
fn test_if() {
    use ast::Statement::Compound;
    use parser::statement;

    assert_eq!(
        statement("if (x) do {} while(y); else z();", &mut Env::new()),
        Ok(IfStatement {
            condition: ident("x"),
            then_statement: DoWhileStatement {
                statement: Compound(vec![]).into(),
                expression: ident("y"),
            }
            .into(),
            else_statement: Some(
                Statement::Expression(Some(
                    CallExpression {
                        callee: ident("z"),
                        arguments: vec![],
                    }
                    .into()
                ))
                .into()
            ),
        }
        .into())
    );
}

// Check that a typedef that can be mistaken for a K&R-style argument declaration is correctly
// parsed as an external declaration. What went wrong: until we encounter bar, the thing looks like
// a function definition, where the name is followed by a two declarations K&R-style, similar to:
//
// ```
// int foo(i)
// int i; // <-- __attribute__ and typedef occupy this slot, since both are valid declarations.
// { }
// ```:
#[test]
fn test_attribute4() {
    use ast::Statement::Compound;
    use ast::StorageClassSpecifier::Typedef;
    use ast::TypeSpecifier::Int;
    use parser::translation_unit;

    let env = &mut Env::new();

    assert_eq!(
        translation_unit(
            r#"
                int foo (int) __attribute__ ((__nothrow__));
                typedef int named;
                int bar (int f) { }
            "#,
            env
        ),
        Ok(TranslationUnit(vec![
            Declaration {
                specifiers: vec![Int.into()],
                declarators: vec![InitDeclarator {
                    declarator: Declarator {
                        kind: ident("foo"),
                        derived: vec![FunctionDeclarator {
                            parameters: vec![ParameterDeclaration {
                                specifiers: vec![Int.into()],
                                declarator: None,
                                extensions: vec![],
                            }
                            .into()],
                            ellipsis: Ellipsis::None,
                        }
                        .into()],
                        extensions: vec![Attribute {
                            name: "__nothrow__".into(),
                            arguments: vec![],
                        }
                        .into()],
                    }
                    .into(),
                    initializer: None,
                }
                .into()],
            }
            .into(),
            Declaration {
                specifiers: vec![Typedef.into(), Int.into()],
                declarators: vec![InitDeclarator {
                    declarator: Declarator {
                        kind: ident("named"),
                        derived: vec![],
                        extensions: vec![],
                    }
                    .into(),
                    initializer: None,
                }
                .into()],
            }
            .into(),
            FunctionDefinition {
                specifiers: vec![Int.into()],
                declarator: Declarator {
                    kind: ident("bar"),
                    derived: vec![FunctionDeclarator {
                        parameters: vec![ParameterDeclaration {
                            specifiers: vec![Int.into()],
                            declarator: Some(
                                Declarator {
                                    kind: ident("f"),
                                    derived: vec![],
                                    extensions: vec![],
                                }
                                .into(),
                            ),
                            extensions: vec![],
                        }
                        .into()],
                        ellipsis: Ellipsis::None,
                    }
                    .into()],
                    extensions: vec![],
                }
                .into(),
                declarations: vec![],
                statement: Compound(vec![]).into(),
            }
            .into(),
        ]))
    );
}

#[test]
fn test_attribute5() {
    use ast::Statement::Compound;
    use ast::TypeSpecifier::Int;
    use parser::translation_unit;

    assert_eq!(
        translation_unit(
            "int foo(int a __attribute__((unused)), int b __attribute__((unused))) {}",
            &mut Env::new(),
        ),
        Ok(TranslationUnit(vec![FunctionDefinition {
            specifiers: vec![Int.into()],
            declarator: Declarator {
                kind: ident("foo"),
                derived: vec![FunctionDeclarator {
                    parameters: vec![
                        ParameterDeclaration {
                            specifiers: vec![Int.into()],
                            declarator: Some(
                                Declarator {
                                    kind: ident("a"),
                                    derived: vec![],
                                    extensions: vec![],
                                }
                                .into(),
                            ),
                            extensions: vec![Attribute {
                                name: "unused".into(),
                                arguments: vec![],
                            }
                            .into()],
                        }
                        .into(),
                        ParameterDeclaration {
                            specifiers: vec![Int.into()],
                            declarator: Some(
                                Declarator {
                                    kind: ident("b"),
                                    derived: vec![],
                                    extensions: vec![],
                                }
                                .into(),
                            ),
                            extensions: vec![Attribute {
                                name: "unused".into(),
                                arguments: vec![],
                            }
                            .into()],
                        }
                        .into(),
                    ],
                    ellipsis: Ellipsis::None,
                }
                .into()],
                extensions: vec![],
            }
            .into(),
            declarations: vec![],
            statement: Compound(vec![]).into(),
        }
        .into()]))
    );
}

#[test]
fn test_declaration6() {
    use ast::Expression::AlignOf;
    use ast::StorageClassSpecifier::Typedef;
    use ast::TypeSpecifier::{Double, Long};
    use parser::declaration;

    assert_eq!(
        declaration(
            r"typedef struct {
              long long __max_align_ll __attribute__((__aligned__(__alignof__(long long))));
              long double __max_align_ld __attribute__((__aligned__(__alignof__(long double))));
            } max_align_t;",
            &mut Env::new()
        ),
        Ok(Declaration {
            specifiers: vec![
                Typedef.into(),
                StructType {
                    kind: StructKind::Struct.into(),
                    identifier: None,
                    declarations: Some(vec![
                        StructField {
                            specifiers: vec![Long.into(), Long.into()],
                            declarators: vec![StructDeclarator {
                                declarator: Some(
                                    Declarator {
                                        kind: ident("__max_align_ll"),
                                        derived: vec![],
                                        extensions: vec![Attribute {
                                            name: "__aligned__".into(),
                                            arguments: vec![AlignOf(
                                                TypeName {
                                                    specifiers: vec![Long.into(), Long.into()],
                                                    declarator: None,
                                                }
                                                .into(),
                                            )
                                            .into()],
                                        }
                                        .into()],
                                    }
                                    .into(),
                                ),
                                bit_width: None,
                            }
                            .into()],
                        }
                        .into(),
                        StructField {
                            specifiers: vec![Long.into(), Double.into()],
                            declarators: vec![StructDeclarator {
                                declarator: Some(
                                    Declarator {
                                        kind: ident("__max_align_ld"),
                                        derived: vec![],
                                        extensions: vec![Attribute {
                                            name: "__aligned__".into(),
                                            arguments: vec![AlignOf(
                                                TypeName {
                                                    specifiers: vec![Long.into(), Double.into()],
                                                    declarator: None,
                                                }
                                                .into(),
                                            )
                                            .into()],
                                        }
                                        .into()],
                                    }
                                    .into(),
                                ),
                                bit_width: None,
                            }
                            .into()],
                        }
                        .into(),
                    ]),
                }
                .into(),
            ],
            declarators: vec![InitDeclarator {
                declarator: Declarator {
                    kind: ident("max_align_t"),
                    derived: vec![],
                    extensions: vec![],
                }
                .into(),
                initializer: None,
            }
            .into()],
        }
        .into())
    );
}

fn make_declaration(name: &str, specifiers: &[Node<DeclarationSpecifier>]) -> Declaration {
    Declaration {
        specifiers: specifiers.to_vec(),
        declarators: vec![InitDeclarator {
            declarator: Declarator {
                kind: ident(name),
                derived: vec![],
                extensions: vec![],
            }
            .into(),
            initializer: None,
        }
        .into()],
    }
}

#[test]
fn test_ambiguous_declaration1() {
    use ast::DerivedDeclarator::KRFunction;
    use ast::StorageClassSpecifier::Typedef;
    use ast::TypeSpecifier::Int;
    use ast::{FunctionDefinition, TranslationUnit};
    use parser::translation_unit;

    let env = &mut Env::new();

    assert_eq!(
        translation_unit(
            r"
            typedef int a;
            int foo() {
                int a;
            }",
            env
        ),
        Ok(TranslationUnit(vec![
            make_declaration("a", &[Typedef.into(), Int.into()]).into(),
            FunctionDefinition {
                specifiers: vec![Int.into()],
                declarator: Declarator {
                    kind: ident("foo"),
                    derived: vec![KRFunction(vec![]).into()],
                    extensions: vec![]
                }
                .into(),
                declarations: vec![],
                statement: Statement::Compound(vec![make_declaration("a", &[Int.into()]).into()])
                    .into()
            }
            .into()
        ]))
    );
}

#[test]
fn test_ambiguous_declaration2() {
    use parser::translation_unit;
    let env = &mut Env::new();
    assert!(translation_unit(
        r"
            typedef int a;
            void foo() {
                unsigned int;
                const a;
                a x;
                unsigned a;
                a = 1;
            }",
        env
    )
    .is_ok());
}

#[test]
fn test_ambiguous_parameter_field_declaration() {
    use parser::translation_unit;
    let env = &mut Env::new();
    // If parameter list treated "a" as a type specifier instead of identifier, this would succeed.
    assert!(translation_unit(
        r"
            typedef int a;
            int foo(int a* b) {}",
        env
    )
    .is_err());
}

#[test]
fn test_ambiguous_struct_field_declaration() {
    use parser::translation_unit;
    let env = &mut Env::new();
    // If struct field treated "a" as a type specifier instead of identifier, this would succeed.
    assert!(translation_unit(
        r"
            typedef int a;
            struct a { a a, b; };",
        env
    )
    .is_ok());
}

#[test]
fn test_struct_name_scope() {
    use parser::translation_unit;
    let env = &mut Env::new();
    // Struct fields maintain a separate
    assert!(translation_unit(
        r"
            typedef int a;
            struct a { a a; a b; };",
        env
    )
    .is_ok());
}

#[test]
fn test_typedef_redefinition() {
    use parser::translation_unit;
    let env = &mut Env::new();
    assert!(translation_unit(
        r"
            typedef int a;
            void foo() {
               a a;
               _Atomic (a) b;
            }",
        env
    )
    .is_err());
    assert!(translation_unit(
        r"
            typedef int a;
            void foo(int a, _Atomic (a) b) {}",
        env
    )
    .is_err());
}

#[test]
fn test_defines_symbol_before_initializer() {
    // This test is currently broken, and should be enabled once symbols are defined at the
    // end of a declarator (not declaration).
    use parser::translation_unit;
    let env = &mut Env::new();
    // Technically, "a" is defined as a symbol before the "= .." part of the initializer is parsed.
    assert!(translation_unit(
        r"
            typedef int a;
            int foo() {
                int a = sizeof(_Atomic(a));
            }",
        env
    )
    .is_err());
}

#[test]
fn test_enum_modifies_scope() {
    // Enable once enum correctly modifies scope.
    use parser::translation_unit;
    let env = &mut Env::new();
    // enum {a} defines a new variable "a" into the current scope. So the next _Atomic(a) must fail.
    assert!(translation_unit(
        r"
            typedef int a;
            int foo() {
                int x = (enum {a})1;
                _Atomic(a) b;
            }",
        env
    )
    .is_err());
    // Similarly, "a" is defined immediately after its declaration.
    assert!(translation_unit(
        r"
            typedef int a;
            int foo() {
                int x = (enum {a, b = (a)1})1;
             }",
        env
    )
    .is_err());
}

#[test]
fn test_restores_scope_after_function_decl() {
    use parser::translation_unit;
    let env = &mut Env::new();
    assert!(translation_unit(
        r"
            typedef int a;
            int foo(a a) {}
            int bar(int a);
            _Atomic (a) b;
            ",
        env
    )
    .is_ok());
}

#[test]
fn test_restores_scope_after_block() {
    use parser::translation_unit;
    let env = &mut Env::new();
    assert!(translation_unit(
        r"
            void foo() {
              typedef int a;
              {
                  a a;
              }
              _Atomic (a) b;
            }",
        env
    )
    .is_ok());
}

#[test]
fn test_restores_scope_after_loops() {
    use parser::translation_unit;
    let env = &mut Env::new();
    assert!(translation_unit(
        r"
            typedef int a;
            void foo() {
                for (a a;;)
                    a = a;
                while (true) {int a;}
                do { int a; } while(true);
                _Atomic (a) b;
            }",
        env
    )
    .is_ok());
}

#[test]
fn test_restores_scope_after_selections() {
    // Enable once enum constants modify scope.
    use parser::translation_unit;
    let env = &mut Env::new();
    // Test that scope of "if" condition and statement is cleaned up.
    assert!(translation_unit(
        r"
            typedef int a, b;
            int x;
            void foo() {
                if (sizeof(enum {a})) x = sizeof(enum{b});
                else x = b;
                switch (sizeof(enum {b})) x = b;
                a x, y;
                b z, w;
            }",
        env
    )
    .is_ok());
    // Test that "if" condition enum constants are defined within its scope.
    assert!(translation_unit(
        r"
            typedef int a;
            void foo() {
                int x;
                if (sizeof(enum {a})) x = (_Atomic(a))1;
            }",
        env
    )
    .is_err());
}

#[test]
fn test_keyword_expr() {
    use parser::expression;

    assert_eq!(
        expression("__func__", &mut Env::new()),
        Ok(ident("__func__"))
    );

    assert_eq!(
        expression("__FUNCTION__", &mut Env::new()),
        Ok(ident("__FUNCTION__"))
    );

    assert_eq!(
        expression("__PRETTY_FUNCTION__", &mut Env::new()),
        Ok(ident("__PRETTY_FUNCTION__"))
    );
}

#[test]
fn test_ts18661_float() {
    use parser::declaration;
    assert_eq!(
        declaration("_Float64 foo = 1.5;", &mut Env::new()),
        Ok(Declaration {
            specifiers: vec![TS18661FloatType {
                format: TS18661FloatFormat::BinaryInterchange,
                width: 64,
            }
            .into()],
            declarators: vec![InitDeclarator {
                declarator: Declarator {
                    kind: ident("foo"),
                    derived: vec![],
                    extensions: vec![],
                }
                .into(),
                initializer: Some(float::dec("1.5")),
            }
            .into()],
        }
        .into())
    );
}

#[test]
fn test_gnu_extension() {
    use ast::TypeSpecifier::Long;
    use parser::translation_unit;
    assert_eq!(
        translation_unit("__extension__ union { long l; };", &mut Env::with_gnu()),
        Ok(TranslationUnit(vec![Declaration {
            specifiers: vec![StructType {
                kind: StructKind::Union.into(),
                identifier: None,
                declarations: Some(vec![StructField {
                    specifiers: vec![Long.into()],
                    declarators: vec![StructDeclarator {
                        declarator: Some(
                            Declarator {
                                kind: ident("l"),
                                derived: vec![],
                                extensions: vec![],
                            }
                            .into(),
                        ),
                        bit_width: None,
                    }
                    .into()],
                }
                .into()]),
            }
            .into()],
            declarators: vec![],
        }
        .into()])
        .into())
    );

    assert_eq!(
        translation_unit(r#"__extension__ _Static_assert(1,"ERR");"#, &mut Env::new()),
        Ok(TranslationUnit(vec![StaticAssert {
            expression: int::dec("1"),
            message: cstr(&[r#""ERR""#]),
        }
        .into()])
        .into())
    );
}

#[test]
fn test_declaration7() {
    use ast::DeclaratorKind::Abstract;
    use ast::DerivedDeclarator::Pointer;
    use ast::TypeQualifier::Nullable;
    use ast::TypeSpecifier::{Int, Void};
    use parser::declaration;

    let env = &mut Env::with_clang();

    assert_eq!(
        // This is the first Clang-specific declaration you'll encounter in macOS
        // if you #include <stdio.h>.
        declaration("int (* _Nullable _close)(void *);", env),
        Ok(Declaration {
            specifiers: vec![Int.into()],
            declarators: vec![InitDeclarator {
                declarator: Declarator {
                    kind: Declarator {
                        kind: ident("_close"),
                        derived: vec![Pointer(vec![Nullable.into()]).into()],
                        extensions: vec![],
                    }
                    .into(),
                    derived: vec![FunctionDeclarator {
                        parameters: vec![ParameterDeclaration {
                            specifiers: vec![Void.into()],
                            declarator: Some(
                                Declarator {
                                    kind: Abstract.into(),
                                    derived: vec![Pointer(vec![]).into()],
                                    extensions: vec![],
                                }
                                .into(),
                            ),
                            extensions: vec![],
                        }
                        .into()],
                        ellipsis: Ellipsis::None,
                    }
                    .into()],
                    extensions: vec![],
                }
                .into(),
                initializer: None,
            }
            .into()],
        }
        .into())
    );
}

#[test]
fn test_kr_definition1() {
    use ast::DerivedDeclarator::{KRFunction, Pointer};
    use ast::Statement::Compound;
    use ast::TranslationUnit;
    use ast::TypeSpecifier::{Char, Int};
    use parser::translation_unit;

    let env = &mut Env::new();

    assert_eq!(
        translation_unit("int main(argc, argv) int argc; char **argv; { }", env),
        Ok(TranslationUnit(vec![FunctionDefinition {
            specifiers: vec![Int.into()],
            declarator: Declarator {
                kind: ident("main"),
                derived: vec![KRFunction(vec![ident("argc"), ident("argv")]).into()],
                extensions: vec![],
            }
            .into(),
            declarations: vec![
                Declaration {
                    specifiers: vec![Int.into()],
                    declarators: vec![InitDeclarator {
                        declarator: Declarator {
                            kind: ident("argc"),
                            derived: vec![],
                            extensions: vec![],
                        }
                        .into(),
                        initializer: None
                    }
                    .into()],
                }
                .into(),
                Declaration {
                    specifiers: vec![Char.into()],
                    declarators: vec![InitDeclarator {
                        declarator: Declarator {
                            kind: ident("argv"),
                            derived: vec![Pointer(vec![]).into(), Pointer(vec![]).into()],
                            extensions: vec![],
                        }
                        .into(),
                        initializer: None
                    }
                    .into()],
                }
                .into(),
            ],
            statement: Compound(vec![]).into(),
        }
        .into()]))
    );
}

#[test]
fn test_clang_availability_attr() {
    use ast::AvailabilityClause::*;
    use ast::TypeSpecifier::Int;
    use parser::declaration;

    let env = &mut Env::with_clang();

    let src = r#"int f __attribute__((availability(p1,introduced=1.2.3))) __attribute__((availability(p2,unavailable,replacement="f2")));"#;

    assert_eq!(
        declaration(src, env),
        Ok(Declaration {
            specifiers: vec![Int.into(),],
            declarators: vec![InitDeclarator {
                declarator: Declarator {
                    kind: ident("f"),
                    derived: vec![],
                    extensions: vec![
                        AvailabilityAttribute {
                            platform: ident("p1"),
                            clauses: vec![Introduced(
                                AvailabilityVersion {
                                    major: "1".into(),
                                    minor: Some("2".into()),
                                    subminor: Some("3".into()),
                                }
                                .into()
                            )
                            .into()],
                        }
                        .into(),
                        AvailabilityAttribute {
                            platform: ident("p2"),
                            clauses: vec![
                                Unavailable.into(),
                                Replacement(cstr(&["\"f2\""])).into(),
                            ],
                        }
                        .into(),
                    ],
                }
                .into(),
                initializer: None,
            }
            .into(),],
        }
        .into())
    );
}

#[test]
fn test_struct_decl() {
    use ast::Declaration;
    use parser::declaration;

    let env = &mut Env::new();

    assert_eq!(
        declaration("struct foo S;", env).unwrap(),
        Declaration {
            specifiers: vec![StructType {
                kind: StructKind::Struct.into(),
                identifier: Some(ident("foo")),
                declarations: None,
            }
            .into()],

            declarators: vec![InitDeclarator {
                declarator: Declarator {
                    kind: ident("S"),
                    derived: vec![],
                    extensions: vec![],
                }
                .into(),
                initializer: None,
            }
            .into()],
        }
        .into()
    );
}

#[test]
fn test_struct_empty_decl() {
    use ast::Declaration;
    use parser::declaration;

    let env = &mut Env::with_core();
    assert!(declaration("struct foo { } S;", env).is_err());

    let env = &mut Env::with_gnu();

    assert_eq!(
        declaration("struct foo { } S;", env).unwrap(),
        Declaration {
            specifiers: vec![StructType {
                kind: StructKind::Struct.into(),
                identifier: Some(ident("foo")),
                declarations: Some(Vec::new()),
            }
            .into()],

            declarators: vec![InitDeclarator {
                declarator: Declarator {
                    kind: ident("S"),
                    derived: vec![],
                    extensions: vec![],
                }
                .into(),
                initializer: None,
            }
            .into()],
        }
        .into()
    );
}

#[test]
fn test_compound_literal() {
    use self::int::dec;
    use ast::Designator::Member;
    use ast::{CompoundLiteral, StructType};
    use parser::expression;

    let env = &mut Env::with_gnu();

    assert_eq!(
        expression("(struct test_struct) { 1, .x = 2, 3 }", env),
        Ok(CompoundLiteral {
            type_name: TypeName {
                specifiers: vec![StructType {
                    kind: StructKind::Struct.into(),
                    identifier: Some(ident("test_struct")),
                    declarations: None,
                }
                .into()],
                declarator: None,
            }
            .into(),
            initializer_list: vec![
                InitializerListItem {
                    designation: vec![],
                    initializer: dec("1"),
                }
                .into(),
                InitializerListItem {
                    designation: vec![Member(ident("x")).into()],
                    initializer: dec("2"),
                }
                .into(),
                InitializerListItem {
                    designation: vec![],
                    initializer: dec("3"),
                }
                .into(),
            ],
        }
        .into())
    );
}

// #23
#[test]
fn test_typedef_const() {
    use ast::Declaration;
    use ast::Declarator;
    use ast::InitDeclarator;
    use ast::StorageClassSpecifier::Typedef;
    use ast::TypeQualifier::Const;
    use ast::TypeSpecifier::Int;
    use parser::declaration;

    let env = &mut Env::with_core();

    assert_eq!(
        declaration("typedef const int foo;", env).unwrap(),
        Declaration {
            specifiers: vec![Typedef.into(), Const.into(), Int.into()],
            declarators: vec![InitDeclarator {
                declarator: Declarator {
                    kind: ident("foo"),
                    derived: vec![],
                    extensions: vec![],
                }
                .into(),
                initializer: None,
            }
            .into()],
        }
        .into()
    );
}
