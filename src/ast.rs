//! Abstract syntax tree
//!
//! Types in this module represent various pieces a C program can
//! contain after preprocessing phase. They mostly follow C11 grammar
//! naming conventions.
//!
//! References to C11 standard given in parenthesis refer to the
//! [ISO/IEC 9899:201x
//! draft](http://www.open-std.org/jtc1/sc22/wg14/www/docs/n1570.pdf)
//! published on April 12, 2011.
//!
//! A number of GNU extensions to the standard C are included here.
//! Types, struct fields or enum variants specific to GNU are marked as
//! "GNU extension" with a link to the relevant section of gcc
//! documentation. Supported extensions are:
//!
//! - attributes in various positions
//! - inline assembly statements and asm labels
//! - extensions to the initializer list syntax
//! - statement expressions
//! - `typeof` type specifiers

use span::Node;

/// Trait that specifies types which can be used for Identifiers and type names
pub trait Name: ::std::fmt::Debug + PartialEq + Eq + ::std::hash::Hash + Clone {
    fn get_from_str<S: AsRef<str>>(string: S) -> Self;
    fn recover_string(&self) -> String;
}

impl Name for String {
    fn get_from_str<S: AsRef<str>>(string: S) -> Self {
        string.as_ref().to_owned()
    }

    fn recover_string(&self) -> String {
        self.clone()
    }
}

// From 6.4 Lexical elements

/// Variable, function and other names that are not type names
///
/// (C11 6.4.2)
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Identifier<T: Name = String> {
    pub name: T,
}

/// Constant literals
///
/// C11 places string literals under primary expressions, thus they
/// are not included here.
///
/// (C11 6.4.4)
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Constant {
    Integer(Integer),
    Float(Float),
    Character(String),
}

/// Integer number literal
///
/// (C11 6.4.4.1)
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Integer {
    pub base: IntegerBase,
    pub number: Box<str>,
    pub suffix: IntegerSuffix,
}

/// Base of the integer literal
///
/// (C11 6.4.4.1)
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum IntegerBase {
    Decimal,
    Octal,
    Hexadecimal,
    /// [GNU extension](https://gcc.gnu.org/onlinedocs/gcc/Binary-constants.html)
    Binary,
}

/// Suffix of an integer literal
///
/// (C11 6.4.4.1)
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct IntegerSuffix {
    /// Minimum size of the integer literal
    pub size: IntegerSize,
    /// Integer literal has unsigned type
    pub unsigned: bool,
    /// Integer literal is an imaginary part of a complex number
    ///
    /// [GNU extension](https://gcc.gnu.org/onlinedocs/gcc/Complex.html) suffixes `i` and `j`.
    pub imaginary: bool,
}

/// Size part of a integer literal suffix
///
/// (C11 6.4.4.1)
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum IntegerSize {
    /// no `l` or `ll`
    Int = 0,
    /// `l`
    Long,
    /// `ll`
    LongLong,
}

/// Floating point number literal
///
/// (C11 6.4.4.2)
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Float {
    pub base: FloatBase,
    pub number: Box<str>,
    pub suffix: FloatSuffix,
}

/// Floating point number base
///
/// (C11 6.4.4.2)
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum FloatBase {
    Decimal,
    Hexadecimal,
}

/// Floating point number suffix
///
/// (C11 6.4.4.2)
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct FloatSuffix {
    pub format: FloatFormat,
    /// Integer literal is an imaginary part of a complex number
    ///
    /// [GNU extension](https://gcc.gnu.org/onlinedocs/gcc/Complex.html) suffixes `i` and `j`.
    pub imaginary: bool,
}

/// Floating point literal format specified by the suffix
///
/// (C11 6.4.4.2)
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum FloatFormat {
    /// `f` suffix
    Float,
    /// no suffix
    Double,
    /// `l` suffix
    LongDouble,
    /// [ISO/IEC TS 18661-2:2015](http://www.open-std.org/jtc1/sc22/wg14/www/docs/n1912.pdf)
    /// `df`, `dd`, `dl` suffixes
    ///
    /// [ISO/IEC TS 18661-3:2015](http://www.open-std.org/jtc1/sc22/wg14/www/docs/n1945.pdf)
    /// `fN`, `fNx`, `dN`, `dNx` suffixes
    TS18661Format(TS18661FloatType),
}

/// String literal
///
/// (C11 6.4.5)
pub type StringLiteral = Vec<String>;

// From 6.5 Expressions

/// Expressions
///
/// (C11 6.5)
#[derive(Debug, PartialEq, Clone)]
pub enum Expression<T: Name = String> {
    /// Identifier
    ///
    /// May be a variable, function name or enumerator. The latter is
    /// different from the standard, where enumerators are classified
    /// as constants.
    ///
    /// (C11 6.5.1)
    Identifier(Box<Node<Identifier<T>>>),
    /// Numeric and character constants
    ///
    /// Enumerator constants, being valid identifiers, are reprented
    /// as `Identifier` in this enum.
    ///
    /// (C11 6.5.1)
    Constant(Box<Node<Constant>>),

