//! Recursive abstract syntax tree traversal
//!
//! ```rust
//! # use lang_c::{ast, span, visit};
//! struct ExprCount(usize);
//!
//! impl<'ast> visit::Visit<'ast> for ExprCount {
//!     fn visit_expression(&mut self, expr: &'ast ast::Expression, span: &'ast span::Span) {
//!         self.0 += 1;
//!         visit::visit_expression(self, expr, span);
//!     }
//! }
//! ```
//!
//! The `Visit` trait is a collection of hooks, one for each type of node in the AST (for each type
//! in the `ast` module). Default implementations will recursively visit the sub-nodes (by calling
//! a corresponding free function in this module).
//!
//! Free functions apply the visitor to sub-nodes of any given AST node.

use ast::*;
use span::Span;

pub trait Visit<'ast> {
    fn visit_identifier(&mut self, identifier: &'ast Identifier, span: &'ast Span) {
        visit_identifier(self, identifier, span)
    }

    fn visit_constant(&mut self, constant: &'ast Constant, span: &'ast Span) {
        visit_constant(self, constant, span)
    }

    fn visit_integer(&mut self, integer: &'ast Integer, span: &'ast Span) {
        visit_integer(self, integer, span)
    }

    fn visit_integer_base(&mut self, integer_base: &'ast IntegerBase, span: &'ast Span) {
        visit_integer_base(self, integer_base, span)
    }

    fn visit_integer_suffix(&mut self, integer_suffix: &'ast IntegerSuffix, span: &'ast Span) {
        visit_integer_suffix(self, integer_suffix, span)
    }

    fn visit_integer_size(&mut self, integer_size: &'ast IntegerSize, span: &'ast Span) {
        visit_integer_size(self, integer_size, span)
    }

    fn visit_float(&mut self, float: &'ast Float, span: &'ast Span) {
        visit_float(self, float, span)
    }

    fn visit_float_base(&mut self, float_base: &'ast FloatBase, span: &'ast Span) {
        visit_float_base(self, float_base, span)
    }

    fn visit_float_suffix(&mut self, float_suffix: &'ast FloatSuffix, span: &'ast Span) {
        visit_float_suffix(self, float_suffix, span)
    }

    fn visit_float_format(&mut self, float_format: &'ast FloatFormat, span: &'ast Span) {
        visit_float_format(self, float_format, span)
    }

    fn visit_string_literal(&mut self, string_literal: &'ast StringLiteral, span: &'ast Span) {
        visit_string_literal(self, string_literal, span)
    }

    fn visit_expression(&mut self, expression: &'ast Expression, span: &'ast Span) {
        visit_expression(self, expression, span)
    }

    fn visit_member_operator(&mut self, member_operator: &'ast MemberOperator, span: &'ast Span) {
        visit_member_operator(self, member_operator, span)
    }

    fn visit_generic_selection(
        &mut self,
        generic_selection: &'ast GenericSelection,
        span: &'ast Span,
    ) {
        visit_generic_selection(self, generic_selection, span)
    }

    fn visit_generic_association(
        &mut self,
        generic_association: &'ast GenericAssociation,
        span: &'ast Span,
    ) {
        visit_generic_association(self, generic_association, span)
    }

    fn visit_generic_association_type(
        &mut self,
        generic_association_type: &'ast GenericAssociationType,
        span: &'ast Span,
    ) {
        visit_generic_association_type(self, generic_association_type, span)
    }

    fn visit_member_expression(
        &mut self,
        member_expression: &'ast MemberExpression,
        span: &'ast Span,
    ) {
        visit_member_expression(self, member_expression, span)
    }

    fn visit_call_expression(&mut self, call_expression: &'ast CallExpression, span: &'ast Span) {
        visit_call_expression(self, call_expression, span)
    }

    fn visit_compound_literal(
        &mut self,
        compound_literal: &'ast CompoundLiteral,
        span: &'ast Span,
    ) {
        visit_compound_literal(self, compound_literal, span)
    }

    fn visit_sizeofty(&mut self, sizeofty: &'ast SizeOfTy, span: &'ast Span) {
        visit_sizeofty(self, sizeofty, span)
    }
    fn visit_sizeofval(&mut self, sizeofval: &'ast SizeOfVal, span: &'ast Span) {
        visit_sizeofval(self, sizeofval, span)
    }

    fn visit_alignof(&mut self, alignofty: &'ast AlignOf, span: &'ast Span) {
        visit_alignof(self, alignofty, span)
    }

    fn visit_unary_operator(&mut self, unary_operator: &'ast UnaryOperator, span: &'ast Span) {
        visit_unary_operator(self, unary_operator, span)
    }

    fn visit_unary_operator_expression(
        &mut self,
        unary_operator_expression: &'ast UnaryOperatorExpression,
        span: &'ast Span,
    ) {
        visit_unary_operator_expression(self, unary_operator_expression, span)
    }

    fn visit_cast_expression(&mut self, cast_expression: &'ast CastExpression, span: &'ast Span) {
        visit_cast_expression(self, cast_expression, span)
    }

    fn visit_binary_operator(&mut self, binary_operator: &'ast BinaryOperator, span: &'ast Span) {
        visit_binary_operator(self, binary_operator, span)
    }

    fn visit_binary_operator_expression(
        &mut self,
        binary_operator_expression: &'ast BinaryOperatorExpression,
        span: &'ast Span,
    ) {
        visit_binary_operator_expression(self, binary_operator_expression, span)
    }

    fn visit_conditional_expression(
        &mut self,
        conditional_expression: &'ast ConditionalExpression,
        span: &'ast Span,
    ) {
        visit_conditional_expression(self, conditional_expression, span)
    }

    fn visit_va_arg_expression(
        &mut self,
        va_arg_expression: &'ast VaArgExpression,
        span: &'ast Span,
    ) {
        visit_va_arg_expression(self, va_arg_expression, span)
    }

    fn visit_offset_of_expression(
        &mut self,
        offset_of_expression: &'ast OffsetOfExpression,
        span: &'ast Span,
    ) {
        visit_offset_of_expression(self, offset_of_expression, span)
    }

    fn visit_offset_designator(
        &mut self,
        offset_designator: &'ast OffsetDesignator,
        span: &'ast Span,
    ) {
        visit_offset_designator(self, offset_designator, span)
    }

    fn visit_offset_member(&mut self, offset_member: &'ast OffsetMember, span: &'ast Span) {
        visit_offset_member(self, offset_member, span)
    }

    fn visit_declaration(&mut self, declaration: &'ast Declaration, span: &'ast Span) {
        visit_declaration(self, declaration, span)
    }

    fn visit_declaration_specifier(
        &mut self,
        declaration_specifier: &'ast DeclarationSpecifier,
        span: &'ast Span,
    ) {
        visit_declaration_specifier(self, declaration_specifier, span)
    }

    fn visit_init_declarator(&mut self, init_declarator: &'ast InitDeclarator, span: &'ast Span) {
        visit_init_declarator(self, init_declarator, span)
    }

    fn visit_storage_class_specifier(
        &mut self,
        storage_class_specifier: &'ast StorageClassSpecifier,
        span: &'ast Span,
    ) {
        visit_storage_class_specifier(self, storage_class_specifier, span)
    }

    fn visit_type_specifier(&mut self, type_specifier: &'ast TypeSpecifier, span: &'ast Span) {
        visit_type_specifier(self, type_specifier, span)
    }

    fn visit_ts18661_float_type(
        &mut self,
        ts18661_float_type: &'ast TS18661FloatType,
        span: &'ast Span,
    ) {
        visit_ts18661_float_type(self, ts18661_float_type, span)
    }

    fn visit_ts18661_float_format(
        &mut self,
        ts18661_float_format: &'ast TS18661FloatFormat,
        span: &'ast Span,
    ) {
        visit_ts18661_float_format(self, ts18661_float_format, span)
    }

    fn visit_struct_type(&mut self, struct_type: &'ast StructType, span: &'ast Span) {
        visit_struct_type(self, struct_type, span)
    }

    fn visit_struct_kind(&mut self, struct_kind: &'ast StructKind, span: &'ast Span) {
        visit_struct_kind(self, struct_kind, span)
    }

    fn visit_struct_declaration(
        &mut self,
        struct_declaration: &'ast StructDeclaration,
        span: &'ast Span,
    ) {
        visit_struct_declaration(self, struct_declaration, span)
    }

    fn visit_struct_field(&mut self, struct_field: &'ast StructField, span: &'ast Span) {
        visit_struct_field(self, struct_field, span)
    }

    fn visit_specifier_qualifier(
        &mut self,
        specifier_qualifier: &'ast SpecifierQualifier,
        span: &'ast Span,
    ) {
        visit_specifier_qualifier(self, specifier_qualifier, span)
    }

    fn visit_struct_declarator(
        &mut self,
        struct_declarator: &'ast StructDeclarator,
        span: &'ast Span,
    ) {
        visit_struct_declarator(self, struct_declarator, span)
    }

    fn visit_enum_type(&mut self, enum_type: &'ast EnumType, span: &'ast Span) {
        visit_enum_type(self, enum_type, span)
    }

    fn visit_enumerator(&mut self, enumerator: &'ast Enumerator, span: &'ast Span) {
        visit_enumerator(self, enumerator, span)
    }

    fn visit_type_qualifier(&mut self, type_qualifier: &'ast TypeQualifier, span: &'ast Span) {
        visit_type_qualifier(self, type_qualifier, span)
    }

    fn visit_function_specifier(
        &mut self,
        function_specifier: &'ast FunctionSpecifier,
        span: &'ast Span,
    ) {
        visit_function_specifier(self, function_specifier, span)
    }

    fn visit_alignment_specifier(
        &mut self,
        alignment_specifier: &'ast AlignmentSpecifier,
        span: &'ast Span,
    ) {
        visit_alignment_specifier(self, alignment_specifier, span)
    }

    fn visit_declarator(&mut self, declarator: &'ast Declarator, span: &'ast Span) {
        visit_declarator(self, declarator, span)
    }

    fn visit_declarator_kind(&mut self, declarator_kind: &'ast DeclaratorKind, span: &'ast Span) {
        visit_declarator_kind(self, declarator_kind, span)
    }

    fn visit_derived_declarator(
        &mut self,
        derived_declarator: &'ast DerivedDeclarator,
        span: &'ast Span,
    ) {
        visit_derived_declarator(self, derived_declarator, span)
    }

    fn visit_array_declarator(
        &mut self,
        array_declarator: &'ast ArrayDeclarator,
        span: &'ast Span,
    ) {
        visit_array_declarator(self, array_declarator, span)
    }

    fn visit_function_declarator(
        &mut self,
        function_declarator: &'ast FunctionDeclarator,
        span: &'ast Span,
    ) {
        visit_function_declarator(self, function_declarator, span)
    }

    fn visit_pointer_qualifier(
        &mut self,
        pointer_qualifier: &'ast PointerQualifier,
        span: &'ast Span,
    ) {
        visit_pointer_qualifier(self, pointer_qualifier, span)
    }

    fn visit_array_size(&mut self, array_size: &'ast ArraySize, span: &'ast Span) {
        visit_array_size(self, array_size, span)
    }

    fn visit_parameter_declaration(
        &mut self,
        parameter_declaration: &'ast ParameterDeclaration,
        span: &'ast Span,
    ) {
        visit_parameter_declaration(self, parameter_declaration, span)
    }

    fn visit_ellipsis(&mut self, ellipsis: &'ast Ellipsis, span: &'ast Span) {
        visit_ellipsis(self, ellipsis, span)
    }

    fn visit_type_name(&mut self, type_name: &'ast TypeName, span: &'ast Span) {
        visit_type_name(self, type_name, span)
    }

    fn visit_initializer(&mut self, initializer: &'ast Initializer, span: &'ast Span) {
        visit_initializer(self, initializer, span)
    }

    fn visit_initializer_list_item(
        &mut self,
        initializer_list_item: &'ast InitializerListItem,
        span: &'ast Span,
    ) {
        visit_initializer_list_item(self, initializer_list_item, span)
    }

    fn visit_designator(&mut self, designator: &'ast Designator, span: &'ast Span) {
        visit_designator(self, designator, span)
    }

    fn visit_range_designator(
        &mut self,
        range_designator: &'ast RangeDesignator,
        span: &'ast Span,
    ) {
        visit_range_designator(self, range_designator, span)
    }

    fn visit_static_assert(&mut self, static_assert: &'ast StaticAssert, span: &'ast Span) {
        visit_static_assert(self, static_assert, span)
    }

    fn visit_statement(&mut self, statement: &'ast Statement, span: &'ast Span) {
        visit_statement(self, statement, span)
    }

    fn visit_labeled_statement(
        &mut self,
        labeled_statement: &'ast LabeledStatement,
        span: &'ast Span,
    ) {
        visit_labeled_statement(self, labeled_statement, span)
    }

    fn visit_if_statement(&mut self, if_statement: &'ast IfStatement, span: &'ast Span) {
        visit_if_statement(self, if_statement, span)
    }

    fn visit_switch_statement(
        &mut self,
        switch_statement: &'ast SwitchStatement,
        span: &'ast Span,
    ) {
        visit_switch_statement(self, switch_statement, span)
    }

    fn visit_while_statement(&mut self, while_statement: &'ast WhileStatement, span: &'ast Span) {
        visit_while_statement(self, while_statement, span)
    }

    fn visit_do_while_statement(
        &mut self,
        do_while_statement: &'ast DoWhileStatement,
        span: &'ast Span,
    ) {
        visit_do_while_statement(self, do_while_statement, span)
    }

    fn visit_for_statement(&mut self, for_statement: &'ast ForStatement, span: &'ast Span) {
        visit_for_statement(self, for_statement, span)
    }

    fn visit_label(&mut self, label: &'ast Label, span: &'ast Span) {
        visit_label(self, label, span)
    }

    fn visit_case_range(&mut self, range: &'ast CaseRange, span: &'ast Span) {
        visit_case_range(self, range, span)
    }

    fn visit_for_initializer(&mut self, for_initializer: &'ast ForInitializer, span: &'ast Span) {
        visit_for_initializer(self, for_initializer, span)
    }

    fn visit_block_item(&mut self, block_item: &'ast BlockItem, span: &'ast Span) {
        visit_block_item(self, block_item, span)
    }

    fn visit_translation_unit(&mut self, translation_unit: &'ast TranslationUnit) {
        visit_translation_unit(self, translation_unit)
    }

    fn visit_external_declaration(
        &mut self,
        external_declaration: &'ast ExternalDeclaration,
        span: &'ast Span,
    ) {
        visit_external_declaration(self, external_declaration, span)
    }

    fn visit_function_definition(
        &mut self,
        function_definition: &'ast FunctionDefinition,
        span: &'ast Span,
    ) {
        visit_function_definition(self, function_definition, span)
    }

    fn visit_extension(&mut self, extension: &'ast Extension, span: &'ast Span) {
        visit_extension(self, extension, span)
    }

    fn visit_attribute(&mut self, attribute: &'ast Attribute, span: &'ast Span) {
        visit_attribute(self, attribute, span)
    }

    fn visit_asm_statement(&mut self, asm_statement: &'ast AsmStatement, span: &'ast Span) {
        visit_asm_statement(self, asm_statement, span)
    }

    fn visit_availability_attribute(
        &mut self,
        availability: &'ast AvailabilityAttribute,
        span: &'ast Span,
    ) {
        visit_availability_attribute(self, availability, span)
    }

    fn visit_availability_clause(&mut self, _clause: &'ast AvailabilityClause, _span: &'ast Span) {}

    fn visit_gnu_extended_asm_statement(
        &mut self,
        gnu_extended_asm_statement: &'ast GnuExtendedAsmStatement,
        span: &'ast Span,
    ) {
        visit_gnu_extended_asm_statement(self, gnu_extended_asm_statement, span)
    }

    fn visit_gnu_asm_operand(&mut self, gnu_asm_operand: &'ast GnuAsmOperand, span: &'ast Span) {
        visit_gnu_asm_operand(self, gnu_asm_operand, span)
    }

    fn visit_type_of(&mut self, type_of: &'ast TypeOf, span: &'ast Span) {
        visit_type_of(self, type_of, span)
    }
}

