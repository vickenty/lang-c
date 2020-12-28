#![allow(unknown_lints)]
#![allow(bare_trait_objects)]

//! Debug rinter for abstract syntax tree
//!
//! ```no_run
//! # use lang_c::print::Printer;
//! use lang_c::visit::Visit;
//! # let unit = panic!();
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
    w: &'a mut fmt::Write,
    offset: usize,
}

impl<'a> Printer<'a> {
    pub fn new(w: &mut fmt::Write) -> Printer {
        Printer { w: w, offset: 0 }
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

    fn write_field(&mut self, f: &fmt::Debug) {
        write!(&mut self.w, " {:?}", f).unwrap();
    }
}

impl<'ast, 'a> Visit<'ast> for Printer<'a> {
    fn visit_identifier(&mut self, n: &'ast Identifier<String>, span: &'ast Span) {
        self.name("Identifier");
        self.write_field(&n.name);
        visit_identifier(&mut self.block(), n, span);
    }
    fn visit_constant(&mut self, n: &'ast Constant, span: &'ast Span) {
        self.name("Constant");
        visit_constant::<_, String>(&mut self.block(), n, span);
    }
    fn visit_integer(&mut self, n: &'ast Integer, span: &'ast Span) {
        self.name("Integer");
        self.write_field(&n.number);
        visit_integer::<_, String>(&mut self.block(), n, span);
    }
    fn visit_integer_base(&mut self, n: &'ast IntegerBase, span: &'ast Span) {
        self.name("IntegerBase");
        self.write_field(&n);
        visit_integer_base::<_, String>(&mut self.block(), n, span);
    }
    fn visit_integer_suffix(&mut self, n: &'ast IntegerSuffix, span: &'ast Span) {
        self.name("IntegerSuffix");
        self.write_field(&n.unsigned);
        self.write_field(&n.imaginary);
        visit_integer_suffix::<_, String>(&mut self.block(), n, span);
    }
    fn visit_integer_size(&mut self, n: &'ast IntegerSize, span: &'ast Span) {
        self.name("IntegerSize");
        self.write_field(&n);
        visit_integer_size::<_, String>(&mut self.block(), n, span);
    }
    fn visit_float(&mut self, n: &'ast Float, span: &'ast Span) {
        self.name("Float");
        self.write_field(&n.number);
        visit_float::<_, String>(&mut self.block(), n, span);
    }
    fn visit_float_base(&mut self, n: &'ast FloatBase, span: &'ast Span) {
        self.name("FloatBase");
        self.write_field(&n);
        visit_float_base::<_, String>(&mut self.block(), n, span);
    }
    fn visit_float_suffix(&mut self, n: &'ast FloatSuffix, span: &'ast Span) {
        self.name("FloatSuffix");
        self.write_field(&n.imaginary);
        visit_float_suffix::<_, String>(&mut self.block(), n, span);
    }
    fn visit_float_format(&mut self, n: &'ast FloatFormat, span: &'ast Span) {
        self.name("FloatFormat");
        print_float_format(self, n);
        visit_float_format::<_, String>(&mut self.block(), n, span);
    }
    fn visit_string_literal(&mut self, n: &'ast StringLiteral, span: &'ast Span) {
        self.name("StringLiteral");
        self.write_field(&n);
        visit_string_literal::<_, String>(&mut self.block(), n, span);
    }
    fn visit_expression(&mut self, n: &'ast Expression<String>, span: &'ast Span) {
        self.name("Expression");
        visit_expression(&mut self.block(), n, span);
    }
    fn visit_member_operator(&mut self, n: &'ast MemberOperator, span: &'ast Span) {
        self.name("MemberOperator");
        self.write_field(&n);
        visit_member_operator::<_, String>(&mut self.block(), n, span);
    }
    fn visit_generic_selection(&mut self, n: &'ast GenericSelection<String>, span: &'ast Span) {
        self.name("GenericSelection");
        visit_generic_selection(&mut self.block(), n, span);
    }
    fn visit_generic_association(&mut self, n: &'ast GenericAssociation<String>, span: &'ast Span) {
        self.name("GenericAssociation");
        visit_generic_association(&mut self.block(), n, span);
    }
    fn visit_generic_association_type(
        &mut self,
        n: &'ast GenericAssociationType<String>,
        span: &'ast Span,
    ) {
        self.name("GenericAssociationType");
        visit_generic_association_type(&mut self.block(), n, span);
    }
    fn visit_member_expression(&mut self, n: &'ast MemberExpression<String>, span: &'ast Span) {
        self.name("MemberExpression");
        visit_member_expression(&mut self.block(), n, span);
    }
    fn visit_call_expression(&mut self, n: &'ast CallExpression<String>, span: &'ast Span) {
        self.name("CallExpression");
        visit_call_expression(&mut self.block(), n, span);
    }
    fn visit_compound_literal(&mut self, n: &'ast CompoundLiteral<String>, span: &'ast Span) {
        self.name("CompoundLiteral");
        visit_compound_literal(&mut self.block(), n, span);
    }
    fn visit_unary_operator(&mut self, n: &'ast UnaryOperator, span: &'ast Span) {
        self.name("UnaryOperator");
        self.write_field(&n);
        visit_unary_operator::<_, String>(&mut self.block(), n, span);
    }
    fn visit_unary_operator_expression(
        &mut self,
        n: &'ast UnaryOperatorExpression<String>,
        span: &'ast Span,
    ) {
        self.name("UnaryOperatorExpression");
        visit_unary_operator_expression(&mut self.block(), n, span);
    }
    fn visit_cast_expression(&mut self, n: &'ast CastExpression<String>, span: &'ast Span) {
        self.name("CastExpression");
        visit_cast_expression(&mut self.block(), n, span);
    }
    fn visit_binary_operator(&mut self, n: &'ast BinaryOperator, span: &'ast Span) {
        self.name("BinaryOperator");
        self.write_field(&n);
        visit_binary_operator::<_, String>(&mut self.block(), n, span);
    }
    fn visit_binary_operator_expression(
        &mut self,
        n: &'ast BinaryOperatorExpression<String>,
        span: &'ast Span,
    ) {
        self.name("BinaryOperatorExpression");
        visit_binary_operator_expression(&mut self.block(), n, span);
    }
    fn visit_conditional_expression(
        &mut self,
        n: &'ast ConditionalExpression<String>,
        span: &'ast Span,
    ) {
        self.name("ConditionalExpression");
        visit_conditional_expression(&mut self.block(), n, span);
    }
    fn visit_va_arg_expression(&mut self, n: &'ast VaArgExpression<String>, span: &'ast Span) {
        self.name("VaArgExpression");
        visit_va_arg_expression(&mut self.block(), n, span);
    }
    fn visit_offset_of_expression(
        &mut self,
        n: &'ast OffsetOfExpression<String>,
        span: &'ast Span,
    ) {
        self.name("OffsetOfExpression");
        visit_offset_of_expression(&mut self.block(), n, span);
    }
    fn visit_offset_designator(&mut self, n: &'ast OffsetDesignator<String>, span: &'ast Span) {
        self.name("OffsetDesignator");
        visit_offset_designator(&mut self.block(), n, span);
    }
    fn visit_offset_member(&mut self, n: &'ast OffsetMember<String>, span: &'ast Span) {
        self.name("OffsetMember");
        print_offset_member(self, n);
        visit_offset_member(&mut self.block(), n, span);
    }
    fn visit_declaration(&mut self, n: &'ast Declaration<String>, span: &'ast Span) {
        self.name("Declaration");
        visit_declaration(&mut self.block(), n, span);
    }
    fn visit_declaration_specifier(
        &mut self,
        n: &'ast DeclarationSpecifier<String>,
        span: &'ast Span,
    ) {
        self.name("DeclarationSpecifier");
        visit_declaration_specifier(&mut self.block(), n, span);
    }
    fn visit_init_declarator(&mut self, n: &'ast InitDeclarator<String>, span: &'ast Span) {
        self.name("InitDeclarator");
        visit_init_declarator(&mut self.block(), n, span);
    }
    fn visit_storage_class_specifier(&mut self, n: &'ast StorageClassSpecifier, span: &'ast Span) {
        self.name("StorageClassSpecifier");
        self.write_field(&n);
        visit_storage_class_specifier::<_, String>(&mut self.block(), n, span);
    }
    fn visit_type_specifier(&mut self, n: &'ast TypeSpecifier<String>, span: &'ast Span) {
        self.name("TypeSpecifier");
        print_type_specifier(self, n);
        visit_type_specifier(&mut self.block(), n, span);
    }
    fn visit_ts18661_float_type(&mut self, n: &'ast TS18661FloatType, span: &'ast Span) {
        self.name("TS18661FloatType");
        self.write_field(&n.width);
        visit_ts18661_float_type::<_, String>(&mut self.block(), n, span);
    }
    fn visit_ts18661_float_format(&mut self, n: &'ast TS18661FloatFormat, span: &'ast Span) {
        self.name("TS18661FloatFormat");
        self.write_field(&n);
        visit_ts18661_float_format::<_, String>(&mut self.block(), n, span);
    }
    fn visit_struct_type(&mut self, n: &'ast StructType<String>, span: &'ast Span) {
        self.name("StructType");
        visit_struct_type(&mut self.block(), n, span);
    }
    fn visit_struct_kind(&mut self, n: &'ast StructKind, span: &'ast Span) {
        self.name("StructKind");
        self.write_field(&n);
        visit_struct_kind::<_, String>(&mut self.block(), n, span);
    }
    fn visit_struct_declaration(&mut self, n: &'ast StructDeclaration<String>, span: &'ast Span) {
        self.name("StructDeclaration");
        visit_struct_declaration(&mut self.block(), n, span);
    }
    fn visit_struct_field(&mut self, n: &'ast StructField<String>, span: &'ast Span) {
        self.name("StructField");
        visit_struct_field(&mut self.block(), n, span);
    }
    fn visit_specifier_qualifier(&mut self, n: &'ast SpecifierQualifier<String>, span: &'ast Span) {
        self.name("SpecifierQualifier");
        visit_specifier_qualifier(&mut self.block(), n, span);
    }
    fn visit_struct_declarator(&mut self, n: &'ast StructDeclarator<String>, span: &'ast Span) {
        self.name("StructDeclarator");
        visit_struct_declarator(&mut self.block(), n, span);
    }
    fn visit_enum_type(&mut self, n: &'ast EnumType<String>, span: &'ast Span) {
        self.name("EnumType");
        visit_enum_type(&mut self.block(), n, span);
    }
    fn visit_enumerator(&mut self, n: &'ast Enumerator<String>, span: &'ast Span) {
        self.name("Enumerator");
        visit_enumerator(&mut self.block(), n, span);
    }
    fn visit_type_qualifier(&mut self, n: &'ast TypeQualifier, span: &'ast Span) {
        self.name("TypeQualifier");
        self.write_field(&n);
        visit_type_qualifier::<_, String>(&mut self.block(), n, span);
    }
    fn visit_function_specifier(&mut self, n: &'ast FunctionSpecifier, span: &'ast Span) {
        self.name("FunctionSpecifier");
        self.write_field(&n);
        visit_function_specifier::<_, String>(&mut self.block(), n, span);
    }
    fn visit_alignment_specifier(&mut self, n: &'ast AlignmentSpecifier<String>, span: &'ast Span) {
        self.name("AlignmentSpecifier");
        visit_alignment_specifier(&mut self.block(), n, span);
    }
    fn visit_declarator(&mut self, n: &'ast Declarator<String>, span: &'ast Span) {
        self.name("Declarator");
        visit_declarator(&mut self.block(), n, span);
    }
    fn visit_declarator_kind(&mut self, n: &'ast DeclaratorKind<String>, span: &'ast Span) {
        self.name("DeclaratorKind");
        print_declarator_kind(self, n);
        visit_declarator_kind(&mut self.block(), n, span);
    }
    fn visit_derived_declarator(&mut self, n: &'ast DerivedDeclarator<String>, span: &'ast Span) {
        self.name("DerivedDeclarator");
        visit_derived_declarator(&mut self.block(), n, span);
    }
    fn visit_array_declarator(&mut self, n: &'ast ArrayDeclarator<String>, span: &'ast Span) {
        self.name("ArrayDeclarator");
        visit_array_declarator(&mut self.block(), n, span);
    }
    fn visit_function_declarator(&mut self, n: &'ast FunctionDeclarator<String>, span: &'ast Span) {
        self.name("FunctionDeclarator");
        visit_function_declarator(&mut self.block(), n, span);
    }
    fn visit_pointer_qualifier(&mut self, n: &'ast PointerQualifier<String>, span: &'ast Span) {
        self.name("PointerQualifier");
        visit_pointer_qualifier(&mut self.block(), n, span);
    }
    fn visit_array_size(&mut self, n: &'ast ArraySize<String>, span: &'ast Span) {
        self.name("ArraySize");
        print_array_size(self, n);
        visit_array_size(&mut self.block(), n, span);
    }
    fn visit_parameter_declaration(
        &mut self,
        n: &'ast ParameterDeclaration<String>,
        span: &'ast Span,
    ) {
        self.name("ParameterDeclaration");
        visit_parameter_declaration(&mut self.block(), n, span);
    }
    fn visit_ellipsis(&mut self, n: &'ast Ellipsis, span: &'ast Span) {
        self.name("Ellipsis");
        self.write_field(&n);
        visit_ellipsis::<_, String>(&mut self.block(), n, span);
    }
    fn visit_type_name(&mut self, n: &'ast TypeName<String>, span: &'ast Span) {
        self.name("TypeName");
        visit_type_name(&mut self.block(), n, span);
    }
    fn visit_initializer(&mut self, n: &'ast Initializer<String>, span: &'ast Span) {
        self.name("Initializer");
        visit_initializer(&mut self.block(), n, span);
    }
    fn visit_initializer_list_item(
        &mut self,
        n: &'ast InitializerListItem<String>,
        span: &'ast Span,
    ) {
        self.name("InitializerListItem");
        visit_initializer_list_item(&mut self.block(), n, span);
    }
    fn visit_designator(&mut self, n: &'ast Designator<String>, span: &'ast Span) {
        self.name("Designator");
        visit_designator(&mut self.block(), n, span);
    }
    fn visit_range_designator(&mut self, n: &'ast RangeDesignator<String>, span: &'ast Span) {
        self.name("RangeDesignator");
        visit_range_designator(&mut self.block(), n, span);
    }
    fn visit_static_assert(&mut self, n: &'ast StaticAssert<String>, span: &'ast Span) {
        self.name("StaticAssert");
        visit_static_assert(&mut self.block(), n, span);
    }
    fn visit_statement(&mut self, n: &'ast Statement<String>, span: &'ast Span) {
        self.name("Statement");
        print_statement(self, n);
        visit_statement(&mut self.block(), n, span);
    }
    fn visit_labeled_statement(&mut self, n: &'ast LabeledStatement<String>, span: &'ast Span) {
        self.name("LabeledStatement");
        visit_labeled_statement(&mut self.block(), n, span);
    }
    fn visit_if_statement(&mut self, n: &'ast IfStatement<String>, span: &'ast Span) {
        self.name("IfStatement");
        visit_if_statement(&mut self.block(), n, span);
    }
    fn visit_switch_statement(&mut self, n: &'ast SwitchStatement<String>, span: &'ast Span) {
        self.name("SwitchStatement");
        visit_switch_statement(&mut self.block(), n, span);
    }
    fn visit_while_statement(&mut self, n: &'ast WhileStatement<String>, span: &'ast Span) {
        self.name("WhileStatement");
        visit_while_statement(&mut self.block(), n, span);
    }
    fn visit_do_while_statement(&mut self, n: &'ast DoWhileStatement<String>, span: &'ast Span) {
        self.name("DoWhileStatement");
        visit_do_while_statement(&mut self.block(), n, span);
    }
    fn visit_for_statement(&mut self, n: &'ast ForStatement<String>, span: &'ast Span) {
        self.name("ForStatement");
        visit_for_statement(&mut self.block(), n, span);
    }
    fn visit_label(&mut self, n: &'ast Label<String>, span: &'ast Span) {
        self.name("Label");
        print_label(self, n);
        visit_label(&mut self.block(), n, span);
    }
    fn visit_for_initializer(&mut self, n: &'ast ForInitializer<String>, span: &'ast Span) {
        self.name("ForInitializer");
        print_for_initializer(self, n);
        visit_for_initializer(&mut self.block(), n, span);
    }
    fn visit_block_item(&mut self, n: &'ast BlockItem<String>, span: &'ast Span) {
        self.name("BlockItem");
        visit_block_item(&mut self.block(), n, span);
    }
    fn visit_external_declaration(
        &mut self,
        n: &'ast ExternalDeclaration<String>,
        span: &'ast Span,
    ) {
        self.name("ExternalDeclaration");
        visit_external_declaration(&mut self.block(), n, span);
    }
    fn visit_function_definition(&mut self, n: &'ast FunctionDefinition<String>, span: &'ast Span) {
        self.name("FunctionDefinition");
        visit_function_definition(&mut self.block(), n, span);
    }
    fn visit_extension(&mut self, n: &'ast Extension<String>, span: &'ast Span) {
        self.name("Extension");
        visit_extension(&mut self.block(), n, span);
    }
    fn visit_attribute(&mut self, n: &'ast Attribute<String>, span: &'ast Span) {
        self.name("Attribute");
        self.write_field(&n.name.node);
        visit_attribute(&mut self.block(), n, span);
    }
    fn visit_asm_statement(&mut self, n: &'ast AsmStatement<String>, span: &'ast Span) {
        self.name("AsmStatement");
        visit_asm_statement(&mut self.block(), n, span);
    }
    fn visit_availability_attribute(
        &mut self,
        n: &'ast AvailabilityAttribute<String>,
        span: &'ast Span,
    ) {
        self.name("AvailabilityAttribute");
        visit_availability_attribute(&mut self.block(), n, span);
    }
    fn visit_gnu_extended_asm_statement(
        &mut self,
        n: &'ast GnuExtendedAsmStatement<String>,
        span: &'ast Span,
    ) {
        self.name("GnuExtendedAsmStatement");
        visit_gnu_extended_asm_statement(&mut self.block(), n, span);
    }
    fn visit_gnu_asm_operand(&mut self, n: &'ast GnuAsmOperand<String>, span: &'ast Span) {
        self.name("GnuAsmOperand");
        visit_gnu_asm_operand(&mut self.block(), n, span);
    }
    fn visit_type_of(&mut self, n: &'ast TypeOf<String>, span: &'ast Span) {
        self.name("TypeOf");
        visit_type_of(&mut self.block(), n, span);
    }
    fn visit_translation_unit(&mut self, translation_unit: &'ast TranslationUnit<String>) {
        self.name("TranslationUnit");
        visit_translation_unit(&mut self.block(), translation_unit);
    }
}

fn print_float_format<'ast>(p: &mut Printer, n: &'ast FloatFormat) {
    match *n {
        FloatFormat::Float => p.w.write_str(" Float").unwrap(),
        FloatFormat::Double => p.w.write_str(" Double").unwrap(),
        FloatFormat::LongDouble => p.w.write_str(" LongDouble").unwrap(),
        _ => {}
    }
}
fn print_declarator_kind<'ast, T: Name>(p: &mut Printer, n: &'ast DeclaratorKind<T>) {
    match *n {
        DeclaratorKind::Abstract => p.w.write_str(" Abstract").unwrap(),
        _ => {}
    }
}
fn print_array_size<'ast, T: Name>(p: &mut Printer, n: &'ast ArraySize<T>) {
    match *n {
        ArraySize::Unknown => p.w.write_str(" Unknown").unwrap(),
        ArraySize::VariableUnknown => p.w.write_str(" VariableUnknown").unwrap(),
        ArraySize::VariableExpression(_) => p.w.write_str(" VariableExpression").unwrap(),
        ArraySize::StaticExpression(_) => p.w.write_str(" StaticExpression").unwrap(),
    }
}
fn print_statement<'ast, T: Name>(p: &mut Printer, n: &'ast Statement<T>) {
    match *n {
        Statement::Compound(_) => p.w.write_str(" Compound").unwrap(),
        Statement::Goto(_) => p.w.write_str(" Goto").unwrap(),
        Statement::Continue => p.w.write_str(" Continue").unwrap(),
        Statement::Break => p.w.write_str(" Break").unwrap(),
        Statement::Return(_) => p.w.write_str(" Return").unwrap(),
        _ => {}
    }
}
fn print_offset_member<'ast, T: Name>(p: &mut Printer, n: &'ast OffsetMember<T>) {
    match *n {
        OffsetMember::Member(_) => p.w.write_str(" Member").unwrap(),
        OffsetMember::IndirectMember(_) => p.w.write_str(" IndirectMember").unwrap(),
        _ => {}
    }
}
fn print_label<'ast, T: Name>(p: &mut Printer, n: &'ast Label<T>) {
    match *n {
        Label::Default => p.w.write_str(" Default").unwrap(),
        _ => {}
    }
}
fn print_for_initializer<'ast, T: Name>(p: &mut Printer, n: &'ast ForInitializer<T>) {
    match *n {
        ForInitializer::Empty => p.w.write_str(" Empty").unwrap(),
        _ => {}
    }
}
fn print_type_specifier<'ast, T: Name>(p: &mut Printer, n: &'ast TypeSpecifier<T>) {
    match *n {
        TypeSpecifier::Void => p.w.write_str(" Void").unwrap(),
        TypeSpecifier::Char => p.w.write_str(" Char").unwrap(),
        TypeSpecifier::Short => p.w.write_str(" Short").unwrap(),
        TypeSpecifier::Int => p.w.write_str(" Int").unwrap(),
        TypeSpecifier::Long => p.w.write_str(" Long").unwrap(),
        TypeSpecifier::Float => p.w.write_str(" Float").unwrap(),
        TypeSpecifier::Double => p.w.write_str(" Double").unwrap(),
        TypeSpecifier::Signed => p.w.write_str(" Signed").unwrap(),
        TypeSpecifier::Unsigned => p.w.write_str(" Unsigned").unwrap(),
        TypeSpecifier::Complex => p.w.write_str(" Complex").unwrap(),
        TypeSpecifier::Atomic(_) => p.w.write_str(" Atomic").unwrap(),
        TypeSpecifier::TypedefName(_) => p.w.write_str(" TypedefName").unwrap(),
        _ => {}
    }
}