    /// String literal
    ///
    /// (C11 6.5.1)
    StringLiteral(Box<Node<StringLiteral>>),

    /// Generic selection
    ///
    /// (C11 6.5.1.1)
    GenericSelection(Box<Node<GenericSelection<T>>>),

    /// Structure and union members
    ///
    /// Both direct (`.`) and indirect (`->`) access.
    ///
    /// (C11 6.5.2)
    Member(Box<Node<MemberExpression<T>>>),

    /// Function call expression
    ///
    /// (C11 6.5.2)
    Call(Box<Node<CallExpression<T>>>),

    /// Compound literal
    ///
    /// (C11 6.5.2)
    CompoundLiteral(Box<Node<CompoundLiteral<T>>>),

    /// Size of a type
    ///
    /// Note: size of an expression is represented with `UnaryOperator::SizeOf`.
    ///
    /// (C11 6.5.3)
    SizeOf(Box<Node<TypeName<T>>>),

    /// Alignment of a type
    ///
    /// (C11 6.5.3)
    AlignOf(Box<Node<TypeName<T>>>),

    /// Unary operators
    ///
    /// This represents both postfix and prefix unary oprators. Postfix expressions that take
    /// additional operands are represented by a separate entry in this enum.
    ///
    /// (C11 6.5.2, c11 6.5.3)
    UnaryOperator(Box<Node<UnaryOperatorExpression<T>>>),

    /// Cast expression
    ///
    /// `(type) expr`
    ///
    /// (C11 6.5.4)
    Cast(Box<Node<CastExpression<T>>>),

    /// Binary operators
    ///
    /// All of C binary operators that can be applied to two expressions.
    ///
    /// (C11 6.5.5 -- 6.5.16)
    BinaryOperator(Box<Node<BinaryOperatorExpression<T>>>),

    /// Conditional operator
    ///
    /// (C11 6.5.15)
    Conditional(Box<Node<ConditionalExpression<T>>>),

    /// Comma operator
    ///
    /// (C11 6.5.17)
    Comma(Box<Vec<Node<Expression<T>>>>),

    /// Member offset expression
    ///
    /// Result of expansion of `offsetof` macro.
    ///
    /// (C11 7.19 §3).
    OffsetOf(Box<Node<OffsetOfExpression<T>>>),

    /// Variable argument list access
    ///
    /// Result of expansion of `va_arg` macro.
    ///
    /// (C11 7.16.1.1).
    VaArg(Box<Node<VaArgExpression<T>>>),

    /// Statement expression
    ///
    /// [GNU extension](https://gcc.gnu.org/onlinedocs/gcc/Statement-Exprs.html)
    Statement(Box<Node<Statement<T>>>),
}

/// Struct or union member access
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum MemberOperator {
    /// `expression.identifier`
    Direct,
    /// `expression->identifier`
    Indirect,
}

/// Generic selection expression
///
/// (C11 6.5.1.1)
#[derive(Debug, PartialEq, Clone)]
pub struct GenericSelection<T: Name = String> {
    pub expression: Box<Node<Expression<T>>>,
    pub associations: Vec<Node<GenericAssociation<T>>>,
}

/// Single element of a generic selection expression
///
/// (C11 6.5.1.1)
#[derive(Debug, PartialEq, Clone)]
pub enum GenericAssociation<T: Name = String> {
    Type(Node<GenericAssociationType<T>>),
    Default(Box<Node<Expression<T>>>),
}

/// Type match case in a generic selection expression
///
/// (C11 6.5.1.1)
#[derive(Debug, PartialEq, Clone)]
pub struct GenericAssociationType<T: Name = String> {
    pub type_name: Node<TypeName<T>>,
    pub expression: Box<Node<Expression<T>>>,
}

/// Structure and union members
///
/// Both direct (`.`) and indirect (`->`) access.
///
/// (C11 6.5.2)
#[derive(Debug, PartialEq, Clone)]
pub struct MemberExpression<T: Name = String> {
    pub operator: Node<MemberOperator>,
    pub expression: Box<Node<Expression<T>>>,
    pub identifier: Node<Identifier<T>>,
}

/// Function call expression
///
/// (C11 6.5.2)
#[derive(Debug, PartialEq, Clone)]
pub struct CallExpression<T: Name = String> {
    pub callee: Box<Node<Expression<T>>>,
    pub arguments: Vec<Node<Expression<T>>>,
}

/// Compound literal
///
/// (C11 6.5.2)
#[derive(Debug, PartialEq, Clone)]
pub struct CompoundLiteral<T: Name = String> {
    pub type_name: Node<TypeName<T>>,
    pub initializer_list: Vec<Node<InitializerListItem<T>>>,
}