pub fn visit_identifier<'ast, V: Visit<'ast> + ?Sized>(
    _visitor: &mut V,
    _identifier: &'ast Identifier,
    _span: &'ast Span,
) {
}

pub fn visit_constant<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    constant: &'ast Constant,
    span: &'ast Span,
) {
    match *constant {
        Constant::Integer(ref i) => visitor.visit_integer(i, span),
        Constant::Float(ref f) => visitor.visit_float(f, span),
        Constant::Character(_) => {}
    }
}

pub fn visit_integer<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    integer: &'ast Integer,
    span: &'ast Span,
) {
    visitor.visit_integer_base(&integer.base, span);
    visitor.visit_integer_suffix(&integer.suffix, span);
}

pub fn visit_integer_base<'ast, V: Visit<'ast> + ?Sized>(
    _visitor: &mut V,
    _integer_base: &'ast IntegerBase,
    _span: &'ast Span,
) {
}

pub fn visit_integer_suffix<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    integer_suffix: &'ast IntegerSuffix,
    span: &'ast Span,
) {
    visitor.visit_integer_size(&integer_suffix.size, span);
}

pub fn visit_integer_size<'ast, V: Visit<'ast> + ?Sized>(
    _visitor: &mut V,
    _integer_size: &'ast IntegerSize,
    _span: &'ast Span,
) {
}

