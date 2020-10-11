//! Debug rinter for abstract syntax tree
//!
//! ```no_run
//! # let unit = todo!();
//! # use lang_c::print::Printer;
//! use lang_c::visit::Visit;
//! let s = &mut String::new();
//! Printer::new(s).visit_translation_unit(unit);
//! ```
use std::fmt;

use ast::*;
use span::Span;
use visit::*;

/// Printing visitor
///
/// Recursively prints the AST tree as indented list of AST nodes, one node per line.
/// Each line contains name of the AST node type, followed by the enum variant
/// (when it does not match name of contained node), and primitive fields.
pub struct Printer<'a> {
    w: &'a mut dyn fmt::Write,
    offset: usize,
}

impl Printer<'_> {
    pub fn new<'a>(w: &'a mut dyn fmt::Write) -> Printer<'a> {
        Printer { w, offset: 0 }
    }

    fn block(&mut self) -> Printer {
        writeln!(&mut self.w, "").unwrap();
        Printer {
            w: &mut self.w,
            offset: self.offset + 1,
        }
    }

    fn name(&mut self, name: &str) {
        write!(&mut self.w, "{2:1$}{0}", name, self.offset * 4, "").unwrap();
    }
}

macro_rules! simple_visit {
    ( $lt:lifetime, $( $id:ident: $ty:ty $(, $repr:tt)? ; )* ) => {
        $(
            fn $id(&mut self, n: &$lt $ty , span: &$lt Span) {
                self.name(stringify!($ty));
                $( simple_visit!(= $repr, self, n ); )*
                $id(&mut self.block(), n, span);
            }
            )*
    };

    (= self, $s:expr, $n:ident ) => { write!(&mut $s.w, " {:?}", $n).unwrap(); };
    (= $fn:ident, $s: expr, $n:ident ) => { $fn($s, $n); };
    (= { $( $ ($f:ident).* ),* }, $s:expr, $n:ident ) => {
        $( write!(&mut $s.w, " {:?}", $n.$($f).*).unwrap(); )*
    };
}

impl<'ast> Visit<'ast> for Printer<'_> {
    simple_visit! {
        'ast,
        visit_identifier: Identifier, { name };
        visit_constant: Constant;
        visit_integer: Integer, { number };
        visit_integer_base: IntegerBase, self;
        visit_integer_suffix: IntegerSuffix, { unsigned, imaginary };
        visit_integer_size: IntegerSize, self;
        visit_float: Float, { number };
        visit_float_base: FloatBase, self;
        visit_float_suffix: FloatSuffix, { imaginary };
        visit_float_format: FloatFormat, print_float_format;
        visit_string_literal: StringLiteral, self;
        visit_expression: Expression;
        visit_member_operator: MemberOperator, self;
        visit_generic_selection: GenericSelection;
        visit_generic_association: GenericAssociation;
        visit_generic_association_type: GenericAssociationType;
        visit_member_expression: MemberExpression;
        visit_call_expression: CallExpression;
        visit_compound_literal: CompoundLiteral;
        visit_unary_operator: UnaryOperator, self;
        visit_unary_operator_expression: UnaryOperatorExpression;
        visit_cast_expression: CastExpression;
        visit_binary_operator: BinaryOperator, self;
        visit_binary_operator_expression: BinaryOperatorExpression;
        visit_conditional_expression: ConditionalExpression;
        visit_va_arg_expression: VaArgExpression;
        visit_offset_of_expression: OffsetOfExpression;
        visit_offset_designator: OffsetDesignator;
        visit_offset_member: OffsetMember, print_offset_member;
        visit_declaration: Declaration;
        visit_declaration_specifier: DeclarationSpecifier;
        visit_init_declarator: InitDeclarator;
        visit_storage_class_specifier: StorageClassSpecifier, self;
        visit_type_specifier: TypeSpecifier, print_type_specifier;
        visit_ts18661_float_type: TS18661FloatType, { width };
        visit_ts18661_float_format: TS18661FloatFormat, self;
        visit_struct_type: StructType;
        visit_struct_kind: StructKind, self;
        visit_struct_declaration: StructDeclaration;
        visit_struct_field: StructField;
        visit_specifier_qualifier: SpecifierQualifier;
        visit_struct_declarator: StructDeclarator;
        visit_enum_type: EnumType;
        visit_enumerator: Enumerator;
        visit_type_qualifier: TypeQualifier, self;
        visit_function_specifier: FunctionSpecifier, self;
        visit_alignment_specifier: AlignmentSpecifier;
        visit_declarator: Declarator;
        visit_declarator_kind: DeclaratorKind, print_declarator_kind;
        visit_derived_declarator: DerivedDeclarator;
        visit_array_declarator: ArrayDeclarator;
        visit_function_declarator: FunctionDeclarator;
        visit_pointer_qualifier: PointerQualifier;
        visit_array_size: ArraySize, print_array_size;
        visit_parameter_declaration: ParameterDeclaration;
        visit_ellipsis: Ellipsis, self;
        visit_type_name: TypeName;
        visit_initializer: Initializer;
        visit_initializer_list_item: InitializerListItem;
        visit_designator: Designator;
        visit_range_designator: RangeDesignator;
        visit_static_assert: StaticAssert;
        visit_statement: Statement, print_statement;
        visit_labeled_statement: LabeledStatement;
        visit_if_statement: IfStatement;
        visit_switch_statement: SwitchStatement;
        visit_while_statement: WhileStatement;
        visit_do_while_statement: DoWhileStatement;
        visit_for_statement: ForStatement;
        visit_label: Label, print_label;
        visit_for_initializer: ForInitializer, print_for_initializer;
        visit_block_item: BlockItem;
        visit_external_declaration: ExternalDeclaration;
        visit_function_definition: FunctionDefinition;
        visit_extension: Extension;
        visit_attribute: Attribute, { name.node };
        visit_asm_statement: AsmStatement;
        visit_availability_attribute: AvailabilityAttribute;
        visit_gnu_extended_asm_statement: GnuExtendedAsmStatement;
        visit_gnu_asm_operand: GnuAsmOperand;
        visit_type_of: TypeOf;
    }

    fn visit_translation_unit(&mut self, translation_unit: &'ast TranslationUnit) {
        self.name("TranslationUnit");
        visit_translation_unit(&mut self.block(), translation_unit);
    }
}

macro_rules! enum_printer {
    ( $( $id:ident: $ty:ident: $( $var:ident ),* ;)* ) => {
        $(
            fn $id<'ast>(p: &mut Printer<'_>, n: &'ast $ty) {
                #[allow(unreachable_patterns)]
                match n {
                    $( $ty::$var { .. } => write!(&mut p.w, " {}", stringify!($var)).unwrap(), )*
                    _ => {},
                }
            }
        )*
    };
}

enum_printer! {
    print_float_format: FloatFormat: Float, Double, LongDouble;
    print_declarator_kind: DeclaratorKind: Abstract;
    print_array_size: ArraySize: Unknown, VariableUnknown, VariableExpression, StaticExpression;
    print_statement: Statement: Compound, Goto, Continue, Break, Return;
    print_offset_member: OffsetMember: Member, IndirectMember;
    print_label: Label: Default;
    print_for_initializer: ForInitializer: Empty;
    print_type_specifier: TypeSpecifier:
        Void, Char, Short, Int, Long, Float, Double, Signed, Unsigned, Complex, Atomic, TypedefName;
}