/// All operators with one operand
///
/// (C11 6.5)
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum UnaryOperator {
    /// `operand++`
    PostIncrement,
    /// `operand--`
    PostDecrement,
    /// `++operand`
    PreIncrement,
    /// `--operand`
    PreDecrement,
    /// `&operand`
    Address,
    /// `*operand`
    Indirection,
    /// `+operand`
    Plus,
    /// `-operand`
    Minus,
    /// `~operand`
    Complement,
    /// `!operand`
    Negate,
    /// `sizeof operand`
    SizeOf,
}

/// Unary operator expression
///
/// This represents both postfix and prefix unary oprators. Postfix expressions that take
/// additional operands are represented by a separate entry in this enum.
///
/// (C11 6.5.2, c11 6.5.3)
#[derive(Debug, PartialEq, Clone)]
pub struct UnaryOperatorExpression<T: Name = String> {
    pub operator: Node<UnaryOperator>,
    pub operand: Box<Node<Expression<T>>>,
}

/// Cast expression
///
/// `(type) expr`
///
/// (C11 6.5.4)
#[derive(Debug, PartialEq, Clone)]
pub struct CastExpression<T: Name = String> {
    pub type_name: Node<TypeName<T>>,
    pub expression: Box<Node<Expression<T>>>,
}

/// All operators with two operands
///
/// (C11 6.5)
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum BinaryOperator {
    /// `lhs[rhs]`
    Index,
    /// `lhs * rhs`
    Multiply,
    /// `lhs / rhs`
    Divide,
    /// `lhs % rhs`
    Modulo,
    /// `lhs + rhs`
    Plus,
    /// `lhs - rhs`
    Minus,
    /// `lhs << rhs`
    ShiftLeft,
    /// `lhs >> rhs`
    ShiftRight,
    /// `lhs < rhs`
    Less,
    /// `lhs > rhs`
    Greater,
    /// `lhs <= rhs`
    LessOrEqual,
    /// `lhs >= rhs`
    GreaterOrEqual,
    /// `lhs == rhs`
    Equals,
    /// `lhs != rhs`
    NotEquals,
    /// `lhs & rhs`
    BitwiseAnd,
    /// `lhs ^ rhs`
    BitwiseXor,
    /// `lhs | rhs`
    BitwiseOr,
    /// `lhs && rhs`
    LogicalAnd,
    /// `lhs || rhs`
    LogicalOr,
    /// `lhs = rhs`
    Assign,
    /// `lhs *= rhs`
    AssignMultiply,
    /// `lhs /= rhs`
    AssignDivide,
    /// `lhs %= rhs`
    AssignModulo,
    /// `lhs += rhs`
    AssignPlus,
    /// `lhs -= rhs`
    AssignMinus,
    /// `lhs <<= rhs`
    AssignShiftLeft,
    /// `lhs >>= rhs`
    AssignShiftRight,
    /// `lhs &= rhs`
    AssignBitwiseAnd,
    /// `lhs ^= rhs`
    AssignBitwiseXor,
    /// `lhs |= rhs`
    AssignBitwiseOr,
}

/// Binary operators
///
/// All of C binary operators that can be applied to two expressions.
///
/// (C11 6.5.5 -- 6.5.16)
#[derive(Debug, PartialEq, Clone)]
pub struct BinaryOperatorExpression<T: Name = String> {
    pub operator: Node<BinaryOperator>,
    pub lhs: Box<Node<Expression<T>>>,
    pub rhs: Box<Node<Expression<T>>>,
}

/// Conditional operator
///
/// (C11 6.5.15)
#[derive(Debug, PartialEq, Clone)]
pub struct ConditionalExpression<T: Name = String> {
    pub condition: Box<Node<Expression<T>>>,
    pub then_expression: Box<Node<Expression<T>>>,
    pub else_expression: Box<Node<Expression<T>>>,
}

/// Variable argument list access
///
/// Result of expansion of `va_arg` macro.
///
/// (C11 7.16.1.1).
#[derive(Debug, PartialEq, Clone)]
pub struct VaArgExpression<T: Name = String> {
    pub va_list: Box<Node<Expression<T>>>,
    pub type_name: Node<TypeName<T>>,
}

/// Member offset expression
///
/// Result of expansion of `offsetof` macro.
///
/// (C11 7.19 §3).
#[derive(Debug, PartialEq, Clone)]
pub struct OffsetOfExpression<T: Name = String> {
    pub type_name: Node<TypeName<T>>,
    pub designator: Node<OffsetDesignator<T>>,
}

/// Offset designator in a `offsetof` macro expansion
///
/// (C11 7.19 §3).
#[derive(Debug, PartialEq, Clone)]
pub struct OffsetDesignator<T: Name = String> {
    pub base: Node<Identifier<T>>,
    pub members: Vec<Node<OffsetMember<T>>>,
}