pub fn visit_float<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    float: &'ast Float,
    span: &'ast Span,
) {
    visitor.visit_float_base(&float.base, span);
    visitor.visit_float_suffix(&float.suffix, span);
}

pub fn visit_float_base<'ast, V: Visit<'ast> + ?Sized>(
    _visitor: &mut V,
    _float_base: &'ast FloatBase,
    _span: &'ast Span,
) {
}

pub fn visit_float_suffix<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    float_suffix: &'ast FloatSuffix,
    span: &'ast Span,
) {
    visitor.visit_float_format(&float_suffix.format, span);
}

pub fn visit_float_format<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    float_format: &'ast FloatFormat,
    span: &'ast Span,
) {
    match *float_format {
        FloatFormat::TS18661Format(ref f) => visitor.visit_ts18661_float_type(f, span),
        _ => {}
    }
}

pub fn visit_string_literal<'ast, V: Visit<'ast> + ?Sized>(
    _visitor: &mut V,
    _string_literal: &'ast StringLiteral,
    _span: &'ast Span,
) {
}

pub fn visit_expression<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    expression: &'ast Expression,
    _span: &'ast Span,
) {
    match *expression {
        Expression::Identifier(ref i) => visitor.visit_identifier(&i.node, &i.span),
        Expression::Constant(ref c) => visitor.visit_constant(&c.node, &c.span),
        Expression::StringLiteral(ref s) => visitor.visit_string_literal(&s.node, &s.span),
        Expression::GenericSelection(ref g) => visitor.visit_generic_selection(&g.node, &g.span),
        Expression::Member(ref m) => visitor.visit_member_expression(&m.node, &m.span),
        Expression::Call(ref c) => visitor.visit_call_expression(&c.node, &c.span),
        Expression::CompoundLiteral(ref c) => visitor.visit_compound_literal(&c.node, &c.span),
        Expression::SizeOfTy(ref s) => visitor.visit_sizeofty(&s.node, &s.span),
        Expression::SizeOfVal(ref s) => visitor.visit_sizeofval(&s.node, &s.span),
        Expression::AlignOf(ref a) => visitor.visit_alignof(&a.node, &a.span),
        Expression::UnaryOperator(ref u) => {
            visitor.visit_unary_operator_expression(&u.node, &u.span)
        }
        Expression::Cast(ref c) => visitor.visit_cast_expression(&c.node, &c.span),
        Expression::BinaryOperator(ref b) => {
            visitor.visit_binary_operator_expression(&b.node, &b.span)
        }
        Expression::Conditional(ref c) => visitor.visit_conditional_expression(&c.node, &c.span),
        Expression::Comma(ref comma) => {
            for c in comma.iter() {
                visitor.visit_expression(&c.node, &c.span);
            }
        }
        Expression::OffsetOf(ref o) => visitor.visit_offset_of_expression(&o.node, &o.span),
        Expression::VaArg(ref v) => visitor.visit_va_arg_expression(&v.node, &v.span),
        Expression::Statement(ref s) => visitor.visit_statement(&s.node, &s.span),
    }
}