/// Single element of an offset designator
///
/// (C11 7.19 §3).
#[derive(Debug, PartialEq, Clone)]
pub enum OffsetMember<T: Name = String> {
    Member(Node<Identifier<T>>),
    IndirectMember(Node<Identifier<T>>),
    Index(Node<Expression<T>>),
}

// From 6.7 Declarations

/// Variable, function or type declaration
///
/// (C11 6.7)
#[derive(Debug, PartialEq, Clone)]
pub struct Declaration<T: Name = String> {
    pub specifiers: Vec<Node<DeclarationSpecifier<T>>>,
    pub declarators: Vec<Node<InitDeclarator<T>>>,
}

/// Common part of a declaration
///
/// These apply to all declarators in a declaration.
///
/// (C11 6.7)
#[derive(Debug, PartialEq, Clone)]
pub enum DeclarationSpecifier<T: Name = String> {
    StorageClass(Node<StorageClassSpecifier>),
    TypeSpecifier(Node<TypeSpecifier<T>>),
    TypeQualifier(Node<TypeQualifier>),
    Function(Node<FunctionSpecifier>),
    Alignment(Node<AlignmentSpecifier<T>>),
    /// Vendor-specific declaration extensions that can be mixed with standard specifiers
    Extension(Vec<Node<Extension<T>>>),
}

/// Defines a single name in a declaration
///
/// (C11 6.7.6)
#[derive(Debug, PartialEq, Clone)]
pub struct InitDeclarator<T: Name = String> {
    pub declarator: Node<Declarator<T>>,
    pub initializer: Option<Node<Initializer<T>>>,
}

// From 6.7.1

/// Storage class
///
/// (C11 6.7.1)
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum StorageClassSpecifier {
    /// `typedef`
    Typedef,
    /// `extern`
    Extern,
    /// `static`
    Static,
    /// `_Thread_local`
    ThreadLocal,
    /// `auto`
    Auto,
    /// `register`
    Register,
}

// From 6.7.2

/// Type specifier
///
/// (C11 6.7.2)
#[derive(Debug, PartialEq, Clone)]
pub enum TypeSpecifier<T: Name = String> {
    /// `void`
    Void,
    /// `char`
    Char,
    /// `short`
    Short,
    /// `int`
    Int,
    /// `long`
    Long,
    /// `float`
    Float,
    /// `double`
    Double,
    /// `signed`
    ///
    /// `__signed`, `__signed__` (GNU extension)
    Signed,
    /// `unsigned`
    Unsigned,
    /// `_Bool`
    Bool,
    /// `_Complex`
    ///
    /// `__complex`, `__complex__` (GNU extension)
    Complex,
    /// `_Atomic(typename)`
    Atomic(Node<TypeName<T>>),
    /// `struct identifier<T: Name = String> { … }`
    ///
    /// `union identifier<T: Name = String> { … }`
    Struct(Node<StructType<T>>),
    /// `enum identifier<T: Name = String> { … }`
    Enum(Node<EnumType<T>>),
    /// Name of a previously defined type
    TypedefName(Node<Identifier<T>>),
    /// Specifies type of another type or expression
    ///
    /// [GNU extension](https://gcc.gnu.org/onlinedocs/gcc/Typeof.html)
    TypeOf(Node<TypeOf<T>>),
    /// Floating point types with guaranteed width and representation
    ///
    /// `_Float16`, `_Float32`, `_Float64`, `_Float128`
    ///
    /// `_Float16x`, `_Float32x`, `_Float64x`, `_Float128x`
    ///
    /// `_Decimal16`, `_Decimal32`, `_Decimal64`, `_Decimal128`
    ///
    /// `_Decimal16x`, `_Decimal32x`, `_Decimal64x`, `_Decimal128x`
    ///
    /// [ISO/IEC TS 18661-3:2015](http://www.open-std.org/jtc1/sc22/wg14/www/docs/n1945.pdf)
    TS18661Float(TS18661FloatType),
}

/// Floating point type with guaranteed width and format
///
/// [ISO/IEC TS 18661-3:2015](http://www.open-std.org/jtc1/sc22/wg14/www/docs/n1945.pdf)
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct TS18661FloatType {
    pub format: TS18661FloatFormat,
    pub width: usize,
}

/// Floating point formats
///
/// [ISO/IEC TS 18661-3:2015](http://www.open-std.org/jtc1/sc22/wg14/www/docs/n1945.pdf)
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum TS18661FloatFormat {
    BinaryInterchange,
    BinaryExtended,
    DecimalInterchange,
    DecimalExtended,
}

// From 6.7.2.1

/// Structure or union type specifier
///
/// (C11 6.7.2.1)
#[derive(Debug, PartialEq, Clone)]
pub struct StructType<T: Name = String> {
    pub kind: Node<StructKind>,
    pub identifier: Option<Node<Identifier<T>>>,
    /// List of structure of union members, when present.
    ///
    /// A [GNU extension](https://gcc.gnu.org/onlinedocs/gcc-8.1.0/gcc/Empty-Structures.html) allows the list to be empty.
    pub declarations: Option<Vec<Node<StructDeclaration<T>>>>,
}

/// The only difference between a `struct` and a `union`
///
/// (C11 6.7.2.1)
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum StructKind {
    Struct,
    Union,
}

/// Single declaration in a struct or a union
///
/// (C11 6.7.2.1)
#[derive(Debug, PartialEq, Clone)]
pub enum StructDeclaration<T: Name = String> {
    Field(Node<StructField<T>>),
    StaticAssert(Node<StaticAssert<T>>),
}

/// Struct field declaration
#[derive(Debug, PartialEq, Clone)]
pub struct StructField<T: Name = String> {
    pub specifiers: Vec<Node<SpecifierQualifier<T>>>,
    pub declarators: Vec<Node<StructDeclarator<T>>>,
}

/// Type and qualifiers for a struct declaration
///
/// C11 also uses this type in a few other places.
///
/// (C11 6.7.2.1)
#[derive(Debug, PartialEq, Clone)]
pub enum SpecifierQualifier<T: Name = String> {
    TypeSpecifier(Node<TypeSpecifier<T>>),
    TypeQualifier(Node<TypeQualifier>),
}

/// Field declarator for a struct or a union
///
/// (C11 6.7.2.1)
#[derive(Debug, PartialEq, Clone)]
pub struct StructDeclarator<T: Name = String> {
    pub declarator: Option<Node<Declarator<T>>>,
    pub bit_width: Option<Box<Node<Expression<T>>>>,
}

// From 6.7.2.2

/// Enumeration type specifier
///
/// (C11 6.7.2.2)
#[derive(Debug, PartialEq, Clone)]
pub struct EnumType<T: Name = String> {
    pub identifier: Option<Node<Identifier<T>>>,
    pub enumerators: Vec<Node<Enumerator<T>>>,
}

/// Single constant inside a `enum` definition
///
/// (C11 6.7.2.2)
#[derive(Debug, PartialEq, Clone)]
pub struct Enumerator<T: Name = String> {
    pub identifier: Node<Identifier<T>>,
    pub expression: Option<Box<Node<Expression<T>>>>,
}

// From 6.7.3

/// Type qualifier
///
/// (C11 6.7.3)
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum TypeQualifier {
    /// `const`
    ///
    /// `__const` (GNU extension)
    Const,
    /// `restrict`
    ///
    /// `__restrict`, `__restrict__` (GNU extension)
    Restrict,
    /// `volatile`
    ///
    /// `__volatile`, `__volatile__` (GNU extension)
    Volatile,
    /// '_Nonnull' (Clang extension)
    ///
    /// [Clang extension](https://clang.llvm.org/docs/AttributeReference.html)
    Nonnull,
    /// '_Null_unspecified' (Clang extension)
    ///
    /// [Clang extension](https://clang.llvm.org/docs/AttributeReference.html)
    NullUnspecified,
    /// '_Nullable' (Clang extension)
    ///
    /// [Clang extension](https://clang.llvm.org/docs/AttributeReference.html)
    Nullable,
    /// `_Atomic`
    Atomic,
}

// From 6.7.4

/// Function specifier
///
/// (C11 6.7.4)
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum FunctionSpecifier {
    /// `inline`
    ///
    /// `__inline`, `__inline__` (GNU extension)
    Inline,
    /// `_Noreturn`
    Noreturn,
}

// From 6.7.5

/// Alignment specifier
///
/// (C11 6.7.5)
#[derive(Debug, PartialEq, Clone)]
pub enum AlignmentSpecifier<T: Name = String> {
    /// `_Alignas(typename)`
    Type(Node<TypeName<T>>),
    /// `_Alignas(expression)`
    Constant(Box<Node<Expression<T>>>),
}

// From 6.7.6 Declarators

/// Single item in a declaration
///
/// Represents both normal and abstract declarators.
///
/// (C11 6.7.6, 6.7.7)
#[derive(Debug, PartialEq, Clone)]
pub struct Declarator<T: Name = String> {
    /// What is being declared
    pub kind: Node<DeclaratorKind<T>>,
    /// Contains pointer, array and function declarator elements
    pub derived: Vec<Node<DerivedDeclarator<T>>>,
    /// Vendor-specific extensions
    pub extensions: Vec<Node<Extension<T>>>,
}