pub fn visit_member_operator<'ast, V: Visit<'ast> + ?Sized>(
    _visitor: &mut V,
    _member_operator: &'ast MemberOperator,
    _span: &'ast Span,
) {
}

pub fn visit_generic_selection<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    generic_selection: &'ast GenericSelection,
    _span: &'ast Span,
) {
    visitor.visit_expression(
        &generic_selection.expression.node,
        &generic_selection.expression.span,
    );
    for association in &generic_selection.associations {
        visitor.visit_generic_association(&association.node, &association.span);
    }
}

pub fn visit_generic_association<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    generic_association: &'ast GenericAssociation,
    _span: &'ast Span,
) {
    match *generic_association {
        GenericAssociation::Type(ref t) => visitor.visit_generic_association_type(&t.node, &t.span),
        GenericAssociation::Default(ref d) => visitor.visit_expression(&d.node, &d.span),
    }
}

pub fn visit_generic_association_type<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    generic_association_type: &'ast GenericAssociationType,
    _span: &'ast Span,
) {
    visitor.visit_type_name(
        &generic_association_type.type_name.node,
        &generic_association_type.type_name.span,
    );
    visitor.visit_expression(
        &generic_association_type.expression.node,
        &generic_association_type.expression.span,
    );
}

pub fn visit_member_expression<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    member_expression: &'ast MemberExpression,
    _span: &'ast Span,
) {
    visitor.visit_member_operator(
        &member_expression.operator.node,
        &member_expression.operator.span,
    );
    visitor.visit_expression(
        &member_expression.expression.node,
        &member_expression.expression.span,
    );
    visitor.visit_identifier(
        &member_expression.identifier.node,
        &member_expression.identifier.span,
    );
}

pub fn visit_call_expression<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    call_expression: &'ast CallExpression,
    _span: &'ast Span,
) {
    visitor.visit_expression(&call_expression.callee.node, &call_expression.callee.span);
    for argument in &call_expression.arguments {
        visitor.visit_expression(&argument.node, &argument.span);
    }
}

pub fn visit_compound_literal<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    compound_literal: &'ast CompoundLiteral,
    _span: &'ast Span,
) {
    visitor.visit_type_name(
        &compound_literal.type_name.node,
        &compound_literal.type_name.span,
    );
    for initializer in &compound_literal.initializer_list {
        visitor.visit_initializer_list_item(&initializer.node, &initializer.span);
    }
}

pub fn visit_sizeofty<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    sizeofty: &'ast SizeOfTy,
    _span: &'ast Span,
) {
    visitor.visit_type_name(&sizeofty.0.node, &sizeofty.0.span);
}

pub fn visit_sizeofval<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    sizeofval: &'ast SizeOfVal,
    _span: &'ast Span,
) {
    visitor.visit_expression(&sizeofval.0.node, &sizeofval.0.span);
}

pub fn visit_alignof<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    alignofty: &'ast AlignOf,
    _span: &'ast Span,
) {
    visitor.visit_type_name(&alignofty.0.node, &alignofty.0.span);
}

pub fn visit_unary_operator<'ast, V: Visit<'ast> + ?Sized>(
    _visitor: &mut V,
    _unary_operator: &'ast UnaryOperator,
    _span: &'ast Span,
) {
}

pub fn visit_unary_operator_expression<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    unary_operator_expression: &'ast UnaryOperatorExpression,
    _span: &'ast Span,
) {
    match unary_operator_expression.operator.node {
        UnaryOperator::PostIncrement | UnaryOperator::PostDecrement => {
            visitor.visit_expression(
                &unary_operator_expression.operand.node,
                &unary_operator_expression.operand.span,
            );
            visitor.visit_unary_operator(
                &unary_operator_expression.operator.node,
                &unary_operator_expression.operator.span,
            );
        }
        _ => {
            visitor.visit_unary_operator(
                &unary_operator_expression.operator.node,
                &unary_operator_expression.operator.span,
            );
            visitor.visit_expression(
                &unary_operator_expression.operand.node,
                &unary_operator_expression.operand.span,
            );
        }
    }
}

pub fn visit_cast_expression<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    cast_expression: &'ast CastExpression,
    _span: &'ast Span,
) {
    visitor.visit_type_name(
        &cast_expression.type_name.node,
        &cast_expression.type_name.span,
    );
    visitor.visit_expression(
        &cast_expression.expression.node,
        &cast_expression.expression.span,
    );
}

pub fn visit_binary_operator<'ast, V: Visit<'ast> + ?Sized>(
    _visitor: &mut V,
    _binary_operator: &'ast BinaryOperator,
    _span: &'ast Span,
) {
}

pub fn visit_binary_operator_expression<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    binary_operator_expression: &'ast BinaryOperatorExpression,
    _span: &'ast Span,
) {
    visitor.visit_expression(
        &binary_operator_expression.lhs.node,
        &binary_operator_expression.lhs.span,
    );
    visitor.visit_expression(
        &binary_operator_expression.rhs.node,
        &binary_operator_expression.rhs.span,
    );
    visitor.visit_binary_operator(
        &binary_operator_expression.operator.node,
        &binary_operator_expression.operator.span,
    );
}

pub fn visit_conditional_expression<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    conditional_expression: &'ast ConditionalExpression,
    _span: &'ast Span,
) {
    visitor.visit_expression(
        &conditional_expression.condition.node,
        &conditional_expression.condition.span,
    );
    visitor.visit_expression(
        &conditional_expression.then_expression.node,
        &conditional_expression.then_expression.span,
    );
    visitor.visit_expression(
        &conditional_expression.else_expression.node,
        &conditional_expression.else_expression.span,
    );
}

pub fn visit_va_arg_expression<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    va_arg_expression: &'ast VaArgExpression,
    _span: &'ast Span,
) {
    visitor.visit_expression(
        &va_arg_expression.va_list.node,
        &va_arg_expression.va_list.span,
    );
    visitor.visit_type_name(
        &va_arg_expression.type_name.node,
        &va_arg_expression.type_name.span,
    );
}

pub fn visit_offset_of_expression<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    offset_of_expression: &'ast OffsetOfExpression,
    _span: &'ast Span,
) {
    visitor.visit_type_name(
        &offset_of_expression.type_name.node,
        &offset_of_expression.type_name.span,
    );
    visitor.visit_offset_designator(
        &offset_of_expression.designator.node,
        &offset_of_expression.designator.span,
    );
}

pub fn visit_offset_designator<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    offset_designator: &'ast OffsetDesignator,
    _span: &'ast Span,
) {
    visitor.visit_identifier(&offset_designator.base.node, &offset_designator.base.span);
    for member in &offset_designator.members {
        visitor.visit_offset_member(&member.node, &member.span);
    }
}

pub fn visit_offset_member<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    offset_member: &'ast OffsetMember,
    _span: &'ast Span,
) {
    match *offset_member {
        OffsetMember::Member(ref m) => visitor.visit_identifier(&m.node, &m.span),
        OffsetMember::IndirectMember(ref m) => visitor.visit_identifier(&m.node, &m.span),
        OffsetMember::Index(ref i) => visitor.visit_expression(&i.node, &i.span),
    }
}

pub fn visit_declaration<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    declaration: &'ast Declaration,
    _span: &'ast Span,
) {
    for specifier in &declaration.specifiers {
        visitor.visit_declaration_specifier(&specifier.node, &specifier.span);
    }

    for declarator in &declaration.declarators {
        visitor.visit_init_declarator(&declarator.node, &declarator.span);
    }
}

pub fn visit_declaration_specifier<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    declaration_specifier: &'ast DeclarationSpecifier,
    _span: &'ast Span,
) {
    match *declaration_specifier {
        DeclarationSpecifier::StorageClass(ref s) => {
            visitor.visit_storage_class_specifier(&s.node, &s.span)
        }
        DeclarationSpecifier::TypeSpecifier(ref t) => {
            visitor.visit_type_specifier(&t.node, &t.span)
        }
        DeclarationSpecifier::TypeQualifier(ref t) => {
            visitor.visit_type_qualifier(&t.node, &t.span)
        }
        DeclarationSpecifier::Function(ref f) => visitor.visit_function_specifier(&f.node, &f.span),
        DeclarationSpecifier::Alignment(ref a) => {
            visitor.visit_alignment_specifier(&a.node, &a.span)
        }
        DeclarationSpecifier::Extension(ref e) => {
            for extension in e {
                visitor.visit_extension(&extension.node, &extension.span);
            }
        }
    }
}

pub fn visit_init_declarator<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    init_declarator: &'ast InitDeclarator,
    _span: &'ast Span,
) {
    visitor.visit_declarator(
        &init_declarator.declarator.node,
        &init_declarator.declarator.span,
    );
    if let Some(ref initializer) = init_declarator.initializer {
        visitor.visit_initializer(&initializer.node, &initializer.span);
    }
}

pub fn visit_storage_class_specifier<'ast, V: Visit<'ast> + ?Sized>(
    _visitor: &mut V,
    _storage_class_specifier: &'ast StorageClassSpecifier,
    _span: &'ast Span,
) {
}

pub fn visit_type_specifier<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    type_specifier: &'ast TypeSpecifier,
    span: &'ast Span,
) {
    match *type_specifier {
        TypeSpecifier::Atomic(ref a) => visitor.visit_type_name(&a.node, &a.span),
        TypeSpecifier::Struct(ref s) => visitor.visit_struct_type(&s.node, &s.span),
        TypeSpecifier::Enum(ref e) => visitor.visit_enum_type(&e.node, &e.span),
        TypeSpecifier::TypedefName(ref t) => visitor.visit_identifier(&t.node, &t.span),
        TypeSpecifier::TypeOf(ref t) => visitor.visit_type_of(&t.node, &t.span),
        TypeSpecifier::TS18661Float(ref t) => visitor.visit_ts18661_float_type(t, span),
        _ => {}
    }
}

pub fn visit_ts18661_float_type<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    ts18661_float_type: &'ast TS18661FloatType,
    span: &'ast Span,
) {
    visitor.visit_ts18661_float_format(&ts18661_float_type.format, span);
}

pub fn visit_ts18661_float_format<'ast, V: Visit<'ast> + ?Sized>(
    _visitor: &mut V,
    _ts18661_float_format: &'ast TS18661FloatFormat,
    _span: &'ast Span,
) {
}

pub fn visit_struct_type<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    struct_type: &'ast StructType,
    _span: &'ast Span,
) {
    visitor.visit_struct_kind(&struct_type.kind.node, &struct_type.kind.span);
    if let Some(ref identifier) = struct_type.identifier {
        visitor.visit_identifier(&identifier.node, &identifier.span);
    }
    if let Some(ref declarations) = struct_type.declarations {
        for declaration in declarations {
            visitor.visit_struct_declaration(&declaration.node, &declaration.span);
        }
    }
}