/// Name of a declarator
///
/// (C11 6.7.6, 6.7.7)
#[derive(Debug, PartialEq, Clone)]
pub enum DeclaratorKind<T: Name = String> {
    /// Unnamed declarator
    ///
    /// E.g. part of a function prototype without parameter names.
    Abstract,
    /// Named declarator
    ///
    /// E.g. a variable or a named function parameter.
    Identifier(Node<Identifier<T>>),
    /// Nested declarator
    ///
    /// Any group of parenthesis inside a declarator. E.g. pointer to
    /// a function.
    Declarator(Box<Node<Declarator<T>>>),
}

/// Modifies declarator type
///
/// (C11 6.7.6)
#[derive(Debug, PartialEq, Clone)]
pub enum DerivedDeclarator<T: Name = String> {
    /// `* qualifiers …`
    Pointer(Vec<Node<PointerQualifier<T>>>),
    /// `… []`
    Array(Node<ArrayDeclarator<T>>),
    /// `… ( parameters )`
    Function(Node<FunctionDeclarator<T>>),
    /// `… ( identifiers )`
    KRFunction(Vec<Node<Identifier<T>>>),
}

/// Array part of a declarator
#[derive(Debug, PartialEq, Clone)]
pub struct ArrayDeclarator<T: Name = String> {
    pub qualifiers: Vec<Node<TypeQualifier>>,
    pub size: ArraySize<T>,
}

/// Function parameter part of a declarator
#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDeclarator<T: Name = String> {
    pub parameters: Vec<Node<ParameterDeclaration<T>>>,
    pub ellipsis: Ellipsis,
}

/// List of qualifiers that can follow a `*` in a declaration
///
/// (C11 6.7.6.1)
#[derive(Debug, PartialEq, Clone)]
pub enum PointerQualifier<T: Name = String> {
    TypeQualifier(Node<TypeQualifier>),
    Extension(Vec<Node<Extension<T>>>),
}

/// Size of an array in a declaration
///
/// (C11 6.7.6.2)
#[derive(Debug, PartialEq, Clone)]
pub enum ArraySize<T: Name = String> {
    /// `[]`
    Unknown,
    /// `[*]`
    VariableUnknown,
    /// `[10]`
    VariableExpression(Box<Node<Expression<T>>>),
    /// `[static 10]`
    StaticExpression(Box<Node<Expression<T>>>),
}

/// Complete parameter declaration in a function prototype or declaration
///
/// This is so called "new-style" or "C89" parameter declaration that
/// follows in parenthesis after a function name. "Old-style" or "K&R"
/// function parameter declarations are collected in the
/// `FunctionDefinition::declarations` field.
///
/// (C11 6.7.6.3)
#[derive(Debug, PartialEq, Clone)]
pub struct ParameterDeclaration<T: Name = String> {
    pub specifiers: Vec<Node<DeclarationSpecifier<T>>>,
    pub declarator: Option<Node<Declarator<T>>>,
    pub extensions: Vec<Node<Extension<T>>>,
}

/// Whether function signature ends with a `...`
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Ellipsis {
    Some,
    None,
}

// From 6.7.7 Type names

/// References to types outside of declarations
///
/// Type names contain only abstract declarators.
///
/// (C11 6.7.7)
#[derive(Debug, PartialEq, Clone)]
pub struct TypeName<T: Name = String> {
    pub specifiers: Vec<Node<SpecifierQualifier<T>>>,
    pub declarator: Option<Node<Declarator<T>>>,
}

// From 6.7.9 Initialization

/// Value that is assigned immediately in a declaration
///
/// (C11 6.7.9)
#[derive(Debug, PartialEq, Clone)]
pub enum Initializer<T: Name = String> {
    Expression(Box<Node<Expression<T>>>),
    List(Vec<Node<InitializerListItem<T>>>),
}

/// Initializes one field or array element in a initializer list
///
/// (C11 6.7.9)
#[derive(Debug, PartialEq, Clone)]
pub struct InitializerListItem<T: Name = String> {
    pub designation: Vec<Node<Designator<T>>>,
    pub initializer: Box<Node<Initializer<T>>>,
}

/// Single element of an designation in an initializer
#[derive(Debug, PartialEq, Clone)]
pub enum Designator<T: Name = String> {
    /// Array element
    ///
    /// `{ [expression] = … }`
    ///
    /// `{ [expression] … }` (obsolete GNU extension)
    Index(Node<Expression<T>>),

    /// Struct or union member
    ///
    /// `{ .identifier = … }`
    ///
    /// `{ identifier: … }` (obsolete GNU extension)
    Member(Node<Identifier<T>>),

    /// Range of array elements
    ///
    /// `{ [from ... to] … }`
    /// ([GNU extension](https://gcc.gnu.org/onlinedocs/gcc/Designated-Inits.html#Designated-Inits))
    Range(Node<RangeDesignator<T>>),
}

/// Range array designator in an initializer
///
/// `[from ... to]`
///
/// ([GNU extension](https://gcc.gnu.org/onlinedocs/gcc/Designated-Inits.html#Designated-Inits))
#[derive(Debug, PartialEq, Clone)]
pub struct RangeDesignator<T: Name = String> {
    pub from: Node<Expression<T>>,
    pub to: Node<Expression<T>>,
}