pub fn visit_struct_kind<'ast, V: Visit<'ast> + ?Sized>(
    _visitor: &mut V,
    _struct_kind: &'ast StructKind,
    _span: &'ast Span,
) {
}

pub fn visit_struct_declaration<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    struct_declaration: &'ast StructDeclaration,
    _span: &'ast Span,
) {
    match *struct_declaration {
        StructDeclaration::Field(ref f) => visitor.visit_struct_field(&f.node, &f.span),
        StructDeclaration::StaticAssert(ref s) => visitor.visit_static_assert(&s.node, &s.span),
    }
}

pub fn visit_struct_field<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    struct_field: &'ast StructField,
    _span: &'ast Span,
) {
    for specifier in &struct_field.specifiers {
        visitor.visit_specifier_qualifier(&specifier.node, &specifier.span);
    }
    for declarator in &struct_field.declarators {
        visitor.visit_struct_declarator(&declarator.node, &declarator.span);
    }
}

pub fn visit_specifier_qualifier<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    specifier_qualifier: &'ast SpecifierQualifier,
    _span: &'ast Span,
) {
    match *specifier_qualifier {
        SpecifierQualifier::TypeSpecifier(ref t) => visitor.visit_type_specifier(&t.node, &t.span),
        SpecifierQualifier::TypeQualifier(ref t) => visitor.visit_type_qualifier(&t.node, &t.span),
        SpecifierQualifier::Extension(ref e) => {
            for n in e {
                visitor.visit_extension(&n.node, &n.span);
            }
        }
    }
}

pub fn visit_struct_declarator<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    struct_declarator: &'ast StructDeclarator,
    _span: &'ast Span,
) {
    if let Some(ref declarator) = struct_declarator.declarator {
        visitor.visit_declarator(&declarator.node, &declarator.span);
    }
    if let Some(ref bit_width) = struct_declarator.bit_width {
        visitor.visit_expression(&bit_width.node, &bit_width.span);
    }
}

pub fn visit_enum_type<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    enum_type: &'ast EnumType,
    _span: &'ast Span,
) {
    if let Some(ref identifier) = enum_type.identifier {
        visitor.visit_identifier(&identifier.node, &identifier.span);
    }
    for enumerator in &enum_type.enumerators {
        visitor.visit_enumerator(&enumerator.node, &enumerator.span);
    }
}

pub fn visit_enumerator<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    enumerator: &'ast Enumerator,
    _span: &'ast Span,
) {
    visitor.visit_identifier(&enumerator.identifier.node, &enumerator.identifier.span);
    if let Some(ref expression) = enumerator.expression {
        visitor.visit_expression(&expression.node, &expression.span);
    }
    for extension in &enumerator.extensions {
        visitor.visit_extension(&extension.node, &extension.span);
    }
}

pub fn visit_type_qualifier<'ast, V: Visit<'ast> + ?Sized>(
    _visitor: &mut V,
    _type_qualifier: &'ast TypeQualifier,
    _span: &'ast Span,
) {
}

pub fn visit_function_specifier<'ast, V: Visit<'ast> + ?Sized>(
    _visitor: &mut V,
    _function_specifier: &'ast FunctionSpecifier,
    _span: &'ast Span,
) {
}

pub fn visit_alignment_specifier<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    alignment_specifier: &'ast AlignmentSpecifier,
    _span: &'ast Span,
) {
    match *alignment_specifier {
        AlignmentSpecifier::Type(ref t) => visitor.visit_type_name(&t.node, &t.span),
        AlignmentSpecifier::Constant(ref c) => visitor.visit_expression(&c.node, &c.span),
    }
}

pub fn visit_declarator<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    declarator: &'ast Declarator,
    _span: &'ast Span,
) {
    visitor.visit_declarator_kind(&declarator.kind.node, &declarator.kind.span);
    for derived in &declarator.derived {
        visitor.visit_derived_declarator(&derived.node, &derived.span);
    }
    for extension in &declarator.extensions {
        visitor.visit_extension(&extension.node, &extension.span);
    }
}

pub fn visit_declarator_kind<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    declarator_kind: &'ast DeclaratorKind,
    _span: &'ast Span,
) {
    match *declarator_kind {
        DeclaratorKind::Identifier(ref i) => visitor.visit_identifier(&i.node, &i.span),
        DeclaratorKind::Declarator(ref d) => visitor.visit_declarator(&d.node, &d.span),
        _ => {}
    }
}

pub fn visit_derived_declarator<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    derived_declarator: &'ast DerivedDeclarator,
    _span: &'ast Span,
) {
    match *derived_declarator {
        DerivedDeclarator::Pointer(ref p) => {
            for pointer in p {
                visitor.visit_pointer_qualifier(&pointer.node, &pointer.span);
            }
        }
        DerivedDeclarator::Array(ref a) => visitor.visit_array_declarator(&a.node, &a.span),
        DerivedDeclarator::Function(ref f) => visitor.visit_function_declarator(&f.node, &f.span),
        DerivedDeclarator::KRFunction(ref k) => {
            for identifier in k {
                visitor.visit_identifier(&identifier.node, &identifier.span);
            }
        }
        DerivedDeclarator::Block(ref qs) => {
            for q in qs {
                visitor.visit_pointer_qualifier(&q.node, &q.span);
            }
        }
    }
}

pub fn visit_array_declarator<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    array_declarator: &'ast ArrayDeclarator,
    span: &'ast Span,
) {
    for qualifier in &array_declarator.qualifiers {
        visitor.visit_type_qualifier(&qualifier.node, &qualifier.span);
    }
    visitor.visit_array_size(&array_declarator.size, span)
}

pub fn visit_function_declarator<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    function_declarator: &'ast FunctionDeclarator,
    span: &'ast Span,
) {
    for parameter in &function_declarator.parameters {
        visitor.visit_parameter_declaration(&parameter.node, &parameter.span);
    }
    visitor.visit_ellipsis(&function_declarator.ellipsis, span);
}

pub fn visit_pointer_qualifier<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    pointer_qualifier: &'ast PointerQualifier,
    _span: &'ast Span,
) {
    match *pointer_qualifier {
        PointerQualifier::TypeQualifier(ref t) => visitor.visit_type_qualifier(&t.node, &t.span),
        PointerQualifier::Extension(ref e) => {
            for extension in e {
                visitor.visit_extension(&extension.node, &extension.span);
            }
        }
    }
}

pub fn visit_array_size<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    array_size: &'ast ArraySize,
    _span: &'ast Span,
) {
    match *array_size {
        ArraySize::VariableExpression(ref e) => visitor.visit_expression(&e.node, &e.span),
        ArraySize::StaticExpression(ref s) => visitor.visit_expression(&s.node, &s.span),
        _ => {}
    }
}

pub fn visit_parameter_declaration<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    parameter_declaration: &'ast ParameterDeclaration,
    _span: &'ast Span,
) {
    for specifier in &parameter_declaration.specifiers {
        visitor.visit_declaration_specifier(&specifier.node, &specifier.span);
    }
    if let Some(ref declarator) = parameter_declaration.declarator {
        visitor.visit_declarator(&declarator.node, &declarator.span);
    }
    for extension in &parameter_declaration.extensions {
        visitor.visit_extension(&extension.node, &extension.span);
    }
}

pub fn visit_ellipsis<'ast, V: Visit<'ast> + ?Sized>(
    _visitor: &mut V,
    _ellipsis: &'ast Ellipsis,
    _span: &'ast Span,
) {
}

pub fn visit_type_name<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    type_name: &'ast TypeName,
    _span: &'ast Span,
) {
    for specifier in &type_name.specifiers {
        visitor.visit_specifier_qualifier(&specifier.node, &specifier.span);
    }
    if let Some(ref declarator) = type_name.declarator {
        visitor.visit_declarator(&declarator.node, &declarator.span);
    }
}

pub fn visit_initializer<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    initializer: &'ast Initializer,
    _span: &'ast Span,
) {
    match *initializer {
        Initializer::Expression(ref e) => visitor.visit_expression(&e.node, &e.span),
        Initializer::List(ref l) => {
            for item in l {
                visitor.visit_initializer_list_item(&item.node, &item.span);
            }
        }
    }
}

pub fn visit_initializer_list_item<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    initializer_list_item: &'ast InitializerListItem,
    _span: &'ast Span,
) {
    for designation in &initializer_list_item.designation {
        visitor.visit_designator(&designation.node, &designation.span);
    }
    visitor.visit_initializer(
        &initializer_list_item.initializer.node,
        &initializer_list_item.initializer.span,
    );
}

pub fn visit_designator<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    designator: &'ast Designator,
    _span: &'ast Span,
) {
    match *designator {
        Designator::Index(ref i) => visitor.visit_expression(&i.node, &i.span),
        Designator::Member(ref m) => visitor.visit_identifier(&m.node, &m.span),
        Designator::Range(ref r) => visitor.visit_range_designator(&r.node, &r.span),
    }
}

pub fn visit_range_designator<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    range_designator: &'ast RangeDesignator,
    _span: &'ast Span,
) {
    visitor.visit_expression(&range_designator.from.node, &range_designator.from.span);
    visitor.visit_expression(&range_designator.to.node, &range_designator.to.span);
}

pub fn visit_static_assert<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    static_assert: &'ast StaticAssert,
    _span: &'ast Span,
) {
    visitor.visit_expression(
        &static_assert.expression.node,
        &static_assert.expression.span,
    );
    visitor.visit_string_literal(&static_assert.message.node, &static_assert.message.span);
}

pub fn visit_statement<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    statement: &'ast Statement,
    _span: &'ast Span,
) {
    match *statement {
        Statement::Labeled(ref l) => visitor.visit_labeled_statement(&l.node, &l.span),
        Statement::Compound(ref c) => {
            for item in c {
                visitor.visit_block_item(&item.node, &item.span);
            }
        }
        Statement::Expression(Some(ref e)) => {
            visitor.visit_expression(&e.node, &e.span);
        }
        Statement::If(ref i) => visitor.visit_if_statement(&i.node, &i.span),
        Statement::Switch(ref s) => visitor.visit_switch_statement(&s.node, &s.span),
        Statement::While(ref w) => visitor.visit_while_statement(&w.node, &w.span),
        Statement::DoWhile(ref d) => visitor.visit_do_while_statement(&d.node, &d.span),
        Statement::For(ref f) => visitor.visit_for_statement(&f.node, &f.span),
        Statement::Goto(ref g) => visitor.visit_identifier(&g.node, &g.span),
        Statement::Return(Some(ref r)) => {
            visitor.visit_expression(&r.node, &r.span);
        }
        Statement::Asm(ref a) => visitor.visit_asm_statement(&a.node, &a.span),
        _ => {}
    }
}

pub fn visit_labeled_statement<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    labeled_statement: &'ast LabeledStatement,
    _span: &'ast Span,
) {
    visitor.visit_label(&labeled_statement.label.node, &labeled_statement.label.span);
    visitor.visit_statement(
        &labeled_statement.statement.node,
        &labeled_statement.statement.span,
    );
}

pub fn visit_if_statement<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    if_statement: &'ast IfStatement,
    _span: &'ast Span,
) {
    visitor.visit_expression(&if_statement.condition.node, &if_statement.condition.span);
    visitor.visit_statement(
        &if_statement.then_statement.node,
        &if_statement.then_statement.span,
    );
    if let Some(ref e) = if_statement.else_statement {
        visitor.visit_statement(&e.node, &e.span);
    }
}

pub fn visit_switch_statement<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    switch_statement: &'ast SwitchStatement,
    _span: &'ast Span,
) {
    visitor.visit_expression(
        &switch_statement.expression.node,
        &switch_statement.expression.span,
    );
    visitor.visit_statement(
        &switch_statement.statement.node,
        &switch_statement.statement.span,
    );
}

pub fn visit_while_statement<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    while_statement: &'ast WhileStatement,
    _span: &'ast Span,
) {
    visitor.visit_expression(
        &while_statement.expression.node,
        &while_statement.expression.span,
    );
    visitor.visit_statement(
        &while_statement.statement.node,
        &while_statement.statement.span,
    );
}

pub fn visit_do_while_statement<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    do_while_statement: &'ast DoWhileStatement,
    _span: &'ast Span,
) {
    visitor.visit_statement(
        &do_while_statement.statement.node,
        &do_while_statement.statement.span,
    );
    visitor.visit_expression(
        &do_while_statement.expression.node,
        &do_while_statement.expression.span,
    );
}