// From 6.7.10 Static assertions

/// Static assertion
///
/// (C11 6.7.10)
#[derive(Debug, PartialEq, Clone)]
pub struct StaticAssert<T: Name = String> {
    pub expression: Box<Node<Expression<T>>>,
    pub message: Node<StringLiteral>,
}

// From 6.8 Statement

/// Element of a function body
///
/// (C11 6.8)
#[derive(Debug, PartialEq, Clone)]
pub enum Statement<T: Name = String> {
    Labeled(Node<LabeledStatement<T>>),
    Compound(Vec<Node<BlockItem<T>>>),
    Expression(Option<Box<Node<Expression<T>>>>),
    If(Node<IfStatement<T>>),
    Switch(Node<SwitchStatement<T>>),
    While(Node<WhileStatement<T>>),
    DoWhile(Node<DoWhileStatement<T>>),
    For(Node<ForStatement<T>>),
    Goto(Node<Identifier<T>>),
    Continue,
    Break,
    Return(Option<Box<Node<Expression<T>>>>),
    /// Vendor specific inline assembly extensions
    Asm(Node<AsmStatement<T>>),
}

/// Labeled statement
///
/// (C11 6.8.1)
#[derive(Debug, PartialEq, Clone)]
pub struct LabeledStatement<T: Name = String> {
    pub label: Node<Label<T>>,
    pub statement: Box<Node<Statement<T>>>,
}

/// If statement
///
/// (C11 6.8.4)
#[derive(Debug, PartialEq, Clone)]
pub struct IfStatement<T: Name = String> {
    pub condition: Box<Node<Expression<T>>>,
    pub then_statement: Box<Node<Statement<T>>>,
    pub else_statement: Option<Box<Node<Statement<T>>>>,
}

/// Switch statement
///
/// (C11 6.8.4)
#[derive(Debug, PartialEq, Clone)]
pub struct SwitchStatement<T: Name = String> {
    pub expression: Box<Node<Expression<T>>>,
    pub statement: Box<Node<Statement<T>>>,
}

/// While statement
///
/// (C11 6.8.5)
#[derive(Debug, PartialEq, Clone)]
pub struct WhileStatement<T: Name = String> {
    pub expression: Box<Node<Expression<T>>>,
    pub statement: Box<Node<Statement<T>>>,
}

/// Do statement
///
/// (C11 6.8.5)
#[derive(Debug, PartialEq, Clone)]
pub struct DoWhileStatement<T: Name = String> {
    pub statement: Box<Node<Statement<T>>>,
    pub expression: Box<Node<Expression<T>>>,
}

/// For statement
///
/// (C11 6.8.5)
#[derive(Debug, PartialEq, Clone)]
pub struct ForStatement<T: Name = String> {
    pub initializer: Node<ForInitializer<T>>,
    pub condition: Option<Box<Node<Expression<T>>>>,
    pub step: Option<Box<Node<Expression<T>>>>,
    pub statement: Box<Node<Statement<T>>>,
}

/// Statement labels for `goto` and `switch`
#[derive(Debug, PartialEq, Clone)]
pub enum Label<T: Name = String> {
    /// Goto label
    ///
    /// `ident: …`
    Identifier(Node<Identifier<T>>),
    /// Case in a `switch` statement
    ///
    /// `case 'a': …`
    Case(Box<Node<Expression<T>>>),
    /// Default case in a `switch` statement
    ///
    /// `default: …`
    Default,
}

/// First element of a `for` statement
#[derive(Debug, PartialEq, Clone)]
pub enum ForInitializer<T: Name = String> {
    /// `for(; …)`
    Empty,
    /// `for(a = 1; …)`
    Expression(Box<Node<Expression<T>>>),
    /// `for(int a = 1; …)`
    Declaration(Node<Declaration<T>>),
    /// `for(_StaticAssert(…); …)`
    StaticAssert(Node<StaticAssert<T>>),
}

// From 6.8.2

/// Element of a compound statement
#[derive(Debug, PartialEq, Clone)]
pub enum BlockItem<T: Name = String> {
    Declaration(Node<Declaration<T>>),
    StaticAssert(Node<StaticAssert<T>>),
    Statement(Node<Statement<T>>),
}

// From 6.9 External definitions

/// Entire C source file after preprocessing
///
/// (C11 6.9)
#[derive(Debug, PartialEq, Clone)]
pub struct TranslationUnit<T: Name = String>(pub Vec<Node<ExternalDeclaration<T>>>);

/// Top-level elements of a C program
///
/// (C11 6.9)
#[derive(Debug, PartialEq, Clone)]
pub enum ExternalDeclaration<T: Name = String> {
    Declaration(Node<Declaration<T>>),
    StaticAssert(Node<StaticAssert<T>>),
    FunctionDefinition(Node<FunctionDefinition<T>>),
}