pub fn visit_for_statement<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    for_statement: &'ast ForStatement,
    _span: &'ast Span,
) {
    visitor.visit_for_initializer(
        &for_statement.initializer.node,
        &for_statement.initializer.span,
    );
    if let Some(ref c) = for_statement.condition {
        visitor.visit_expression(&c.node, &c.span);
    }
    if let Some(ref s) = for_statement.step {
        visitor.visit_expression(&s.node, &s.span);
    }
    visitor.visit_statement(&for_statement.statement.node, &for_statement.statement.span);
}

pub fn visit_label<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    label: &'ast Label,
    _span: &'ast Span,
) {
    match *label {
        Label::Identifier(ref i) => visitor.visit_identifier(&i.node, &i.span),
        Label::Case(ref c) => visitor.visit_expression(&c.node, &c.span),
        Label::CaseRange(ref c) => visitor.visit_case_range(&c.node, &c.span),
        Label::Default => {}
    }
}

pub fn visit_case_range<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    range: &'ast CaseRange,
    _span: &'ast Span,
) {
    visitor.visit_expression(&range.low.node, &range.low.span);
    visitor.visit_expression(&range.high.node, &range.high.span);
}

pub fn visit_for_initializer<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    for_initializer: &'ast ForInitializer,
    _span: &'ast Span,
) {
    match *for_initializer {
        ForInitializer::Empty => {}
        ForInitializer::Expression(ref e) => visitor.visit_expression(&e.node, &e.span),
        ForInitializer::Declaration(ref d) => visitor.visit_declaration(&d.node, &d.span),
        ForInitializer::StaticAssert(ref s) => visitor.visit_static_assert(&s.node, &s.span),
    }
}

pub fn visit_block_item<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    block_item: &'ast BlockItem,
    _span: &'ast Span,
) {
    match *block_item {
        BlockItem::Declaration(ref d) => visitor.visit_declaration(&d.node, &d.span),
        BlockItem::StaticAssert(ref s) => visitor.visit_static_assert(&s.node, &s.span),
        BlockItem::Statement(ref s) => visitor.visit_statement(&s.node, &s.span),
    }
}

pub fn visit_translation_unit<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    translation_unit: &'ast TranslationUnit,
) {
    for element in &translation_unit.0 {
        visitor.visit_external_declaration(&element.node, &element.span);
    }
}

pub fn visit_external_declaration<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    external_declaration: &'ast ExternalDeclaration,
    _span: &'ast Span,
) {
    match *external_declaration {
        ExternalDeclaration::Declaration(ref d) => visitor.visit_declaration(&d.node, &d.span),
        ExternalDeclaration::StaticAssert(ref s) => visitor.visit_static_assert(&s.node, &s.span),
        ExternalDeclaration::FunctionDefinition(ref f) => {
            visitor.visit_function_definition(&f.node, &f.span)
        }
    }
}

pub fn visit_function_definition<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    function_definition: &'ast FunctionDefinition,
    _span: &'ast Span,
) {
    for specifier in &function_definition.specifiers {
        visitor.visit_declaration_specifier(&specifier.node, &specifier.span);
    }
    visitor.visit_declarator(
        &function_definition.declarator.node,
        &function_definition.declarator.span,
    );
    for declaration in &function_definition.declarations {
        visitor.visit_declaration(&declaration.node, &declaration.span);
    }
    visitor.visit_statement(
        &function_definition.statement.node,
        &function_definition.statement.span,
    );
}

pub fn visit_extension<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    extension: &'ast Extension,
    span: &'ast Span,
) {
    match *extension {
        Extension::Attribute(ref a) => visitor.visit_attribute(a, span),
        Extension::AsmLabel(ref a) => visitor.visit_string_literal(&a.node, &a.span),
        Extension::AvailabilityAttribute(ref a) => {
            visitor.visit_availability_attribute(&a.node, &a.span)
        }
    }
}

pub fn visit_attribute<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    attribute: &'ast Attribute,
    _span: &'ast Span,
) {
    for argument in &attribute.arguments {
        visitor.visit_expression(&argument.node, &argument.span);
    }
}

pub fn visit_asm_statement<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    asm_statement: &'ast AsmStatement,
    span: &'ast Span,
) {
    match *asm_statement {
        AsmStatement::GnuBasic(ref g) => visitor.visit_string_literal(&g.node, &g.span),
        AsmStatement::GnuExtended(ref g) => visitor.visit_gnu_extended_asm_statement(g, span),
    }
}

pub fn visit_availability_attribute<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    availability: &'ast AvailabilityAttribute,
    _span: &'ast Span,
) {
    for clause in &availability.clauses {
        visitor.visit_availability_clause(&clause.node, &clause.span);
    }
}

pub fn visit_gnu_extended_asm_statement<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    gnu_extended_asm_statement: &'ast GnuExtendedAsmStatement,
    _span: &'ast Span,
) {
    if let Some(ref qualifier) = gnu_extended_asm_statement.qualifier {
        visitor.visit_type_qualifier(&qualifier.node, &qualifier.span);
    }
    visitor.visit_string_literal(
        &gnu_extended_asm_statement.template.node,
        &gnu_extended_asm_statement.template.span,
    );
    for output in &gnu_extended_asm_statement.outputs {
        visitor.visit_gnu_asm_operand(&output.node, &output.span);
    }
    for input in &gnu_extended_asm_statement.inputs {
        visitor.visit_gnu_asm_operand(&input.node, &input.span);
    }
    for clobber in &gnu_extended_asm_statement.clobbers {
        visitor.visit_string_literal(&clobber.node, &clobber.span);
    }
}

pub fn visit_gnu_asm_operand<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    gnu_asm_operand: &'ast GnuAsmOperand,
    _span: &'ast Span,
) {
    if let Some(ref name) = gnu_asm_operand.symbolic_name {
        visitor.visit_identifier(&name.node, &name.span);
    }
    visitor.visit_string_literal(
        &gnu_asm_operand.constraints.node,
        &gnu_asm_operand.constraints.span,
    );
    visitor.visit_expression(
        &gnu_asm_operand.variable_name.node,
        &gnu_asm_operand.variable_name.span,
    );
}

pub fn visit_type_of<'ast, V: Visit<'ast> + ?Sized>(
    visitor: &mut V,
    type_of: &'ast TypeOf,
    _span: &'ast Span,
) {
    match *type_of {
        TypeOf::Expression(ref e) => visitor.visit_expression(&e.node, &e.span),
        TypeOf::Type(ref t) => visitor.visit_type_name(&t.node, &t.span),
    }
}