/// Function definition
///
/// (C11 6.9.1)
#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDefinition<T: Name = String> {
    /// Return type of the function, possibly mixed with other specifiers
    pub specifiers: Vec<Node<DeclarationSpecifier<T>>>,
    /// Contains function name and parameter list
    pub declarator: Node<Declarator<T>>,
    /// K&R style parameter type definitions (C11 6.9.1 §6)
    pub declarations: Vec<Node<Declaration<T>>>,
    /// Body of the function.
    pub statement: Node<Statement<T>>,
}

// Syntax extensions

/// Extended vendor-specific syntax that does not fit elsewhere
#[derive(Debug, PartialEq, Clone)]
pub enum Extension<T: Name = String> {
    /// Attributes
    ///
    /// [GNU extension](https://gcc.gnu.org/onlinedocs/gcc/Attribute-Syntax.html)
    Attribute(Attribute<T>),
    /// Assembler name for an object
    ///
    /// [GNU extension](https://gcc.gnu.org/onlinedocs/gcc/Asm-Labels.html)
    AsmLabel(Node<StringLiteral>),
    /// Platform availability
    ///
    /// [Clang extension](https://clang.llvm.org/docs/AttributeReference.html#availability)
    AvailabilityAttribute(Node<AvailabilityAttribute<T>>),
}

/// Attributes
///
/// [GNU extension](https://gcc.gnu.org/onlinedocs/gcc/Attribute-Syntax.html)
#[derive(Debug, PartialEq, Clone)]
pub struct Attribute<T: Name = String> {
    pub name: Node<String>,
    pub arguments: Vec<Node<Expression<T>>>,
}

/// Platform availability attribute
///
/// [Clang extension](https://clang.llvm.org/docs/AttributeReference.html#availability)
#[derive(Debug, PartialEq, Clone)]
pub struct AvailabilityAttribute<T: Name = String> {
    pub platform: Node<Identifier<T>>,
    pub clauses: Vec<Node<AvailabilityClause>>,
}

/// Platfrom availability attribute clause
///
/// [Clang extension](https://clang.llvm.org/docs/AttributeReference.html#availability)
#[derive(Debug, PartialEq, Clone)]
pub enum AvailabilityClause {
    Introduced(Node<AvailabilityVersion>),
    Deprecated(Node<AvailabilityVersion>),
    Obsoleted(Node<AvailabilityVersion>),
    Unavailable,
    Message(Node<StringLiteral>),
    Replacement(Node<StringLiteral>),
}

/// Platfrom version inside availability attribute
///
/// [Clang extension](https://clang.llvm.org/docs/AttributeReference.html#availability)
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct AvailabilityVersion {
    pub major: String,
    pub minor: Option<String>,
    pub subminor: Option<String>,
}

/// Inline assembler
#[derive(Debug, PartialEq, Clone)]
pub enum AsmStatement<T: Name = String> {
    /// Basic asm statement with just source code
    ///
    /// [GNU extension](https://gcc.gnu.org/onlinedocs/gcc/Basic-Asm.html)
    GnuBasic(Node<StringLiteral>),

    /// Extended statement that has access to C variables
    ///
    /// [GNU extension](https://gcc.gnu.org/onlinedocs/gcc/Extended-Asm.html)
    GnuExtended(GnuExtendedAsmStatement<T>),
}

/// Extended statement that has access to C variables
///
/// [GNU extension](https://gcc.gnu.org/onlinedocs/gcc/Extended-Asm.html)
#[derive(Debug, PartialEq, Clone)]
pub struct GnuExtendedAsmStatement<T: Name = String> {
    pub qualifier: Option<Node<TypeQualifier>>,
    pub template: Node<StringLiteral>,
    pub outputs: Vec<Node<GnuAsmOperand<T>>>,
    pub inputs: Vec<Node<GnuAsmOperand<T>>>,
    pub clobbers: Vec<Node<StringLiteral>>,
}

/// Single input or output operand specifier for GNU extended asm statement
///
/// [GNU extension](https://gcc.gnu.org/onlinedocs/gcc/Extended-Asm.html#Output-Operands)
#[derive(Debug, PartialEq, Clone)]
pub struct GnuAsmOperand<T: Name = String> {
    pub symbolic_name: Option<Node<Identifier<T>>>,
    pub constraints: Node<StringLiteral>,
    pub variable_name: Node<Expression<T>>,
}

/// Type of an expression or type
///
/// [GNU extension](https://gcc.gnu.org/onlinedocs/gcc/Typeof.html)
#[derive(Debug, PartialEq, Clone)]
pub enum TypeOf<T: Name = String> {
    Expression(Node<Expression<T>>),
    Type(Node<TypeName<T>>),
}
